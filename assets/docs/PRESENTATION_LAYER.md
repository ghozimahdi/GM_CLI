# Presentation Layer Guidelines

This document describes the rules and conventions for the Presentation layer.

## Directory Structure

```
feature_{xxx}/
├── lib/
│   ├── feature_{xxx}.dart          # Main export file
│   ├── src/
│   │   ├── bloc/
│   │   │   ├── {feature}_bloc.dart
│   │   │   ├── {feature}_event.dart
│   │   │   └── {feature}_state.dart
│   │   ├── config/
│   │   │   ├── feature_{xxx}_config.dart
│   │   │   └── feature_{xxx}_route.dart
│   │   ├── di/
│   │   │   └── di.dart
│   │   ├── page/
│   │   │   └── {page_name}_page.dart
│   │   └── widget/
│   │       └── {widget_name}.dart
```

## Page Rules

### Route Provider Pattern

1. Check if `RouteProvider` is available in `feature_common/lib/src/route/{page_name}_route_provider.dart`
2. If not available, create the abstract class in feature_common
3. Implement the route provider in the page file

**Abstract Route Provider (in feature_common):**
```dart
// feature_common/lib/src/route/login_route_provider.dart
import 'package:auto_route/auto_route.dart';

abstract class LoginRouteProvider {
  PageRouteInfo route();
}
```

**Implementation (in feature page):**
```dart
import 'package:auto_route/auto_route.dart';
import 'package:feature_auth/src/config/feature_auth_route.gr.dart';
import 'package:feature_common/feature_common.dart';
import 'package:injectable/injectable.dart';

@Injectable(as: LoginRouteProvider)
class LoginRouteProviderImpl implements LoginRouteProvider {
  @override
  PageRouteInfo route() {
    return LoginRoute();
  }
}

@RoutePage()
class LoginPage extends StatelessWidget {
  // ...
}
```

### StatelessWidget Page (No Controllers)

Use when the page doesn't need controllers that require disposal.

```dart
@Injectable(as: LoginRouteProvider)
class LoginRouteProviderImpl implements LoginRouteProvider {
  @override
  PageRouteInfo route() {
    return LoginRoute();
  }
}

@RoutePage()
class LoginPage extends StatelessWidget {
  LoginPage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => getIt<LoginBloc>()
        ..add(const LoginEvent.init()),
      child: Builder(
        builder: (context) {
          return Scaffold(
            body: SafeArea(
              child: ...,
            ),
          );
        },
      ),
    );
  }
}
```

### StatefulWidget Page (With Controllers)

Use when the page needs controllers like `TabController`, `TextEditingController`, `ScrollController`.

```dart
@Injectable(as: LoginRouteProvider)
class LoginRouteProviderImpl implements LoginRouteProvider {
  @override
  PageRouteInfo route() {
    return LoginRoute();
  }
}

@RoutePage()
class LoginPage extends StatefulWidget {
  LoginPage({super.key});

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  final _bloc = getIt<LoginBloc>();
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => _bloc
        ..add(const LoginEvent.init()),
      child: Scaffold(
        body: SafeArea(
          child: ...,
        ),
      ),
    );
  }
}
```

### Important Page Rules

1. **Do NOT create separate content classes** - Keep all content in the same page class
2. **Do NOT store state variables in Page** - Store them in BLoC state instead
3. **Extract deep nested layouts into functions** - Use `_buildXxx(BuildContext context)` methods
4. **Always import the generated route file** - `import 'package:feature_xxx/src/config/feature_xxx_route.gr.dart';`

## BlocSelector vs BlocBuilder

### Use BlocSelector for Single Field

```dart
BlocSelector<HomeBloc, HomeState, String>(
  selector: (state) => state.title,
  builder: (context, title) => Text(
    title,
    style: textTheme.bodyLarge,
  ),
),
```

### Use BlocBuilder for Multiple Fields

```dart
BlocBuilder<HomeBloc, HomeState>(
  buildWhen: (previous, current) {
    return previous.title != current.title ||
      previous.subtitle != current.subtitle;
  },
  builder: (context, state) => Text(
    '${state.title} - ${state.subtitle}',
    style: textTheme.bodyLarge,
  ),
),
```

## Widget Extraction

### List Items in Separate Widget Files

Create separate widget files for list items in the `widget/` folder:

**Page file:**
```dart
BlocSelector<HomeBloc, HomeState, List<ExpenseModel>>(
  selector: (state) => state.transactionState.transactionList,
  builder: (context, transactionList) {
    return ListView.builder(
      itemCount: transactionList.length,
      itemBuilder: (context, index) {
        return SummaryItem(item: transactionList[index]);
      },
    );
  },
),
```

**Widget file (`widget/summary_item.dart`):**
```dart
import 'package:domain_auth/domain_auth.dart';

class SummaryItem extends StatelessWidget {
  final BoosterPurchaseModel item;

  const SummaryItem({required this.item});

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Icon(item.icon),
        Column(
          children: [
            Text(item.label),
            Text(item.value),
          ],
        ),
      ],
    );
  }
}
```

## Configuration

### Feature Config (`feature_{xxx}_config.dart`)

```dart
import 'package:feature_auth/src/di/di.dart';
import 'package:library_common/library_common.dart';

class FeatureAuthConfig extends AppConfig {
  FeatureAuthConfig._();

  factory FeatureAuthConfig.getInstance() => _instance;

  static final FeatureAuthConfig _instance = FeatureAuthConfig._();

  @override
  Future<bool> config({required String env}) async {
    await configureInjection(env: env);
    return true;
  }
}
```

### Route Config (`feature_{xxx}_route.dart`)

```dart
import 'package:auto_route/auto_route.dart';

@AutoRouterConfig()
abstract class FeatureAuthRoute {}
```

### DI Setup (`di.dart`)

```dart
import 'package:feature_auth/src/di/di.config.dart';
import 'package:get_it/get_it.dart';
import 'package:injectable/injectable.dart';

final getIt = GetIt.instance;

@injectableInit
Future<void> configureInjection({required String env}) async =>
    getIt.init(environment: env);
```

## Export Rules

Every new file MUST be exported in `feature_{xxx}.dart`:

```dart
// feature_booster.dart
export 'src/config/feature_booster_config.dart';
export 'src/config/feature_booster_route.dart';
export 'src/config/feature_booster_route.gr.dart';
```

Export order:
1. Config exports
2. Route exports
3. Route generated exports

## Import Rules

- All imports MUST use full package paths
- Never use relative imports

```dart
// Correct
import 'package:domain_agent/domain_agent.dart';
import 'package:feature_auth/src/config/feature_auth_route.gr.dart';

// Incorrect
import '../model/...';
import './widget/...';
```
