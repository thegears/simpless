# Maintainer: thegears <github/thegears>
pkgname=simpless
pkgver=0.1.0
pkgrel=1
pkgdesc="Yet another screenshot tool written in Rust"
arch=('x86_64')
url="https://github.com/thegears/simpless"
license=('MIT')
depends=('gcc-libs' 'libxcb')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('5a099e51edd1be16445462a8f8e82b0ec7ffb929a8052dbb46524923f5ad17b5')

prepare() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --bin "$pkgname"
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
