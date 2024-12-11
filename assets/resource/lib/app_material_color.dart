import 'package:flutter/material.dart';

/// A custom implementation of [ColorSwatch] to define material colors with various shades.
/// This class facilitates accessing different shades of a color, typically used in material design.
///
/// Author: Ghozi Mahdi
/// Website: www.mbahgojol.com
/// GitHub: github.com/mbahgojol
/// YouTube: https://www.youtube.com/@mbahgojol
class AppMaterialColor extends ColorSwatch<int> {
  /// Constructs an instance of [AppMaterialColor].
  ///
  /// [primary] is the default color shade (usually shade500).
  /// [swatch] is a map of color shades, where the key is the shade's identifier (e.g., 100, 200, 300, etc.),
  /// and the value is the color for that shade.
  const AppMaterialColor(super.primary, super.swatch);
  Color get shade100 => this[100]!;
  Color get shade200 => this[200]!;
  Color get shade300 => this[300]!;
  Color get shade400 => this[400]!;
  Color get shade500 => this[500]!;
  Color get shade600 => this[600]!;
  Color get shade700 => this[700]!;
  Color get shade800 => this[800]!;
  Color get shade900 => this[900]!;
  Color get shade1000 => this[1000]!;

  /// Factory method to create an instance of [AppMaterialColor] with no defined shades.
  /// This can be useful as a default or placeholder value.
  factory AppMaterialColor.empty() => const AppMaterialColor(0, <int, Color>{
        100: Colors.white,
        200: Colors.white,
        300: Colors.white,
        400: Colors.white,
        500: Colors.white,
        600: Colors.white,
        700: Colors.white,
        800: Colors.white,
        900: Colors.white,
        1000: Colors.white,
      });
}

