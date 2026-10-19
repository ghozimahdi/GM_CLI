# Mapper Guidelines

This document describes the rules and conventions for creating mappers.

## General Rules

- Mappers should be in the **data layer**
- Each mapper class should only map **one model** (single responsibility)
- Use **classes** for mapping, **not extensions**
- Handle null safety properly
- Keep mapping logic simple and clear

## Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Data to Domain | `{Entity}ModelMapper` | `CustomerModelMapper` |
| Response to Output | `{Action}OutputMapper` | `GetCustomerListOutputMapper` |
| Domain to Request | `{Action}RequestMapper` | `CreateCustomerRequestMapper` |

## Entity Model Mapper

Maps a single data model to a domain model.

```dart
import 'package:domain_agent/domain_agent.dart';
import 'package:injectable/injectable.dart';

import 'package:data_agent/src/model/customer_data.dart';

@lazySingleton
class CustomerModelMapper {
  CustomerModel mapFromData(CustomerData? data) {
    return CustomerModel(
      id: data?.id ?? 0,
      name: data?.name ?? '',
      email: data?.email ?? '',
    );
  }
}
```

## Output Mapper

Maps a response to an output, using entity mappers for nested objects.

```dart
import 'package:domain_agent/domain_agent.dart';
import 'package:injectable/injectable.dart';

import 'package:data_agent/src/mapper/customer_model_mapper.dart';
import 'package:data_agent/src/model/get_customer_list_response.dart';

@lazySingleton
class GetCustomerListOutputMapper {
  final CustomerModelMapper _customerModelMapper;

  GetCustomerListOutputMapper(this._customerModelMapper);

  GetCustomerListOutput mapFromData(GetCustomerListResponse? response) {
    return GetCustomerListOutput(
      customers: (response?.data ?? [])
          .map(_customerModelMapper.mapFromData)
          .toList(),
    );
  }
}
```

## Request Mapper

Maps domain input to a data request.

```dart
import 'package:domain_agent/domain_agent.dart';
import 'package:injectable/injectable.dart';

import 'package:data_agent/src/model/create_customer_request.dart';

@lazySingleton
class CreateCustomerRequestMapper {
  CreateCustomerRequest mapToRequest(CreateCustomerInput input) {
    return CreateCustomerRequest(
      name: input.name,
      email: input.email,
      phone: input.phone,
    );
  }
}
```

## Complex Nested Mapper

For responses with multiple nested objects:

```dart
import 'package:domain_order/domain_order.dart';
import 'package:injectable/injectable.dart';

import 'package:data_order/src/mapper/order_item_model_mapper.dart';
import 'package:data_order/src/mapper/shipping_model_mapper.dart';
import 'package:data_order/src/model/get_order_detail_response.dart';

@lazySingleton
class GetOrderDetailOutputMapper {
  final OrderItemModelMapper _orderItemMapper;
  final ShippingModelMapper _shippingMapper;

  GetOrderDetailOutputMapper(
    this._orderItemMapper,
    this._shippingMapper,
  );

  GetOrderDetailOutput mapFromData(GetOrderDetailResponse? response) {
    final data = response?.data;
    return GetOrderDetailOutput(
      id: data?.id ?? '',
      orderId: data?.orderId ?? '',
      status: data?.orderStatus ?? '',
      items: (data?.items ?? [])
          .map(_orderItemMapper.mapFromData)
          .toList(),
      shipping: _shippingMapper.mapFromData(data?.shipping),
    );
  }
}
```

## Import Rules

### Import Ordering

```dart
import 'dart:async';

import 'package:domain_agent/domain_agent.dart';
import 'package:injectable/injectable.dart';

import 'package:data_agent/src/model/customer_data.dart';
import 'package:data_agent/src/model/get_customer_list_response.dart';
```

Order:
1. Dart core imports
2. Package imports (domain, injectable, etc.)
3. Data package imports

### Required Imports

Every mapper MUST import:
1. The corresponding data model
2. The main domain package (e.g., `domain_agent.dart`) - **DO NOT import specific domain files**
3. Injectable package
4. Other mappers if needed as dependencies

## Mapper Reuse Rules

1. Check for existing mappers in:
   - Current data module
   - `data_common` module

2. Reuse existing mappers if they handle the same model types

3. Only create new mappers if no existing mapper matches

## Best Practices

1. Use `@lazySingleton` annotation for dependency injection
2. Inject required mappers in the constructor
3. Handle null cases with null-coalescing operator (`??`)
4. Use proper type casting when needed
5. Keep mapping logic simple and clear
6. Document complex mappings
7. Test mapping logic
8. Handle edge cases
9. Use proper error handling
10. Follow single responsibility principle

## File Formatting

Every generated file **must end with a single empty line** to avoid linter issues.

## Example: Full Mapper Flow

**Data Model:**
```dart
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

**Domain Model:**
```dart
@freezed
abstract class UserTagModel with _$UserTagModel {
  const factory UserTagModel({
    @Default('') String id,
    @Default('') String name,
  }) = _UserTagModel;
}
```

**Entity Mapper:**
```dart
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

**Output Mapper:**
```dart
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
