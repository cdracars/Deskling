// SPDX-License-Identifier: MIT
#include "pack.hpp"
#include <QBuffer>
#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QImage>
#include <QImageReader>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonParseError>
#include <QMap>
#include <QRegularExpression>
#include <QSet>
#include <QUrl>
#include <cmath>

namespace {
constexpr qint64 ManifestLimit = 64 * 1024;
constexpr qint64 ImageLimit = 16 * 1024 * 1024;
constexpr qint64 PixelLimit = 16 * 1024 * 1024;
constexpr int DimensionLimit = 16384;

bool fail(QString &error, const QString &message) { error = message; return false; }
bool keys(const QJsonObject &o, const QSet<QString> &allowed, const QString &where, QString &error) {
    for (auto it = o.begin(); it != o.end(); ++it)
        if (!allowed.contains(it.key()))
            return fail(error, where + ": unsupported field '" + it.key() + "'. Remove it; packs contain data only.");
    return true;
}
bool integer(const QJsonObject &o, const QString &key, int low, int high, int &out, QString &error) {
    const auto v = o.value(key);
    const double n = v.toDouble(-1);
    if (!v.isDouble() || !std::isfinite(n) || std::floor(n) != n || n < low || n > high)
        return fail(error, QString("'%1' must be a whole number from %2 to %3.").arg(key).arg(low).arg(high));
    out = static_cast<int>(n);
    return true;
}
bool textField(const QJsonObject &o, const QString &key, int max, QString &out, QString &error) {
    const auto v = o.value(key);
    out = v.toString();
    if (!v.isString() || out.trimmed().isEmpty() || out.size() > max)
        return fail(error, QString("'%1' must be nonempty text of at most %2 characters.").arg(key).arg(max));
    // Qt Quick Controls can auto-detect rich text. Keep pack strings inert,
    // including labels containing HTML image tags that could load remote assets.
    if (out.contains('<') || out.contains('>'))
        return fail(error, "'" + key + "' must be plain text without < or >; rich-text markup is not allowed.");
    for (const QChar c : out)
        if (c.category() == QChar::Other_Control)
            return fail(error, "'" + key + "' must not contain control characters.");
    return true;
}
bool slug(const QString &s) {
    static const QRegularExpression pattern(QStringLiteral("^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$"));
    return s.size() <= 64 && pattern.match(s).hasMatch();
}
QVariantMap animation(const QString &id, const QString &label, int row, int frames, int interval) {
    return {{"id", id}, {"label", label}, {"row", row}, {"frames", frames}, {"intervalMs", interval}};
}
QVariantMap behavior(int index, int loops, int weight) {
    return {{"animation", index}, {"loops", loops}, {"weight", weight}};
}
QVariantMap builtin() {
    return {{"id", "diagnostic-shape"}, {"name", "Diagnostic shape"}, {"source", ""},
        {"cellWidth", 128}, {"cellHeight", 160},
        {"animations", QVariantList{animation("idle", "Idle", 0, 6, 180),
            animation("wave", "Wave", 1, 4, 150), animation("think", "Think", 2, 6, 180),
            animation("bounce", "Bounce", 3, 5, 130)}},
        {"idleIndex", 0}, {"waveIndex", 1}, {"dragLeftIndex", 0}, {"dragRightIndex", 0},
        {"behaviors", QVariantList{behavior(1, 1, 3), behavior(2, 2, 3), behavior(3, 1, 1)}}};
}
}

