import 'package:component/component.dart';
import 'package:component/design_system/app_colors.dart';
import 'package:component/design_system/dtos/colors_token_dto.dart';
import 'package:component/generated/assets.gen.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

final dsystem = AppDesignSystem();

/// Represents the application's overall design system, including color schemes, text styles, and more.
///
/// Author: Ghozi Mahdi
/// Website: www.mbahgojol.com
/// GitHub: github.com/mbahgojol
/// YouTube: https://www.youtube.com/@mbahgojol
class AppDesignSystem {
  /// Initializes the design system by loading color schemes from JSON files.
  /// /// This includes setting up both light and dark color schemes based on the color token JSON.
  Future<void> initialize({required bool useDarkTheme}) async {
    String lightColorTokenJson =
        await rootBundle.loadString(AppAssets.themes.lightColorToken);
    // TODO: color token for dark is missing, noted by ghozi

    final lightColorToken =
        ColorsTokenDTO.fromJsonString(lightColorTokenJson).toColorsToken();
    final darkColorToken =
        ColorsTokenDTO.fromJsonString(lightColorTokenJson).toColorsToken();

    if (useDarkTheme) {
      colors.initialize(darkColorToken);
    } else {
      colors.initialize(lightColorToken);
    }
  }

  /// Provides the light color scheme, or a default light scheme if not initialized.
  /// The color scheme is constructed using the primary, secondary, background, onBackground,
  /// surface, onSurface, tertiary, onTertiary, and outline color properties.
  ColorScheme get colorScheme {
    return ColorScheme.light(
      primary: colors.primary.shade500,
      // primary-text
      secondary: colors.neural.shade500,
      surface: colors.surface.white,
      onSurface: colors.surface.black,
      onSurfaceVariant: colors.surface.background,
      // secondary-text
      tertiary: colors.neural.shade400,
      outline: colors.outline.main,
    );
  }

  /// Defines the base text theme for the app.
  /// It includes styles for headlineSmall, titleLarge, titleMedium, titleSmall,
  /// bodyLarge, bodySmall, and labelSmall.
  TextTheme get textTheme {
    return TextTheme(
      headlineLarge: TextStyle(
        fontWeight: FontWeight.w400,
        fontSize: 36.sp,
        // Calculate from spacing (figma) / fontSize
        letterSpacing: -(36.sp * 0.022),
        // Calculate from height (figma) / fontSize
        height: (44.sp / 36.sp),
      ),
      headlineMedium: TextStyle(
        fontWeight: FontWeight.w400,
        fontSize: 32.sp,
        letterSpacing: -(32.sp * 0.03),
        height: (40.sp / 32.sp),
      ),
      headlineSmall: TextStyle(
        fontWeight: FontWeight.w400,
        fontSize: 24.sp,
        letterSpacing: -(24.sp * 0.03),
        height: (32.sp / 24.sp),
      ),
      titleLarge: TextStyle(
        fontWeight: FontWeight.w400,
        fontSize: 20.sp,
        letterSpacing: -(20.sp * 0.02),
        height: (28.sp / 20.sp),
      ),
      titleMedium: TextStyle(
        fontWeight: FontWeight.w400,
        fontSize: 18.sp,
        letterSpacing: -(18.sp * 0.02),
        height: (28.sp / 18.sp),
      ),
      titleSmall: TextStyle(
        fontWeight: FontWeight.w400,
        fontSize: 16.sp,
        letterSpacing: -(16.sp * 0.02),
        height: (26.sp / 16.sp),
      ),
      bodyLarge: TextStyle(
        fontWeight: FontWeight.w400,
        fontSize: 14.sp,
        letterSpacing: -(14.sp * 0.02),
        height: (22.sp / 14.sp),
      ),
      bodySmall: TextStyle(
        fontWeight: FontWeight.w400,
        fontSize: 12.sp,
        letterSpacing: 0,
        height: (16.sp / 12.sp),
      ),
      labelSmall: TextStyle(
        fontWeight: FontWeight.w400,
        fontSize: 11.sp,
        letterSpacing: 0,
        height: (14.sp / 10.sp),
      ),
    ).apply(
      fontFamily: 'Inter',
      bodyColor: colorScheme.secondary,
      displayColor: colorScheme.onSurface,
    );
  }
}
