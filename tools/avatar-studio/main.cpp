// SPDX-License-Identifier: MIT
// Deskling Avatar Studio: a local, authoring-only Qt Widgets front end.
// Every displayed character frame is rendered by the Rust rig engine.
#include "rig_model.hpp"
#include <QApplication>
#include <QCheckBox>
#include <QCloseEvent>
#include <QComboBox>
#include <QCoreApplication>
#include <QDateTime>
#include <QDoubleSpinBox>
#include <QFileDialog>
#include <QFileInfo>
#include <QFormLayout>
#include <QGraphicsEllipseItem>
#include <QGraphicsPixmapItem>
#include <QGraphicsScene>
#include <QGraphicsSimpleTextItem>
#include <QGraphicsView>
#include <QGroupBox>
#include <QHBoxLayout>
#include <QInputDialog>
#include <QLabel>
#include <QLineEdit>
#include <QMainWindow>
#include <QMenuBar>
#include <QMessageBox>
#include <QPainter>
#include <QProcess>
#include <QPushButton>
#include <QScrollArea>
#include <QSignalBlocker>
#include <QSlider>
#include <QSpinBox>
#include <QSplitter>
#include <QStandardPaths>
#include <QStatusBar>
#include <QTemporaryDir>
#include <QTimer>
#include <QVBoxLayout>
#include <QListWidget>
#include <functional>
#include <memory>
#include <algorithm>

static QImage checkerboard(int w,int h) {
    QImage canvas(w,h,QImage::Format_RGB32);
    for(int y=0;y<h;y++)for(int x=0;x<w;x++)
        canvas.setPixelColor(x,y,((x/12+y/12)&1)?QColor("#424653"):QColor("#30343f"));
    return canvas;
}
class Canvas : public QGraphicsView {
public:
    std::function<void(int,QPointF)> moved;
    explicit Canvas(QWidget *parent=nullptr):QGraphicsView(parent) {
        setRenderHint(QPainter::Antialiasing);
        setRenderHint(QPainter::SmoothPixmapTransform,false);
        setDragMode(QGraphicsView::NoDrag);
        setBackgroundBrush(QColor("#20242c"));
        setViewportUpdateMode(QGraphicsView::FullViewportUpdate);
    }
protected:
    QGraphicsItem *dragged=nullptr;
    void mousePressEvent(QMouseEvent *event) override {
        auto *it=itemAt(event->pos());
        dragged=(it && it->data(0).isValid())?it:nullptr;
        QGraphicsView::mousePressEvent(event);
    }
    void mouseReleaseEvent(QMouseEvent *event) override {
        QGraphicsView::mouseReleaseEvent(event);
        auto *handle=dragged;dragged=nullptr;
        if(handle && handle->data(0).isValid() && moved)
            moved(handle->data(0).toInt(), handle->sceneBoundingRect().center());
    }
    void resizeEvent(QResizeEvent *event) override {
        QGraphicsView::resizeEvent(event);
        if(scene() && !scene()->sceneRect().isEmpty())fitInView(scene()->sceneRect(),Qt::KeepAspectRatio);
    }
};

