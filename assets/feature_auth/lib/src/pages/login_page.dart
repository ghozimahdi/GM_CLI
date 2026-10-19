import 'package:auto_route/auto_route.dart';
import 'package:feature_auth/src/config/feature_auth_route.gr.dart';
import 'package:feature_common/feature_common.dart';
import 'package:flutter/material.dart';
import 'package:injectable/injectable.dart';

@Injectable(as: LoginRouteProvider)
class LoginRouteProviderImpl implements LoginRouteProvider {
  @override
  PageRouteInfo route() {
    return const LoginRoute();
  }
}

@RoutePage()
class LoginPage extends StatelessWidget {

  const LoginPage({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Text(
          cw.loginDesc,
          style: textTheme.titleLarge,
          textAlign: TextAlign.center,
        ),
      ),
    );
  }
}
