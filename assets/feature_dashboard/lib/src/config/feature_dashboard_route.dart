import 'package:auto_route/auto_route.dart';
import 'package:feature_dashboard/src/config/feature_dashboard_route.gr.dart';

@AutoRouterConfig()
class FeatureDashboardRoute extends RootStackRouter {
  @override
  List<AutoRoute> get routes => [
        AutoRoute(
          page: HomeRoute.page,
          initial: true,
        ),
      ];
}
