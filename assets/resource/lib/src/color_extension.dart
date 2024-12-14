import 'package:flutter/material.dart';

extension ColorSchemeExt on ColorScheme {
  Color get appDivider => secondary.withOpacity(0.3);

  Color get primaryText => secondary;

  Color get secondaryText => tertiary;
}

extension ColorThemeExt on BuildContext {
  ColorScheme get colorScheme => Theme.of(this).colorScheme;
}
