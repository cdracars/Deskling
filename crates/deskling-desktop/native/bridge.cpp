#include "bridge.hpp"
#include "pack.hpp"
#include <QCoreApplication>
#include <QCursor>
#include <QDebug>
#include <QDesktopServices>
#include <QDir>
#include <QFileInfo>
#include <QFile>
#include <QGuiApplication>
#include <QIcon>
#include <QImage>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQuickItem>
#include <QQuickItemGrabResult>
#include <QQuickWindow>
#include <QScreen>
#include <QSharedPointer>
#include <QTimer>
#include <QUrl>
#include <algorithm>
#include <cmath>
#include <sys/stat.h>

extern "C" bool deskling_validate_link_url(const char *url);

// Keep the resource initializer in C++ linkage, outside the C ABI entry point.
static void initResources() { Q_INIT_RESOURCE(deskling); }

static constexpr int iconSizes[] = {16, 24, 32, 48, 64, 128, 256, 512};

static QIcon applicationIcon() {
    QIcon icon;
    for (const int size : iconSizes) {
        icon.addFile(QStringLiteral(":/deskling/icons/deskling-%1.png").arg(size), QSize(size, size));
    }
    return icon;
}

static bool validLinkUrl(const QString &url) {
    if (url.contains(QChar('\0'))) return false;
    const auto utf8 = url.toUtf8();
    return deskling_validate_link_url(utf8.constData());
}

Bridge::Bridge(bool smokeTest, const QString &linkUrl, QObject *parent)
    : QObject(parent), m_settings(QSettings::IniFormat, QSettings::UserScope,
          QStringLiteral("Deskling"), QStringLiteral("desktop")), m_smokeTest(smokeTest) {
    // Smoke tests never read or write the user's settings.
    if (!m_smokeTest) {
        const QString saved = m_settings.value("linkUrl").toString();
        if (validLinkUrl(saved)) m_linkUrl = saved;
        const double scale = m_settings.value("scale", 1.0).toDouble();
        if (std::isfinite(scale)) m_scale = std::clamp(scale, 0.75, 2.0);
    }
    // A CLI URL is a session override, not an implicit persistent change.
    if (!linkUrl.isEmpty() && validLinkUrl(linkUrl)) m_linkUrl = linkUrl;
    if (m_linkUrl.isEmpty()) setStatus("Set an optional HTTPS link to open in your browser.");
}

void Bridge::setStatus(const QString &status) {
    if (m_status == status) return;
    m_status = status;
    emit statusChanged();
}

void Bridge::setWindow(QWindow *window, bool reset) {
    m_window = window;
    if (!m_smokeTest && !reset && m_settings.contains("position")) {
        window->setPosition(m_settings.value("position").toPoint());
        ensureVisible();
    } else resetPosition();
    connect(qGuiApp, &QGuiApplication::screenRemoved, this, [this](QScreen *) {
        QTimer::singleShot(0, this, [this]() { ensureVisible(); });
    });
    for (auto *screen : QGuiApplication::screens()) {
        connect(screen, &QScreen::availableGeometryChanged, this, [this]() { ensureVisible(); });
    }
    connect(qGuiApp, &QGuiApplication::screenAdded, this, [this](QScreen *screen) {
        connect(screen, &QScreen::availableGeometryChanged, this, [this]() { ensureVisible(); });
        ensureVisible();
    });
}

void Bridge::setPetScale(double scale) {
    if (!std::isfinite(scale)) return;
    scale = std::clamp(scale, 0.75, 2.0);
    if (qFuzzyCompare(m_scale, scale)) return;
    m_scale = scale;
    if (!m_smokeTest) m_settings.setValue("scale", scale);
    emit petScaleChanged();
    QTimer::singleShot(0, this, [this]() { ensureVisible(); savePosition(); });
}

bool Bridge::saveLinkUrl(const QString &input) {
    const QString url = input.trimmed();
    if (!url.isEmpty() && !validLinkUrl(url)) {
        setStatus("Use a complete HTTPS URL without credentials, a query, or a fragment.");
        return false;
    }
    m_linkUrl = url;
    if (!m_smokeTest) {
        if (url.isEmpty()) m_settings.remove("linkUrl");
        else m_settings.setValue("linkUrl", url);
        m_settings.sync();
        const bool privateFile = QFile::setPermissions(m_settings.fileName(), QFileDevice::ReadOwner | QFileDevice::WriteOwner);
        if (m_settings.status() != QSettings::NoError || !privateFile) {
            setStatus("Using this URL for now, but could not save settings. Check your config folder permissions.");
            emit linkUrlChanged();
            return false;
        }
    }
    emit linkUrlChanged();
    setStatus(url.isEmpty() ? "Browser shortcut cleared." : "Browser shortcut saved. Open link uses your normal browser.");
    return true;
}

