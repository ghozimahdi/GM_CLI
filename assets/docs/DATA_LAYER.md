# Data Layer Guidelines

This document describes the rules and conventions for the Data layer.

## Directory Structure

Each feature must have its own data package (e.g., `data_auth`, `data_agent`).

```
data_{feature}/
├── lib/
│   ├── data_{feature}.dart       # Main export file
│   ├── src/
│   │   ├── config/
│   │   │   └── data_{feature}_config.dart
│   │   ├── data_source/
│   │   │   └── {feature}_data_source.dart
│   │   ├── di/
│   │   │   └── di.dart
│   │   ├── mapper/
│   │   │   ├── {entity}_model_mapper.dart
│   │   │   └── {action}_output_mapper.dart
│   │   ├── model/
│   │   │   ├── {entity}_data.dart
│   │   │   ├── {action}_response.dart
│   │   │   └── {action}_request.dart
│   │   └── repository/
│   │       └── {feature}_repository_impl.dart
```

## Models (DTOs)

### File Structure

- Each model MUST be in its own file
- NO multiple classes in a single model file
- Must include both `.freezed.dart` and `.g.dart` parts

```dart
import 'package:freezed_annotation/freezed_annotation.dart';

part 'customer_data.freezed.dart';
part 'customer_data.g.dart';
```

### Model Structure

- Must use `@freezed` annotation
- Must extend generated class with `_$` prefix
- Must include `fromJson` factory constructor
- All fields must be nullable and use `@JsonKey` annotation

```dart
@freezed
class CustomerData with _$CustomerData {
  const factory CustomerData({
    @JsonKey(name: "id") int? id,
    @JsonKey(name: "name") String? name,
    @JsonKey(name: "email") String? email,
  }) = _CustomerData;

  factory CustomerData.fromJson(Map<String, dynamic> json) =>
      _$CustomerDataFromJson(json);
}
```

### Response Model Example

```dart
@freezed
class GetCustomerListResponse with _$GetCustomerListResponse {
  const factory GetCustomerListResponse({
    @JsonKey(name: "success") bool? success,
    @JsonKey(name: "data") List<CustomerData>? data,
  }) = _GetCustomerListResponse;

  factory GetCustomerListResponse.fromJson(Map<String, dynamic> json) =>
      _$GetCustomerListResponseFromJson(json);
}
```

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Response models | `{Action}Response` | `GetCustomerListResponse` |
| Data models | `{Entity}Data` | `CustomerData` |
| Request models | `{Action}Request` | `GetCustomerListRequest` |

### Field Rules

- All fields must be nullable (`Type?`)
- Use `@JsonKey` with `name` parameter for JSON field mapping
- For nested objects, create separate data models
- For lists, use `List<Type>?`

### Model Reuse Rules

1. Before creating new models, check for existing models in:
   - Current data module (e.g., `data_order`)
   - `data_common` module
   - Other data modules that might have similar models

2. Reuse existing models if:
   - The model has the same fields and structure
   - The model is in `data_common` and is meant to be shared

## Data Source

### Structure

- Must be an abstract class with `@RestApi()` annotation
- Must use `@lazySingleton` for dependency injection
- Must have factory constructor with Dio and config
- Methods must be annotated with HTTP methods

```dart
import 'package:dio/dio.dart';
import 'package:injectable/injectable.dart';
import 'package:retrofit/retrofit.dart';

import 'package:data_agent/src/model/get_customer_list_response.dart';

part 'agent_data_source.g.dart';

@RestApi()
@lazySingleton
abstract class AgentDataSource {
  @factoryMethod
  factory AgentDataSource(Dio dio, {String baseUrl}) = _AgentDataSource;

  @GET('/api/customers')
  Future<GetCustomerListResponse> getCustomerList(
    @Query('page') int page,
    @Query('search') String search,
  );

  @POST('/api/customers')
  Future<CreateCustomerResponse> createCustomer(
    @Body() CreateCustomerRequest request,
  );
}
```

### Parameter Annotations

| Annotation | Use Case |
|------------|----------|
| `@Query()` | Query parameters |
| `@Field()` | Form data |
| `@Body()` | Request body (JSON) |
| `@Part()` | Multipart data |
| `@Path()` | Path parameters |

## Repository Implementation

### Structure

- Must implement domain repository interface
- Must have fields for ALL required mappers
- Must inject ALL required mappers in constructor
- Must handle errors using try-catch blocks
- Must return domain models or failures

```dart
import 'package:domain_agent/domain_agent.dart';
import 'package:domain_common/domain_common.dart';
import 'package:injectable/injectable.dart';

import 'package:data_agent/src/data_source/agent_data_source.dart';
import 'package:data_agent/src/mapper/get_customer_list_output_mapper.dart';

@LazySingleton(as: AgentRepository)
class AgentRepositoryImpl implements AgentRepository {
  final AgentDataSource _dataSource;
  final GetCustomerListOutputMapper _getCustomerListOutputMapper;

  AgentRepositoryImpl(
    this._dataSource,
    this._getCustomerListOutputMapper,
  );

  @override
  Future<GetCustomerListOutput> getCustomerList({
    required int page,
    required String searchText,
  }) async {
    try {
      final response = await _dataSource.getCustomerList(page, searchText);
      return _getCustomerListOutputMapper.mapFromData(response);
    } catch (e) {
      return GetCustomerListOutput(
        failure: Failure.unexpectedError(message: e.toString()),
      );
    }
  }
}
```

### Error Handling

- Use `Failure.unexpectedError(message: e.toString())` for errors
- Include proper error messages
- Handle null cases properly

## Import Rules

### Import Ordering

Group imports in the following order:
1. Dart core imports (e.g., `dart:async`)
2. Package imports (e.g., `package:domain_agent/domain_agent.dart`)
3. Data package imports (e.g., `package:data_agent/src/model/...`)

Each group must be separated by a single empty line.

```dart
import 'dart:async';

import 'package:domain_agent/domain_agent.dart';
import 'package:injectable/injectable.dart';

import 'package:data_agent/src/model/customer_data.dart';
import 'package:data_agent/src/mapper/customer_model_mapper.dart';
```

### Import Path Rules

- NEVER use relative imports
- ALWAYS use full package paths

```dart
// Correct
import 'package:data_agent/src/model/customer_data.dart';

// Incorrect
import '../model/customer_data.dart';
import './customer_model_mapper.dart';
```

## Configuration

### DI Setup (`di.dart`)

```dart
import 'package:data_agent/src/di/di.config.dart';
import 'package:get_it/get_it.dart';
import 'package:injectable/injectable.dart';

final getIt = GetIt.instance;

@injectableInit
Future<void> configureInjection({required String env}) async =>
    getIt.init(environment: env);
```

### Config Class

```dart
import 'package:data_agent/src/di/di.dart';
import 'package:library_common/library_common.dart';

class DataAgentConfig extends AppConfig {
  DataAgentConfig._();

  factory DataAgentConfig.getInstance() => _instance;

  static final DataAgentConfig _instance = DataAgentConfig._();

  @override
  Future<bool> config({required String env}) async {
    await configureInjection(env: env);
    return true;
  }
}
```
