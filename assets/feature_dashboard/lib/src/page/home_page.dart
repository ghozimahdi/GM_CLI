import 'package:auto_route/auto_route.dart';
import 'package:feature_common/feature_common.dart';
import 'package:feature_dashboard/src/config/feature_dashboard_route.gr.dart';
import 'package:feature_dashboard/src/di/di.dart';
import 'package:flutter/material.dart';
import 'package:injectable/injectable.dart';
import 'package:resource/resource.dart';

@Injectable(as: HomeRouteProvider)
class HomeRouteProviderImpl implements HomeRouteProvider {
  @override
  PageRouteInfo route() {
    return const HomeRoute();
  }
}

@RoutePage()
class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: Padding(
          padding: EdgeInsets.all(16.dm),
          child: Column(
            spacing: 16.h,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(
                'Hello Welcome, Ghozi Mahdi CLI is Flutter project tools management',
                style: res.textTheme.titleSmall,
                textAlign: TextAlign.center,
              ),
              FilledButton(
                onPressed: () {
                  context.pushRoute(
                    getIt<LoginRouteProvider>().route(name: 'Ghozi Mahdi'),
                  );
                },
                child: const Text('Login'),
              )
            ],
          ),
        ),
      ),
    );
  }
}
