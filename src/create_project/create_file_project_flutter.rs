use crate::command::Command;
use include_dir::{include_dir, Dir};
use indoc::indoc;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

static ASSETS: Dir = include_dir!("assets");

pub struct CreateFileProjectFlutter;

impl CreateFileProjectFlutter {
    pub fn build(project_name: &str, package_name: &str) {
        Self::create_docs_in_root();
        Self::create_gitignore_in_app();
        Self::create_files_in_root();
        Self::generate_resource_files();
        Self::generate_common_files();
        Self::create_app_main_files();
        Self::create_flutter_run_config_in_root(project_name);
        Self::generate_feature_common_files();
        Self::create_feature_auth_files();
        Self::create_feature_dashboard_files();
        Self::generate_domain_common_files();
        Self::generate_domain_auth_files();
        Self::generate_data_common_files();
        Self::generate_data_auth_files();
        Self::generate_icon_logo_files();
        Self::generate_flavorizr_yaml(project_name, package_name);
    }

    fn create_app_main_files() {
        let source_dir = "app";
        let dest_lib_dir = "packages/app";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create app lib directory");

        let root_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find resource directory");
        Self::copy_contents_only(root_dir, Path::new(dest_lib_dir));

        let main_file_path = "packages/app/lib/main.dart";
        if fs::metadata(main_file_path).is_ok() {
            println!("Removing file: {}", main_file_path);
            fs::remove_file(main_file_path).expect("Failed to remove main.dart");
        }

        let widget_test_file_path = "packages/app/test/widget_test.dart";
        if fs::metadata(widget_test_file_path).is_ok() {
            println!("Removing file: {}", widget_test_file_path);
            fs::remove_file(widget_test_file_path).expect("Failed to remove widget_test.dart");
        }

        let pubspec_path = "packages/app/pubspec.yaml";
        if Path::new(pubspec_path).exists() {
            println!("Updating pubspec.yaml at {}", pubspec_path);

            let mut existing_content = String::new();

            {
                let mut file = fs::File::open(pubspec_path).expect("Failed to open pubspec.yaml");
                file.read_to_string(&mut existing_content)
                    .expect("Failed to read pubspec.yaml");
            }

            if !existing_content.contains("assets:\n    - assets/icons/") {
                let updated_content = existing_content.replace(
                    "uses-material-design: true",
                    "uses-material-design: true\n\n  assets:\n    - assets/icons/",
                );

                {
                    let mut file =
                        fs::File::create(pubspec_path).expect("Failed to write pubspec.yaml");
                    file.write_all(updated_content.as_bytes())
                        .expect("Failed to update pubspec.yaml");
                }

                println!("Updated pubspec.yaml with assets section.");
            } else {
                println!("Assets section already exists in pubspec.yaml.");
            }
        } else {
            println!("pubspec.yaml not found at {}", pubspec_path);
        }
    }

    fn create_feature_auth_files() {
        let source_dir = "feature_auth";
        let dest_lib_dir = "packages/presentation/feature_auth";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find lib directory in feature_auth");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("All files and folders from feature_auth successfully generated.");
    }

    fn create_feature_dashboard_files() {
        let source_dir = "feature_dashboard";
        let dest_lib_dir = "packages/presentation/feature_dashboard";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find lib directory in feature_dashboard");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("All files and folders from feature_dashboard successfully generated.");
    }

    fn generate_icon_logo_files() {
        let icon_logo = "ic_logo.png";
        let drawable_path = "packages/app/android/app/src/main/res/drawable";
        let file_path = format!("{}/{}", drawable_path, icon_logo);

        let file = ASSETS
            .get_file(icon_logo)
            .expect("Failed to find ic_logo.png in assets");

        fs::create_dir_all(drawable_path).expect("Failed to generate icon_logo directory");

        println!("Generating file: {}", file_path);
        fs::write(&file_path, file.contents()).expect("Failed to generate icon_logo file");
    }

    fn create_gitignore_in_app() {
        let app_path = "packages/app";
        let gitignore_path = format!("{}/.gitignore", app_path);

        let file = ASSETS
            .get_file(".gitignore")
            .expect("Failed to find .gitignore in assets");

        fs::create_dir_all(app_path).expect("Failed to create app directory");

        println!("Create a .gitignore in: {}", gitignore_path);
        fs::write(&gitignore_path, file.contents()).expect("Failed to write .gitignore file");
    }

