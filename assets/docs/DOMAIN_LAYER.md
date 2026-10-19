# Domain Layer Guidelines

This document describes the rules and conventions for the Domain layer.

## Directory Structure

Each feature must have its own domain package (e.g., `domain_auth`, `domain_agent`).

```
domain_{feature}/
├── lib/
│   ├── domain_{feature}.dart     # Main export file
│   ├── src/
│   │   ├── config/
│   │   │   └── domain_{feature}_config.dart
│   │   ├── di/
│   │   │   └── di.dart
│   │   ├── model/
│   │   │   ├── {entity}_model.dart
│   │   │   ├── {action}_input.dart
│   │   │   └── {action}_output.dart
│   │   ├── repository/
│   │   │   └── {feature}_repository.dart
│   │   └── use_case/
│   │       └── {action}_use_case.dart
```

## Models

### General Rules

- Use Freezed for immutable models
- Naming: `{entity}_model.dart` (e.g., `agent_model.dart`)
- Must use the `@freezed` annotation
- Do **NOT** use `fromJson` or `JsonKey` annotations (these are only for the data layer)
- Keep business logic in the model
- No external dependencies

### Field Rules

**For fields that can have default values:**
```dart
import 'package:freezed_annotation/freezed_annotation.dart';

part 'transaction_model.freezed.dart';

@freezed
abstract class TransactionModel with _$TransactionModel {
  const factory TransactionModel({
    @Default(0) int id,
    @Default('') String name,
    @Default([]) List<String> tags,
    @Default(false) bool isActive,
  }) = _TransactionModel;
}
```

**For fields that cannot have default values (DateTime, complex objects):**
```dart
import 'package:freezed_annotation/freezed_annotation.dart';

part 'transaction_model.freezed.dart';

@freezed
abstract class TransactionModel with _$TransactionModel {
  const factory TransactionModel({
    @Default(0) int id,
    @Default('') String description,
    required DateTime createdAt,  // Cannot use @Default
    required DateTime updatedAt,  // Cannot use @Default
    @Default(0.0) double amount,
  }) = _TransactionModel;

  factory TransactionModel.empty() => TransactionModel(
    createdAt: DateTime.now(),
    updatedAt: DateTime.now(),
  );
}
```

### Model Reuse Rules

1. Before creating new models, check for existing models in:
   - Current domain module (e.g., `domain_order`)
   - `domain_common` module
   - Other domain modules that might have similar models

2. Reuse existing models if:
   - The model has the same fields and structure
   - The model is in `domain_common` and is meant to be shared
   - The model is in another module but has the exact same purpose

3. Model checking process:
   - Check field names and types
   - Check default values
   - Check nested object structures
   - If all match, reuse the existing model

## Repository Interfaces

- Naming: `{feature}_repository.dart`
- Define abstract methods for data operations
- Return domain models or failures
- Use domain models for input/output
- No implementation details

```dart
abstract class AgentRepository {
  Future<GetAgentListOutput> getAgentList(GetAgentListInput input);
  Future<GetCustomerListOutput> getCustomerList(String customerId);
}
```

## Use Cases

- Naming: `{action}_use_case.dart` (e.g., `get_agent_list_use_case.dart`)
- Use `@lazySingleton` for dependency injection
- Follow the single responsibility principle
- Handle business logic
- Return domain models or failures
- Use repository interfaces

```dart
import 'package:domain_agent/src/model/get_agent_list_input.dart';
import 'package:domain_agent/src/model/get_agent_list_output.dart';
import 'package:domain_agent/src/repository/agent_repository.dart';
import 'package:injectable/injectable.dart';

@lazySingleton
class GetAgentListUseCase {
  final AgentRepository repository;

  GetAgentListUseCase(this.repository);

  Future<GetAgentListOutput> call(GetAgentListInput input) {
    return repository.getAgentList(input);
  }
}
```

## Input/Output Models

### Input Rules

**If the input has only 1 field:**
- Do NOT create an input class
- Use the field type directly

```dart
// Repository
Future<GetCustomerListOutput> getCustomerList(String id);

// Use Case
Future<GetCustomerListOutput> call(String id) {
  return repository.getCustomerList(id);
}
```

**If the input has 2-3 fields:**
- Do NOT create an input class
- Use required named parameters directly

```dart
// Repository
Future<GetAgentListOutput> getAgentList({
  required int page,
  required String searchText,
});

// Use Case
Future<GetAgentListOutput> call({
  required int page,
  required String searchText,
}) {
  return repository.getAgentList(
    page: page,
    searchText: searchText,
  );
}
```

**If the input has more than 3 fields:**
- MUST create an input class

```dart
@freezed
abstract class GetAgentListInput with _$GetAgentListInput {
  const factory GetAgentListInput({
    @Default(1) int page,
    @Default([]) List<String> selectedDistrictIds,
    @Default([]) List<String> selectedHdbTownIds,
    @Default([]) List<String> selectedAreaSpecializations,
    @Default('') String searchText,
  }) = _GetAgentListInput;
}
```

### Output Rules

Output models should have default values for model and failure:

```dart
import 'package:domain_agent/src/model/customer_model.dart';
import 'package:domain_common/domain_common.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part 'get_agent_list_output.freezed.dart';

@freezed
abstract class GetAgentListOutput with _$GetAgentListOutput {
  const factory GetAgentListOutput({
    @Default([]) List<CustomerModel> customers,
    @Default(Failure.noFailure()) Failure failure,
  }) = _GetAgentListOutput;
}
```

## Export Rules

Every new file created in the domain layer MUST be exported in the main `domain_{feature}.dart` file:

```dart
// domain_agent.dart
export 'src/config/domain_agent_config.dart';
export 'src/model/customer_model.dart';
export 'src/model/get_customer_list_output.dart';
export 'src/repository/agent_repository.dart';
export 'src/use_case/get_customer_list_use_case.dart';
```

Export order should follow:
1. Config exports
2. Model exports
3. Repository exports
4. Use case exports

## Import Rules

- All imports MUST use full package paths
- Never use relative imports

```dart
// Correct
import 'package:domain_agent/src/model/customer_model.dart';
import 'package:domain_agent/src/repository/agent_repository.dart';
import 'package:injectable/injectable.dart';

// Incorrect
import '../model/customer_model.dart';
import './customer_model.dart';
```
