import 'package:core/src/env/app_env_fields.dart';
import 'package:core/src/env/dev_env.dart';
import 'package:core/src/env/flavor.dart';
import 'package:core/src/env/prod_env.dart';
import 'package:core/src/env/stg_env.dart';

mixin AppEnv {
  static AppEnvFields? _fields;
  static Flavor? _currentFlavor;

  static Future<AppEnvFields> initialize(Flavor flavor) async {
    _setEnvironment(flavor);
    _currentFlavor ??= flavor;
    return _fields!;
  }

  /// Sets the application environment based on the specified [flavor].
  /// If the environment is for QA or the fields are not yet initialized, it updates the environment settings.
  static void _setEnvironment(Flavor flavor) {
    if (_fields == null) {
      switch (flavor) {
        case Flavor.dev:
          _fields = DevEnv();
          return;
        case Flavor.staging:
          _fields = StgEnv();
          return;
        case Flavor.prod:
          _fields = ProdEnv();
          return;
      }
    }
  }

  static String get flavorName => _currentFlavor?.name ?? '';

  static Flavor get currentFlavor => _currentFlavor ?? Flavor.dev;

  static String get title {
    switch (_currentFlavor) {
      case Flavor.dev:
        return 'App Dev';
      case Flavor.staging:
        return 'App Staging';
      case Flavor.prod:
        return 'App';
      default:
        return '';
    }
  }

  static AppEnvFields get env {
    if (_fields == null) {
      throw Exception(
        "AppEnv is not initialized. Call 'initialize' before accessing 'env'.",
      );
    }
    return _fields!;
  }
}
