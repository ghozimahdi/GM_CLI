import 'package:auto_route/auto_route.dart';
import 'package:feature_auth/src/config/feature_auth_route.gr.dart';

@AutoRouterConfig()
class FeatureAuthRoute extends RootStackRouter {
  @override
  List<AutoRoute> get routes => [
        AutoRoute(page: AuthRoute.page),
      ];
}