    fn create_files_in_root() {
        let editorconfig_path = ".editorconfig";
        let license_path = "LICENSE";
        let readme_path = "README.md";
        let cryptify_path = "cryptify";

        let editorconfig_file = ASSETS
            .get_file(".editorconfig")
            .expect("Failed to find .editorconfig in assets");
        println!("Creating a .editorconfig in: {}", editorconfig_path);
        fs::write(editorconfig_path, editorconfig_file.contents())
            .expect("Failed to write .editorconfig file");

        let license_file = ASSETS
            .get_file("LICENSE")
            .expect("Failed to find LICENSE in assets");
        println!("Creating a LICENSE file in: {}", license_path);
        fs::write(license_path, license_file.contents()).expect("Failed to write LICENSE file");

        let readme_file = ASSETS
            .get_file("README.md")
            .expect("Failed to find README.md in assets");
        println!("Creating a README.md file in: {}", readme_path);
        fs::write(readme_path, readme_file.contents()).expect("Failed to write README.md file");

        let cryptify_file = ASSETS
            .get_file("cryptify")
            .expect("Failed to find cryptify in assets");
        println!("Creating a cryptify file in: {}", cryptify_path);
        fs::write(cryptify_path, cryptify_file.contents()).expect("Failed to write cryptify file");
    }

    fn create_docs_in_root() {
        let source_dir = "docs";
        let dest_lib_dir = "docs";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create docs directory");

        let lib_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find docs directory in assets");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("📚 Documentation is ready and available in the project! 🚀");

        let source_dir = "release";
        let dest_lib_dir = "release";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create release directory");

        let lib_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find release directory in assets");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("📦 Release assets are prepared and ready for deployment! 🎉");
    }

    fn create_flutter_run_config_in_root(project_name: &str) {
        let run_folder_path = ".run";
        fs::create_dir_all(run_folder_path).expect("Failed to create .run directory");

        let environments = ["dev", "staging", "prod"];

        for env in environments.iter() {
            let run_file_path = format!(
                "{}/Flutter Run -_'{}_{}'.run.xml",
                run_folder_path, project_name, env
            );

            let run_file_content = format!(
                indoc! {r#"
                <component name="ProjectRunConfigurationManager">
                  <configuration default="false" name="Flutter Run -&gt; '{}_{}'" type="FlutterRunConfigurationType" factoryName="Flutter">
                    <option name="additionalArgs" value="--flavor {}" />
                    <option name="filePath" value="$PROJECT_DIR$/app/lib/main_{}.dart" />
                    <method v="2" />
                  </configuration>
                </component>
                "#
                },
                project_name, env, env, env
            );

            println!(
                "Creating Flutter Run configuration for '{}' environment in: {}",
                env, run_file_path
            );
            Command::write_file(&run_file_path, &run_file_content);
        }
    }

    fn generate_resource_files() {
        let source_dir = "resource";
        let dest_lib_dir = "packages/library/resource";
        let dest_pubspec_path = "packages/library/resource/pubspec.yaml";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let root_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find resource directory");
        Self::copy_contents_only(root_dir, Path::new(dest_lib_dir));

        if let Some(flutter_gen_file) = root_dir
            .files()
            .find(|f| f.path().file_name().unwrap() == "flutter_gen.yaml")
        {
            println!("Appending flutter_gen.yaml content to pubspec.yaml");

            let mut existing_pubspec_content = String::new();
            if Path::new(dest_pubspec_path).exists() {
                if let Ok(content) = fs::read_to_string(dest_pubspec_path) {
                    existing_pubspec_content = content;
                }
            }

            let updated_content = format!(
                "{}\n\n# Auto-generated flutter_gen configuration\n{}",
                existing_pubspec_content,
                String::from_utf8_lossy(flutter_gen_file.contents())
            );

            fs::write(dest_pubspec_path, updated_content).expect("Failed to update pubspec.yaml");
        } else {
            eprintln!("flutter_gen.yaml not found in resource directory!");
        }

        println!("All files and folders from resource successfully generated.");
    }

    fn generate_common_files() {
        let source_dir = "common";
        let dest_lib_dir = "packages/library/common/lib";
        let dest_env_dir = "packages/library/common";
        let dest_pubspec_path = "packages/library/common/pubspec.yaml";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(format!("{}/lib", source_dir))
            .expect("Failed to find lib directory in common");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        let root_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find common directory");
        Self::copy_contents_only(root_dir, Path::new(dest_env_dir));

        if let Some(flutter_gen_file) = root_dir
            .files()
            .find(|f| f.path().file_name().unwrap() == "flutter_env.yaml")
        {
            println!("Appending flutter_env.yaml content to pubspec.yaml");

            let mut existing_pubspec_content = String::new();
            if Path::new(dest_pubspec_path).exists() {
                if let Ok(content) = fs::read_to_string(dest_pubspec_path) {
                    existing_pubspec_content = content;
                }
            }

            let updated_content = format!(
                "{}\n\n# Auto-generated flutter_env configuration\n{}",
                existing_pubspec_content,
                String::from_utf8_lossy(flutter_gen_file.contents())
            );

            fs::write(dest_pubspec_path, updated_content).expect("Failed to update pubspec.yaml");
        } else {
            eprintln!("flutter_env.yaml not found in resource directory!");
        }

        println!("All files and folders from common successfully generated.");
    }

