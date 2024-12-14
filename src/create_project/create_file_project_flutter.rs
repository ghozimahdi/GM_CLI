use crate::command::Command;
use include_dir::{include_dir, Dir};
use indoc::indoc;
use std::fs;
use std::path::Path;

static ASSETS: Dir = include_dir!("assets");

pub struct CreateFileProjectFlutter;

impl CreateFileProjectFlutter {
    pub fn build(project_name: &str) {
        Self::create_app_main_files();
        Self::create_feature_login_page_files();
        Self::create_gitignore_in_app();
        Self::create_editorconfig_in_root();
        Self::create_flutter_run_config_in_root(project_name);
        Self::generate_resource_files();
        Self::generate_core_files();
        Self::generate_feature_common_files();
        Self::generate_domain_common_files();
        Self::generate_data_common_files();
        Self::generate_icon_logo_files();
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
    }

    fn create_feature_login_page_files() {
        let feature_login_path = "packages/presentation/feature_login/lib/src";
        let file_path = format!("{}/login_page.dart", feature_login_path);

        let file = ASSETS
            .get_file("login_page.dart")
            .expect("Failed to find login_page.dart in assets");

        fs::create_dir_all(feature_login_path).expect("Failed to create feature login directory");

        println!("Generating file: {}", file_path);
        fs::write(&file_path, file.contents()).expect("Failed to write login_page.dart file");
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

    fn create_editorconfig_in_root() {
        let editorconfig_path = ".editorconfig";

        let file = ASSETS
            .get_file(".editorconfig")
            .expect("Failed to find .editorconfig in assets");

        println!("Creating a .editorconfig in: {}", editorconfig_path);
        fs::write(editorconfig_path, file.contents()).expect("Failed to write .editorconfig file");
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
                    <option name="filePath" value="$PROJECT_DIR$/packages/app/lib/main_{}.dart" />
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

    fn generate_core_files() {
        let source_dir = "core";
        let dest_lib_dir = "packages/library/core/lib";
        let dest_env_dir = "packages/library/core";
        let dest_pubspec_path = "packages/library/core/pubspec.yaml";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(format!("{}/lib", source_dir))
            .expect("Failed to find lib directory in core");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        let root_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find core directory");
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

        println!("All files and folders from resource successfully generated.");
    }

    fn generate_feature_common_files() {
        let source_dir = "feature_common";
        let dest_lib_dir = "packages/presentation/feature_common/lib";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(format!("{}/lib", source_dir))
            .expect("Failed to find lib directory in feature_common");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("All files and folders from feature_common successfully generated.");
    }

    fn generate_domain_common_files() {
        let source_dir = "domain_common";
        let dest_lib_dir = "packages/domain/domain_common/lib";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(format!("{}/lib", source_dir))
            .expect("Failed to find lib directory in domain_common");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("All files and folders from domain_common successfully generated.");
    }

    fn generate_data_common_files() {
        let source_dir = "data_common";
        let dest_lib_dir = "packages/data/data_common/lib";

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(format!("{}/lib", source_dir))
            .expect("Failed to find lib directory in data_common");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!("All files and folders from data_common successfully generated.");
    }

    fn copy_contents_only(dir: &Dir, dest_path: &Path) {
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
