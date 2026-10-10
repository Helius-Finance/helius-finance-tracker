#!/bin/sh
# Install the helius binary from a GitHub release.
#
#   curl -fsSL https://raw.githubusercontent.com/Helius-Finance/helius-finance-tracker/main/install.sh | sh
#
# Environment overrides:
#   HELIUS_VERSION      release tag to install, such as v1.4.4 (default: latest)
#   HELIUS_INSTALL_DIR  directory that receives the binary (default: ~/.local/bin)

set -eu

REPO="Helius-Finance/helius-finance-tracker"
INSTALL_DOCS="https://github.com/$REPO#installation"

say() {
    printf '%s\n' "$1"
}

fail() {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

need() {
    command -v "$1" >/dev/null 2>&1 || fail "'$1' is required but was not found on PATH"
}

detect_platform() {
    os=$(uname -s)
    arch=$(uname -m)
    if [ "$os" != "Linux" ]; then
        fail "no prebuilt helius binary for $os. Build from source instead: $INSTALL_DOCS"
    fi
    case "$arch" in
        x86_64 | amd64) ;;
        *) fail "no prebuilt helius binary for $arch. Build from source instead: $INSTALL_DOCS" ;;
    esac
    PLATFORM="linux-x86_64"
}

resolve_version() {
    if [ -n "${HELIUS_VERSION:-}" ]; then
        case "$HELIUS_VERSION" in
            v*) VERSION=$HELIUS_VERSION ;;
            *) VERSION="v$HELIUS_VERSION" ;;
        esac
        return
    fi

    # GitHub redirects /releases/latest to /releases/tag/<tag>. Reading the
    # redirect avoids the rate-limited REST API.
    latest_url=$(curl -fsSLI -o /dev/null -w '%{url_effective}' "https://github.com/$REPO/releases/latest") ||
        fail "could not reach GitHub to look up the latest release"
    VERSION=${latest_url##*/}
    case "$VERSION" in
        v[0-9]*) ;;
        *) fail "could not determine the latest release from $latest_url" ;;
    esac
}

main() {
    need uname
    need curl
    need tar
    need sha256sum
    detect_platform
    resolve_version

    install_dir=${HELIUS_INSTALL_DIR:-$HOME/.local/bin}
    archive="helius-$VERSION-$PLATFORM.tar.gz"
    base_url="https://github.com/$REPO/releases/download/$VERSION"

    tmp_dir=$(mktemp -d)
    trap 'rm -rf "$tmp_dir"' EXIT
    trap 'exit 130' INT TERM

    say "Downloading helius $VERSION for $PLATFORM"
    curl -fsSL -o "$tmp_dir/$archive" "$base_url/$archive" ||
        fail "download failed: $base_url/$archive"
    curl -fsSL -o "$tmp_dir/$archive.sha256.txt" "$base_url/$archive.sha256.txt" ||
        fail "download failed: $base_url/$archive.sha256.txt"

    expected=$(cut -d ' ' -f 1 <"$tmp_dir/$archive.sha256.txt")
    actual=$(sha256sum "$tmp_dir/$archive" | cut -d ' ' -f 1)
    if [ "$expected" != "$actual" ]; then
        fail "checksum mismatch for $archive (expected $expected, got $actual)"
    fi

    tar -xzf "$tmp_dir/$archive" -C "$tmp_dir"
    binary="$tmp_dir/helius/helius"
    [ -f "$binary" ] || fail "$archive does not contain helius/helius"

    # Run the binary before installing it, so a host that cannot execute it
    # (musl, or glibc older than 2.34) keeps any existing install untouched.
    if ! version_line=$("$binary" --version 2>&1); then
        printf '%s\n' "$version_line" >&2
        fail "the prebuilt binary does not run on this system; it needs glibc 2.34 or newer. Use Docker or build from source instead: $INSTALL_DOCS"
    fi

    # Copy next to the target and rename, so replacing a running helius works.
    mkdir -p "$install_dir"
    staged="$install_dir/.helius.$$"
    cp "$binary" "$staged"
    chmod 755 "$staged"
    mv -f "$staged" "$install_dir/helius"

    say "Installed $version_line to $install_dir/helius"

    case ":$PATH:" in
        *":$install_dir:"*)
            found=$(command -v helius || true)
            if [ "$found" != "$install_dir/helius" ]; then
                say "Note: $found comes first on your PATH and shadows this install."
            fi
            say "Run 'helius' to get started."
            ;;
        *)
            say "$install_dir is not on your PATH. Add this line to your shell profile, then open a new terminal:"
            say ""
            say "    export PATH=\"$install_dir:\$PATH\""
            ;;
    esac
}

main "$@"
