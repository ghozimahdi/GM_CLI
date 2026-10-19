# JSON to Dart Conversion Guidelines

This document describes the rules for converting JSON API responses to Dart models.

## Generation Process

### 1. Response Class Generation

- Generate response class with name `{Action}Response` (e.g., `GetUserTagResponse`)
- Check if similar response class exists in:
  1. Current data module (e.g., `data_order`)
  2. `data_common` module
- If exists, reuse existing class

### 2. Data Class Generation

- For each nested object, generate data class with name `{Entity}Data` (e.g., `UserTagData`)
- Check if similar data class exists before creating
- For nested objects within data classes, follow the same process

### 3. Domain Model Generation

- Generate domain model with name `{Entity}Model` (e.g., `UserTagModel`)
- Check if similar model exists in domain modules
- Reuse existing models when possible

### 4. Mapper Generation

- Generate mappers for converting between data and domain models
- Check if similar mappers exist before creating

## Data Layer Models

### Response Model

```dart
import 'package:freezed_annotation/freezed_annotation.dart';

import 'package:data_order/src/model/user_tag_data.dart';

part 'get_user_tag_response.freezed.dart';
part 'get_user_tag_response.g.dart';

@freezed
class GetUserTagResponse with _$GetUserTagResponse {
  const factory GetUserTagResponse({
    @JsonKey(name: "success") bool? success,
    @JsonKey(name: "data") List<UserTagData>? data,
  }) = _GetUserTagResponse;

  factory GetUserTagResponse.fromJson(Map<String, dynamic> json) =>
      _$GetUserTagResponseFromJson(json);
}
```

### Data Model

```dart
import 'package:freezed_annotation/freezed_annotation.dart';

part 'user_tag_data.freezed.dart';
part 'user_tag_data.g.dart';

@freezed
class UserTagData with _$UserTagData {
  const factory UserTagData({
    @JsonKey(name: "_id") String? id,
    @JsonKey(name: "name") String? name,
  }) = _UserTagData;

  factory UserTagData.fromJson(Map<String, dynamic> json) =>
      _$UserTagDataFromJson(json);
}
```

### Data Model Rules

1. Each model MUST be in its own file
2. Must include both `.freezed.dart` and `.g.dart` parts
3. Must use `@freezed` annotation
4. Must include `fromJson` factory constructor
5. All fields must be nullable (`Type?`)
6. Use `@JsonKey(name: "json_field_name")` for all fields

## Domain Layer Models

### Entity Model

```dart
import 'package:freezed_annotation/freezed_annotation.dart';

part 'user_tag_model.freezed.dart';

@freezed
abstract class UserTagModel with _$UserTagModel {
  const factory UserTagModel({
    @Default('') String id,
    @Default('') String name,
  }) = _UserTagModel;
}
```

### Output Model

```dart
import 'package:domain_common/domain_common.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

import 'package:domain_order/src/model/user_tag_model.dart';

part 'get_user_tag_output.freezed.dart';

@freezed
abstract class GetUserTagOutput with _$GetUserTagOutput {
  const factory GetUserTagOutput({
    @Default(false) bool success,
    @Default([]) List<UserTagModel> tags,
    @Default(Failure.noFailure()) Failure failure,
  }) = _GetUserTagOutput;
}
```

### Domain Model Rules

1. Each model MUST be in its own file
2. Must include `.freezed.dart` part (NO `.g.dart`)
3. Must use `@freezed` annotation
4. **Do NOT use** `fromJson` or `@JsonKey`
5. All fields must use `@Default()` annotation
6. All fields must be non-nullable

## Default Values Reference

| Type       | Default Value                      |
| ---------- | ---------------------------------- |
| `int`      | `@Default(0)`                      |
| `double`   | `@Default(0.0)`                    |
| `String`   | `@Default('')`                     |
| `bool`     | `@Default(false)`                  |
| `List<T>`  | `@Default([])`                     |
| `DateTime` | Use `required` + `empty()` factory |
| `Enum`     | `@Default(EnumType.value)`         |
| `Object`   | `@Default(ObjectType())`           |
| `Failure`  | `@Default(Failure.noFailure())`    |

## Mappers

### Entity Mapper

```dart
import 'package:domain_order/domain_order.dart';
import 'package:injectable/injectable.dart';

import 'package:data_order/src/model/user_tag_data.dart';

@lazySingleton
class UserTagModelMapper {
  UserTagModel mapFromData(UserTagData? data) {
    return UserTagModel(
      id: data?.id ?? '',
      name: data?.name ?? '',
    );
  }
}
```

### Output Mapper

```dart
import 'package:domain_order/domain_order.dart';
import 'package:injectable/injectable.dart';

import 'package:data_order/src/mapper/user_tag_model_mapper.dart';
import 'package:data_order/src/model/get_user_tag_response.dart';

@lazySingleton
class GetUserTagOutputMapper {
  final UserTagModelMapper _userTagModelMapper;

  GetUserTagOutputMapper(this._userTagModelMapper);

  GetUserTagOutput mapFromData(GetUserTagResponse? response) {
    return GetUserTagOutput(
      success: response?.success ?? false,
      tags: (response?.data ?? [])
          .map(_userTagModelMapper.mapFromData)
          .toList(),
    );
  }
}
```

## Complex JSON Example

### JSON Response

```json
{
  "success": true,
  "data": {
    "_id": "68235e46e252e84e62c9dcce",
    "orderId": "ORD-001",
    "trackingStatus": {
      "status": "pending",
      "message": "Order is being processed"
    }
  }
}
```

### Generated Models

**Data Models:**

- `GetOrderDetailResponse` - Root response
- `OrderDetailData` - Main data object
- `TrackingStatusData` - Nested object

**Domain Models:**

- `GetOrderDetailOutput` - Output with failure
- `OrderDetailModel` - Business entity
- `TrackingStatusModel` - Nested business entity

**Mappers:**

- `TrackingStatusModelMapper` - Maps `TrackingStatusData` → `TrackingStatusModel`
- `OrderDetailModelMapper` - Maps `OrderDetailData` → `OrderDetailModel` (uses `TrackingStatusModelMapper`)
- `GetOrderDetailOutputMapper` - Maps response → output (uses `OrderDetailModelMapper`)
