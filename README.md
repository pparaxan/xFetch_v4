<img src="assets/xFetch.jpg" align="right" width="300">

### xFetch

World's _fastest_* and simplest fetch.

![Made with Rust](https://img.shields.io/badge/made%20with%20rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Only for Arch Linux](https://img.shields.io/badge/Only%20For%20Arch%20Linux-1793D1?logo=arch-linux&logoColor=fff&style=for-the-badge)
![Binary Size](https://img.shields.io/badge/Binary_Size-Miniscule_(100%20kb)-7ED321?logo=hack-the-box&logoColor=fff&style=for-the-badge)


## Arch User Repository

```sh
<AUR> -S xfetch-bin
```

### Build

```sh
git clone https://codeberg.org/XDR/xFetch.git
cd xFetch
RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-unknown-linux-gnu --profile release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```
