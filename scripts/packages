#!/usr/bin/env bash
set -euo pipefail
RAPISCM_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$RAPISCM_ROOT"

TYPE="${1:-help}"
VERSION="$(cargo metadata --format-version=1 --no-deps 2>/dev/null | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")"

case "$TYPE" in
  deb)
    echo "building .deb..."
    cargo build --release
    PKG_ROOT="$(mktemp -d)"
    mkdir -p "${PKG_ROOT}/usr/local/bin"
    cp target/release/rapiscm "${PKG_ROOT}/usr/local/bin/"
    mkdir -p "${PKG_ROOT}/DEBIAN"
    cat > "${PKG_ROOT}/DEBIAN/control" <<EOF
Package: rapiscm
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: amd64
Maintainer: rapiscm
Description: Rust API scanner
 Point at an API spec or a URL to scan endpoints.
EOF
    dpkg-deb --build "${PKG_ROOT}" "rapiscm_${VERSION}_amd64.deb"
    rm -rf "${PKG_ROOT}"
    echo "created: rapiscm_${VERSION}_amd64.deb"
    ;;

  rpm)
    echo "building .rpm..."
    cargo build --release
    mkdir -p rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
    cat > rpmbuild/SPECS/rapiscm.spec <<EOF
Name: rapiscm
Version: ${VERSION}
Release: 1
Summary: Rust API scanner
License: MIT
%description
API scanner that works with OpenAPI specs and URLs.
%install
mkdir -p %{buildroot}%{_bindir}
cp %{_topdir}/../target/release/rapiscm %{buildroot}%{_bindir}/
%files
%{_bindir}/rapiscm
EOF
    rpmbuild --define "_topdir ${PWD}/rpmbuild" -bb rpmbuild/SPECS/rapiscm.spec
    find rpmbuild/RPMS -name "*.rpm" -exec mv {} . \;
    rm -rf rpmbuild
    echo "done"
    ;;

  tarball)
    echo "building tarball..."
    cargo build --release
    tar czf "rapiscm_${VERSION}_linux_amd64.tar.gz" -C target/release rapiscm
    echo "created: rapiscm_${VERSION}_linux_amd64.tar.gz"
    ;;

  help|*)
    echo "Usage: $0 [deb|rpm|tarball]"
    exit 1
    ;;
esac
