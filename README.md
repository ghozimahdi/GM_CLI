
# **Ghozi Mahdi CLI**

A simple CLI tool designed to simplify the process of managing **Flutter** projects and modules. Built using **Rust**, this tool provides an intuitive interface to streamline project management tasks.

---

## **Features**
- **Clear**: Clean the Flutter project and remove `pubspec.lock` for selected modules.
- **App**: Interactively build APK or IPA files.
- **Flutter**: Run Flutter commands like `pub add <package>`.
- **Dart**: Run Dart commands like `pub add <package>`.
- **Route**: Automatically generate route files for Flutter project.
- **Create**: 
  - **Create Project**: Generate a new Flutter project with a well-organized Clean Architecture and modular structure.  
  - **Create Module**: Create a module for modularization to enhance scalability and maintainability.
- **Build**: Run `build_runner` after selecting a module.
- **Get**: Execute `flutter pub get` for selected modules.
- **Donate**: Show gratitude and support the development of this CLI tool.
- **Info**: Display prerequisites and usage instructions for the `gm CLI`.
- Built-in support for running tasks such as localization and asset generation.
- High performance and efficiency powered by **Rust**.

---

## **Prerequisites**
Ensure you have the following prerequisites installed before using this CLI:

1. **Flutter**:
   - Ensure Flutter is installed on your system.
   - Install Flutter: [https://flutter.dev/docs/get-started/install](https://flutter.dev/docs/get-started/install)

---

## **Installation via Homebrew**
This repository contains the source code. Prebuilt binaries are distributed through the Homebrew tap [ghozimahdi/homebrew-ghozimahdi-cli](https://github.com/ghozimahdi/homebrew-ghozimahdi-cli), which is updated automatically on every release.

1. **Add the Ghozi Mahdi CLI tap**:
   ```bash
   brew tap ghozimahdi/homebrew-ghozimahdi-cli
   ```

2. **Install the Ghozi Mahdi CLI** via Homebrew:
   ```bash
   brew install gm
   ```

   Or in a single command, without tapping first:
   ```bash
   brew install ghozimahdi/homebrew-ghozimahdi-cli/gm
   ```

3. Once installed, you can run the CLI tool from your Flutter project directory:
   ```bash
   gm help
   ```

---

## **Update via Homebrew**

To update the Ghozi Mahdi CLI to the latest version, run:
```bash
brew update
brew upgrade gm
```

---

## **Build from Source**
Requires [Rust](https://www.rust-lang.org/tools/install):
```bash
git clone https://github.com/ghozimahdi/GM_CLI.git
cd GM_CLI
cargo build --release
./target/release/gm help
```

---

## **Usage**
Run the following command inside your Flutter project directory:
```bash
gm [OPTIONS] [COMMAND]
```

### Examples
- `gm -b` : Run `build_runner` (shortcut for `gm build`).
- `gm -g` : Run `flutter pub get` (shortcut for `gm get`).
- `gm -c` : Run `flutter clean` (shortcut for `gm clear`).

### **Commands**
- `clear`: Run `flutter_clean` and remove `pubspec.lock` for selected modules.
- `app`: Interactively build APK or IPA files.
- `flutter`: Run Flutter commands like `pub add <package>`.
- `dart`: Run Dart commands like `pub add <package>`.
- `route`: Automatically generate route files for Flutter project.
- `create project`: Generate a new Flutter project with a well-organized Clean Architecture and modular structure.
- `create module`: Create a new module specifically for modularization to enhance scalability and maintainability.
- `build`: Run `build_runner` after selecting a module.
- `get`: Run `flutter pub get` after selecting a module.
- `donate`: Show gratitude and support the development of this CLI tool.
- `info`: Displays prerequisites and usage instructions for the gm CLI.
- `help`: Print this message or the help of the given subcommand(s).

### **Options**
- `-b`, `--build`: Run `build_runner` after module selection.
- `-l`, `--localize`: Generate `easy_localization` files.
- `-a`, `--assets`: Generate assets with `flutter gen`.
- `-c`, `--flutter_clean`: Flutter clear for a single module.
- `-g`, `--flutter_get`: Run `flutter pub get` for a single module.
- `--all`: Run `build_runner`, generate `easy_localization`, and assets after selecting a module.
- `-h`, `--help`: Print help information.
- `-V`, `--version`: Print version information.

### Notes

- You can use **commands** (e.g., `gm build`) or their corresponding **options** (e.g., `gm -b`) for faster access.
- **Options** are designed to simplify frequently used commands.

---

## **Generated Project Structure**

When you use the `gm create` command, the generated project follows the **Clean Architecture** principles with a modular structure. Below is an example of the generated structure:

```
app/
packages/
  ├── library/
  │   ├── library_common/
  ├── data/
  │   ├── data_common/
  │   ├── data_auth/
  │   └── data_analytic/
  │       ├── di/
  │       ├── config/
  │       ├── model/
  │       ├── mapper/
  │       └── repository/
  ├── domain/
  │   ├── domain_common/
  │   ├── domain_auth/
  │   └── domain_analytic/
  │       ├── di/
  │       ├── config/
  │       ├── model/
  │       ├── use_case/
  │       └── repository/
  └── presentation/
      ├── feature_common/
      ├── feature_dashboard/
      └── feature_auth/
          ├── bloc/
          ├── config/
          ├── di/
          └── page/
```

### **Description of Main Directories**
1. **Library**:
   - Contains shared utilities, components, and resources that are commonly used across multiple modules.
2. **Data**:
   - Responsible for managing data sources, configurations, models, mappers, and repository implementations for different modules.
3. **Domain**:
   - Defines business logic, including use cases, repository interfaces, and domain-specific configurations for each module.
4. **Presentation**:
   - Manages the UI layer, which includes features, pages, state management (e.g., BLoC), and configurations for the presentation layer.

### **Highlights**
- **Modularity**: Each feature (e.g., `auth`, `dashboard`, `analytic`) is modularized to improve scalability and maintainability.
- **Separation of Concerns**: Code is structured to clearly separate the responsibilities of each layer (Data, Domain, Presentation).
- **Reusable Components**: Core utilities and shared resources in the `library` directory enable easy reuse across modules.

## **Result of `build_runner`**

The `gm build` command provides an interactive interface to select specific modules for code generation using the `build_runner`. This is designed for managing multi-module Flutter projects.

### **Example Interface**:
```dart
+----------------+------------------+----------------------+---------------------+
| Data Modules   | Domain Modules   | Feature Modules      | Other Modules       |
+----------------+------------------+----------------------+---------------------+
| 3. data_auth   | 5. domain_auth   | 7. feature_auth      | 1. app              |
| 4. data_common | 6. domain_common | 8. feature_common    | 2. library_common   |
|                |                  | 9. feature_dashboard |                     |
|                |                  |                      |                     |
+----------------+------------------+----------------------+---------------------+
Total modules available: 9, Select modules (example: 1,2,3 or 0 for all): 
```

### **Key Features**:
- **Module Selection**: Choose specific modules (e.g., `1,2,3`) or generate code for all modules (`0`).
- **Multi-Module Support**: Easily handle code generation across multiple modules in your Flutter project.
- **Flexibility**: Allows developers to focus on relevant modules during the code generation process.

## **🎯 Support Us**
This project is maintained by an independent developer. Support further development through sponsorship!
💖 Your donations help us to:
- Add new features.
- Fix bugs faster.
- Improve documentation.

[![Sponsor](https://img.shields.io/badge/Sponsor-GitHub%20Sponsors-blue)](https://github.com/sponsors/ghozimahdi)

Ko-fi: [ghozimahdi-ko-fi](https://ko-fi.com/ghozimahdi)  
Saweria: [ghozimahdi-saweria](https://saweria.co/ghozimahdi)
