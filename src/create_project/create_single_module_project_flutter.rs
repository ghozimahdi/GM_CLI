use crate::create_project::create_file_project_flutter::CreateFileProjectFlutter;
use crate::create_project::create_module_project_flutter::CreateModuleProjectFlutter;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use include_dir::{include_dir, Dir};
use indoc::indoc;
use qrcode::render::string::Element;
use serde_json::Value::String;
use std::fmt::format;
use std::fs;
use std::io::Write;
use std::path::Path;

static ASSETS: Dir = include_dir!("assets");

pub struct CreateSingleModuleProjectFlutter;

impl CreateSingleModuleProjectFlutter {
    pub fn build() {
        let choices = &["feature", "data", "domain", "library", "in this path"];

        // Prompt to select where to create the module
        let selected_option = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Where do you want to create the module?")
            .default(0)
            .items(choices)
            .interact()
            .unwrap();

        // Display the chosen location
        let selected_choice = choices[selected_option];
        println!("You chose to create the module in: {}", selected_choice);

        // Generate a default module name example
        let default_name = match selected_choice {
            "feature" => "feature_xxx",
            "data" => "data_xxx",
            "domain" => "domain_xxx",
            "library" => "library_xxx",
            _ => "module_xxx",
        };

        // Prompt to enter the module name with a default example
        let module_name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Enter the module name (e.g., {}):", default_name))
            .interact_text()
            .unwrap();

        // Print the final result
        println!(
            "Module '{}' will be created in the '{}' folder.",
            module_name, selected_choice
        );

        let local_module: (&str, &str);
        match selected_option {
            /// Feature
            0 => {
                local_module = (module_name, &format!("../presentation/{}", module_name));
                CreateModuleProjectFlutter::create_module(
                    module_name,
                    &format!("./packages/presentation/{}", module_name),
                    vec![
                        "flutter_screenutil",
                        "freezed_annotation",
                        "flutter_bloc",
                        "get_it",
                        "injectable",
                        "auto_route",
                    ],
                    vec![
                        "auto_route_generator",
                        "injectable_generator",
                        "build_runner",
                        "freezed",
                        "lint",
                    ],
                    vec![
                        ("core", "../../library/core"),
                        ("feature_common", "../../presentation/feature_common"),
                        ("resource", "../../library/resource"),
                    ],
                );
                Self::create_default_feature_files(module_name);
            }
            /// Data
            1 => {
                local_module = (module_name, &format!("../data/{}", module_name));
                CreateModuleProjectFlutter::create_module(
                    module_name,
                    &format!("./packages/data/{}", module_name),
                    vec!["injectable", "get_it"],
                    vec![
                        "lint",
                        "json_serializable",
                        "injectable_generator",
                        "build_runner",
                    ],
                    vec![
                        ("core", "../../library/core"),
                        ("data_common", "../../data/data_common"),
                        ("domain_auth", "../../domain/domain_auth"),
                    ],
                );
            }
            /// Domain
            2 => {
                local_module = (module_name, &format!("../domain/{}", module_name));
                CreateModuleProjectFlutter::create_module(
                    module_name,
                    &format!("./packages/domain/{}", module_name),
                    vec!["injectable", "get_it", "freezed_annotation"],
                    vec!["lint", "injectable_generator", "build_runner", "freezed"],
                    vec![
                        ("core", "../../library/core"),
                        ("domain_common", "../../domain/domain_common"),
                    ],
                );
            }
            /// Library
            3 => {
                local_module = (module_name, &format!("../library/{}", module_name));
                CreateModuleProjectFlutter::create_module(
                    module_name,
                    &format!("./packages/library/{}", module_name),
                    vec![],
                    vec!["build_runner", "lint"],
                    vec![],
                );
            }
            /// in this path
            _ => {
                local_module = (module_name, &format!("./{}", module_name));
                CreateModuleProjectFlutter::create_module(
                    module_name,
                    &format!("./{}", module_name),
                    vec![],
                    vec!["build_runner", "lint"],
                    vec![],
                );
            }
        }

        CreateModuleProjectFlutter::update_app_pubspec_dependencies(
            vec![],
            vec![],
            vec![local_module],
        );
    }

