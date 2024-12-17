import 'package:core/core.dart';
import 'package:feature_login/src/di/di.dart';

class FeatureLoginConfig extends AppConfig {
  FeatureLoginConfig._();

  factory FeatureLoginConfig.getInstance() {
    return _instance;
  }

  static final FeatureLoginConfig _instance = FeatureLoginConfig._();

  @override
  Future<bool> config({
    required String env,
  }) async {
    await configureInjection(env: env);
    return true;
  }
}
