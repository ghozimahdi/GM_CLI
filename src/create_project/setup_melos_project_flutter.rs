use crate::command::Command;
use crate::create_project::create_module_project_flutter::CreateModuleProjectFlutter;
use indoc::indoc;
use std::fs::File;
use std::io::Write;

pub struct SetupMelosProjectFlutter;

impl SetupMelosProjectFlutter {
    pub fn build(project_name: &str) {
        let sdk_version = CreateModuleProjectFlutter::read_environment_from_main_project();

        let melos_config = indoc! {r#"
            name: {project_name}
            packages:
              - packages/app
              - packages/library/core
              - packages/library/resource
              - packages/domain/domain_auth
              - packages/domain/domain_common
              - packages/data/data_auth
              - packages/data/data_common
              - packages/presentation/feature_common
              - packages/presentation/feature_login

            command:
              bootstrap:
                usePubspecOverrides: true

            scripts:
              build:
                run: melos exec -c 1 --depends-on="build_runner" -- "dart run build_runner build -d"
                description: "Run build runner with depends-on"

              locale:
                run: melos exec --depends-on="easy_localization" -- "flutter pub run easy_localization:generate -f keys -S assets/translations"
                description: "Run easy_localization with depends-on"

              assets:
                run: melos exec --depends-on="flutter_gen_runner" -- "dart run build_runner build -d"
                description: "Run flutter_gen with depends-on"

              env:
                run: melos exec --depends-on="envied" -- "dart run build_runner build -d"
                description: "Run envied with depends-on"

              upgrade:
                run: melos exec --fail-fast -- flutter pub upgrade --major-versions
                description: "Run envied with depends-on"

              flavors:
                run: melos exec --depends-on="flutter_flavorizr" -- "flutter pub run flutter_flavorizr"
                description: "Run flutter_flavorizr with depends-on"
        "#}
        .replace("{project_name}", project_name);

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
