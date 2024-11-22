use colored::*;
use qrcode::render::unicode;
use qrcode::QrCode;

pub fn print_header() {
    let ko_fi_link = "https://ko-fi.com/ghozimahdi";
    let trakteer_link = "https://teer.id/ghozimahdi";
    let github_sponsor_link = "https://github.com/sponsors/mbahgojol";

    let header = format!(
        "\
Dibuat oleh  : Ghozi Mahdi
Profesi      : Mobile Software Engineer
LinkedIn     : https://id.linkedin.com/in/ghozimahdi
YouTube      : https://www.youtube.com/@mbahgojol
Github       : https://github.com/mbahgojol
Email        : ghozimahdi@carpkingdom.com
Website      : www.mbahgojol.com

Dukung Karya Saya:
Ko-fi      -> {}
Trakteer   -> {}
Github Sponsor   -> {}
",
        ko_fi_link, trakteer_link, github_sponsor_link
    );

    println!("{}", header.green());
    let kofi_qr = generate_qr_code(ko_fi_link);
    println!("{}\n{}", "QR Code Ko-fi :".bold(), kofi_qr);
}

fn generate_qr_code(data: &str) -> String {
    let code = QrCode::new(data).unwrap();
    code.render::<unicode::Dense1x2>()
        .quiet_zone(false)
        .module_dimensions(1, 1)
        .build()
}