bool loadCharacterPack(const QString &manifestPath, QVariantMap &pack, QString &error) {
    pack.clear();
    error.clear();
    if (manifestPath.isEmpty()) { pack = builtin(); return true; }
    const QFileInfo manifestInfo(manifestPath);
    if (!manifestInfo.isFile()) return fail(error, "Choose an existing local JSON manifest file.");
    QFile manifest(manifestInfo.canonicalFilePath());
    if (!manifest.open(QIODevice::ReadOnly)) return fail(error, "Cannot read manifest: " + manifest.errorString());
    if (manifest.size() > ManifestLimit) return fail(error, "Manifest exceeds 64 KiB. Reduce its size.");
    const QByteArray json = manifest.read(ManifestLimit + 1);
    if (manifest.error() != QFileDevice::NoError) return fail(error, "Cannot finish reading manifest: " + manifest.errorString());
    if (json.size() > ManifestLimit) return fail(error, "Manifest exceeds 64 KiB. Reduce its size.");
    QJsonParseError parseError;
    const QJsonDocument document = QJsonDocument::fromJson(json, &parseError);
    if (parseError.error != QJsonParseError::NoError || !document.isObject())
        return fail(error, "Manifest must be a JSON object: " + parseError.errorString());
    const QJsonObject root = document.object();
    if (!keys(root, {"schemaVersion", "id", "name", "spriteSheet", "cellWidth", "cellHeight", "animations",
                     "idle", "interactions", "behaviors", "metadata", "author", "license"}, "Manifest", error)) return false;
    int version, width, height;
    QString id, name, sheetName, idle;
    if (!integer(root, "schemaVersion", 1, 1, version, error) ||
        !integer(root, "cellWidth", 16, 512, width, error) ||
        !integer(root, "cellHeight", 16, 512, height, error) ||
        !textField(root, "id", 64, id, error) || !textField(root, "name", 128, name, error) ||
        !textField(root, "spriteSheet", 255, sheetName, error) || !textField(root, "idle", 64, idle, error)) return false;
    if (!slug(id)) return fail(error, "'id' must be a lowercase slug, for example 'sample-pet'.");
    for (const QString &key : {QString("author"), QString("license")}) {
        QString ignored;
        if (root.contains(key) && !textField(root, key, 512, ignored, error)) return false;
    }
    if (root.contains("metadata")) {
        if (!root.value("metadata").isObject()) return fail(error, "'metadata' must be an object.");
        const auto metadata = root.value("metadata").toObject();
        if (!keys(metadata, {"author", "license"}, "Metadata", error)) return false;
        for (auto it = metadata.begin(); it != metadata.end(); ++it) {
            QString ignored;
            if (!textField(metadata, it.key(), 512, ignored, error)) return false;
        }
    }
    if (sheetName.contains('/') || sheetName.contains('\\') || sheetName.contains(':') ||
        !sheetName.endsWith(".png", Qt::CaseInsensitive) || sheetName == ".png")
        return fail(error, "'spriteSheet' must be a local PNG basename, such as 'sprites.png', without paths or URLs.");
    const QDir directory(QFileInfo(manifest.fileName()).absolutePath());
    const QFileInfo imageInfo(directory.filePath(sheetName));
    const QString imagePath = imageInfo.canonicalFilePath();
    if (!imageInfo.isFile() || imagePath.isEmpty()) return fail(error, "PNG not found beside the manifest: " + sheetName);
    if (QFileInfo(imagePath).absolutePath() != directory.canonicalPath())
        return fail(error, "The PNG resolves outside the manifest directory. Copy the PNG into that directory.");
    QFile imageFile(imagePath);
    if (!imageFile.open(QIODevice::ReadOnly)) return fail(error, "Cannot read PNG: " + imageFile.errorString());
    if (imageFile.size() > ImageLimit) return fail(error, "PNG exceeds 16 MiB. Use a smaller sprite sheet.");
    QByteArray imageBytes = imageFile.read(ImageLimit + 1);
    if (imageFile.error() != QFileDevice::NoError) return fail(error, "Cannot finish reading PNG: " + imageFile.errorString());
    if (imageBytes.size() > ImageLimit) return fail(error, "PNG exceeds 16 MiB. Use a smaller sprite sheet.");
    if (!imageBytes.startsWith(QByteArray::fromHex("89504e470d0a1a0a")))
        return fail(error, "Sprite sheet must contain PNG image data, not merely a .png filename.");
    QBuffer imageBuffer(&imageBytes);
    imageBuffer.open(QIODevice::ReadOnly);
    QImageReader reader(&imageBuffer, "png");
    reader.setAutoTransform(false);
    if (!reader.canRead()) return fail(error, "Cannot read PNG: " + reader.errorString());
    const QSize size = reader.size();
    if (size.width() <= 0 || size.height() <= 0 || size.width() > DimensionLimit ||
        size.height() > DimensionLimit || qint64(size.width()) * size.height() > PixelLimit)
        return fail(error, "PNG must have positive dimensions, at most 16384 per side and 16777216 pixels total.");
    const QImage decoded = reader.read();
    if (decoded.isNull() || decoded.size() != size)
        return fail(error, "PNG could not be decoded completely: " + reader.errorString());
    if (!root.value("animations").isArray()) return fail(error, "'animations' must be an array.");
    const QJsonArray entries = root.value("animations").toArray();
    if (entries.isEmpty() || entries.size() > 32) return fail(error, "Provide between 1 and 32 animations.");
    QVariantList animations;
    QMap<QString, int> indices;
    for (const QJsonValue &entry : entries) {
        if (!entry.isObject()) return fail(error, "Each animation must be an object.");
        const auto a = entry.toObject();
        if (!keys(a, {"id", "label", "row", "frames", "fps"}, "Animation", error)) return false;
        QString animationId, label;
        int row, frames, fps;
        if (!textField(a, "id", 64, animationId, error) || !textField(a, "label", 128, label, error) ||
            !integer(a, "row", 0, 255, row, error) || !integer(a, "frames", 1, 64, frames, error) ||
            !integer(a, "fps", 1, 30, fps, error)) return false;
        if (!slug(animationId) || indices.contains(animationId))
            return fail(error, "Animation IDs must be unique lowercase slugs: " + animationId);
        if (frames * width > size.width() || (row + 1) * height > size.height())
            return fail(error, "Animation '" + animationId + "' extends outside the PNG. Check row, frames and cell dimensions.");
        indices.insert(animationId, static_cast<int>(animations.size()));
        animations.append(animation(animationId, label, row, frames, qRound(1000.0 / fps)));
    }
    if (!indices.contains(idle)) return fail(error, "'idle' must reference an existing animation ID.");
    const int idleIndex = indices.value(idle);
    QVariantMap result{{"id", id}, {"name", name}, {"source", QUrl::fromLocalFile(imagePath).toString()},
        {"cellWidth", width}, {"cellHeight", height}, {"animations", animations}, {"idleIndex", idleIndex}};
    QJsonObject interactions;
    if (root.contains("interactions")) {
        if (!root.value("interactions").isObject()) return fail(error, "'interactions' must be an object.");
        interactions = root.value("interactions").toObject();
        if (!keys(interactions, {"wave", "dragLeft", "dragRight"}, "Interactions", error)) return false;
    }
    for (const QString &key : {QString("wave"), QString("dragLeft"), QString("dragRight")}) {
        int index = idleIndex;
        if (interactions.contains(key)) {
            const auto reference = interactions.value(key);
            if (!reference.isString() || !indices.contains(reference.toString()))
                return fail(error, "Interaction '" + key + "' must reference an existing animation ID.");
            index = indices.value(reference.toString());
        }
        result.insert(key + "Index", index);
    }
    QVariantList behaviors;
    if (root.contains("behaviors")) {
        if (!root.value("behaviors").isArray()) return fail(error, "'behaviors' must be an array.");
        const auto list = root.value("behaviors").toArray();
        if (list.size() > 32) return fail(error, "Use at most 32 behaviors.");
        for (const auto &entry : list) {
            if (!entry.isObject()) return fail(error, "Each behavior must be an object.");
            const auto b = entry.toObject();
            if (!keys(b, {"animation", "loops", "weight"}, "Behavior", error)) return false;
            QString reference;
            int loops, weight;
            if (!textField(b, "animation", 64, reference, error) || !integer(b, "loops", 1, 10, loops, error) ||
                !integer(b, "weight", 1, 100, weight, error)) return false;
            if (!indices.contains(reference)) return fail(error, "Behavior references an unknown animation: " + reference);
            behaviors.append(behavior(indices.value(reference), loops, weight));
        }
    }
    result.insert("behaviors", behaviors);
    pack = result;
    return true;
}
