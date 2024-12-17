import 'dart:async';
import 'dart:ui';

import 'package:app/app_router.dart';
import 'package:app/injector.dart';
import 'package:app/run_app.dart';
import 'package:core/core.dart';
import 'package:data_common/data_common.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:feature_common/feature_common.dart';
import 'package:firebase_crashlytics/firebase_crashlytics.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:resource/resource.dart';

Future<void> mainCommon(Flavor flavor) async {
  WidgetsFlutterBinding.ensureInitialized();

  await configureDependencies(environment: flavor.environment);

  var appEnv = await AppEnv.initialize(flavor);

  // Uncomment the following code to enable Firebase functionality.
  // For detailed documentation, refer to the .firebase folder in the packages/app directory.
  // await FirebaseHelper.initialize();

  await EasyLocalization.ensureInitialized();

  await res.initialize(useDarkTheme: false);

  await SystemChrome.setPreferredOrientations([
    DeviceOrientation.portraitUp,
    DeviceOrientation.portraitDown,
  ]);

  switch (flavor) {
    case Flavor.prod:
    case Flavor.staging:
      await FirebaseCrashlytics.instance.setCrashlyticsCollectionEnabled(true);

      PlatformDispatcher.instance.onError = (exception, stackTrace) {
        FirebaseCrashlytics.instance.recordError(
          exception,
          stackTrace,
          fatal: true,
        );
        return true;
      };

      FlutterError.onError = (details) {
        FirebaseCrashlytics.instance.recordFlutterFatalError(details);
      };

      runZonedGuarded(() async {
        customRunApp(
          path: 'IgnoreThisPath',
          child: MyApp(),
        );
      }, (error, stack) async {
        await FirebaseCrashlytics.instance.recordError(error, stack);
      });
      return;
    default:
      customRunApp(
        path: AppLocale.path,
        child: MyApp(),
      );
      return;
  }
}

class MyApp extends StatelessWidget {
  MyApp({super.key});

  final _appRouter = getIt<AppRouter>();

  @override
  Widget build(BuildContext context) {
    return AnnotatedRegion<SystemUiOverlayStyle>(
      value: const SystemUiOverlayStyle(
        statusBarColor: Colors.transparent,
        statusBarIconBrightness: Brightness.light,
      ),
      child: ScreenUtilInit(
        builder: (context, child) {
          return MaterialApp.router(
            routerConfig: _appRouter.config(
              navigatorObservers: () => [
                ChuckerFlutter.navigatorObserver,
              ],
            ),
            title: AppEnv.title,
            localizationsDelegates: context.localizationDelegates,
            supportedLocales: context.supportedLocales,
            locale: context.locale,
            theme: res.themeData,
            builder: (context, child) => Overlay(
              initialEntries: [
                if (child != null) ...[
                  OverlayEntry(
                    builder: (context) {
                      return MediaQuery.withClampedTextScaling(
                        minScaleFactor: 1,
                        maxScaleFactor: 1,
                        child: FlavorBanner(
                          child: GestureDetector(
                            child: child,
                            onTap: () {
                              FocusScope.of(context).requestFocus(FocusNode());
                            },
                          ),
                        ),
                      );
                    },
                  ),
                ],
              ],
            ),
          );
        },
      ),
    );
  }
}
