import 'package:auto_route/auto_route.dart';
import 'package:feature_login/src/config/feature_login_route.gr.dart';

@AutoRouterConfig()
class FeatureLoginRoute extends RootStackRouter {
  @override
  List<AutoRoute> get routes => [
        AutoRoute(page: LoginRoute.page),
      ];
}
