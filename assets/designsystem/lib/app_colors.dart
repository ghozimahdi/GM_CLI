import 'package:design_system/app_material_color.dart';
import 'package:design_system/models/add_on_shades.dart';
import 'package:design_system/models/colors_token.dart';
import 'package:design_system/models/outline_shades.dart';
import 'package:design_system/models/surface_shades.dart';

/// Global instance of AppColors for easy access throughout the application.
final colors = AppColors();

/// Manages the color scheme of the application, providing easy access to app-specific colors.
///
/// Author: Ghozi Mahdi
/// Website: www.mbahgojol.com
/// GitHub: github.com/mbahgojol
/// YouTube: https://www.youtube.com/@mbahgojol
class AppColors {
  // Holds the color tokens loaded from a design token source, like a JSON file from Figma.
  ColorsToken _colorsToken = ColorsToken.defaults();

  /// Initializes the color manager with a set of color tokens.
  ///
  /// This method sets up the color scheme based on the provided [colorsToken],
  /// which contains definitions for various colors used throughout the app.
  ///
  /// [colorsToken] - The color definitions conforming to the `ColorsToken` structure.
  void initialize(ColorsToken colorsToken) {
    _colorsToken = colorsToken;
  }

  AppMaterialColor get primary => _colorsToken.primary;
  AppMaterialColor get neural => _colorsToken.neural;
  AppMaterialColor get semanticInfo => _colorsToken.semanticInfo;
  AppMaterialColor get semanticPurple => _colorsToken.semanticPurple;
  AppMaterialColor get semanticWarning => _colorsToken.semanticWarning;
  AppMaterialColor get semanticSuccess => _colorsToken.semanticSuccess;
  AppMaterialColor get semanticError => _colorsToken.semanticError;
  AddOnShades get addOn => _colorsToken.addOn;
  OutlineShades get outline => _colorsToken.outline;
  SurfaceShades get surface => _colorsToken.surface;
}

