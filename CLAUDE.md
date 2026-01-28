# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

GM CLI (Ghozi Mahdi CLI) is a Rust-based command-line tool for managing Flutter projects using Clean Architecture and modular structure. It provides project scaffolding, multi-module build management, and various Flutter workflow automations.

## Build Commands

```bash
# Build for current architecture (fastest for development)
cargo build --release

# Build for ARM Mac (M1/M2/M3/M4)
cargo build --release --target aarch64-apple-darwin

# Build for all platforms (Intel Mac, ARM Mac, Windows, Linux)
./build.sh

# Run directly during development
cargo run -- [command]
```

Binary output location: `target/release/gm` or `target/{target}/release/gm`

## Architecture

### Source Structure
- **`src/main.rs`** - Entry point, CLI argument parsing with clap, command routing
- **`src/command.rs`** - Utility for executing system commands via `std::process::Command`
- **`src/flutter_commands.rs`** - Core Flutter operations (pub get, clean, build_runner)
- **`src/module_manager.rs`** - Module discovery via Melos, categorization, and interactive selection
- **`src/flutter_build_app.rs`** - APK/IPA build with flavor and version management
- **`src/create_project/`** - Project and module scaffolding:
  - `create_project_flutter.rs` - Full project creation
  - `create_single_module_project_flutter.rs` - Individual module creation
  - `create_module_project_flutter.rs` - Module file generation utilities
  - `create_file_project_flutter.rs` - Route template generation

### Assets (`assets/`)
Embedded project templates bundled into the binary via `include_dir!()`:
- `app/` - Flutter app entry points (main_dev.dart, main_staging.dart, main_prod.dart)
- `data_auth/`, `data_common/` - Data layer templates
- `domain_auth/`, `domain_common/` - Domain layer templates
- `feature_auth/`, `feature_dashboard/`, `feature_common/` - Presentation layer templates
- `default_*_module/` - Templates for creating new modules via `gm create module`

### Folder Naming Convention
All generated folders use **plural names**: `data_sources`, `repositories`, `use_cases`, `models`, `mappers`, `pages`, `blocs`, `routes`, `themes`, `mixins`, `extensions`, `widgets`

## Key Dependencies
- **clap** - CLI argument parsing with derive macros
- **dialoguer** - Interactive terminal prompts
- **colored** - Terminal color output
- **include_dir** - Embed assets directory into binary
- **indoc** - Multi-line string literals for code templates

## External Tool Dependencies
The CLI expects these tools to be available:
- **Flutter** - Mobile framework
- **FVM** - Flutter Version Manager (commands prefixed with `fvm flutter` or `fvm dart`)
- **Melos** - Multi-package workspace management (auto-installed if missing)

## Module Build Priority
When running build_runner across multiple modules, the CLI builds in dependency order:
1. `domain_common` → `common` modules
2. `domain` modules
3. `data` modules
4. `feature_common`
5. `presentation` modules

## Generated Project Structure
```
project/
├── app/                    # Flutter app entry point
├── packages/
│   ├── library/library_common/
│   ├── data/{data_common, data_auth, ...}
│   ├── domain/{domain_common, domain_auth, ...}
│   └── presentation/{feature_common, feature_auth, ...}
├── melos.yaml
└── pubspec.yaml
```

## Testing Changes
After modifying Rust code or assets:
```bash
cargo build --release
./target/release/gm create project  # Test project scaffolding
./target/release/gm create module   # Test module creation
./target/release/gm build           # Test module selection
```
