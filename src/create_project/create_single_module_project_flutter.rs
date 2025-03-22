use crate::command::Command;
use crate::create_project::create_file_project_flutter::CreateFileProjectFlutter;
use crate::create_project::create_module_project_flutter::CreateModuleProjectFlutter;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use include_dir::{include_dir, Dir};
use indoc::indoc;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

static ASSETS: Dir = include_dir!("assets");

pub struct CreateSingleModuleProjectFlutter;

impl CreateSingleModuleProjectFlutter {
    pub fn build() {
        let choices = &["feature", "data", "domain", "library", "in this path"];

        let selected_option = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Where do you want to create the module?")
            .default(0)
            .items(choices)
            .interact()
            .unwrap();

        let selected_choice = choices[selected_option];
        println!("You chose to create the module in: {}", selected_choice);

        let default_name = match selected_choice {
            "feature" => "feature_xxx",
            "data" => "data_xxx",
            "domain" => "domain_xxx",
            "library" => "library_xxx",
            _ => "module_xxx",
        };

        let mut module_name = String::new();
        print!("Enter the module name (e.g., {}): ", default_name);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut module_name)
            .expect("Failed to read module name");
        let module_name = module_name.trim().to_string(); // Convert to String

        println!(
            "Module '{}' will be created in the '{}' folder.",
            module_name, selected_choice
        );

        let local_module: (String, String);

        match selected_option {
            // Feature
            0 => {
                let path = format!("../packages/presentation/{}", module_name);
                local_module = (module_name.clone(), path.clone());

                CreateModuleProjectFlutter::create_module(
                    &module_name,
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
                        ("common", "../../library/common"),
                        ("feature_common", "../../presentation/feature_common"),
                        ("domain_common", "../../domain/domain_common"),
                    ],
                );
                Self::update_app_injector_file(&module_name, &selected_choice);
                Self::create_default_feature_files(&module_name);

                Self::refresh_module(&format!("./packages/presentation/{}", module_name));
            }
            // Data
            1 => {
                let path = format!("../packages/data/{}", module_name);
                local_module = (module_name.clone(), path.clone());

                CreateModuleProjectFlutter::create_module(
                    &module_name,
                    &format!("./packages/data/{}", module_name),
                    vec![
                        "injectable",
                        "get_it",
                        "freezed_annotation",
                        "json_annotation",
                        "dio",
                        "retrofit",
                    ],
                    vec![
                        "lint",
                        "retrofit_generator",
                        "json_serializable",
                        "injectable_generator",
                        "freezed",
                        "build_runner",
                    ],
                    vec![
                        ("common", "../../library/common"),
                        ("data_common", "../../data/data_common"),
                        ("domain_common", "../../domain/domain_common"),
                    ],
                );
                Self::update_app_injector_file(&module_name, &selected_choice);
                Self::create_default_data_files(&module_name);

                Self::refresh_module(&format!("./packages/data/{}", module_name));
            }
            // Domain
            2 => {
                let path = format!("../packages/domain/{}", module_name);
                local_module = (module_name.clone(), path.clone());

                CreateModuleProjectFlutter::create_module(
                    &module_name,
                    &format!("./packages/domain/{}", module_name),
                    vec!["injectable", "get_it", "freezed_annotation"],
                    vec!["lint", "injectable_generator", "build_runner", "freezed"],
                    vec![
                        ("common", "../../library/common"),
                        ("domain_common", "../../domain/domain_common"),
                    ],
                );
                Self::update_app_injector_file(&module_name, &selected_choice);
                Self::create_default_domain_files(&module_name);

                Self::refresh_module(&format!("./packages/domain/{}", module_name));
            }
            // Library
            3 => {
                let path = format!("../packages/library/{}", module_name);
                local_module = (module_name.clone(), path.clone());

                CreateModuleProjectFlutter::create_module(
                    &module_name,
                    &format!("./packages/library/{}", module_name),
                    vec![],
                    vec!["build_runner", "lint"],
                    vec![],
                );

                Self::refresh_module(&format!("./packages/library/{}", module_name));
            }
            // In this path
            _ => {
                let path = format!("./{}", module_name);
                local_module = (module_name.clone(), path.clone());
                CreateModuleProjectFlutter::create_module(
                    &module_name,
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
            vec![(&*local_module.0, &*local_module.1)],
        );
        Self::refresh_module("./app");

        let _ = std::process::Command::new("fvm")
            .arg("dart")
            .arg("fix")
            .arg("--apply")
            .arg("./app/lib/injector.dart")
            .output()
            .expect("Failed to run dart fix on injector.dart");

        println!("dart fix applied to injector.dart.");
    }

