import 'package:flutter/material.dart';

extension ColorSchemeExt on ColorScheme {
  Color get appDivider => secondary.withAlpha(30);

  Color get primaryText => secondary;

  Color get secondaryText => tertiary;
}

extension ColorThemeExt on BuildContext {
  ColorScheme get colorScheme => Theme.of(this).colorScheme;
}