    fn generate_feature_common_files() {
        let source_dir = "feature_common";
        let dest_lib_dir = "packages/presentation/feature_common";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find lib directory in feature_common");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("All files and folders from feature_common successfully generated.");
    }

    fn generate_domain_common_files() {
        let source_dir = "domain_common";
        let dest_lib_dir = "packages/domain/domain_common";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find lib directory in domain_common");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("All files and folders from domain_common successfully generated.");
    }

    fn generate_domain_auth_files() {
        let source_dir = "domain_auth";
        let dest_lib_dir = "packages/domain/domain_auth";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find lib directory in domain_auth");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("All files and folders from domain_auth successfully generated.");
    }

    fn generate_data_common_files() {
        let source_dir = "data_common";
        let dest_lib_dir = "packages/data/data_common";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find lib directory in data_common");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("All files and folders from data_common successfully generated.");
    }

    fn generate_data_auth_files() {
        let source_dir = "data_auth";
        let dest_lib_dir = "packages/data/data_auth";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find lib directory in data_auth");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("All files and folders from data_auth successfully generated.");
    }

    fn format_project_name(project_name: &str) -> String {
        project_name
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn generate_flavorizr_yaml(project_name: &str, package_name: &str) {
        let module_dir = "packages/app";
        let file_path = format!("{}/flavorizr.yaml", module_dir);

        if !Path::new(module_dir).exists() {
            fs::create_dir_all(module_dir).expect("Failed to create the module directory");
        }

        let formatted_project_name = Self::format_project_name(project_name);

        let flavorizr_content = indoc! {r#"
            app:
              android:
                flavorDimensions: "flavor-type"

            flavors:
              dev:
                app:
                  name: "{project_name} Dev"

                android:
                  icon: "assets/icons/android_app_dev.png"
                  applicationId: "{package_name}.dev"
                ios:
                  icon: "assets/icons/ios_app_dev.png"
                  bundleId: "{package_name}.dev"

              staging:
                app:
                  name: "{project_name} Stg"

                android:
                  icon: "assets/icons/android_app_stg.png"
                  applicationId: "{package_name}.staging"
                ios:
                  icon: "assets/icons/ios_app_stg.png"
                  bundleId: "{package_name}.staging"

              prod:
                app:
                  name: "{project_name}"

                android:
                  icon: "assets/icons/android_app_prod.png"
                  applicationId: "{package_name}"
                ios:
                  icon: "assets/icons/ios_app_prod.png"
                  bundleId: "{package_name}"

            ide: idea

            instructions:
              - assets:download
              - assets:extract
              - android:androidManifest
              - android:buildGradle
              - android:dummyAssets
              - android:icons
              - ios:podfile
              - ios:xcconfig
              - ios:buildTargets
              - ios:schema
              - ios:dummyAssets
              - ios:icons
              - ios:plist
              - ios:launchScreen
              - macos:podfile
              - macos:xcconfig
              - macos:configs
              - macos:buildTargets
              - macos:schema
              - macos:dummyAssets
              - macos:icons
              - macos:plist
              - google:firebase
              - huawei:agconnect
              - assets:clean
              - ide:config

        "#}
        .replace("{project_name}", &formatted_project_name)
        .replace("{package_name}", package_name);

        let mut file = fs::File::create(&file_path).expect("Failed to create flavorizr.yaml");
        file.write_all(flavorizr_content.as_bytes())
            .expect("Failed to write to flavorizr.yaml");

        println!(
            "flavorizr.yaml has been successfully generated at {}",
            file_path
        );
    }

    pub fn copy_contents_only(dir: &Dir, dest_path: &Path) {
        for file in dir.files() {
            let file_dest_path = dest_path.join(file.path().file_name().unwrap());
            println!("Generating file: {}", file_dest_path.display());
            fs::write(&file_dest_path, file.contents()).expect("Failed to write file");
        }

        for subdir in dir.dirs() {
            let subdir_dest_path = dest_path.join(subdir.path().file_name().unwrap());
            println!("Creating directory: {}", subdir_dest_path.display());
            fs::create_dir_all(&subdir_dest_path).expect("Failed to create subdirectory");
            Self::copy_contents_only(subdir, &subdir_dest_path);
        }
    }
}