class Studio : public QMainWindow {
public:
    Studio(QString renderer,QString deskling,bool smoke,QString capture,QWidget *parent=nullptr):
        QMainWindow(parent), renderer_(std::move(renderer)), deskling_(std::move(deskling)),
        smoke_(smoke),capture_(std::move(capture)), workspace_(std::make_unique<QTemporaryDir>()) {
        setWindowTitle("Deskling Avatar Studio");
        resize(1280,820);
        setup();
        if(!workspace_->isValid())QMessageBox::critical(this,"Temporary Workspace","Cannot create secure preview directory.");
        previewTimer_.setSingleShot(true);
        previewTimer_.setInterval(160);
        connect(&previewTimer_,&QTimer::timeout,this,[this]{ startPreview(); });
        connect(&playback_,&QTimer::timeout,this,[this]{
            if(model_.filePath.isEmpty())return;
            const int total=model_.frameCount(animation_->currentIndex());
            frame_->setValue((frame_->value()+1)%total);
        });
        process_=new QProcess(this);
        connect(process_,&QProcess::finished,this,[this](int exit,QProcess::ExitStatus state) {
            if(state!=QProcess::NormalExit||exit!=0) {
                const QString error=QString::fromUtf8(process_->readAllStandardError()).trimmed();
                showProblem("Rust preview failed: "+error);
                return;
            }
            QPixmap pix;
            const QString path=workspace_->filePath("pose.png");
            if(!pix.load(path)) { showProblem("Rust returned no readable RGBA preview");return; }
            renderPixmap(pix);
            statusBar()->showMessage(QString("Rust preview: %1 / frame %2%3")
                .arg(model_.animationId(animation_->currentIndex())).arg(frame_->value())
                .arg(model_.dirty?"  • unsaved":""));
            if(smoke_) {
                QTimer::singleShot(100,this,[this]{
                    if(!capture_.isEmpty() && !grab().save(capture_)) {
                        showProblem("Studio screenshot capture failed");
                        QCoreApplication::exit(4);
                    } else QCoreApplication::exit(0);
                });
            }
        });
        if(smoke_) {
            QTimer::singleShot(25000,this,[]{QCoreApplication::exit(5);});
        }
    }
    bool open(const QString &path) {
        if(!confirmDiscard())return false;
        RigModel candidate;
        if(!candidate.load(path)) {showProblem(candidate.lastError);return false;}
        model_=std::move(candidate);
        if(!prepareWorkspace())return false;
        layers_->clear();
        for(int i=0;i<model_.parts().size();++i)
            layers_->addItem(model_.partId(i));
        animation_->clear();
        for(const auto &v:model_.animations()) {
            const auto a=v.toObject();
            animation_->addItem(a.value("label").toString(a.value("id").toString()),a.value("id"));
        }
        if(animation_->count()==0) {showProblem("Rig contains no animations");return false;}
        selected_=0;
        layers_->setCurrentRow(0);
        animation_->setCurrentIndex(0);
        setTimeline(0);
        updateProperties();
        schedulePreview();
        setWindowTitle("Deskling Avatar Studio — "+QFileInfo(path).fileName());
        return true;
    }
    void showProblem(const QString &error) {
        statusBar()->showMessage(error,10000);
        if(smoke_) {QCoreApplication::exit(3);}
        else QMessageBox::warning(this,"Deskling Studio",error);
    }
protected:
    void closeEvent(QCloseEvent *event) override {
        if(confirmDiscard())event->accept();else event->ignore();
    }
private:
    RigModel model_;
    QString renderer_,deskling_,capture_;
    bool smoke_=false;
    std::unique_ptr<QTemporaryDir> workspace_;
    QListWidget *layers_=nullptr;
    QComboBox *animation_=nullptr;
    Canvas *canvas_=nullptr;
    QGraphicsScene *scene_=nullptr;
    QGraphicsPixmapItem *background_=nullptr,*sprite_=nullptr;
    QSlider *timeline_=nullptr;
    QSpinBox *frame_=nullptr;
    QLabel *frameLabel_=nullptr,*keyLabel_=nullptr;
    QDoubleSpinBox *dx_=nullptr,*dy_=nullptr,*rotate_=nullptr,*sx_=nullptr,*sy_=nullptr;
    QDoubleSpinBox *atx_=nullptr,*aty_=nullptr,*px_=nullptr,*py_=nullptr;
    QCheckBox *joints_=nullptr;
    QPushButton *playButton_=nullptr;
    QProcess *process_=nullptr;
    QTimer previewTimer_,playback_;
    int selected_=-1;
    QList<QGraphicsItem *> handles_;
    QString lastPack_;
    bool controlsChanging_=false;

