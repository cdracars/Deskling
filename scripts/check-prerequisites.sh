#!/bin/sh
# Read-only probe. Never installs packages or changes desktop configuration.
set -u
missing=0
for tool in cargo rustc c++ ar pkg-config; do
    if command -v "$tool" >/dev/null 2>&1; then
        printf 'OK   %s\n' "$tool"
    else
        printf 'MISS %s\n' "$tool"
        missing=1
    fi
done
if command -v pkg-config >/dev/null 2>&1; then
    for module in Qt6Core Qt6Gui Qt6Qml Qt6Quick; do
        if pkg-config --atleast-version=6.4 "$module"; then
            printf 'OK   %s %s\n' "$module" "$(pkg-config --modversion "$module")"
        else
            printf 'MISS %s >= 6.4 development metadata\n' "$module"
            missing=1
        fi
    done
    libexec=$(pkg-config --variable=libexecdir Qt6Core 2>/dev/null || true)
    for tool in moc rcc; do
        override=''
        case "$tool" in moc) override=${QT_MOC:-};; rcc) override=${QT_RCC:-};; esac
        found=''
        for candidate in "$override" "$libexec/$tool" "/usr/lib/qt6/libexec/$tool" "/usr/lib64/qt6/libexec/$tool" "/usr/lib/x86_64-linux-gnu/qt6/libexec/$tool"; do
            if [ -n "$candidate" ] && [ -x "$candidate" ]; then found=$candidate; break; fi
        done
        if [ -n "$found" ]; then printf 'OK   %s: %s\n' "$tool" "$found";
        else printf 'MISS Qt 6 %s (see README)\n' "$tool"; missing=1; fi
    done
fi
if [ -n "${DISPLAY:-}" ]; then printf 'OK   X11 DISPLAY is set\n';
else printf 'NOTE DISPLAY is unset; GUI launch needs an X11 desktop, smoke test does not\n'; fi
printf '\nQML runtime modules and compositor behavior are checked at runtime.\n'
exit "$missing"
