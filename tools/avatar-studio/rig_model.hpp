// SPDX-License-Identifier: MIT
#pragma once
// Authoring model only. Rust is the source of truth for rig validation and rendering.
#include <QFile>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QPointF>
#include <QSaveFile>
#include <QString>
#include <QTransform>
#include <QtMath>
#include <cmath>

struct Pose {
    double dx = 0, dy = 0, rotate = 0, sx = 1, sy = 1;
};
inline Pose readPose(const QJsonObject &value) {
    Pose p;
    p.dx = value.value("dx").toDouble(0);
    p.dy = value.value("dy").toDouble(0);
    p.rotate = value.value("rotate").toDouble(0);
    double scale = value.value("scale").toDouble(1);
    p.sx = value.value("scaleX").toDouble(scale);
    p.sy = value.value("scaleY").toDouble(scale);
    return p;
}
inline QJsonObject writePose(const Pose &p) {
    return {{"dx", p.dx}, {"dy", p.dy}, {"rotate", p.rotate},
            {"scaleX", p.sx}, {"scaleY", p.sy}};
}
inline Pose between(const Pose &a, const Pose &b, double t) {
    double delta = std::fmod(b.rotate - a.rotate + 180.0 + 720.0, 360.0) - 180.0;
    return {a.dx + (b.dx-a.dx)*t, a.dy+(b.dy-a.dy)*t,
            a.rotate+delta*t, a.sx+(b.sx-a.sx)*t, a.sy+(b.sy-a.sy)*t};
}
inline QPointF asPoint(const QJsonValue &value) {
    auto a = value.toArray();
    return {a.value(0).toDouble(), a.value(1).toDouble()};
}
inline QJsonArray jsonPoint(const QPointF &p) {
    return {p.x(), p.y()};
}

class RigModel {
public:
    QString filePath;
    QJsonObject doc;
    bool dirty = false;
    QString lastError;

    bool load(const QString &path) {
        QFile f(path);
        if (!f.open(QIODevice::ReadOnly)) { lastError = f.errorString(); return false; }
        if (f.size() > 65536) { lastError = "Rig JSON must be 64 KiB or less"; return false; }
        QJsonParseError err;
        QJsonDocument value = QJsonDocument::fromJson(f.readAll(), &err);
        if (err.error != QJsonParseError::NoError || !value.isObject()) {
            lastError = "Invalid rig JSON: " + err.errorString(); return false;
        }
        const auto root = value.object();
        if (root.value("schemaVersion").toInt() != 1 ||
            !root.value("pack").isObject() ||
            !root.value("parts").isArray()) {
            lastError = "Expected schemaVersion 1 with pack and parts"; return false;
        }
        if (root.value("parts").toArray().isEmpty()) {
            lastError = "Rig has no parts"; return false;
        }
        doc = root;
        filePath = path;
        dirty = false;
        lastError.clear();
        return true;
    }
    bool save() {
        if (filePath.isEmpty()) { lastError = "No project loaded"; return false; }
        QSaveFile file(filePath);
        if (!file.open(QIODevice::WriteOnly)) { lastError = file.errorString(); return false; }
        QByteArray bytes = QJsonDocument(doc).toJson(QJsonDocument::Indented);
        if (bytes.size() > 65536 || file.write(bytes) != bytes.size() || !file.commit()) {
            lastError = "Could not atomically save rig (or exceeds 64 KiB): " + file.errorString(); return false;
        }
        dirty = false;
        lastError.clear();
        return true;
    }
    QJsonObject pack() const { return doc.value("pack").toObject(); }
    QJsonArray parts() const { return doc.value("parts").toArray(); }
    QJsonArray animations() const { return pack().value("animations").toArray(); }
    QJsonObject part(int i) const {
        return (i >= 0 && i < parts().size()) ? parts().at(i).toObject() : QJsonObject{};
    }
    QString partId(int i) const { return part(i).value("id").toString(); }
    int partIndex(const QString &id) const {
        auto a = parts();
        for (int i=0; i<a.size(); ++i)
            if (a[i].toObject().value("id").toString() == id) return i;
        return -1;
    }
    int animationIndex(const QString &id) const {
        auto a = animations();
        for (int i=0; i<a.size(); ++i)
            if (a[i].toObject().value("id").toString() == id) return i;
        return -1;
    }
    QJsonObject animation(int index) const {
        auto a=animations();
        return index>=0 && index<a.size()?a[index].toObject():QJsonObject{};
    }
    QString animationId(int index) const { return animation(index).value("id").toString(); }
    int frameCount(int index) const { return animation(index).value("frames").toInt(1); }
    int frameRate(int index) const { return animation(index).value("fps").toInt(10); }

