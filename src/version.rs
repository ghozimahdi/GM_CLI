use colored::Colorize;
use qrcode::render::unicode;
use qrcode::QrCode;

pub fn show_info() {
    println!(
        "Prerequisites:\n\
            1. Ensure you have Melos installed:\n\
               - Visit https://melos.invertase.dev/ for installation instructions.\n\
               - Verify installation by running 'melos --version'.\n\
            2. Ensure you have Flutter installed:\n\
               - Visit https://flutter.dev/ for installation instructions.\n\
               - Verify installation by running 'flutter --version'.\n\
            3. Ensure your project contains a valid 'melos.yaml' file:\n\
               - This file should define the structure and configuration for your project modules.\n\n\
        Usage Instructions:\n\
            1. Navigate to the root directory of your Flutter project.\n\
            2. Run the 'gm' command to start the CLI.\n\
            3. Follow the on-screen instructions to select and manage your project modules."
    );
}

pub fn generate_saweria_qr_code() {
    let saweria_link = "https://saweria.co/ghozimahdi";
    let saweria_qr = generate_qr_code(saweria_link);
    println!("{}\n{}", "Saweria QR Code:".bold(), saweria_qr);
}

fn generate_qr_code(data: &str) -> String {
    let code = QrCode::new(data).unwrap();
    code.render::<unicode::Dense1x2>()
        .quiet_zone(false)
        .module_dimensions(1, 1)
        .build()
}
