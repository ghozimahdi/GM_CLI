# Architecture Overview

This document describes the overall architecture of the ninaite_app project.

## Clean Architecture

The project follows Clean Architecture principles with a modular structure using Melos as a monorepo tool.

```
┌─────────────────────────────────────────────────────────────┐
│                          APP                                 │
│  (main_*.dart, injector.dart, app_router.dart)              │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    PRESENTATION LAYER                        │
│  packages/presentation/feature_*                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │ feature_    │  │ feature_    │  │ feature_    │         │
│  │ common      │  │ auth        │  │ dashboard   │         │
│  │ (widgets,   │  │ (login,     │  │ (home,      │         │
│  │  themes,    │  │  register)  │  │  profile)   │         │
│  │  i18n)      │  │             │  │             │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      DOMAIN LAYER                            │
│  packages/domain/domain_*                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  - Models (business entities)                        │   │
│  │  - Use Cases (business logic)                        │   │
│  │  - Repository Interfaces (contracts)                 │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                       DATA LAYER                             │
│  packages/data/data_*                                        │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  - DTOs (data transfer objects from API)             │   │
│  │  - Mappers (DTO <-> Model conversion)                │   │
│  │  - Repository Implementations                        │   │
│  │  - Data Sources (API, Local)                         │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     LIBRARY LAYER                            │
│  packages/library/library_common                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  - App Config                                        │   │
│  │  - Flavor Configuration                              │   │
│  │  - Environment Variables                             │   │
│  │  - Core Utilities                                    │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Package Structure

### Library Layer (`packages/library/`)

**library_common** - Core utilities shared across all packages
```
lib/src/
├── app_config.dart    # Application configuration
├── flavor.dart        # Build flavor enum
└── app_env.dart       # Environment variables
```

### Domain Layer (`packages/domain/`)

**domain_common** - Shared domain logic
```
lib/
├── di/               # Dependency injection modules
├── model/            # Domain models (business entities)
├── use_case/         # Base use case classes
└── repository/       # Repository interfaces
```

**domain_{feature}** - Feature-specific domain
```
lib/
├── di/
├── model/            # Feature models (e.g., UserModel, AuthModel)
├── use_case/         # Feature use cases (e.g., LoginUseCase)
└── repository/       # Repository interfaces
```

### Data Layer (`packages/data/`)

**data_common** - Shared data infrastructure
```
lib/
├── di/               # Network module, storage module
├── config/           # API configuration, DataSourceConfig
├── model/            # Base DTOs
├── mapper/           # Base mappers
└── repository/       # Base repository implementation
```

**data_{feature}** - Feature-specific data
```
lib/
├── di/
├── model/            # DTOs (e.g., UserDto with fromJson/toJson)
├── mapper/           # Mappers (Dto -> Model conversion)
├── repository/       # Repository implementations
└── data_source/      # API services (Retrofit)
```

### Presentation Layer (`packages/presentation/`)

**feature_common** - Shared UI components
```
lib/
├── generated/        # flutter_gen assets
├── i18n/             # Localization (slang)
├── di/
├── config/           # Theme, routes
├── widget/           # Reusable widgets
├── route/            # Route providers (abstract)
└── util/             # UI utilities
```

**feature_{feature}** - Feature-specific UI
```
lib/
├── di/
├── bloc/             # BLoC files (bloc, event, state)
├── page/             # Pages
├── widget/           # Feature-specific widgets
└── config/           # Feature config and routes
```

## Build Flavors

| Flavor | Entry Point | Description |
|--------|-------------|-------------|
| dev | `main_dev.dart` | Development |
| staging | `main_staging.dart` | Pre-production |
| production | `main_prod.dart` | Production |

## Dependencies Flow

```
Presentation → Domain ← Data
     │            │        │
     ▼            ▼        ▼
  feature_*   domain_*  data_*
     │            │        │
     └────────────┴────────┘
              │
              ▼
         library_common
```

**Rules:**
- Presentation layer can only depend on Domain layer
- Data layer can only depend on Domain layer
- Domain layer has no dependencies on other layers
- All layers can depend on Library layer

## Key Principles

1. **Separation of Concerns** - Each layer has a specific responsibility
2. **Dependency Inversion** - High-level modules don't depend on low-level modules
3. **Single Responsibility** - Each class/module has one reason to change
4. **Interface Segregation** - Repository interfaces in Domain, implementations in Data
5. **Modular Design** - Features are isolated in their own packages
