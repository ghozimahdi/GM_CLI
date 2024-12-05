use crate::command::Command;
use indoc::indoc;
use std::fs;

pub struct CreateFileProjectFlutter;

impl CreateFileProjectFlutter {
    pub fn build(project_name: &str) {
        Self::create_app_main_files();
        Self::create_feature_login_page_files();
        Self::create_gitignore_in_app();
        Self::create_editorconfig_in_root();
        Self::create_flutter_run_config_in_root(project_name);
    }

    fn create_app_main_files() {
        let app_lib_path = "packages/app/lib";
        fs::create_dir_all(app_lib_path).expect("Failed to create app lib directory");

        let files = [
            ("main_dev.dart", include_str!("../../assets/main_dev.dart")),
            (
                "main_prod.dart",
                include_str!("../../assets/main_prod.dart"),
            ),
            (
                "main_staging.dart",
                include_str!("../../assets/main_staging.dart"),
            ),
            (
                "main_common.dart",
                include_str!("../../assets/main_common.dart"),
            ),
        ];

        for (file_name, content) in files.iter() {
            let file_path = format!("{}/{}", app_lib_path, file_name);
            println!("Writing to: {}", file_path);
            Command::write_file(&file_path, content);
        }
    }

    fn create_feature_login_page_files() {
        let feature_login_path = "packages/presentation/feature_login/lib";
        fs::create_dir_all(feature_login_path).expect("Failed to create feature login directory");

        let file_path = format!("{}/login_page.dart", feature_login_path);

        let dart_file_content = include_str!("../../assets/login_page.dart");

        println!("Writing to: {}", file_path);
        Command::write_file(&file_path, dart_file_content);
    }

    fn create_gitignore_in_app() {
        let app_path = "packages/app";
        let gitignore_path = format!("{}/.gitignore", app_path);
        let gitignore_content = include_str!("../../assets/.gitignore");
        fs::create_dir_all(app_path).expect("Failed to create app directory");

        println!("Create a .gitignore in: {}", gitignore_path);
        Command::write_file(&gitignore_path, gitignore_content);
    }

    fn create_editorconfig_in_root() {
        let editorconfig_path = ".editorconfig";
        let editorconfig_content = include_str!("../../assets/.editorconfig");

        println!("Creating a .editorconfig in: {}", editorconfig_path);
        Command::write_file(editorconfig_path, editorconfig_content);
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
}
