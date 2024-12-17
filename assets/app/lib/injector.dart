import 'package:app/app_router.dart';
import 'package:app/injector.config.dart';
import 'package:feature_dashboard/feature_dashboard.dart';
import 'package:feature_login/feature_login.dart';
import 'package:get_it/get_it.dart';
import 'package:injectable/injectable.dart';

final getIt = GetIt.instance;

@InjectableInit()
Future<void> configureDependencies({required String environment}) async {
  getIt.registerSingleton<AppRouter>(AppRouter());

  // TODO: Add injection here
  FeatureDashboardConfig.getInstance().config(env: environment);
  FeatureLoginConfig.getInstance().config(env: environment);

  getIt.init(environment: environment);
}
