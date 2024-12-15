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

pub fn show_donate() {
    let ko_fi_link = "https://ko-fi.com/ghozimahdi";
    let trakteer_link = "https://teer.id/ghozimahdi";
    let github_sponsor_link = "https://github.com/sponsors/mbahgojol";
    let saweria_link = "https://saweria.co/ghozimahdi";

    println!("{}", "❤️ Support My Work! ❤️".bold().yellow());
    println!(
        "{}",
        "If you enjoy using this CLI, consider buying me a coffee or helping me stay motivated!"
            .italic()
    );
    println!("{}", "Here are some ways to show your love:".green());
    println!();

    println!("{}: {}", "☕ Ko-fi".cyan(), ko_fi_link);
    println!("{}: {}", "🍵 Trakteer".cyan(), trakteer_link);
    println!("{}: {}", "⭐ Github Sponsor".cyan(), github_sponsor_link);
    println!("{}: {}", "🎉 Saweria".cyan(), saweria_link);

    println!();
    println!(
        "{}",
        "Even the smallest contribution means the world to me. Thank you for being amazing!"
            .italic()
            .bold()
            .yellow()
    );

    generate_saweria_qr_code();
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
