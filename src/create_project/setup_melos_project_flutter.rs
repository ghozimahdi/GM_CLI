use crate::command::Command;
use crate::create_project::create_module_project_flutter::CreateModuleProjectFlutter;
use indoc::indoc;
use std::fs::File;
use std::io::Write;

pub struct SetupMelosProjectFlutter;

impl SetupMelosProjectFlutter {
    pub fn build(project_name: &str) {
        let sdk_version = CreateModuleProjectFlutter::read_environment_from_main_project();
        let melos_config = format!("name: {}\npackages:\n  - packages/**\n", project_name);

        let mut melos_file = File::create("melos.yaml").expect("Failed to create melos.yaml");
        melos_file
            .write_all(melos_config.as_bytes())
            .expect("Failed to write melos.yaml");

        let pubspec_content = format!(
            indoc! {r#"
            name: {}
            description: Modular project for Flutter with Melos
            version: 1.0.0

            environment:
              {}

            dev_dependencies:
              melos: any
            "#},
            project_name, sdk_version
        );

        let mut pubspec_file = File::create("pubspec.yaml").expect("Failed to create pubspec.yaml");
        pubspec_file
            .write_all(pubspec_content.as_bytes())
            .expect("Failed to write pubspec.yaml");

        println!("Adding Melos dependency...");
        Command::run(&["flutter", "pub", "add", "melos", "--dev"]);
        println!("Upgrading Melos to the latest version...");
        Command::run(&["flutter", "pub", "upgrade", "melos"]);
        println!("Running `flutter pub get`...");
        Command::run(&["flutter", "pub", "get"]);
    }
}
