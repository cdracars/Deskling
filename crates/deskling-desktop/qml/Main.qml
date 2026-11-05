import QtQuick
import QtQuick.Window
import QtQuick.Controls
import QtQuick.Layouts

Window {
    id: pet
    objectName: "desklingWindow"
    title: "Deskling · " + characterPack.name
    visible: true
    width: Math.round(characterPack.cellWidth * bridge.petScale)
    height: Math.round(characterPack.cellHeight * bridge.petScale)
    color: "transparent"
    flags: Qt.Tool | Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint

    property bool paused: false
    property int selectedAnimation: characterPack.idleIndex
    property int activeAnimation: characterPack.idleIndex
    property int frame: 0
    property bool dragging: false
    property bool waving: false
    readonly property var animations: characterPack.animations
    readonly property var frameCounts: {
        var values = []
        for (var i = 0; i < animations.length; ++i) values.push(animations[i].frames)
        return values
    }
    readonly property var frameIntervals: {
        var values = []
        for (var i = 0; i < animations.length; ++i) values.push(animations[i].intervalMs)
        return values
    }
    readonly property var animationNames: {
        var values = []
        for (var i = 0; i < animations.length; ++i) values.push(animations[i].label)
        return values
    }
    readonly property var automaticBehaviors: characterPack.behaviors
    property bool autonomous: automaticBehaviors.length > 0
    property int gestureLoopsRemaining: 0
    property int automaticWaitMs: randomWait(true)

    function randomWait(first) {
        return first ? 5000 + Math.floor(Math.random() * 5001)
                     : 20000 + Math.floor(Math.random() * 25001)
    }
    function resetQuietGap() {
        automaticWaitMs = randomWait(false)
    }
    function returnToBase() {
        waving = false
        gestureLoopsRemaining = 0
        activeAnimation = autonomous ? characterPack.idleIndex : selectedAnimation
        frame = 0
        resetQuietGap()
    }
    function setAutonomous(enabled) {
        autonomous = enabled
        if (enabled) selectedAnimation = characterPack.idleIndex
        returnToBase()
    }
    function startGesture(index, loops) {
        activeAnimation = index
        frame = 0
        gestureLoopsRemaining = loops
        waving = index === characterPack.waveIndex
    }
    function startAutomaticGesture() {
        if (automaticBehaviors.length === 0) { resetQuietGap(); return }
        var total = 0
        for (var i = 0; i < automaticBehaviors.length; ++i)
            total += automaticBehaviors[i].weight
        var pick = Math.random() * total
        for (var j = 0; j < automaticBehaviors.length; ++j) {
            pick -= automaticBehaviors[j].weight
            if (pick < 0) {
                startGesture(automaticBehaviors[j].animation, automaticBehaviors[j].loops)
                return
            }
        }
    }
    function advanceAnimation() {
        // Also guard the handler against a queued tick after pause/hide.
        if (paused || !visible) return
        var elapsed = frameIntervals[activeAnimation]
        var next = frame + 1
        if (next >= frameCounts[activeAnimation] && gestureLoopsRemaining > 0) {
            gestureLoopsRemaining -= 1
            if (gestureLoopsRemaining === 0) {
                returnToBase()
                return
            }
        }
        frame = next % frameCounts[activeAnimation]
        if (autonomous && !dragging && gestureLoopsRemaining === 0) {
            automaticWaitMs -= elapsed
            if (automaticWaitMs <= 0) startAutomaticGesture()
        }
    }
    function beginGestureDrag() {
        dragging = true
        waving = false
        gestureLoopsRemaining = 0
        resetQuietGap()
    }
    function chooseAnimation(index) {
        autonomous = false
        selectedAnimation = index
        returnToBase()
    }
    function wave() {
        if (paused || dragging) return
        resetQuietGap()
        startGesture(characterPack.waveIndex, 1)
    }
    function showControls() {
        bridge.placeControls(controls)
        controls.show()
        controls.raise()
        controls.requestActivate()
    }
    function openLink() {
        if (!bridge.openLink()) showControls()
    }

    Item {
        id: sprite
        objectName: "desklingSprite"
        anchors.fill: parent
        Image {
            anchors.fill: parent
            visible: characterPack.source.length > 0
            source: characterPack.source
            sourceClipRect: Qt.rect(pet.frame * characterPack.cellWidth,
                                    pet.animations[pet.activeAnimation].row * characterPack.cellHeight,
                                    characterPack.cellWidth, characterPack.cellHeight)
            fillMode: Image.Stretch
            smooth: true
            cache: true
            asynchronous: false
        }
        // Code-drawn diagnostic geometry, not the project's final mascot.
        Item {
            anchors.centerIn: parent
            width: 128; height: 160
            scale: Math.min(parent.width / width, parent.height / height)
            visible: characterPack.source.length === 0
            Rectangle { x: 25; y: 139; width: 78; height: 10; radius: 6; color: "#253949"; opacity: 0.35 }
            Item {
                x: 0
                y: characterPack.source.length === 0 && pet.activeAnimation === 3 ? -[0, 7, 14, 7, 0][pet.frame % 5] : 0
                Rectangle { x: 28; y: 51; width: 72; height: 86; radius: 28; color: pet.activeAnimation === 2 ? "#8276c9" : "#4f9baf"; border.width: 2; border.color: "#b5edf2" }
                Rectangle { x: 43; y: 79; width: 8; height: pet.activeAnimation === 0 && pet.frame === 3 ? 2 : 11; radius: 4; color: "#112b34" }
                Rectangle { x: 77; y: 79; width: 8; height: pet.activeAnimation === 0 && pet.frame === 3 ? 2 : 11; radius: 4; color: "#112b34" }
                Rectangle { x: 54; y: 106; width: 20; height: 4; radius: 2; color: "#d2f2f4" }
                Rectangle {
                    x: 95; y: 77; width: 13; height: 32; radius: 6
                    color: "#4f9baf"; transformOrigin: Item.Top
                    rotation: characterPack.source.length === 0 && pet.activeAnimation === 1 ? [-20, -105, -70, -20][pet.frame % 4] : -20
                }
            }
        }
    }

    Timer {
        id: animationClock
        interval: pet.frameIntervals[pet.activeAnimation]
        running: !pet.paused && pet.visible
        repeat: true
        onTriggered: pet.advanceAnimation()
    }

    MouseArea {
        id: hitArea
        anchors.fill: parent
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        hoverEnabled: true
        cursorShape: pressed && pet.dragging ? Qt.ClosedHandCursor : Qt.OpenHandCursor
        property point pressPoint: Qt.point(0, 0)
        property bool moved: false

        onPressed: function(mouse) {
            if (mouse.button !== Qt.LeftButton) return
            pressPoint = Qt.point(mouse.x, mouse.y)
            moved = false
            bridge.beginDrag()
        }
        onPositionChanged: function(mouse) {
            if (!(pressedButtons & Qt.LeftButton)) return
            if (!moved && Math.abs(mouse.x - pressPoint.x) + Math.abs(mouse.y - pressPoint.y) < Qt.styleHints.startDragDistance) return
            if (!moved) pet.beginGestureDrag()
            moved = true
            var dx = bridge.drag()
            var next = dx < 0 ? characterPack.dragLeftIndex : characterPack.dragRightIndex
            if (!pet.paused && dx !== 0 && pet.activeAnimation !== next) { pet.activeAnimation = next; pet.frame = 0 }
        }
        onReleased: function(mouse) {
            if (mouse.button !== Qt.LeftButton) return
            pet.dragging = false
            bridge.endDrag()
            if (moved) pet.returnToBase()
        }
        onCanceled: {
            pet.dragging = false
            bridge.endDrag()
            if (moved) pet.returnToBase()
        }
        onClicked: function(mouse) {
            if (mouse.button === Qt.RightButton) pet.showControls()
            else if (!moved) pet.wave()
        }
        onDoubleClicked: function(mouse) {
            if (mouse.button === Qt.LeftButton && !moved) pet.openLink()
        }
    }

    Shortcut { sequence: "Ctrl+O"; onActivated: pet.openLink() }
    Shortcut { sequence: "Ctrl+,"; onActivated: pet.showControls() }
    Shortcut { sequence: "Ctrl+Q"; onActivated: Qt.quit() }
    // WindowShortcut keeps typing in the separate URL field from pausing Deskling.
    Shortcut { sequence: "Space"; context: Qt.WindowShortcut; onActivated: pet.paused = !pet.paused }

    onClosing: Qt.quit()

    Window {
        id: controls
        objectName: "controlsWindow"
        title: "Deskling controls"
        width: 368
        height: Math.min(620, Screen.desktopAvailableHeight - 40)
        minimumWidth: 340
        minimumHeight: 360
        visible: false
        color: "#101722"
        flags: Qt.Tool | Qt.WindowTitleHint | Qt.WindowCloseButtonHint | Qt.WindowStaysOnTopHint

        Shortcut { sequence: "Escape"; context: Qt.WindowShortcut; onActivated: controls.close() }

        ScrollView {
            id: controlsScroll
            anchors.fill: parent
            anchors.margins: 20
            contentWidth: availableWidth
            clip: true
            ColumnLayout {
                width: controlsScroll.availableWidth
                spacing: 12

                RowLayout {
                    Layout.fillWidth: true
                    Text { text: characterPack.name; textFormat: Text.PlainText; color: "#62edf4"; font.pixelSize: 23; font.bold: true }
                    Item { Layout.fillWidth: true }
                    Text { text: "X11 · 0.1"; color: "#96a6ba"; font.pixelSize: 12 }
                }
                Text {
                    Layout.fillWidth: true
                    text: "A local desktop character engine.\nRight-click your character for controls."
                    color: "#ccd5e1"
                    wrapMode: Text.WordWrap
                    font.pixelSize: 14
                }
                Rectangle { Layout.fillWidth: true; height: 1; color: "#29364a" }
                Label { text: "OPTIONAL BROWSER SHORTCUT"; color: "#96a6ba"; font.pixelSize: 11; font.bold: true }
                TextField {
                    id: urlInput
                    objectName: "linkUrlInput"
                    Layout.fillWidth: true
                    text: bridge.linkUrl
                    placeholderText: "Paste an HTTPS link"
                    selectByMouse: true
                    onAccepted: bridge.saveLinkUrl(text)
                }
                RowLayout {
                    Layout.fillWidth: true
                    Button {
                        objectName: "saveLinkButton"
                        text: "Save link"
                        onClicked: bridge.saveLinkUrl(urlInput.text)
                    }
                    Button {
                        objectName: "openLinkButton"
                        text: "Open link ↗"
                        Layout.fillWidth: true
                        enabled: bridge.linkUrl.length > 0
                        onClicked: pet.openLink()
                    }
                }
                Text {
                    Layout.fillWidth: true
                    text: bridge.status
                    visible: text.length > 0
                    color: "#90e5e9"
                    wrapMode: Text.WordWrap
                    font.pixelSize: 12
                }
                Rectangle { Layout.fillWidth: true; height: 1; color: "#29364a" }
                Label { text: "ANIMATION"; color: "#96a6ba"; font.pixelSize: 11; font.bold: true }
                Switch {
                    objectName: "autonomousToggle"
                    text: "Automatic behavior"
                    checked: pet.autonomous
                    enabled: pet.automaticBehaviors.length > 0
                    onToggled: pet.setAutonomous(checked)
                }
                Text {
                    Layout.fillWidth: true
                    text: "Mostly idle, with occasional little gestures. Choosing an animation below turns this off."
                    color: "#96a6ba"
                    wrapMode: Text.WordWrap
                    font.pixelSize: 12
                }
                ComboBox {
                    objectName: "animationChoice"
                    Layout.fillWidth: true
                    model: pet.animationNames
                    currentIndex: pet.selectedAnimation
                    onActivated: function(index) { pet.chooseAnimation(index) }
                }
                RowLayout {
                    Layout.fillWidth: true
                    Button {
                        objectName: "pauseButton"
                        text: pet.paused ? "Resume" : "Pause"
                        onClicked: pet.paused = !pet.paused
                    }
                    Button { text: "Wave"; enabled: !pet.paused; onClicked: pet.wave() }
                    Button { text: "Reset position"; Layout.fillWidth: true; onClicked: bridge.resetPosition() }
                }
                RowLayout {
                    Label { text: "Size"; color: "#ccd5e1" }
                    Slider {
                        objectName: "sizeSlider"
                        Layout.fillWidth: true
                        from: 0.75; to: 2.0; stepSize: 0.25
                        value: bridge.petScale
                        onMoved: bridge.petScale = value
                    }
                    Label { text: Math.round(bridge.petScale * 100) + "%"; color: "#ccd5e1" }
                }
                Text {
                    Layout.fillWidth: true
                    text: "Drag Deskling to move. Click to wave. Double-click for your saved link. Right-click for controls.\n\nDecorative behavior only. No provider connectors are implemented. No messages, cookies, or credentials are read."
                    color: "#96a6ba"
                    wrapMode: Text.WordWrap
                    font.pixelSize: 12
                }
                RowLayout {
                    Layout.fillWidth: true
                    Button { text: "Quit Deskling"; onClicked: Qt.quit() }
                    Item { Layout.fillWidth: true }
                    Button { text: "Done"; onClicked: controls.close() }
                }
            }
        }
    }
}
