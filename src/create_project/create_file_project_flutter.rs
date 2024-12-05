use std::fs;
use std::io::Write;

pub struct CreateFileProjectFlutter;

impl CreateFileProjectFlutter {
    pub fn build() {
        Self::create_app_main_files();
        Self::create_feature_login_page_files();
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
            Self::write_file(&file_path, content);
        }
    }

    fn create_feature_login_page_files() {
        let feature_login_path = "packages/presentation/feature_login/lib";
        fs::create_dir_all(feature_login_path).expect("Failed to create feature login directory");

        let file_path = format!("{}/login_page.dart", feature_login_path);

        let dart_file_content = include_str!("../../assets/login_page.dart");

        println!("Writing to: {}", file_path);
        Self::write_file(&file_path, dart_file_content);
    }

    fn write_file(file_path: &str, content: &str) {
        let mut file = fs::File::create(file_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");
    }
}
