
# **Ghozi Mahdi CLI**

CLI sederhana yang dibuat untuk mempermudah proses seleksi, pembuatan, dan pengelolaan modul dalam proyek **Flutter** yang menggunakan **Melos**. Aplikasi ini dirancang dengan tampilan yang user-friendly dan mendukung penggunaan kategori modul (Data, Domain, Feature, Other).

---

## **Fitur**
- Menampilkan daftar modul dari proyek Melos.
- Memungkinkan pemilihan modul secara spesifik atau membangun semua modul.
- Mendukung kategori modul:
  - **Data Modules**
  - **Domain Modules**
  - **Feature Modules**
  - **Other Modules**
- Dibangun menggunakan **Rust** untuk performa tinggi dan efisiensi.

---

## **Prasyarat**
Pastikan Anda memiliki prasyarat berikut sebelum menggunakan CLI ini:

1. **Rust**:
   - Rust harus terinstal di sistem Anda.
   - Instal Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. **Melos**:
   - Pastikan Melos sudah diinstal di proyek Flutter Anda.
   - Instal Melos:
     ```bash
     dart pub global activate melos
     ```

3. **Flutter**:
   - Pastikan Flutter sudah terinstal di sistem Anda.
   - Instal Flutter: [https://flutter.dev/docs/get-started/install](https://flutter.dev/docs/get-started/install)

---

## **Instalasi**
1. Clone repository ini ke lokal Anda:
   ```bash
   git clone https://github.com/mbahgojol/ghozimahdi_CLI.git
   ```
2. Masuk ke folder proyek:
   ```bash
   cd ghozimahdi_CLI
   ```
3. Build aplikasi CLI untuk digunakan:
   ```bash
   cargo build --release
   tar -czvf gm-v106.tar.gz gm
   shasum -a 256 gm.tar.gz
   ```

4. Copy file binary ke direktori global:
   ```bash
   cp target/release/gm /usr/local/bin/
   ```

5. Jalankan aplikasi CLI dari proyek Flutter Anda:
   ```bash
   gm
   ```

---

## **Cara Penggunaan**
1. Jalankan perintah berikut di dalam direktori proyek Flutter Anda:
   ```bash
   gm
   ```

2. Pilih modul yang ingin dibangun:
   - Untuk membangun semua modul, ketik `0`.
   - Untuk membangun modul tertentu, masukkan nomor modul yang dipisahkan dengan koma (contoh: `3,5,8`).

3. CLI akan otomatis menjalankan perintah **`melos`** sesuai dengan modul yang dipilih.

---

## **Contoh Tampilan**
```
 _____ _              _  ___  ___      _         _ _ 
|  __ \ |            (_) |  \/  |     | |       | (_)
| |  \/ |__   ___ _____  | .  . | __ _| |__   __| |_ 
| | __| '_ \ / _ \_  / | | |\/| |/ _` | '_ \ / _` | |
| |_\ \ | | | (_) / /| | | |  | | (_| | | | | (_| | |
 \____/_| |_|\___/___|_| \_|  |_/\__,_|_| |_|\__,_|_|
                            
Dibuat oleh  : Ghozi Mahdi
Profesi      : Mobile Software Engineer
LinkedIn     : https://id.linkedin.com/in/ghozimahdi
YouTube      : https://www.youtube.com/@mbahgojol
Github       : https://github.com/mbahgojol
Email        : ghozimahdi@carpkingdom.com
Website      : www.mbahgojol.com
Number       : -

+------------------------------+------------------------------+------------------------------+------------------------------+
| Data Modules                 | Domain Modules               | Feature Modules              | Other Modules                |
+------------------------------+------------------------------+------------------------------+------------------------------+
| 3. data_analytic             | 18. domain_analytic          | 33. feature_booster          | 1. common                    |
| 4. data_auth                 | 19. domain_auth              | 34. feature_common           | 2. croppy                    |
| ...                          | ...                          | ...                          | ...                          |
+------------------------------+------------------------------+------------------------------+------------------------------+
Total modules available: 49, Pilih module (contoh: 1,2,3 atau 0 untuk semua):
```

---

## **Kontribusi**
Jika Anda ingin berkontribusi pada proyek ini, silakan fork repository ini dan buat pull request. Semua jenis kontribusi diterima dengan senang hati!

---

## **Lisensi**
Proyek ini dilisensikan di bawah lisensi MIT. Silakan lihat file `LICENSE` untuk detailnya.

---
