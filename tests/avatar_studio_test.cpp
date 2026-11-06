// SPDX-License-Identifier: MIT
#include "../tools/avatar-studio/rig_model.hpp"
#include <QCoreApplication>
#include <QDir>
#include <QFile>
#include <QTemporaryDir>
#include <iostream>
#include <cstdlib>
static void check(bool ok,const char *message) {
    if(!ok){std::cerr<<"FAIL: "<<message<<"\n";std::exit(1);}
}
int main(int argc,char **argv) {
    QCoreApplication app(argc,argv);
    check(argc==2,"usage: avatar-studio-test generated-demo/rig.json");
    RigModel model;
    check(model.load(QString::fromLocal8Bit(argv[1])),"load generated demo");
    check(model.parts().size()==9,"nine independent layers");
    check(model.animations().size()==4,"four animations");
    check(model.partIndex("arm-right")>=0,"find selected arm");
    check(model.frameCount(model.animationIndex("wave"))==16,"wave timeline length");
    const int arm=model.partIndex("arm-right");
    const int body=model.partIndex("body");
    const QPointF baseline=model.pivotWorld(body,"idle",0);
    check(qAbs(baseline.x()-96)<.001 && qAbs(baseline.y()-103)<.001,"root geometry");
    const QPointF armBaseline=model.pivotWorld(arm,"wave",0);
    check(armBaseline.x()>120 && armBaseline.y()>85,"nested arm world position");
    check(!model.hasKeyframe("wave",2),"frame initially interpolated");
    const Pose before=model.poseAt("wave","arm-right",2);
    check(before.rotate<0,"wave rotates between keyframes");
    check(model.addKeyframe("wave",2,15),"insert evaluated keyframe");
    check(model.hasKeyframe("wave",2),"inserted keyframe present");
    const Pose after=model.poseAt("wave","arm-right",2);
    check(qAbs(before.rotate-after.rotate)<.001,"keyframe insertion preserves pose");
    check(model.editPose("wave",arm,2,15,"rotate",-150),"edit arm rotation");
    check(qAbs(model.poseAt("wave","arm-right",2).rotate+150)<.001,"pose edit visible");
    check(model.setPartPosition(arm,{150,72},"wave",2),"drag to new world position");
    const QPointF moved=model.pivotWorld(arm,"wave",2);
    check((moved-QPointF(150,72)).manhattanLength()<1.0,"dragged pivot follows point");
    check(!model.removeKeyframe("wave",0,15),"reject deletion of start pose");
    check(model.removeKeyframe("wave",2,15),"remove intermediate pose");
    check(!model.hasKeyframe("wave",2),"key removed");
    QTemporaryDir temporary;
    check(temporary.isValid(),"temporary location");
    const QString file=QDir(temporary.path()).filePath("rig.json");
    model.filePath=file;
    check(model.save(),"atomic JSON save");
    RigModel reopened;
    check(reopened.load(file),"load saved authoring project");
    check(reopened.partIndex("arm-right")==arm,"part identities preserved");
    check(reopened.part(arm).value("at").toArray()==model.part(arm).value("at").toArray(),"joint coordinates persisted");
    check(reopened.animations().size()==4,"animation manifest preserved");
    std::cout<<"Avatar Studio authoring model tests passed\n";
    return 0;
}
