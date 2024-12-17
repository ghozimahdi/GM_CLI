import 'package:auto_route/auto_route.dart';
import 'package:feature_dashboard/feature_dashboard.dart';
import 'package:feature_login/feature_login.dart';

@AutoRouterConfig()
class AppRouter extends RootStackRouter {
  @override
  List<AutoRoute> get routes => [
        ...FeatureDashboardRoute().routes,
        ...FeatureLoginRoute().routes,
      ];
}
