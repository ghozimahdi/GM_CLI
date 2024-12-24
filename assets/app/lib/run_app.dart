import 'package:easy_localization/easy_localization.dart';
import 'package:feature_common/feature_common.dart';
import 'package:flutter/material.dart';

Future<void> customRunApp({
  required Widget child,
  String? path,
  AssetLoader? assetLoader,
}) async {
  runApp(
    EasyLocalization(
      supportedLocales: AppLocale.values.map((e) => e.toLocale).toList(),
      startLocale: AppLocale.indonesian.toLocale,
      fallbackLocale: AppLocale.indonesian.toLocale,
      path: path ?? AppLocale.path,
      assetLoader: assetLoader ?? const RootBundleAssetLoader(),
      child: child,
    ),
  );
}
