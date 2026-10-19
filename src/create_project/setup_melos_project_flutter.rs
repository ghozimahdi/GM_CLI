use crate::create_project::create_module_project_flutter::CreateModuleProjectFlutter;
use include_dir::{include_dir, Dir};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

static ASSETS: Dir = include_dir!("assets");
pub struct SetupMelosProjectFlutter;

impl SetupMelosProjectFlutter {
    pub fn build(project_name: &str, app_path: &str) {
        let pubspec_path = format!("{}/pubspec.yaml", app_path);
        let file = File::open(&pubspec_path).expect("Failed to open pubspec.yaml");
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = vec![];

        let mut resolution_exists = false;

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            if line.trim_start().starts_with("resolution:") {
                resolution_exists = true;
            }
            lines.push(line);
        }

        if !resolution_exists {
            lines.insert(4, "resolution: workspace".to_string());
            let file = File::create(&pubspec_path).expect("Failed to write pubspec.yaml");
            let mut writer = BufWriter::new(file);
            for line in lines {
                writeln!(writer, "{line}").expect("Failed to write line");
            }
            println!("✅ Added 'resolution: workspace' to {pubspec_path}");
        } else {
            println!("ℹ️ 'resolution: workspace' already exists in {pubspec_path}");
        }

        let sdk_version = CreateModuleProjectFlutter::read_environment_from_main_project();

        let template_content = ASSETS
            .get_file("root_pubspec_template.yaml")
            .expect("Failed to read pubspec template from assets")
            .contents_utf8()
            .expect("Failed to convert pubspec template to UTF-8");

        let pubspec_content = template_content
            .replace("{{project_name}}", &project_name)
            .replace("{{sdk_version}}", &sdk_version);

        let mut pubspec_file = File::create("pubspec.yaml").expect("Failed to create pubspec.yaml");
        pubspec_file
            .write_all(pubspec_content.as_bytes())
            .expect("Failed to write pubspec.yaml");
    }
}
