import 'package:auto_route/auto_route.dart';
import 'package:feature_common/feature_common.dart';
import 'package:feature_login/src/config/feature_login_route.gr.dart';
import 'package:flutter/material.dart';
import 'package:injectable/injectable.dart';
import 'package:resource/resource.dart';

@Injectable(as: LoginRouteProvider)
class LoginRouteProviderImpl implements LoginRouteProvider {
  @override
  PageRouteInfo route({
    required String name,
  }) {
    return LoginRoute(
      name: name,
    );
  }
}

@RoutePage()
class LoginPage extends StatelessWidget {
  final String name;

  const LoginPage({
    super.key,
    required this.name,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Text(
          'Hello There, Are You $name?',
          style: res.textTheme.titleLarge,
          textAlign: TextAlign.center,
        ),
      ),
    );
  }
}
