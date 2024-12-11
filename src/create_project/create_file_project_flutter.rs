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
    }

    fn create_app_main_files() {
        let app_lib_path = "packages/app/lib";
        fs::create_dir_all(app_lib_path).expect("Failed to create app lib directory");

        let files = [
            ("main_dev.dart", "main_dev.dart"),
            ("main_prod.dart", "main_prod.dart"),
            ("main_staging.dart", "main_staging.dart"),
            ("main_common.dart", "main_common.dart"),
        ];

        for (file_name, asset_path) in files.iter() {
            let file = ASSETS
                .get_file(asset_path)
                .expect(&format!("Failed to find {} in assets", asset_path));

            let file_path = format!("{}/{}", app_lib_path, file_name);

            println!("Generating file: {}", file_path);
            fs::write(&file_path, file.contents())
                .expect(&format!("Failed to write {}", file_name));
        }
    }

    fn create_feature_login_page_files() {
        let feature_login_path = "packages/presentation/feature_login/lib";
        let file_path = format!("{}/login_page.dart", feature_login_path);

        let file = ASSETS
            .get_file("login_page.dart")
            .expect("Failed to find login_page.dart in assets");

        fs::create_dir_all(feature_login_path).expect("Failed to create feature login directory");

        println!("Generating file: {}", file_path);
        fs::write(&file_path, file.contents()).expect("Failed to write login_page.dart file");
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
        let dest_assets_dir = "packages/library/resource/assets";
        let dest_lib_dir = "packages/library/resource/lib";
        let dest_pubspec_path = "packages/library/resource/pubspec.yaml";

        fs::create_dir_all(dest_assets_dir).expect("Failed to create assets directory");
        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let assets_dir = ASSETS
            .get_dir(format!("{}/assets", source_dir))
            .expect("Failed to find assets directory in resource");
        Self::copy_contents_only(assets_dir, Path::new(dest_assets_dir));

        let lib_dir = ASSETS
            .get_dir(format!("{}/lib", source_dir))
            .expect("Failed to find lib directory in resource");
        Self::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        let root_dir = ASSETS
            .get_dir(source_dir)
            .expect("Failed to find resource directory");

        if let Some(flutter_gen_file) = root_dir.get_file("flutter_gen.yaml") {
            println!("Appending flutter_gen.yaml content to pubspec.yaml");

            let mut existing_pubspec_content = String::new();
            if let Ok(content) = fs::read_to_string(dest_pubspec_path) {
                existing_pubspec_content = content;
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

        println!("All files and folders from resource successfully moved.");
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
