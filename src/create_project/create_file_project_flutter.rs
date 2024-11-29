use indoc::indoc;
use std::fs::File;
use std::io::Write;

pub struct CreateFileProjectFlutter;

impl CreateFileProjectFlutter {
    pub fn build() {
        Self::create_app_main_files();
        Self::create_feature_login_files();
    }

    fn create_app_main_files() {
        let app_lib_path = "packages/app/lib";
        let files = [
            ("main_dev.dart", "void main() => runApp(MyApp(env: 'dev'));"),
            (
                "main_prod.dart",
                "void main() => runApp(MyApp(env: 'prod'));",
            ),
            (
                "main_staging.dart",
                "void main() => runApp(MyApp(env: 'staging'));",
            ),
        ];

        for (file_name, content) in files.iter() {
            let file_path = format!("{}/{}", app_lib_path, file_name);
            Self::write_file(&file_path, content);
        }
    }

    fn create_feature_login_files() {
        let feature_login_path = "packages/presentation/feature_login/lib";
        let file_path = format!("{}/login_page.dart", feature_login_path);
        let content = indoc! {r#"
        import 'package:flutter/material.dart';

        class LoginPage extends StatelessWidget {
            @override
            Widget build(BuildContext context) {
                return Scaffold(
                    appBar: AppBar(
                        title: Text('Login Page'),
                    ),
                    body: Center(
                        child: Text('Welcome to Login Page'),
                    ),
                );
            }
        }
        "#};

        Self::write_file(&file_path, content);
    }

    fn write_file(file_path: &str, content: &str) {
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");
    }
}
