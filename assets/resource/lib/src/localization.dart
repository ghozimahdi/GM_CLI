import 'package:easy_localization/easy_localization.dart';
import 'package:resource/generated/codegen_loader.g.dart';

// ignore: camel_case_types
typedef cw = LocaleKeys;

extension LocaleKeysExt on String {
  String get t => tr(this);
}
