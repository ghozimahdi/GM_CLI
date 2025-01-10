import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import 'package:resource/resource.dart';

extension AppThemesExt on AppDesignSystem {
  ThemeData get themeData {
    return ThemeData.light(useMaterial3: true).copyWith(
      colorScheme: colorScheme,
      scaffoldBackgroundColor: colorScheme.onSurfaceVariant,
      dividerColor: Colors.transparent,
      listTileTheme: const ListTileThemeData(
        contentPadding: EdgeInsets.all(0),
      ),
      iconTheme: IconThemeData(
        color: colors.neural.shade500,
      ),
      textTheme: textTheme,
      dividerTheme: DividerThemeData(
        thickness: 1,
        color: colorScheme.outline,
        space: 1,
      ),
      checkboxTheme: CheckboxThemeData(
        side: BorderSide(
          color: colors.neural.shade300,
        ),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(5.r),
        ),
      ),
      appBarTheme: const AppBarTheme(
        centerTitle: false,
        scrolledUnderElevation: 0,
      ),
      badgeTheme: BadgeThemeData(
        backgroundColor: colorScheme.tertiary,
        textColor: colorScheme.surface,
      ),
      bottomNavigationBarTheme: BottomNavigationBarThemeData(
        backgroundColor: colorScheme.surface,
        elevation: 4.dm,
        showSelectedLabels: true,
        showUnselectedLabels: true,
        selectedItemColor: colorScheme.primary,
        unselectedItemColor: colorScheme.secondaryText,
        type: BottomNavigationBarType.fixed,
        selectedLabelStyle: textTheme.bodySmallMedium?.copyWith(
          color: colorScheme.primary,
        ),
        unselectedLabelStyle: textTheme.captionRegular
            ?.copyWith(color: colorScheme.secondaryText),
      ),
      tabBarTheme: TabBarTheme(
        splashFactory: NoSplash.splashFactory,
        indicatorSize: TabBarIndicatorSize.label,
        unselectedLabelColor: colorScheme.secondaryText,
        unselectedLabelStyle: textTheme.bodySmallRegular,
        labelColor: colorScheme.primary,
        labelStyle: textTheme.bodySmallMedium,
        dividerColor: Colors.transparent,
      ),
      filledButtonTheme: FilledButtonThemeData(
        style: FilledButton.styleFrom(
          disabledBackgroundColor: colors.neural.shade100,
          disabledForegroundColor: colorScheme.tertiary,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8.r),
          ),
          padding: EdgeInsets.symmetric(
            vertical: 12.h,
            horizontal: 16.w,
          ),
          textStyle: textTheme.bodyLargeMedium!.copyWith(height: 0),
        ),
      ),
      inputDecorationTheme: InputDecorationTheme(
        isDense: true,
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(8.r),
          borderSide: BorderSide(color: colors.outline.main),
        ),
        prefixStyle: textTheme.bodyLargeRegular?.copyWith(
          color: colorScheme.secondary,
        ),
        suffixIconColor: colorScheme.secondaryText,
        labelStyle: textTheme.bodyLargeRegular?.copyWith(
          color: colorScheme.secondaryText,
        ),
        hintStyle: textTheme.bodyLargeRegular?.copyWith(
          color: colorScheme.secondaryText,
        ),
        disabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(8.r),
          borderSide: BorderSide(
            color: colorScheme.secondary.withOpacity(0.2),
          ),
        ),
      ),
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8.r),
          ),
          padding: EdgeInsets.symmetric(
            vertical: 12.h,
            horizontal: 16.h,
          ),
          textStyle: textTheme.bodyLargeMedium!.copyWith(height: 0),
          backgroundColor: colorScheme.surface,
          foregroundColor: colorScheme.primary,
          disabledBackgroundColor: colors.neural.shade100,
          disabledForegroundColor: colorScheme.tertiary,
        ).copyWith(side: WidgetStateProperty.resolveWith(
          (states) {
            if (states.contains(WidgetState.disabled)) {
              return BorderSide(color: colors.neural.shade100);
            }
            return BorderSide(color: colorScheme.primary);
          },
        )),
      ),
      textButtonTheme: TextButtonThemeData(
        style: TextButton.styleFrom(
          foregroundColor: colorScheme.primary,
          textStyle: textTheme.bodyLargeMedium,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8.r),
          ),
          padding: EdgeInsets.symmetric(
            vertical: 12.h,
            horizontal: 16.h,
          ),
        ),
      ),
      floatingActionButtonTheme: FloatingActionButtonThemeData(
        foregroundColor: colorScheme.surface,
        backgroundColor: colorScheme.primary,
        extendedSizeConstraints: BoxConstraints(
          minHeight: 44.h,
        ),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(24.r),
        ),
      ),
      expansionTileTheme: ExpansionTileThemeData(
        iconColor: colors.neural.shade400,
        collapsedIconColor: colors.neural.shade400,
      ),
    );
  }
}
