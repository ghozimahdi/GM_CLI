import 'package:auto_route/auto_route.dart';
import 'package:feature_auth/feature_auth.dart';
import 'package:feature_dashboard/feature_dashboard.dart';

@AutoRouterConfig()
class AppRouter extends RootStackRouter {
  @override
  List<AutoRoute> get routes => [
        ...FeatureDashboardRoute().routes,
        ...FeatureAuthRoute().routes,
      ];
}