    Pose poseAt(const QString &motion, const QString &part, int frame) const {
        const QJsonArray frames=doc.value("motions").toObject().value(motion).toArray();
        if (frames.isEmpty()) return {};
        QJsonObject a=frames.first().toObject(), b=frames.last().toObject();
        if (frame <= a.value("frame").toInt()) b=a;
        else if (frame >= b.value("frame").toInt()) a=b;
        else {
            for (int i=1; i<frames.size(); ++i) {
                auto next=frames[i].toObject();
                if (next.value("frame").toInt()>=frame) {
                    a=frames[i-1].toObject();b=next;break;
                }
            }
        }
        const Pose start=readPose(a.value("poses").toObject().value(part).toObject());
        const Pose end=readPose(b.value("poses").toObject().value(part).toObject());
        const int lo=a.value("frame").toInt(), hi=b.value("frame").toInt();
        const double t=(hi==lo)?0:double(frame-lo)/double(hi-lo);
        return between(start,end,t);
    }
    QTransform localTransform(int partIndex, const QString &motion, int frame) const {
        const QJsonObject item=part(partIndex);
        const QPointF at=asPoint(item.value("at"));
        const QPointF pivot=asPoint(item.value("pivot"));
        const Pose pose=poseAt(motion, partId(partIndex),frame);
        QTransform m;
        m.translate(at.x()+pose.dx,at.y()+pose.dy);
        m.rotate(pose.rotate);
        m.scale(pose.sx,pose.sy);
        m.translate(-pivot.x(),-pivot.y());
        return m;
    }
    QTransform worldTransform(int index,const QString &motion,int frame,int depth=0) const {
        if (index<0 || depth>64) return {};
        QTransform m=localTransform(index,motion,frame);
        const int parent=partIndex(part(index).value("parent").toString());
        if(parent>=0 && parent!=index) m=m*worldTransform(parent,motion,frame,depth+1);
        return m;
    }
    QPointF pivotWorld(int index,const QString &motion,int frame) const {
        return worldTransform(index,motion,frame).map(asPoint(part(index).value("pivot")));
    }
    bool setPartPosition(int index,const QPointF &world,const QString &motion,int frame) {
        if (index<0 || index>=parts().size()) return false;
        const QJsonObject item=part(index);
        const int parent=partIndex(item.value("parent").toString());
        QPointF local=world;
        if(parent>=0) {
            bool invertible=false;
            QTransform inv=worldTransform(parent,motion,frame).inverted(&invertible);
            if(!invertible) { lastError="Parent transform cannot be inverted";return false; }
            local=inv.map(world);
        }
        Pose p=poseAt(motion,partId(index),frame);
        return setPartPoint(index,"at",QPointF(qRound(local.x()-p.dx),qRound(local.y()-p.dy)));
    }
    bool setPartPoint(int index,const QString &key,const QPointF &value) {
        auto list=parts();
        if(index<0 || index>=list.size() || (key!="at"&&key!="pivot"))return false;
        auto p=list[index].toObject();
        if(asPoint(p.value(key))==value) return true;
        p.insert(key,jsonPoint(value));
        list.replace(index,p);
        doc.insert("parts",list);
        dirty=true;
        return true;
    }
    QList<int> keyframes(const QString &motion) const {
        QList<int> list;
        for(const auto &v:doc.value("motions").toObject().value(motion).toArray())
            list.append(v.toObject().value("frame").toInt());
        return list;
    }
    bool hasKeyframe(const QString &motion,int frame) const {
        return keyframes(motion).contains(frame);
    }
    bool addKeyframe(const QString &motion,int frame,int endFrame) {
        if(motion.isEmpty() || frame<0 || frame>=endFrame+1) return false;
        auto motions=doc.value("motions").toObject();
        auto entries=motions.value(motion).toArray();
        if(hasKeyframe(motion,frame))return true;
        // Freeze the interpolated pose of every part at insertion time. That
        // prevents unrelated layers snapping to neutral at the new keyframe.
        QJsonObject poses;
        for(int i=0;i<parts().size();++i)
            poses.insert(partId(i),writePose(poseAt(motion,partId(i),frame)));
        entries.append(QJsonObject{{"frame",frame},{"poses",poses}});
        if(entries.size()==1) {
            const int other=frame==0?endFrame:0;
            entries.append(QJsonObject{{"frame",other},{"poses",poses}});
        }
        std::sort(entries.begin(),entries.end(),[](const QJsonValue &l,const QJsonValue &r) {
            return l.toObject().value("frame").toInt()<r.toObject().value("frame").toInt();
        });
        motions.insert(motion,entries);
        doc.insert("motions",motions);
        dirty=true;
        return true;
    }
    bool editPose(const QString &motion,int partIndex,int frame,int endFrame,
                  const QString &key,double value) {
        if(partIndex<0 || !addKeyframe(motion,frame,endFrame)) return false;
        auto motions=doc.value("motions").toObject();
        auto frames=motions.value(motion).toArray();
        for(int i=0;i<frames.size();++i) {
            auto k=frames[i].toObject();
            if(k.value("frame").toInt()!=frame)continue;
            auto all=k.value("poses").toObject();
            auto p=all.value(partId(partIndex)).toObject();
            if(key=="scaleX" || key=="scaleY")p.remove("scale");
            p.insert(key,value);
            all.insert(partId(partIndex),p);
            k.insert("poses",all);
            frames.replace(i,k);
            motions.insert(motion,frames);
            doc.insert("motions",motions);
            dirty=true;
            return true;
        }
        return false;
    }
    bool removeKeyframe(const QString &motion,int frame,int endFrame) {
        if(frame==0 || frame==endFrame) {lastError="First and last frames are required";return false;}
        auto motions=doc.value("motions").toObject();
        auto entries=motions.value(motion).toArray();
        for(int i=0;i<entries.size();++i)
            if(entries[i].toObject().value("frame").toInt()==frame) {
                entries.removeAt(i);motions.insert(motion,entries);doc.insert("motions",motions);dirty=true;return true;
            }
        return false;
    }
};