    pub fn update_melos_config(new_package: &str) {
        let pubspec_path = "pubspec.yaml";

        if !Path::new(pubspec_path).exists() {
            println!("Pubspec.yaml configuration file not found. Please ensure pubspec.yaml exists in the root directory.");
            return;
        }

        let pubspec_content =
            fs::read_to_string(pubspec_path).expect("Failed to read pubspec.yaml");
        let mut lines: Vec<String> = pubspec_content.lines().map(String::from).collect();

        if let Some(start_index) = lines
            .iter()
            .position(|line| line.trim_start().starts_with("workspace:"))
        {
            let mut last_index = start_index;

            for (i, line) in lines.iter().enumerate().skip(start_index + 1) {
                if line.trim().starts_with("- ") {
                    last_index = i;
                } else {
                    break;
                }
            }

            let package_already_exists = lines
                .iter()
                .skip(start_index + 1)
                .any(|line| line.trim() == format!("- {}", new_package));

            if !package_already_exists {
                lines.insert(last_index + 1, format!("  - {}", new_package));
            }
        } else {
            lines.push(String::from("workspace:"));
            lines.push(format!("  - {}", new_package));
        }

        let updated_pubspec_content = lines.join("\n");
        let mut pubspec_file = File::create(pubspec_path).expect("Failed to create pubspec.yaml");
        pubspec_file
            .write_all(updated_pubspec_content.as_bytes())
            .expect("Failed to write pubspec.yaml");

        println!(
            "Package '{}' added to pubspec.yaml workspace at the bottom successfully.",
            new_package
        );
    }

    pub fn refresh_module(dir: &str) {
        let steps: Vec<(&str, Box<dyn FnOnce() + Send>)> = vec![
            (
                "Running `flutter pub get`",
                Box::new({
                    let dir = dir.to_string();
                    move || {
                        println!("{}", "Running `flutter pub get`...".yellow());
                        Command::run_in_dir(&["fvm", "flutter", "pub", "get"], &dir);
                    }
                }),
            ),
            (
                "Running `flutter run build_runner`",
                Box::new({
                    let dir = dir.to_string();
                    move || {
                        println!("{}", "Running `flutter run build_runner`...".yellow());
                        Command::run_in_dir(
                            &["fvm", "dart", "run", "build_runner", "build", "-d"],
                            &dir,
                        );
                    }
                }),
            ),
        ];

        CreateModuleProjectFlutter::run_steps_with_loading(steps);

        println!("{}", "Module refresh complete.".green());
    }

    pub fn update_app_injector_file(module_name: &str, selected_choice: &str) {
        let file_path = "./app/lib/injector.dart";

        let mut content = fs::read_to_string(file_path).expect("Failed to read injector.dart");

        let pascal_case_name = module_name
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

        let insert_after = match selected_choice {
            "domain" => "await DomainCommonConfig.getInstance().config(env: environment);",
            "data" => "await DataCommonConfig.getInstance().config(env: environment);",
            "feature" => "await FeatureCommonConfig.getInstance().config(env: environment);",
            _ => return,
        };

        let config_line = format!(
            "  {}Config.getInstance().config(env: environment);",
            pascal_case_name
        );
        let import_line = format!("import 'package:{0}/{0}.dart';", module_name);

        if !content.contains(&import_line) {
            if let Some(last_import_pos) = content.rfind("import ") {
                let semi_colon_pos = content[last_import_pos..]
                    .find(';')
                    .map(|p| p + last_import_pos + 1)
                    .unwrap_or(last_import_pos);
                content.insert_str(semi_colon_pos, &format!("\n{}", import_line));
                println!("Added import line: {}", import_line);
            }
        }

        if !content.contains(&config_line) {
            if let Some(pos) = content.rfind(insert_after) {
                let insert_pos = pos + insert_after.len();
                content.insert_str(insert_pos, &format!("\n{}", config_line));
                println!("Inserted config line: {}", config_line);
            } else {
                println!("Target config call not found in injector.dart.");
                return;
            }
        } else {
            println!("Config line already exists.");
        }

        fs::write(file_path, content).expect("Failed to write injector.dart");

        println!("injector.dart successfully updated.");
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
        Self::generated_route_provider_feature_common(
            "packages/presentation/feature_common/lib/src/route",
            module_name,
        );
        Self::generated_feature_route(&format!("{}/lib/src/config", dest_lib_dir), module_name);
        Self::generated_feature_page(&format!("{}/lib/src/page", dest_lib_dir), module_name);
    }

