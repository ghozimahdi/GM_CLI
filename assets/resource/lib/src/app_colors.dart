import 'package:resource/src/app_material_color.dart';
import 'package:resource/src/models/add_on_shades.dart';
import 'package:resource/src/models/colors_token.dart';
import 'package:resource/src/models/outline_shades.dart';
import 'package:resource/src/models/surface_shades.dart';

/// Global instance of AppColors for easy access throughout the application.
final colors = AppColors();

/// Manages the color scheme of the application, providing easy access to app-specific colors.
///
/// Author: Ghozi Mahdi
/// Website: www.mbahgojol.com
/// GitHub: github.com/mbahgojol
/// YouTube: https://www.youtube.com/@mbahgojol
class AppColors {
  ColorsToken colorsToken = ColorsToken.defaults();

  AppMaterialColor get primary => colorsToken.primary;

  AppMaterialColor get neural => colorsToken.neural;

  AppMaterialColor get semanticInfo => colorsToken.semanticInfo;

  AppMaterialColor get semanticPurple => colorsToken.semanticPurple;

  AppMaterialColor get semanticWarning => colorsToken.semanticWarning;

  AppMaterialColor get semanticSuccess => colorsToken.semanticSuccess;

  AppMaterialColor get semanticError => colorsToken.semanticError;

  AddOnShades get addOn => colorsToken.addOn;

  OutlineShades get outline => colorsToken.outline;

  SurfaceShades get surface => colorsToken.surface;
}
