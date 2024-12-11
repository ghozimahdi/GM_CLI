import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:resource/models/add_on_shades.dart';
import 'package:resource/models/outline_shades.dart';
import 'package:resource/models/surface_shades.dart';
import 'package:resource/resource.dart';

// part directive needed for freezed code generation.
part 'colors_token.freezed.dart';

/// ColorsToken class defines a set of color schemes used across the app.
/// It encapsulates different color shades for various UI elements, ensuring a consistent look and feel.
///
/// Author: Ghozi Mahdi
/// Website: www.mbahgojol.com
/// GitHub: github.com/mbahgojol
/// YouTube: https://www.youtube.com/@mbahgojol
@freezed
class ColorsToken with _$ColorsToken {
  /// Factory constructor for ColorsToken, requiring all color properties to be initialized.
  const factory ColorsToken({
    required AppMaterialColor primary,
    required AppMaterialColor neural,
    required OutlineShades outline,
    required SurfaceShades surface,
    required AppMaterialColor semanticInfo,
    required AppMaterialColor semanticPurple,
    required AppMaterialColor semanticError,
    required AppMaterialColor semanticWarning,
    required AppMaterialColor semanticSuccess,
    required AddOnShades addOn,
  }) = _ColorsToken;

  factory ColorsToken.defaults() => ColorsToken(
        primary: AppMaterialColor.empty(),
        neural: AppMaterialColor.empty(),
        outline: OutlineShades.defaults(),
        surface: SurfaceShades.defaults(),
        semanticInfo: AppMaterialColor.empty(),
        semanticPurple: AppMaterialColor.empty(),
        semanticError: AppMaterialColor.empty(),
        semanticWarning: AppMaterialColor.empty(),
        semanticSuccess: AppMaterialColor.empty(),
        addOn: AddOnShades.defaults(),
      );
}