    void setup() {
        auto *fileMenu=menuBar()->addMenu("&File");
        QAction *openAction=fileMenu->addAction("&Open rig…");
        QAction *demoAction=fileMenu->addAction("Create &demo rig…");
        QAction *saveAction=fileMenu->addAction("&Save project");
        QAction *exportAction=fileMenu->addAction("&Export native pack…");
        QAction *runAction=fileMenu->addAction("Run exported pack in &Deskling");
        fileMenu->addSeparator();
        fileMenu->addAction("&Quit",this,&QWidget::close);
        openAction->setShortcut(QKeySequence::Open);
        saveAction->setShortcut(QKeySequence::Save);
        connect(openAction,&QAction::triggered,this,[this]{
            const QString path=QFileDialog::getOpenFileName(this,"Open Rig","", "Rig projects (rig.json);;JSON (*.json)");
            if(!path.isEmpty())open(path);
        });
        connect(demoAction,&QAction::triggered,this,[this]{createDemo();});
        connect(saveAction,&QAction::triggered,this,[this]{saveProject();});
        connect(exportAction,&QAction::triggered,this,[this]{exportPack();});
        connect(runAction,&QAction::triggered,this,[this]{runPack();});

        auto *main=new QWidget(this);
        setCentralWidget(main);
        auto *layout=new QVBoxLayout(main);
        auto *top=new QHBoxLayout;
        auto *openButton=new QPushButton("Open");
        auto *saveButton=new QPushButton("Save");
        auto *exportButton=new QPushButton("Export Pack");
        auto *demoButton=new QPushButton("New Demo");
        for(auto *b:{openButton,saveButton,exportButton,demoButton})top->addWidget(b);
        top->addStretch(1);
        auto *rendererLabel=new QLabel("Rust rendering • authoring workspace");
        rendererLabel->setStyleSheet("color:#979eaf");
        top->addWidget(rendererLabel);
        layout->addLayout(top);
        connect(openButton,&QPushButton::clicked,openAction,&QAction::trigger);
        connect(saveButton,&QPushButton::clicked,saveAction,&QAction::trigger);
        connect(exportButton,&QPushButton::clicked,exportAction,&QAction::trigger);
        connect(demoButton,&QPushButton::clicked,demoAction,&QAction::trigger);

        auto *split=new QSplitter(Qt::Horizontal);
        split->setChildrenCollapsible(false);
        layout->addWidget(split,1);
        auto *left=new QWidget;auto *leftLayout=new QVBoxLayout(left);
        leftLayout->addWidget(new QLabel("LAYERS · back to front"));
        layers_=new QListWidget;
        layers_->setSelectionMode(QAbstractItemView::SingleSelection);
        leftLayout->addWidget(layers_,1);
        leftLayout->addWidget(new QLabel("Select a layer, then drag its joint handle.\nLayer images are never repainted."));
        split->addWidget(left);
        connect(layers_,&QListWidget::currentRowChanged,this,[this](int i) {selected_=i;updateProperties();updateHandles();});

        auto *middle=new QWidget;auto *mid=new QVBoxLayout(middle);
        scene_=new QGraphicsScene(this);
        canvas_=new Canvas;
        canvas_->setScene(scene_);
        mid->addWidget(canvas_,1);
        joints_=new QCheckBox("Show draggable attachment joints");
        joints_->setChecked(true);
        mid->addWidget(joints_);
        connect(joints_,&QCheckBox::toggled,this,[this]{updateHandles();});
        canvas_->moved=[this](int i,QPointF at) {
            if(i<0 || model_.filePath.isEmpty())return;
            if(model_.setPartPosition(i,at,model_.animationId(animation_->currentIndex()),frame_->value())) {
                updateProperties();schedulePreview();
            }
        };
        split->addWidget(middle);

        auto *right=new QWidget;auto *properties=new QVBoxLayout(right);
        properties->addWidget(new QLabel("SELECTED LAYER"));
        auto *form=new QFormLayout;
        auto spin=[&](QString name,double min,double max,double step) {
            auto *s=new QDoubleSpinBox;
            s->setRange(min,max);s->setDecimals(2);s->setSingleStep(step);
            s->setKeyboardTracking(false);form->addRow(name,s);return s;
        };
        atx_=spin("Attach X",-2048,2048,1);
        aty_=spin("Attach Y",-2048,2048,1);
        px_=spin("Pivot X",0,16383,1);
        py_=spin("Pivot Y",0,16383,1);
        properties->addLayout(form);
        properties->addWidget(new QLabel("KEYFRAME POSE"));
        auto *poseForm=new QFormLayout;
        auto poseSpin=[&](QString name,double lo,double hi,double inc) {
            auto *s=new QDoubleSpinBox;
            s->setRange(lo,hi);s->setDecimals(2);s->setSingleStep(inc);
            s->setKeyboardTracking(false);poseForm->addRow(name,s);return s;
        };
        dx_=poseSpin("Offset X",-1024,1024,1);
        dy_=poseSpin("Offset Y",-1024,1024,1);
        rotate_=poseSpin("Rotation °",-360,360,5);
        sx_=poseSpin("Scale X",0.15,4.0,0.05);
        sy_=poseSpin("Scale Y",0.15,4.0,0.05);
        properties->addLayout(poseForm);
        properties->addWidget(new QLabel("Editing a pose inserts a keyframe\nwhile preserving other parts' poses."));
        auto *addKey=new QPushButton("Add Keyframe");
        auto *removeKey=new QPushButton("Delete Keyframe");
        properties->addWidget(addKey);
        properties->addWidget(removeKey);
        keyLabel_=new QLabel;
        properties->addWidget(keyLabel_);
        properties->addStretch(1);
        split->addWidget(right);
        split->setSizes({230,690,290});

        auto *timelineArea=new QWidget;
        auto *timelineLayout=new QVBoxLayout(timelineArea);
        auto *timelineTop=new QHBoxLayout;
        timelineTop->addWidget(new QLabel("ANIMATION"));
        animation_=new QComboBox;
        timelineTop->addWidget(animation_,1);
        playButton_=new QPushButton("Play");
        timelineTop->addWidget(playButton_);
        timelineLayout->addLayout(timelineTop);
        auto *scrub=new QHBoxLayout;
        timeline_=new QSlider(Qt::Horizontal);
        timeline_->setRange(0,1);
        frame_=new QSpinBox;
        frame_->setRange(0,1);
        frameLabel_=new QLabel("Frame 0 / 0");
        scrub->addWidget(timeline_,1);
        scrub->addWidget(frame_);
        scrub->addWidget(frameLabel_);
        timelineLayout->addLayout(scrub);
        layout->addWidget(timelineArea);
        connect(timeline_,&QSlider::valueChanged,frame_,&QSpinBox::setValue);
        connect(frame_,qOverload<int>(&QSpinBox::valueChanged),this,[this](int n){
            QSignalBlocker hold(timeline_);timeline_->setValue(n);
            refreshFrame();schedulePreview();
        });
        connect(animation_,qOverload<int>(&QComboBox::currentIndexChanged),this,[this](int) {
            if(model_.filePath.isEmpty())return;
            setTimeline(0);refreshFrame();schedulePreview();
        });
        connect(playButton_,&QPushButton::clicked,this,[this] {
            if(playback_.isActive()) {playback_.stop();playButton_->setText("Play");}
            else if(!model_.filePath.isEmpty()) {
                playback_.start(qMax(17,qRound(1000.0/model_.frameRate(animation_->currentIndex()))));
                playButton_->setText("Pause");
            }
        });
        connect(addKey,&QPushButton::clicked,this,[this]{
            if(model_.addKeyframe(model_.animationId(animation_->currentIndex()),frame_->value(),
                                  model_.frameCount(animation_->currentIndex())-1))refreshFrame();
        });
        connect(removeKey,&QPushButton::clicked,this,[this]{
            if(!model_.removeKeyframe(model_.animationId(animation_->currentIndex()),frame_->value(),
                                     model_.frameCount(animation_->currentIndex())-1))
                statusBar()->showMessage(model_.lastError,5000);
            refreshFrame();
        });
        auto hook=[this](QDoubleSpinBox *s,const QString &key) {
            connect(s,qOverload<double>(&QDoubleSpinBox::valueChanged),this,[this,key](double v){
                if(controlsChanging_ || selected_<0 || model_.filePath.isEmpty())return;
                if(key=="atX" || key=="atY" || key=="pivotX" || key=="pivotY") {
                    const bool attachment=key.startsWith("at");
                    QPointF point=asPoint(model_.part(selected_).value(attachment?"at":"pivot"));
                    if(key.endsWith('X'))point.setX(v);else point.setY(v);
                    model_.setPartPoint(selected_,attachment?"at":"pivot",point);
                } else {
                    model_.editPose(model_.animationId(animation_->currentIndex()),selected_,frame_->value(),
                                    model_.frameCount(animation_->currentIndex())-1,key,v);
                }
                refreshFrame();schedulePreview();
            });
        };
        hook(atx_,"atX");hook(aty_,"atY");hook(px_,"pivotX");hook(py_,"pivotY");
        hook(dx_,"dx");hook(dy_,"dy");hook(rotate_,"rotate");
        hook(sx_,"scaleX");hook(sy_,"scaleY");
        statusBar()->showMessage("Open rig.json or create a demo to begin.");
    }
    bool confirmDiscard() {
        if(!model_.dirty)return true;
        auto choice=QMessageBox::question(this,"Unsaved Changes",
              "Save changes to the current rig?",QMessageBox::Save|QMessageBox::Discard|QMessageBox::Cancel);
        if(choice==QMessageBox::Cancel)return false;
        if(choice==QMessageBox::Save)return saveProject();
        return true;
    }
    bool saveProject() {
        if(model_.filePath.isEmpty())return false;
        if(!model_.save()) {showProblem(model_.lastError);return false;}
        statusBar()->showMessage("Saved "+model_.filePath,4500);return true;
    }
    bool prepareWorkspace() {
        if(!workspace_->isValid())return false;
        const QString link=workspace_->filePath("layers");
        QFile::remove(link);
        const QString source=QFileInfo(model_.filePath).absolutePath()+"/layers";
        if(!QDir(source).exists() || !QFile::link(source,link)) {
            showProblem("Cannot link rig layers into temporary preview workspace: "+source);
            return false;
        }
        return true;
    }
    bool stageRig() {
        if(model_.filePath.isEmpty() || !workspace_->isValid())return false;
        QFile file(workspace_->filePath("rig.json"));
        if(!file.open(QIODevice::WriteOnly|QIODevice::Truncate)) {showProblem(file.errorString());return false;}
        QByteArray bytes=QJsonDocument(model_.doc).toJson(QJsonDocument::Compact);
        if(bytes.size()>65536 || file.write(bytes)!=bytes.size()) {
            showProblem("Cannot write temporary preview source (possibly oversized)");
            return false;
        }
        file.close();
        return true;
    }
    void schedulePreview() {
        if(!model_.filePath.isEmpty())previewTimer_.start();
    }
    void startPreview() {
        if(!stageRig())return;
        if(process_->state()!=QProcess::NotRunning) {
            process_->kill();process_->waitForFinished(500);
        }
        QStringList args{"pose",workspace_->filePath("rig.json"),
                         model_.animationId(animation_->currentIndex()),
                         QString::number(frame_->value()),workspace_->filePath("pose.png")};
        process_->setProgram(renderer_);
        process_->setArguments(args);
        process_->start();
        if(!process_->waitForStarted(1500))showProblem("Cannot launch Rust renderer: "+renderer_);
    }
    void renderPixmap(const QPixmap &image) {
        for(auto *item:handles_)scene_->removeItem(item);
        qDeleteAll(handles_);handles_.clear();
        scene_->clear();
        const QPixmap backdrop=QPixmap::fromImage(checkerboard(image.width(),image.height()));
        background_=scene_->addPixmap(backdrop);
        background_->setZValue(-2);
        sprite_=scene_->addPixmap(image);
        sprite_->setZValue(-1);
        scene_->setSceneRect(0,0,image.width(),image.height());
        canvas_->fitInView(scene_->sceneRect(),Qt::KeepAspectRatio);
        updateHandles();
    }
    void updateHandles() {
        for(auto *item:handles_) {scene_->removeItem(item);delete item;}
        handles_.clear();
        if(model_.filePath.isEmpty() || !joints_->isChecked())return;
        const auto motion=model_.animationId(animation_->currentIndex());
        for(int i=0;i<model_.parts().size();++i) {
            const QPointF p=model_.pivotWorld(i,motion,frame_->value());
            auto *dot=scene_->addEllipse(-3,-3,6,6,
                QPen(i==selected_?QColor("#ffba70"):QColor("#87dfe4"),1.2),
                QBrush(i==selected_?QColor("#ffba70"):QColor("#87dfe4")));
            dot->setPos(p);
            dot->setData(0,i);
            dot->setFlag(QGraphicsItem::ItemIsMovable,true);
            dot->setZValue(3+(i==selected_));
            dot->setToolTip("Drag attachment: "+model_.partId(i));
            handles_.append(dot);
        }
    }
    void setTimeline(int frame) {
        const int max=qMax(0,model_.frameCount(animation_->currentIndex())-1);
        QSignalBlocker a(frame_),b(timeline_);
        frame_->setRange(0,max);timeline_->setRange(0,max);
        frame_->setValue(qBound(0,frame,max));
        timeline_->setValue(frame_->value());
        refreshFrame();
    }
    void refreshFrame() {
        if(model_.filePath.isEmpty())return;
        frameLabel_->setText(QString("Frame %1 / %2").arg(frame_->value())
                             .arg(model_.frameCount(animation_->currentIndex())-1));
        const auto motion=model_.animationId(animation_->currentIndex());
        const auto frames=model_.keyframes(motion);
        QStringList markers;
        for(int n:frames) markers<<QString::number(n);
        keyLabel_->setText("Keyframes: "+(markers.isEmpty()?"(neutral)":markers.join(", "))+
                            (model_.hasKeyframe(motion,frame_->value())?"\n● On keyframe":"\n○ Between keyframes"));
        updateProperties();
        updateHandles();
    }
    void updateProperties() {
        if(!atx_ || selected_<0 || model_.filePath.isEmpty())return;
        const QJsonObject item=model_.part(selected_);
        const QPointF at=asPoint(item.value("at")),pivot=asPoint(item.value("pivot"));
        const Pose pose=model_.poseAt(model_.animationId(animation_->currentIndex()),
                                      model_.partId(selected_),frame_->value());
        controlsChanging_=true;
        atx_->setValue(at.x());aty_->setValue(at.y());
        px_->setValue(pivot.x());py_->setValue(pivot.y());
        dx_->setValue(pose.dx);dy_->setValue(pose.dy);rotate_->setValue(pose.rotate);
        sx_->setValue(pose.sx);sy_->setValue(pose.sy);
        controlsChanging_=false;
    }
    void createDemo() {
        if(!confirmDiscard())return;
        const QString parent=QFileDialog::getExistingDirectory(this,"Select parent directory for new rig demo");
        if(parent.isEmpty())return;
        const QString dest=QDir(parent).filePath("deskling-demo-source");
        if(QFileInfo::exists(dest)) {showProblem("Demo target already exists: "+dest);return;}
        QProcess process;
        process.start(renderer_,{"demo",dest});
        if(!process.waitForStarted(3000)||!process.waitForFinished(30000)||process.exitCode()!=0) {
            showProblem("Demo generation failed: "+QString::fromUtf8(process.readAllStandardError()));return;
        }
        open(QDir(dest).filePath("rig.json"));
    }
    void exportPack() {
        if(model_.filePath.isEmpty())return;
        const QString parent=QFileDialog::getExistingDirectory(this,"Select destination parent directory");
        if(parent.isEmpty())return;
        const QString id=model_.pack().value("id").toString("character");
        const QString dest=QDir(parent).filePath(id+"-export");
        if(QFileInfo::exists(dest)) {
            showProblem("Output exists. Move it or select another parent: "+dest);return;
        }
        if(!stageRig())return;
        QProcess process;
        process.start(renderer_,{"rig",workspace_->filePath("rig.json"),dest});
        if(!process.waitForStarted(3000)||!process.waitForFinished(120000)||process.exitCode()!=0) {
            showProblem("Rust export failed: "+QString::fromUtf8(process.readAllStandardError()));return;
        }
        lastPack_=QDir(dest).filePath("pack.json");
        statusBar()->showMessage("Exported to "+lastPack_,10000);
        QMessageBox::information(this,"Native Pack Exported",
            "Created the exact native Deskling pack and quality report:\n"+lastPack_+
            "\n\nThe source rig and layer artwork are preserved.");
    }
    void runPack() {
        if(lastPack_.isEmpty() || !QFileInfo::exists(lastPack_)) {
            showProblem("Export the current rig before launching Deskling.");return;
        }
        if(deskling_.isEmpty() || !QFileInfo::exists(deskling_)) {
            showProblem("No Deskling desktop executable found; pass --deskling /path/to/deskling-desktop.");return;
        }
        if(!QProcess::startDetached(deskling_,{"--pack",lastPack_,"--reset-position"}))
            showProblem("Failed to launch Deskling.");
    }
};

int main(int argc,char **argv) {
    QApplication app(argc,argv);
    app.setApplicationName("Deskling Avatar Studio");
    QString rig,renderer,deskling,capture;
    bool smoke=false;
    const QStringList args=app.arguments();
    for(int i=1;i<args.size();++i) {
        if(args[i]=="--rig" && i+1<args.size())rig=args[++i];
        else if(args[i]=="--renderer" && i+1<args.size())renderer=args[++i];
        else if(args[i]=="--deskling" && i+1<args.size())deskling=args[++i];
        else if(args[i]=="--capture" && i+1<args.size())capture=args[++i];
        else if(args[i]=="--smoke-test")smoke=true;
        else {qCritical("Usage: deskling-avatar-studio [--rig rig.json] [--renderer path] [--deskling path] [--smoke-test --capture screenshot.png]");return 2;}
    }
    if(renderer.isEmpty())renderer=QStandardPaths::findExecutable("deskling-avatar-factory");
    if(renderer.isEmpty())renderer="deskling-avatar-factory";
    Studio window(renderer,deskling,smoke,capture);
    window.show();
    if(!rig.isEmpty()&&!window.open(rig))return 3;
    if(smoke&&rig.isEmpty())return 2;
    return app.exec();
}
