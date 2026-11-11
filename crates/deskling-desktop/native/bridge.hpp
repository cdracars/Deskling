#pragma once
#include <QImage>
#include <QMutex>
#include <QObject>
#include <QPointer>
#include <QPoint>
#include <QSettings>
#include <QWindow>

class Bridge final : public QObject {
    Q_OBJECT
    Q_PROPERTY(QString linkUrl READ linkUrl NOTIFY linkUrlChanged)
    Q_PROPERTY(QString status READ status NOTIFY statusChanged)
    Q_PROPERTY(double petScale READ petScale WRITE setPetScale NOTIFY petScaleChanged)
    Q_PROPERTY(bool smokeTest READ smokeTest CONSTANT)
    Q_PROPERTY(bool screenReviewReady READ screenReviewReady NOTIFY screenReviewChanged)
    Q_PROPERTY(int screenReviewVersion READ screenReviewVersion NOTIFY screenReviewChanged)
public:
    explicit Bridge(bool smokeTest, const QString &linkUrl, QObject *parent = nullptr);
    QString linkUrl() const { return m_linkUrl; }
    QString status() const { return m_status; }
    double petScale() const { return m_scale; }
    bool smokeTest() const { return m_smokeTest; }
    bool screenReviewReady() const;
    int screenReviewVersion() const { return m_screenReviewVersion; }
    QImage screenReviewImage() const;
    void setWindow(QWindow *window, bool resetPosition);
    void setPetScale(double scale);
    Q_INVOKABLE bool saveLinkUrl(const QString &url);
    Q_INVOKABLE bool openLink();
    Q_INVOKABLE bool captureScreenReview();
    Q_INVOKABLE bool copyScreenReview();
    Q_INVOKABLE void clearScreenReview();
    Q_INVOKABLE void beginDrag();
    Q_INVOKABLE int drag();
    Q_INVOKABLE void endDrag();
    Q_INVOKABLE void resetPosition();
    Q_INVOKABLE void placeControls(QWindow *controls);
    void savePosition();
    void ensureVisible();
signals:
    void linkUrlChanged();
    void statusChanged();
    void petScaleChanged();
    void screenReviewChanged();
private:
    void setStatus(const QString &status);
    QSettings m_settings;
    QPointer<QWindow> m_window;
    QPoint m_dragPointer;
    QPoint m_dragWindow;
    QString m_linkUrl;
    QString m_status;
    double m_scale = 1.0;
    bool m_smokeTest = false;
    mutable QMutex m_screenReviewMutex;
    QImage m_screenReview;
    int m_screenReviewVersion = 0;
};
