import 'package:flutter/material.dart';

// TODO Please remove this colors, noted by ghozi
const kOldSilver = Color(0xFF828282);
const kTopaz = Color(0xFFfbd283);
const kLightPeriwinkle = Color(0xFFBFCEE0);
const kGhostWhite = Color(0xFFF6F8FC);
const kSlateGray = Color(0xFF6C7E95);
const kEgyptianBlue = Color(0xFF19359D);
const kBlueberry = Color(0xFF5276FF);
const kBrightGray = Color(0xFFE7ECF4);

extension ColorSchemeExt on ColorScheme {
  Color get appDivider => secondary.withOpacity(0.3);

  Color get primaryText => secondary;
  Color get secondaryText => tertiary;
  Color get keyColors => primary;
}

extension ColorThemeExt on BuildContext {
  ColorScheme get colorScheme => Theme.of(this).colorScheme;
}
