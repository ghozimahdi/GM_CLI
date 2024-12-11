import 'package:flutter/material.dart';
import 'package:resource/dtos/colors_token_dto.dart';
import 'package:resource/resource.dart';

typedef ShadesDTO = Map<String, ColorShadeValueDTO>;

/// Extension on ShadesDTO to facilitate color manipulation and conversion to AppMaterialColor.
///
/// Author: Ghozi Mahdi
/// Website: www.mbahgojol.com
/// GitHub: github.com/mbahgojol
/// YouTube: https://www.youtube.com/@mbahgojol
extension ShadesDTOExt on ShadesDTO {
  // Accessor methods for each shade, providing a default color if the shade is not found.
  Color get shade100 => this['100']?.color ?? Colors.white;

  Color get shade200 => this['200']?.color ?? Colors.white;

  Color get shade300 => this['300']?.color ?? Colors.white;

  Color get shade400 => this['400']?.color ?? Colors.white;

  Color get shade500 => this['500']?.color ?? Colors.white;

  Color get shade600 => this['600']?.color ?? Colors.white;

  Color get shade700 => this['700']?.color ?? Colors.white;

  Color get shade800 => this['800']?.color ?? Colors.white;

  Color get shade900 => this['900']?.color ?? Colors.white;

  Color get shade1000 => this['1000']?.color ?? Colors.white;

  /// Converts the ShadesDTO to an AppMaterialColor, utilizing shade500 as the primary color.
  AppMaterialColor toMaterialColor() => AppMaterialColor(
        shade500.value,
        <int, Color>{
          100: shade100,
          200: shade200,
          300: shade300,
          400: shade400,
          500: shade500,
          600: shade600,
          700: shade700,
          800: shade800,
          900: shade900,
          1000: shade1000,
        },
      );
}

/// Extension on ColorShadeValueDTO to provide a method for converting hexadecimal color values to Color objects.
///
/// Author: Ghozi Mahdi
/// Website: www.mbahgojol.com
/// GitHub: github.com/mbahgojol
/// YouTube: https://www.youtube.com/@mbahgojol
extension ColorShadeValueExt on ColorShadeValueDTO {
  /// Converts a hexadecimal color value to a Color object.
  Color get color => _getColorFromHex(value);

  /// Helper method to convert a hex color string to a Color object.
  Color _getColorFromHex(String? hexColor) {
    try {
      if (hexColor == null) return Colors.white;
      hexColor = hexColor.toUpperCase().replaceAll("#", "");
      if (hexColor.length == 6) {
        hexColor = "FF$hexColor";
      }
      return Color(int.parse(hexColor, radix: 16));
    } catch (e) {
      rethrow;
    }
  }
}
