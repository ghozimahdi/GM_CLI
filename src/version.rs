use clap::Command as ClapCommand;

pub fn handle_version() {
    let _version_info = ClapCommand::new("gm")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Ghozi Mahdi CLI for managing Melos modules\n\n\
            Prerequisites:\n\
            1. Ensure that you have Melos installed.\n\
            2. Run the 'melos list' command in your Flutter project to ensure Melos is working.\n\
            3. If you encounter the error 'Error: Failed to retrieve module list.', check if your project is correctly configured for Melos.\n\n\
            Usage Instructions:\n\
            1. Navigate to the root directory of your Flutter project.\n\
            2. Run the 'gm' command to start the CLI.\n\
            3. Follow the on-screen instructions to select the module you want to manage.")
        .author("Ghozi Mahdi <ghozimahdi@carpkingdom.com>")
        .render_version();
}
