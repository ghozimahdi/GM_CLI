import 'dart:async';
import 'dart:ui';

import 'package:app/firebase/firebase_helper.dart';
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

  await FirebaseHelper.initialize();

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
          child: const MyApp(),
        );
      }, (error, stack) async {
        await FirebaseCrashlytics.instance.recordError(error, stack);
      });
      return;
    default:
      customRunApp(
        path: AppLocale.path,
        child: const MyApp(),
      );
      return;
  }
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return AnnotatedRegion<SystemUiOverlayStyle>(
      value: const SystemUiOverlayStyle(
        statusBarColor: Colors.transparent,
        statusBarIconBrightness: Brightness.light,
      ),
      child: ScreenUtilInit(
        designSize: const Size(375, 812),
        builder: (context, child) {
          return MaterialApp(
            title: AppEnv.title,
            localizationsDelegates: context.localizationDelegates,
            supportedLocales: context.supportedLocales,
            locale: context.locale,
            theme: res.themeData,
            navigatorObservers: [
              ChuckerFlutter.navigatorObserver,
            ],
            home: const MyHomePage(title: ''),
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

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;

  void _incrementCounter() {
    setState(() {
      // This call to setState tells the Flutter framework that something has
      // changed in this State, which causes it to rerun the build method below
      // so that the display can reflect the updated values. If we changed
      // _counter without calling setState(), then the build method would not be
      // called again, and so nothing would appear to happen.
      _counter++;
    });
  }

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Scaffold(
      appBar: AppBar(
        // TRY THIS: Try changing the color here to a specific color (to
        // Colors.amber, perhaps?) and trigger a hot reload to see the AppBar
        // change color while the other colors stay the same.
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: Text(widget.title),
      ),
      body: Center(
        // Center is a layout widget. It takes a single child and positions it
        // in the middle of the parent.
        child: Column(
          // Column is also a layout widget. It takes a list of children and
          // arranges them vertically. By default, it sizes itself to fit its
          // children horizontally, and tries to be as tall as its parent.
          //
          // Column has various properties to control how it sizes itself and
          // how it positions its children. Here we use mainAxisAlignment to
          // center the children vertically; the main axis here is the vertical
          // axis because Columns are vertical (the cross axis would be
          // horizontal).
          //
          // TRY THIS: Invoke "debug painting" (choose the "Toggle Debug Paint"
          // action in the IDE, or press "p" in the console), to see the
          // wireframe for each widget.
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ), // This trailing comma makes auto-formatting nicer for build methods.
    );
  }
}
