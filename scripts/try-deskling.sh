#!/bin/sh
# Opt-in local trial. Do not install, enable autostart, or reuse real user settings.
set -eu

usage() {
    printf '%s\n' \
        'Try Deskling on an Ubuntu 24.04/X11 desktop (development characters only).' \
        'Usage: sh scripts/try-deskling.sh [--character kipwick|gremlin|diagnostic]' \
        '       sh scripts/try-deskling.sh --help' \
        'Builds locally only if the release executable is missing.' \
        'Each run uses fresh temporary settings, removed when Deskling exits.'
}

character=kipwick
case $# in
    0) ;;
    1)
        if [ "$1" = '--help' ] || [ "$1" = '-h' ]; then usage; exit 0; fi
        usage >&2; exit 2 ;;
    2)
        if [ "$1" != '--character' ]; then usage >&2; exit 2; fi
        case "$2" in
            kipwick|gremlin|diagnostic) character=$2 ;;
            *) printf 'Unknown character: %s\n' "$2" >&2; usage >&2; exit 2 ;;
        esac ;;
    *) usage >&2; exit 2 ;;
esac

if [ -z "${DISPLAY:-}" ]; then
    printf '%s\n' 'Deskling needs a running X11 desktop with DISPLAY set.' \
        'Run this script in your desktop terminal; headless CI uses --smoke-test instead.' >&2
    exit 2
fi

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
binary="$root/target/release/deskling-desktop"
if [ ! -x "$binary" ]; then
    printf '%s\n' 'Deskling release binary not found. Checking build prerequisites...'
    if ! sh "$root/scripts/check-prerequisites.sh"; then
        printf '%s\n' 'Missing prerequisites; see README.md. No software was installed.' >&2
        exit 1
    fi
    if ! (cd "$root" && cargo build --offline --locked --release); then
        printf '%s\n' 'Local build failed; see errors above. No software was installed.' >&2
        exit 1
    fi
fi
if [ ! -x "$binary" ]; then
    printf '%s\n' 'Build finished without a runnable Deskling release binary.' >&2
    exit 1
fi

pack=''
case "$character" in
    kipwick) pack="$root/assets/characters/kipwick-alpha/pack.json" ;;
    gremlin) pack="$root/assets/characters/cyber-gremlin/pack.json" ;;
esac
if [ -n "$pack" ] && [ ! -r "$pack" ]; then
    printf 'Character pack unavailable: %s\n' "$pack" >&2
    exit 1
fi

trial_config=$(mktemp -d "${TMPDIR:-/tmp}/deskling-trial.XXXXXXXX")
trap 'rm -rf -- "$trial_config"' 0
trap 'exit 130' 2
trap 'exit 143' 15
printf 'Starting %s with isolated temporary settings. Right-click for Pause and Quit.\n' "$character"
status=0
if [ -n "$pack" ]; then
    XDG_CONFIG_HOME="$trial_config" "$binary" --pack "$pack" --reset-position || status=$?
else
    XDG_CONFIG_HOME="$trial_config" "$binary" --reset-position || status=$?
fi
exit "$status"
