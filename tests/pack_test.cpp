// SPDX-License-Identifier: MIT
#include "../crates/deskling-desktop/native/pack.hpp"
#include <QCoreApplication>
#include <QDir>
#include <QFile>
#include <QImage>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QList>
#include <QTemporaryDir>
#include <QUrl>
#include <QVariantList>
#include <cstdlib>
#include <iostream>

namespace {
void check(bool ok, const char *message) {
    if (!ok) { std::cerr << "FAIL: " << message << '\n'; std::exit(1); }
}
void write(const QString &path, const QByteArray &bytes) {
    QFile f(path);
    check(f.open(QIODevice::WriteOnly | QIODevice::Truncate), "open fixture");
    check(f.write(bytes) == bytes.size(), "write fixture");
}
QJsonObject valid() {
    return {{"schemaVersion", 1}, {"id", "test-shape"}, {"name", "Test shape"},
        {"spriteSheet", "sheet.png"}, {"cellWidth", 16}, {"cellHeight", 16}, {"idle", "idle"},
        {"animations", QJsonArray{QJsonObject{{"id", "idle"}, {"label", "Idle"}, {"row", 0}, {"frames", 2}, {"fps", 10}},
            QJsonObject{{"id", "wave"}, {"label", "Wave"}, {"row", 1}, {"frames", 2}, {"fps", 20}}}},
        {"interactions", QJsonObject{{"wave", "wave"}}},
        {"behaviors", QJsonArray{QJsonObject{{"animation", "wave"}, {"loops", 2}, {"weight", 3}}}}};
}
}
int main(int argc, char **argv) {
    QCoreApplication app(argc, argv);
    QTemporaryDir dir;
    check(dir.isValid(), "temporary directory");
    const QString manifest = dir.filePath("pack.json");
    QVariantMap pack;
    QString error;
    check(loadCharacterPack({}, pack, error), "builtin loads");
    check(error.isEmpty() && pack.value("source").toString().isEmpty(), "builtin has no artwork");
    check(pack.value("animations").toList().size() == 4 && pack.value("waveIndex").toInt() == 1, "builtin states");
    QImage sheet(32, 32, QImage::Format_RGBA8888);
    sheet.fill(Qt::transparent);
    check(sheet.save(dir.filePath("sheet.png"), "PNG"), "save transparent PNG");
    auto run = [&](const QJsonObject &root, bool expected) {
        write(manifest, QJsonDocument(root).toJson());
        const bool loaded = loadCharacterPack(manifest, pack, error);
        if (loaded != expected) std::cerr << error.toStdString() << '\n';
        check(loaded == expected, expected ? "valid pack" : "reject invalid pack");
        check(expected ? error.isEmpty() : (!error.isEmpty() && pack.isEmpty()), "atomic error result");
    };
    run(valid(), true);
    check(pack.value("idleIndex").toInt() == 0 && pack.value("waveIndex").toInt() == 1 &&
          pack.value("dragLeftIndex").toInt() == 0, "interaction indices and fallback");
    check(pack.value("animations").toList()[1].toMap().value("intervalMs").toInt() == 50, "fps conversion");
    check(pack.value("behaviors").toList()[0].toMap().value("animation").toInt() == 1, "behavior index");
    for (const QString &bad : {QString("../sheet.png"), QString("https://host/sheet.png"),
                              QString("sub\\sheet.png"), QString("missing.png")}) {
        auto root = valid(); root["spriteSheet"] = bad; run(root, false);
    }
    QTemporaryDir outside;
    check(outside.isValid(), "outside directory");
    check(sheet.save(outside.filePath("outside.png"), "PNG"), "outside PNG");
    check(QFile::link(outside.filePath("outside.png"), dir.filePath("escape.png")), "create symlink");
    auto root = valid(); root["spriteSheet"] = "escape.png"; run(root, false);
    write(manifest, "{ broken json");
    check(!loadCharacterPack(manifest, pack, error) && !error.isEmpty(), "malformed JSON");
    write(manifest, QByteArray(65537, ' '));
    check(!loadCharacterPack(manifest, pack, error), "manifest size limit");
    for (const QString &key : {QString("schemaVersion"), QString("cellWidth"), QString("cellHeight")}) {
        root = valid(); root[key] = 1.5; run(root, false);
    }
    root = valid(); root["id"] = "Bad Slug"; run(root, false);
    root = valid(); root["idle"] = "missing"; run(root, false);
    root = valid(); root["interactions"] = QJsonObject{{"wave", "missing"}}; run(root, false);
    root = valid(); root["action"] = "https://example.com"; run(root, false);
    root = valid(); root["metadata"] = QJsonObject{{"license", "Not inferred"}, {"author", "Test"}}; run(root, true);
    root = valid(); root["metadata"] = QJsonObject{{"license", 42}}; run(root, false);
    auto changeAnimation = [&](const QString &key, const QJsonValue &value) {
        auto r = valid(); auto entries = r["animations"].toArray(); auto a = entries[1].toObject();
        a[key] = value; entries[1] = a; r["animations"] = entries; run(r, false);
    };
    root = valid(); root["name"] = "<img src=\"https://example.com/pixel.png\">"; run(root, false);
    changeAnimation("label", "<img src=\"https://example.com/pixel.png\">");
    changeAnimation("label", "A > B");
    changeAnimation("id", "idle");
    changeAnimation("row", 2);
    changeAnimation("row", 256);
    changeAnimation("frames", 3);
    changeAnimation("frames", 65);
    changeAnimation("fps", 0);
    changeAnimation("fps", 31);
    changeAnimation("fps", 2.5);
    changeAnimation("row", "1");
    root = valid(); root["animations"] = QJsonArray{}; run(root, false);
    QJsonArray tooMany;
    for (int i = 0; i < 33; ++i) tooMany.append(valid()["animations"].toArray()[0]);
    root = valid(); root["animations"] = tooMany; run(root, false);
    for (const QString &key : {QString("loops"), QString("weight")}) {
        root = valid(); auto b = root["behaviors"].toArray()[0].toObject(); b[key] = 0;
        root["behaviors"] = QJsonArray{b}; run(root, false);
    }
    root = valid(); root["behaviors"] = QJsonArray{QJsonObject{{"animation", "missing"}, {"loops", 1}, {"weight", 1}}}; run(root, false);
    write(dir.filePath("sheet.png"), "not a PNG"); run(valid(), false);
    write(dir.filePath("sheet.png"), QByteArray(16 * 1024 * 1024 + 1, 'x')); run(valid(), false);
    // Header dimension check happens before full image decode.
    QImage wide(16385, 1, QImage::Format_RGBA8888); wide.fill(Qt::transparent);
    check(wide.save(dir.filePath("sheet.png"), "PNG"), "wide PNG fixture"); run(valid(), false);
    check(sheet.save(dir.filePath("sheet.png"), "PNG"), "restore PNG");
    QFile png(dir.filePath("sheet.png")); check(png.open(QIODevice::ReadOnly), "open PNG");
    QByteArray truncated = png.readAll(); png.close(); truncated.truncate(40);
    write(dir.filePath("sheet.png"), truncated); run(valid(), false);
    // An optional real pack supplements the generated invalid-input fixtures.
    // verify.sh supplies the checked-in mascot; no pixels are changed here.
    if (argc > 1) {
        check(argc == 2, "usage: pack-tests [real-pack.json]");
        QVariantMap actual;
        check(loadCharacterPack(QString::fromLocal8Bit(argv[1]), actual, error), "real mascot pack loads");
        check(actual.value("id").toString() == "cyber-gremlin", "real mascot identity");
        check(actual.value("cellWidth").toInt() == 443 && actual.value("cellHeight").toInt() == 443, "real mascot cell size");
        check(actual.value("animations").toList().size() == 3, "real mascot animation count");
        check(actual.value("idleIndex").toInt() == 0 && actual.value("waveIndex").toInt() == 2, "real mascot interaction mapping");
        const QImage realSheet(QUrl(actual.value("source").toString()).toLocalFile());
        check(!realSheet.isNull() && realSheet.hasAlphaChannel(), "real mascot PNG decodes with alpha");
        QList<QImage> uniqueFrames;
        for (int row = 0; row < 2; ++row) {
            for (int col = 0; col < 4; ++col) {
                const QImage frame = realSheet.copy(col * 443, row * 443, 443, 443);
                int opaque = 0, transparent = 0;
                for (int y = 0; y < frame.height(); ++y) {
                    for (int x = 0; x < frame.width(); ++x) {
                        const int alpha = frame.pixelColor(x, y).alpha();
                        opaque += alpha > 128;
                        transparent += alpha == 0;
                    }
                }
                check(opaque > 0 && transparent > 0, "real mascot frame is visible with transparent padding");
                for (const auto &previous : uniqueFrames)
                    check(frame != previous, "real mascot frames contain distinct pixels");
                uniqueFrames.append(frame);
            }
        }
        check(uniqueFrames.size() == 8, "eight real mascot frames checked");
    }
    std::cout << "Pack validation tests passed\n";
}
