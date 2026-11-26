### KAKURU

KAKURU adalah aplikasi berbasis Rust untuk Windows dan Android yang bisa kamu gunakan buat berbagai perhitungan matematika sehari-hari.

Tampilan aplikasinya dibuat menggunakan egui/eframe, sedangkan semua perhitungan dilakukan langsung di perangkat kamu, jadi nggak perlu koneksi internet.

### Persyaratan

* Rust stable

* Windows: Toolchain GCC MinGW-w64

* Android: Android SDK, NDK, dan `cargo-apk`

Instal `cargo-apk` dengan perintah berikut:

```shell
cargo install cargo-apk
```

### Menjalankan dan Menguji di Windows

Jalankan aplikasi:

```shell
cargo run
```

Jalankan pengujian:

```shell
cargo test
```

### Build Aplikasi

Untuk membuat versi rilis di Windows:

```shell
cargo build --release
```

Untuk membuat APK Android:

```shell
cargo apk build --lib
```

### Catatan

* Pengaturan metadata paket Android dan target ARM yang didukung sudah tersedia di `Cargo.toml`.

* Formulir masukan akan membuka aplikasi email di perangkat kamu.

* Rumus perhitungan yang sudah ada tetap dipertahankan.

* Fitur SPLDV masih berupa placeholder untuk pengembangan selanjutnya.

* APK yang dihasilkan menggunakan tanda tangan pengembangan. Kalau ingin menerbitkan aplikasi ke toko aplikasi, kamu perlu mengatur keystore rilis di `Cargo.toml` terlebih dahulu.

Semua perhitungan tetap berjalan secara lokal di perangkat, jadi data perhitungan kamu nggak perlu dikirim ke server.
