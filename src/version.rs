use clap::Command as ClapCommand;

pub fn handle_version() {
    let _version_info = ClapCommand::new("gm")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Ghozi Mahdi CLI untuk mengelola modul Melos\n\n\
            Prasyarat:\n\
            1. Pastikan Anda telah menginstal Melos.\n\
            2. Jalankan perintah 'melos list' di proyek Flutter Anda untuk memastikan Melos berfungsi.\n\
            3. Jika mengalami kesalahan 'Error: Gagal mendapatkan daftar modul.', periksa apakah proyek Anda dikonfigurasi dengan benar untuk Melos.\n\n\
            Cara Penggunaan:\n\
            1. Navigasikan ke direktori root proyek Flutter Anda.\n\
            2. Jalankan perintah 'gm' untuk memulai CLI.\n\
            3. Ikuti petunjuk di layar untuk memilih modul yang ingin Anda kelola.")
        .author("Ghozi Mahdi <ghozimahdi@carpkingdom.com>")
        .render_version();
}