bool Bridge::openLink() {
    if (m_smokeTest) { setStatus("Smoke test: opening a browser is disabled."); return false; }
    if (!validLinkUrl(m_linkUrl)) {
        setStatus("Set a browser shortcut first.");
        return false;
    }
    if (!QDesktopServices::openUrl(QUrl(m_linkUrl, QUrl::StrictMode))) {
        setStatus("Your desktop could not launch the browser. Check the default HTTPS handler.");
        return false;
    }
    setStatus("Sent the link to your default browser.");
    return true;
}

void Bridge::beginDrag() {
    if (!m_window) return;
    m_dragPointer = QCursor::pos();
    m_dragWindow = m_window->position();
}

int Bridge::drag() {
    if (!m_window) return 0;
    const QPoint next = m_dragWindow + QCursor::pos() - m_dragPointer;
    const int dx = next.x() - m_window->x();
    m_window->setPosition(next);
    return dx;
}

void Bridge::endDrag() { ensureVisible(); savePosition(); }

void Bridge::savePosition() {
    if (m_window && !m_smokeTest) m_settings.setValue("position", m_window->position());
}

void Bridge::resetPosition() {
    if (!m_window || !QGuiApplication::primaryScreen()) return;
    const QRect area = QGuiApplication::primaryScreen()->availableGeometry();
    m_window->setPosition(area.right() - m_window->width() - 24,
                         area.bottom() - m_window->height() - 24);
    ensureVisible();
    savePosition();
}

void Bridge::ensureVisible() {
    if (!m_window) return;
    QScreen *screen = QGuiApplication::screenAt(m_window->geometry().center());
    if (!screen) screen = QGuiApplication::primaryScreen();
    if (!screen) return;
    const QRect area = screen->availableGeometry();
    m_window->setPosition(std::clamp(m_window->x(), area.left(), std::max(area.left(), area.right() - m_window->width() + 1)),
                         std::clamp(m_window->y(), area.top(), std::max(area.top(), area.bottom() - m_window->height() + 1)));
}

void Bridge::placeControls(QWindow *controls) {
    if (!controls || !m_window) return;
    auto *screen = m_window->screen();
    if (!screen) screen = QGuiApplication::primaryScreen();
    if (!screen) return;
    const QRect area = screen->availableGeometry();
    controls->setPosition(std::clamp(m_window->x() - controls->width() + m_window->width(), area.left(), std::max(area.left(), area.right() - controls->width() + 1)),
                          std::clamp(m_window->y() - controls->height(), area.top(), std::max(area.top(), area.bottom() - controls->height() + 1)));
}

