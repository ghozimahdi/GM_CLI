# BLoC Pattern Guidelines

This document describes the rules and conventions for implementing BLoC pattern.

## File Structure

Each BLoC consists of 3 files in the `bloc/` folder:
- `{feature}_bloc.dart` - Main BLoC class
- `{feature}_event.dart` - Event definitions
- `{feature}_state.dart` - State definitions

## Bloc File (`{feature}_bloc.dart`)

```dart
import 'package:domain_auth/domain_auth.dart';
import 'package:domain_common/domain_common.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:injectable/injectable.dart';

part 'home_bloc.freezed.dart';
part 'home_event.dart';
part 'home_state.dart';

@injectable
class HomeBloc extends Bloc<HomeEvent, HomeState> {
  final GetTransactionUseCase _getTransactionUseCase;
  final AddTransactionUseCase _addTransactionUseCase;

  HomeBloc(
    this._getTransactionUseCase,
    this._addTransactionUseCase,
  ) : super(const HomeState()) {
    on<_InitEvent>(_onInitEvent);
    on<_GetTransactionEvent>(_onGetTransactionEvent);
    on<_AddTransactionEvent>(_onAddTransactionEvent);
  }

  Future<void> _onInitEvent(
    _InitEvent event,
    Emitter<HomeState> emit,
  ) async {
    emit(event.state);
  }

  Future<void> _onGetTransactionEvent(
    _GetTransactionEvent event,
    Emitter<HomeState> emit,
  ) async {
    // Set loading state (preserve existing data)
    emit(
      state.copyWith(
        getTransactionState: GetTransactionState.loading(
          transactionList: state.getTransactionState.transactionList,
        ),
        alertState: AlertState.idle(),
      ),
    );

    // Call use case
    final input = GetTransactionInput();
    final output = await _getTransactionUseCase.call(input);

    // Handle error
    if (!output.isSuccess) {
      emit(
        state.copyWith(
          getTransactionState: GetTransactionState.idle(
            transactionList: state.getTransactionState.transactionList,
          ),
          alertState: AlertState.getTransactionError(),
        ),
      );
      return;
    }

    // Handle success
    emit(
      state.copyWith(
        getTransactionState: GetTransactionState.idle(
          transactionList: output.transactionList,
        ),
      ),
    );
  }
}
```

### Bloc Rules

1. Use `@injectable` annotation
2. Use `part` to link event, state, and freezed files
3. Inject use cases via constructor
4. Register event handlers in constructor using `on<EventType>(handler)`
5. Event handlers must be `Future<void>` with `(event, emit)` parameters
6. Use `emit(state.copyWith(...))` to update state
7. Always handle loading, error, and success states

## Event File (`{feature}_event.dart`)

```dart
part of 'home_bloc.dart';

@freezed
abstract class HomeEvent with _$HomeEvent {
  const factory HomeEvent.init({
    @Default(HomeState()) HomeState state,
  }) = _InitEvent;

  const factory HomeEvent.getTransaction({
    required DateTime start,
    required DateTime end,
  }) = _GetTransactionEvent;

  const factory HomeEvent.addTransaction({
    required String title,
    required double amount,
  }) = _AddTransactionEvent;
}
```

### Event Rules

1. Add `part of '{feature}_bloc.dart';` at the top
2. Use `@freezed` annotation
3. Each event is a factory constructor
4. **Init event is mandatory** with state parameter
5. Use `@Default()` for parameters with default values
6. Use `required` for parameters without default values

## State File (`{feature}_state.dart`)

```dart
part of 'home_bloc.dart';

@freezed
abstract class HomeState with _$HomeState {
  const factory HomeState({
    @Default(GetTransactionState.idle()) GetTransactionState getTransactionState,
    @Default(AddTransactionState.idle()) AddTransactionState addTransactionState,
    @Default(AlertState.idle()) AlertState alertState,
    @Default(-1) int selectedId,
    @Default('') String title,
    @Default(0.0) double amount,
  }) = _HomeState;
}

@freezed
abstract class GetTransactionState with _$GetTransactionState {
  const factory GetTransactionState.loading({
    @Default([]) List<TransactionModel> transactionList,
  }) = GetTransactionLoadingState;

  const factory GetTransactionState.idle({
    @Default([]) List<TransactionModel> transactionList,
  }) = GetTransactionIdleState;

  const factory GetTransactionState.success({
    @Default([]) List<TransactionModel> transactionList,
  }) = GetTransactionSuccessState;

  const factory GetTransactionState.error({
    @Default([]) List<TransactionModel> transactionList,
  }) = GetTransactionErrorState;
}

@freezed
abstract class AddTransactionState with _$AddTransactionState {
  const factory AddTransactionState.loading() = AddTransactionLoadingState;
  const factory AddTransactionState.idle() = AddTransactionIdleState;
  const factory AddTransactionState.success() = AddTransactionSuccessState;
  const factory AddTransactionState.error() = AddTransactionErrorState;
}

@freezed
abstract class AlertState with _$AlertState {
  const factory AlertState.idle() = AlertIdleState;
  const factory AlertState.getTransactionError() = AlertGetTransactionErrorState;
  const factory AlertState.addTransactionSuccess() = AlertAddTransactionSuccessState;
}
```

### State Rules

1. Add `part of '{feature}_bloc.dart';` at the top
2. Use `@freezed` annotation
3. **Use single main state class** - `HomeState`
4. All variables must be **non-nullable with default values**
5. **Each use case call needs its own sub-state class** - `GetTransactionState`, `AddTransactionState`
6. Sub-states should have: `loading`, `idle`, `success`, `error` variants
7. Preserve data in sub-states (e.g., keep `transactionList` during loading)

### Handling Non-Defaultable Fields (DateTime)

For fields that cannot have default const values:

```dart
@freezed
abstract class HomeState with _$HomeState {
  const factory HomeState({
    @Default(TransactionState.idle()) TransactionState transactionState,
    @Default(-1) int selectedId,
    required DateTime transactionDate,  // Cannot use @Default
  }) = _HomeState;

  factory HomeState.empty() => HomeState(
    transactionDate: DateTime.now(),
  );
}
```

## Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Bloc class | `{Feature}Bloc` | `HomeBloc` |
| Event class | `{Feature}Event` | `HomeEvent` |
| State class | `{Feature}State` | `HomeState` |
| Sub-state class | `{Action}State` | `GetTransactionState` |
| File names | snake_case | `home_bloc.dart` |

## Best Practices

1. **Always use freezed** for event and state classes
2. **Use default values** in the state
3. **Separate sub-states** for async processes
4. **Use dependency injection** for use cases in the Bloc
5. **Async event handlers** must handle loading, error, and success states
6. **Use copyWith** to update the state
7. **Always use full package import paths**
8. **Preserve existing data** during loading states
9. **Reset alert state** before making new API calls