    fn create_default_data_files(module_name: &str) {
        let source_dir = "default_data_module";
        let dest_lib_dir = &format!("packages/data/{}", module_name);

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
        Self::generated_data_source(
            &format!("{}/lib/src/data_source", dest_lib_dir),
            module_name,
        );
        Self::generated_exposed_file(&format!("{}/lib", dest_lib_dir), module_name)
    }

    fn create_default_domain_files(module_name: &str) {
        let source_dir = "default_domain_module";
        let dest_lib_dir = &format!("packages/domain/{}", module_name);

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
        Self::generated_exposed_file(&format!("{}/lib", dest_lib_dir), module_name)
    }

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
            import 'package:common/common.dart';
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

        let mut file = File::create(&file_path).expect(&format!("Failed to create {}", file_name));

        file.write_all(file_content.as_bytes())
            .expect(&format!("Failed to generate to {}", file_name));

        println!(
            "{} has been successfully generated at {}",
            file_name, file_path
        );
    }

    fn generated_data_source(module_dir: &str, module_name: &str) {
        let name = module_name.strip_prefix("data_").unwrap_or(module_name);

        let class_name = format!(
            "{}{}",
            name.chars().next().unwrap().to_uppercase(),
            &name[1..]
        );

        let file_name = format!("{}_data_source", name);
        let file_path = format!("{}/{}.dart", module_dir, file_name);

        if !Path::new(module_dir).exists() {
            fs::create_dir_all(module_dir).expect("Failed to create the module directory");
        }

        let file_content = indoc! {r#"
            import 'package:data_common/data_common.dart';
            import 'package:dio/dio.dart';
            import 'package:injectable/injectable.dart';
            import 'package:retrofit/retrofit.dart';

            part '{name}_data_source.g.dart';

            @lazySingleton
            @RestApi()
            abstract class {class_name}DataSource {
              @factoryMethod
              factory {class_name}DataSource(Dio dio, DataSourceConfig config) => _{class_name}DataSource(
                    dio,
                    baseUrl: config.baseUrl,
                  );

              @POST('/v1/path')
              Future<void> example({
                @Field('action') required String action,
              });
            }
        "#}
            .replace("{name}", name)
            .replace("{class_name}", &class_name);

        let mut file = File::create(&file_path).expect(&format!("Failed to create {}", file_name));

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

        let mut file = File::create(&file_path).expect(&format!("Failed to create {}", file_name));

        file.write_all(file_content.as_bytes())
            .expect(&format!("Failed to generated to {}", file_name));

        println!(
            "{} has been successfully generated at {}",
            file_name, file_path
        );
    }

    fn generated_route_provider_feature_common(module_dir: &str, module_name: &str) {
        let split_name = module_name.replace("feature_", "");
        let file_name = format!("{}_route_provider.dart", split_name);
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

        let route_name = format!("{}Route", pascal_case_name);

        let file_content = indoc! {r#"
            import 'package:auto_route/auto_route.dart';

            abstract class {route_name}Provider {
              PageRouteInfo route();
            }
        "#}
        .replace("{route_name}", &route_name);

        let mut file = File::create(&file_path).expect(&format!("Failed to create {}", file_name));

        file.write_all(file_content.as_bytes())
            .expect(&format!("Failed to generate {}", file_name));

        println!(
            "{} has been successfully generated at {}",
            file_name, file_path
        );

        Self::update_feature_common_export(&file_name);
    }

    pub fn update_feature_common_export(file_name: &str) {
        let feature_common_path = "packages/presentation/feature_common/lib/feature_common.dart";

        let mut feature_common_content = if Path::new(feature_common_path).exists() {
            fs::read_to_string(feature_common_path).expect("Failed to read feature_common.dart")
        } else {
            String::new()
        };

        let export_line = format!("export 'src/route/{}';\n", file_name);

        if !feature_common_content.contains(&export_line) {
            feature_common_content.push_str(&export_line);
            fs::write(feature_common_path, feature_common_content)
                .expect("Failed to update feature_common.dart");

            println!(
                "Updated feature_common.dart with new export: {}",
                export_line
            );

            let _ = std::process::Command::new("fvm")
                .arg("dart")
                .arg("fix")
                .arg("--apply")
                .arg(feature_common_path)
                .output()
                .expect("Failed to run dart format on feature_common.dart");

            println!("Formatted feature_common.dart.");
        }
    }

    fn generated_feature_route(module_dir: &str, module_name: &str) {
        let file_name = format!("{}_route.dart", module_name);
        let file_path = format!("{}/{}", module_dir, file_name);

        if !Path::new(&module_dir).exists() {
            fs::create_dir_all(&module_dir).expect("Failed to create the module directory");
        }

        let pascal_case_class_name = module_name
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

        let class_name = format!("{}Route", pascal_case_class_name);

        let file_content = indoc! {r#"
            import 'package:auto_route/auto_route.dart';

            @AutoRouterConfig()
            abstract class {class_name} {}

        "#}
        .replace("{class_name}", &class_name);

        let mut file = File::create(&file_path).expect(&format!("Failed to create {}", file_name));

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

        let pascal_case_name = &split_name
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
            import 'package:flutter/material.dart';
            import 'package:injectable/injectable.dart';
            import 'package:{module_name}/src/config/{module_name}_route.gr.dart';

            @Injectable(as: {route_name}Provider)
            class {route_name}ProviderImpl implements {route_name}Provider {
              @override
              PageRouteInfo route() {
                return const {route_name}();
              }
            }

            @RoutePage()
            class {class_name} extends StatelessWidget {

              const {class_name}({
                super.key,
              });

              @override
              Widget build(BuildContext context) {
                return Scaffold(
                  body: Center(
                    child: Text(
                      'Hello! I thought you’d never show up! 😅',
                      style: textTheme.titleLarge,
                      textAlign: TextAlign.center,
                    ),
                  ),
                );
              }
            }

        "#}
        .replace("{module_name}", module_name)
        .replace("{class_name}", &class_name)
        .replace("{split_name}", &split_name)
        .replace("{route_name}", &route_name);

        let mut file = File::create(&file_path).expect(&format!("Failed to create {}", file_name));

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
            export 'src/config/{module_name}_config.dart';
            export 'src/config/{module_name}_route.gr.dart';
        "#}
        .replace("{module_name}", module_name);

        let mut file = File::create(&file_path).expect(&format!("Failed to create {}", file_name));

        file.write_all(file_content.as_bytes())
            .expect(&format!("Failed to generated to {}", file_name));

        println!(
            "{} has been successfully generated at {}",
            file_name, file_path
        );
    }

    fn generated_exposed_file(module_dir: &str, module_name: &str) {
        let file_name = format!("{}.dart", module_name);
        let file_path = format!("{}/{}", module_dir, file_name);

        if !Path::new(module_dir).exists() {
            fs::create_dir_all(module_dir).expect("Failed to create the module directory");
        }

        let file_content = indoc! {r#"
            export 'src/config/{module_name}_config.dart';
        "#}
        .replace("{module_name}", module_name);

        let mut file = File::create(&file_path).expect(&format!("Failed to create {}", file_name));

        file.write_all(file_content.as_bytes())
            .expect(&format!("Failed to generated to {}", file_name));

        println!(
            "{} has been successfully generated at {}",
            file_name, file_path
        );
    }
}