    fn create_default_feature_files(module_name: &str) {
        let source_dir = "default_feature_module";
        let dest_lib_dir = &format!("packages/presentation/{}", module_name);

        fs::create_dir_all(dest_lib_dir).expect("Failed to create lib directory");

        let lib_dir = ASSETS
            .get_dir(source_dir)
            .expect(&format!("Failed to find lib directory in {}", module_name));
        CreateFileProjectFlutter::copy_contents_only(lib_dir, Path::new(dest_lib_dir));

        println!(
            "All files and folders from {} successfully generated.",
            module_name
        );

        Self::generated_di(&format!("{}/lib/src/di", dest_lib_dir), module_name);
        Self::generated_config(&format!("{}/lib/src/config", dest_lib_dir), module_name);
        Self::generated_exposed_feature_file(&format!("{}/lib", dest_lib_dir), module_name);
        Self::generated_feature_route_provider(
            &format!("{}/lib/src/config", dest_lib_dir),
            module_name,
        );
        Self::generated_feature_page(&format!("{}/lib/src/page", dest_lib_dir), module_name);
    }

    fn create_default_data_files() {}
    fn create_default_domain_files() {}

    fn generated_config(module_dir: &str, module_name: &str) {
        let file_name = format!("{}_config", module_name);
        let file_path = format!("{}/{}.dart", module_dir, file_name);

        if !Path::new(module_dir).exists() {
            fs::create_dir_all(module_dir).expect("Failed to create the module directory");
        }

        let class_name = module_name
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                chars
                    .next()
                    .map(|c| c.to_uppercase().to_string())
                    .unwrap_or_default()
                    + chars.as_str()
            })
            .collect::<String>()
            + "Config";

        let file_content = indoc! {r#"
            import 'package:core/core.dart';
            import 'package:{module_name}/src/di/di.dart';

            class {class_name} extends AppConfig {
              {class_name}._();

              factory {class_name}.getInstance() {
                return _instance;
              }

              static final {class_name} _instance = {class_name}._();

              @override
              Future<bool> config({
                required String env,
              }) async {
                await configureInjection(env: env);
                return true;
              }
            }
        "#}
        .replace("{module_name}", module_name)
        .replace("{class_name}", &class_name);

        let mut file =
            fs::File::create(&file_path).expect(&format!("Failed to create {}", file_name));

        file.write_all(file_content.as_bytes())
            .expect(&format!("Failed to generate to {}", file_name));

        println!(
            "{} has been successfully generated at {}",
            file_name, file_path
        );
    }

    fn generated_di(module_dir: &str, module_name: &str) {
        let file_name = "di.dart";
        let file_path = format!("{}/{}", module_dir, file_name);

        if !Path::new(module_dir).exists() {
            fs::create_dir_all(module_dir).expect("Failed to create the module directory");
        }

        let file_content = indoc! {r#"
            import 'package:{module_name}/src/di/di.config.dart';
            import 'package:get_it/get_it.dart';
            import 'package:injectable/injectable.dart';

            final getIt = GetIt.instance;

            @injectableInit
            Future<void> configureInjection({required String env}) async =>
                getIt.init(environment: env);
        "#}
        .replace("{module_name}", module_name);

        let mut file =
            fs::File::create(&file_path).expect(&format!("Failed to create {}", file_name));

        file.write_all(file_content.as_bytes())
            .expect(&format!("Failed to generated to {}", file_name));

        println!(
            "{} has been successfully generated at {}",
            file_name, file_path
        );
    }

    fn generated_feature_route_provider(module_dir: &str, module_name: &str) {
        let file_name = format!("{}_route.dart", module_name);
        let file_path = format!("{}/{}", module_dir, file_name);

        if !Path::new(&module_dir).exists() {
            fs::create_dir_all(&module_dir).expect("Failed to create the module directory");
        }

        let split_name = module_name.replace("feature_", "");
        let pascal_case_name = split_name
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                chars
                    .next()
                    .map(|c| c.to_uppercase().to_string())
                    .unwrap_or_default()
                    + chars.as_str()
            })
            .collect::<String>();

        let class_name = format!("{}Page", pascal_case_name);
        let route_name = format!("{}Route", pascal_case_name);

        let file_content = indoc! {r#"
            import 'package:auto_route/auto_route.dart';

            // TODO: Move this file to `feature_common` if this route will be shared and reused in other feature modules.
            //       Placing shared routes in `feature_common` ensures better modularization and avoids duplication.
            abstract class {route_name}Provider {
              PageRouteInfo route({
                required String name,
              });
            }
        "#}
        .replace("{route_name}", &route_name);

        let mut file =
            fs::File::create(&file_path).expect(&format!("Failed to create {}", file_name));

        file.write_all(file_content.as_bytes())
            .expect(&format!("Failed to generate {}", file_name));

        println!(
            "{} has been successfully generated at {}",
            file_name, file_path
        );
    }

    fn generated_feature_page(module_dir: &str, module_name: &str) {
        let split_name = module_name.replace("feature_", "");
        let file_name = format!("{}_page.dart", split_name);
        let file_path = format!("{}/{}", module_dir, file_name);

        if !Path::new(&module_dir).exists() {
            fs::create_dir_all(&module_dir).expect("Failed to create the module directory");
        }

        let pascal_case_name = split_name
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                chars
                    .next()
                    .map(|c| c.to_uppercase().to_string())
                    .unwrap_or_default()
                    + chars.as_str()
            })
            .collect::<String>();

        let class_name = format!("{}Page", pascal_case_name);
        let route_name = format!("{}Route", pascal_case_name);

        let file_content = indoc! {r#"
            import 'package:auto_route/auto_route.dart';
            import 'package:feature_common/feature_common.dart';
            import 'package:{module_name}/src/config/{module_name}_route.gr.dart';
            import 'package:{module_name}/src/config/{module_name}_route.dart';
            import 'package:flutter/material.dart';
            import 'package:injectable/injectable.dart';
            import 'package:resource/resource.dart';

            @Injectable(as: {route_name}Provider)
            class {route_name}ProviderImpl implements {route_name}Provider {
              @override
              PageRouteInfo route({
                required String name,
              }) {
                return {route_name}(
                  name: name,
                );
              }
            }

            @RoutePage()
            class {class_name} extends StatelessWidget {
              final String name;

              const {class_name}({
                super.key,
                required this.name,
              });

              @override
              Widget build(BuildContext context) {
                return Scaffold(
                  body: Center(
                    child: Text(
                      'Hello $name! I thought you’d never show up! 😅',
                      style: res.textTheme.titleLarge,
                      textAlign: TextAlign.center,
                    ),
                  ),
                );
              }
            }

        "#}
        .replace("{module_name}", module_name)
        .replace("{class_name}", &class_name)
        .replace("{route_name}", &route_name);

        let mut file =
            fs::File::create(&file_path).expect(&format!("Failed to create {}", file_name));

        file.write_all(file_content.as_bytes())
            .expect(&format!("Failed to generate {}", file_name));

        println!(
            "{} has been successfully generated at {}",
            file_name, file_path
        );
    }

    fn generated_exposed_feature_file(module_dir: &str, module_name: &str) {
        let file_name = format!("{}.dart", module_name);
        let file_path = format!("{}/{}", module_dir, file_name);

        if !Path::new(module_dir).exists() {
            fs::create_dir_all(module_dir).expect("Failed to create the module directory");
        }

        let file_content = indoc! {r#"
            export 'package:{module_name}/src/config/{module_name}_config.dart';
            export 'package:{module_name}/src/config/{module_name}_route.dart';
        "#}
        .replace("{module_name}", module_name);

        let mut file =
            fs::File::create(&file_path).expect(&format!("Failed to create {}", file_name));

        file.write_all(file_content.as_bytes())
            .expect(&format!("Failed to generated to {}", file_name));

        println!(
            "{} has been successfully generated at {}",
            file_name, file_path
        );
    }
}
