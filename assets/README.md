# Project Name

Generated using [gm-cli](https://github.com/mbahgojol/homebrew-ghozimahdi-cli)

## Description

This is a Flutter project generated with **Clean Architecture** and **Modular Structure**. It provides a robust and
scalable starting point for Flutter applications, adhering to best practices for maintainable and testable code.

## Features

- **Clean Architecture**: Separation of concerns with layers for Domain, Data, and Presentation.
- **Modular Structure**: Encapsulation of features for better scalability and maintainability.
- **State Management**: [BLoC](https://bloclibrary.dev/) for predictable state management.
- **Dependency Injection**: Powered by [Injectable](https://pub.dev/packages/injectable).
- **Flutter Flavorizr**: Ready for multiple environments (e.g., Development, Production).

## Project Structure

```plaintext
packages/
  ├── library/
  │   ├── core/
  │   └── resource/
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

## Getting Started

### Prerequisites

- [Flutter SDK](https://flutter.dev/docs/get-started/install)
- `gm-cli` installed (for regenerating similar projects)

### Installation

1. Add the Ghozi Mahdi CLI tap:

   ```bash
   brew tap mbahgojol/homebrew-ghozimahdi-cli
   ```

2. Install the Ghozi Mahdi CLI via Homebrew:

   ```bash
   brew install gm
   # for update gm-cli
   brew update
   brew upgrade gm
   ```

3. Create Project with Ghozi Mahdi CLI:

   ```bash
   gm create project
   ```

### Flavors

This project is pre-configured for multiple environments. To run a specific flavor:

```bash
flutter run --flavor development -t lib/main_development.dart
flutter run --flavor production -t lib/main_production.dart
```

## Usage

Run the following command inside your Flutter project directory:

```bash
gm [OPTIONS] [COMMAND]
```

### **Commands**

- `clear`: Run `flutter_clean` and remove `pubspec.lock` for selected modules.
- `app`: Interactively build APK or IPA files.
- `create`: Create a new Flutter project or module.
- `build`: Run `build_runner` after selecting a module.
- `get`: Run `flutter pub get` after selecting a module.
- `donate`: Show gratitude and support the development of this CLI tool.
- `info`: Displays prerequisites and usage instructions for the gm CLI.
- `help`: Print this message or the help of the given subcommand(s).

### **Options**

- `-b`, `--build`: Run `build_runner` after module selection.
- `-l`, `--localize`: Generate `easy_localization` files.
- `-a`, `--assets`: Generate assets with `flutter gen`.
- `-c`, `--flutter_clean`: Flutter clean for a single module.
- `-g`, `--flutter_get`: Run `flutter pub get` for a single module.
- `--all`: Run `build_runner`, generate `easy_localization`, and assets after selecting a module.
- `-h`, `--help`: Print help information.
- `-V`, `--version`: Print version information.

---

## Support Us

🎯 **Support Us**

This project is maintained by an independent developer. Support further development through sponsorship! 💖  
Your donations help us to:

- Add new features.
- Fix bugs faster.
- Improve documentation.

[![GitHub Sponsors](https://img.shields.io/badge/Sponsor-GitHub%20Sponsors-blue.svg)](https://github.com/sponsors/ghozimahdi)

- Ko-fi: [ghozimahdi-ko-fi](https://ko-fi.com/ghozimahdi)
- Trakteer: [ghozimahdi-trakteer](https://trakteer.id/ghozimahdi)

## Contributing

Feel free to contribute to this project! If you find any issues or have suggestions for improvements, please create an
issue or submit a pull request.

## License

This project is licensed under the [Apache License 2.0](LICENSE).