extern "C" int deskling_run(int argc, char **argv, const char *linkUrl,
                        bool resetPosition, bool smokeTest, const char *capturePath, const char *packPath) {
    // New local settings and captures should not expose conversation identifiers
    // to other accounts on a shared machine. This only affects this process.
    ::umask(S_IRWXG | S_IRWXO);
    // Real runs target X11 explicitly. Headless rendering is opt-in for tests.
    if (smokeTest) {
        if (qEnvironmentVariableIsEmpty("QT_QPA_PLATFORM")) qputenv("QT_QPA_PLATFORM", "offscreen");
        if (qEnvironmentVariableIsEmpty("QT_QUICK_BACKEND")) qputenv("QT_QUICK_BACKEND", "software");
    } else {
        if (qEnvironmentVariableIsEmpty("DISPLAY")) {
            qCritical("Deskling needs an X11 DISPLAY. Run it inside your desktop session, or use --smoke-test.");
            return 2;
        }
        qputenv("QT_QPA_PLATFORM", "xcb");
    }
    // A transparent native surface must be selected before creating the window.
    QQuickWindow::setDefaultAlphaBuffer(true);
    QGuiApplication app(argc, argv);
    QCoreApplication::setOrganizationName("Deskling");
    QCoreApplication::setApplicationName("Deskling Desktop");
    QCoreApplication::setApplicationVersion("0.1.0");
    app.setQuitOnLastWindowClosed(false);
    initResources(); // Pull the resources object out of the static archive.
    // Match packaging/linux/deskling.desktop and set the default for all windows.
    // Embedded resources also work when launching a build from another directory.
    QGuiApplication::setDesktopFileName(QStringLiteral("deskling"));
    QGuiApplication::setWindowIcon(applicationIcon());

    QVariantMap characterPack;
    QString packError;
    if (!loadCharacterPack(packPath ? QString::fromUtf8(packPath) : QString(), characterPack, packError)) {
        qCritical().noquote() << "Invalid character pack:" << packError;
        return 8;
    }
    Bridge bridge(smokeTest, linkUrl ? QString::fromUtf8(linkUrl) : QString());
    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty("bridge", &bridge);
    engine.rootContext()->setContextProperty("characterPack", characterPack);
    engine.load(QUrl(QStringLiteral("qrc:/deskling/Main.qml")));
    if (engine.rootObjects().isEmpty()) return 3;
    auto *window = qobject_cast<QQuickWindow *>(engine.rootObjects().constFirst());
    if (!window) return 3;
    if (smokeTest) {
        for (const int size : iconSizes) {
            const QImage image(QStringLiteral(":/deskling/icons/deskling-%1.png").arg(size));
            if (image.size() != QSize(size, size) ||
                !window->icon().availableSizes().contains(QSize(size, size)) ||
                window->icon().pixmap(QSize(size, size)).isNull()) {
                qCritical("SMOKE FAIL: application icon missing or invalid at %dpx", size);
                return 9;
            }
        }
        qInfo("SMOKE PASS: all eight embedded Kipwick icon sizes decode and reach the Qt window. Desktop launcher appearance still requires desktop checks.");
    }
    if (smokeTest) window->setProperty("paused", true);
    bridge.setWindow(window, resetPosition);
    QObject::connect(&app, &QCoreApplication::aboutToQuit, &bridge, &Bridge::savePosition);

    QSharedPointer<QQuickItemGrabResult> smokeGrab;
    QTimer smokeDeadline;
    if (smokeTest) {
        const QString capture = capturePath ? QString::fromUtf8(capturePath) : QString();
        smokeDeadline.setSingleShot(true);
        QObject::connect(&smokeDeadline, &QTimer::timeout, &app, [&app]() {
            qCritical("SMOKE FAIL: item capture did not complete within 5 seconds");
            app.exit(7);
        });
        smokeDeadline.start(5000);
        QTimer::singleShot(1200, &app, [&app, window, capture, &smokeGrab, &smokeDeadline]() {
            auto *sprite = window->findChild<QQuickItem *>(QStringLiteral("desklingSprite"));
            if (!sprite) { qCritical("SMOKE FAIL: desklingSprite item missing"); app.exit(4); return; }
            // Qt 6.4's offscreen software window backing store is RGB32, so
            // grabWindow() cannot test alpha. Capture the rendered sprite layer.
            // This tests item RGBA, not native X11 compositing or window flags.
            smokeGrab = sprite->grabToImage();
            if (!smokeGrab) { qCritical("SMOKE FAIL: item capture could not start"); app.exit(4); return; }
            QObject::connect(smokeGrab.data(), &QQuickItemGrabResult::ready, &app,
                             [&app, capture, &smokeGrab, &smokeDeadline]() {
                smokeDeadline.stop();
                const QImage image = smokeGrab->image();
                if (image.isNull()) { qCritical("SMOKE FAIL: item capture returned no image"); app.exit(4); return; }
                // Keep failure evidence instead of exiting before saving it.
                if (!capture.isEmpty() && !image.save(capture)) { qCritical("SMOKE FAIL: could not save capture"); app.exit(6); return; }
                int opaquePixels = 0, transparentPixels = 0;
                for (int y = 0; y < image.height(); ++y) for (int x = 0; x < image.width(); ++x) {
                    const int alpha = image.pixelColor(x, y).alpha();
                    opaquePixels += alpha > 128;
                    transparentPixels += alpha == 0;
                }
                qInfo("SMOKE DIAGNOSTIC: Qt=%s platform=%s size=%dx%d format=%d has_alpha=%s opaque=%d transparent=%d",
                      qVersion(), qPrintable(QGuiApplication::platformName()), image.width(), image.height(),
                      static_cast<int>(image.format()), image.hasAlphaChannel() ? "yes" : "no", opaquePixels, transparentPixels);
                if (opaquePixels < 1) { qCritical("SMOKE FAIL: rendered sprite pixels missing"); app.exit(5); return; }
                if (!image.hasAlphaChannel() || transparentPixels < 1) { qCritical("SMOKE FAIL: rendered item transparency missing"); app.exit(5); return; }
                qInfo("SMOKE PASS: QML sprite item rendered with RGBA transparency. Native X11 transparency, animation, drag, and stacking still require desktop checks. No browser opened or settings changed.");
                app.exit(0);
            });
        });
    }
    return app.exec();
}
