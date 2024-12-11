import 'dart:convert';

import 'package:component/design_system/app_material_color.dart';
import 'package:component/design_system/dtos/shades_dto.dart';
import 'package:component/design_system/models/add_on_shades.dart';
import 'package:component/design_system/models/colors_token.dart';
import 'package:component/design_system/models/outline_shades.dart';
import 'package:component/design_system/models/surface_shades.dart';
import 'package:flutter/material.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part 'colors_token_dto.freezed.dart';
part 'colors_token_dto.g.dart';

@freezed
class ColorsTokenDTO with _$ColorsTokenDTO {
  const ColorsTokenDTO._();

  const factory ColorsTokenDTO({
    @JsonKey(name: "primary") ShadesDTO? primary,
    @JsonKey(name: "neural") ShadesDTO? neural,
    @JsonKey(name: "semantic") SemanticDTO? semantic,
    @JsonKey(name: "surface") SurfaceDTO? surface,
    @JsonKey(name: "outline") OutlineDTO? outline,
    @JsonKey(name: "Add On") AddOnDTO? addOn,
  }) = _ColorsTokenDTO;

  factory ColorsTokenDTO.fromJson(Map<String, dynamic> json) =>
      _$ColorsTokenDTOFromJson(json);

  factory ColorsTokenDTO.fromJsonString(String str) =>
      ColorsTokenDTO.fromJson(json.decode(str));

  ColorsToken toColorsToken() => ColorsToken(
        primary: primary?.toMaterialColor() ?? AppMaterialColor.empty(),
        neural: neural?.toMaterialColor() ?? AppMaterialColor.empty(),
        outline: OutlineShades(main: outline?.main?.color ?? Colors.white),
        surface: SurfaceShades(
          white: surface?.white?.color ?? Colors.white,
          black: surface?.black?.color ?? Colors.black,
          background: surface?.background?.color ?? Colors.white,
        ),
        semanticPurple: semantic?.purple?.toMaterialColor() ?? AppMaterialColor.empty(),
        semanticError: semantic?.error?.toMaterialColor() ?? AppMaterialColor.empty(),
        semanticWarning: semantic?.warning?.toMaterialColor() ?? AppMaterialColor.empty(),
        semanticSuccess: semantic?.success?.toMaterialColor() ?? AppMaterialColor.empty(),
        semanticInfo: semantic?.info?.toMaterialColor() ?? AppMaterialColor.empty(),
        addOn: AddOnShades(
          info: addOn?.info?.color ?? Colors.white,
          success: addOn?.success?.color ?? Colors.white,
          warning: addOn?.warning?.color ?? Colors.white,
          notification: addOn?.notification?.color ?? Colors.white,
          error: addOn?.error?.color ?? Colors.white,
          gold: addOn?.gold?.color ?? Colors.white,
          grey: addOn?.grey?.color ?? Colors.white,
        ),
      );
}

@freezed
class ColorShadeValueDTO with _$ColorShadeValueDTO {
  const factory ColorShadeValueDTO({
    @JsonKey(name: "value") String? value,
    @JsonKey(name: "type") String? type,
    @JsonKey(name: "parent") String? parent,
    @JsonKey(name: "description") String? description,
  }) = _ColorShadeValueDTO;

  factory ColorShadeValueDTO.fromJson(Map<String, dynamic> json) =>
      _$ColorShadeValueDTOFromJson(json);
}

@freezed
class OutlineDTO with _$OutlineDTO {
  const factory OutlineDTO({
    @JsonKey(name: "main") ColorShadeValueDTO? main,
  }) = _OutlineDTO;

  factory OutlineDTO.fromJson(Map<String, dynamic> json) =>
      _$OutlineDTOFromJson(json);
}

@freezed
class AddOnDTO with _$AddOnDTO {
  const factory AddOnDTO({
    @JsonKey(name: "Info") ColorShadeValueDTO? info,
    @JsonKey(name: "Success") ColorShadeValueDTO? success,
    @JsonKey(name: "Warning") ColorShadeValueDTO? warning,
    @JsonKey(name: "Notification") ColorShadeValueDTO? notification,
    @JsonKey(name: "Error") ColorShadeValueDTO? error,
    @JsonKey(name: "Gold") ColorShadeValueDTO? gold,
    @JsonKey(name: "Grey") ColorShadeValueDTO? grey,
  }) = _AddOnDTO;

  factory AddOnDTO.fromJson(Map<String, dynamic> json) =>
      _$AddOnDTOFromJson(json);
}

@freezed
class SemanticDTO with _$SemanticDTO {
  const factory SemanticDTO({
    @JsonKey(name: "info") ShadesDTO? info,
    @JsonKey(name: "success") ShadesDTO? success,
    @JsonKey(name: "warning") ShadesDTO? warning,
    @JsonKey(name: "error") ShadesDTO? error,
    @JsonKey(name: "purple") ShadesDTO? purple,
  }) = _SemanticDTO;

  factory SemanticDTO.fromJson(Map<String, dynamic> json) =>
      _$SemanticDTOFromJson(json);
}

@freezed
class SurfaceDTO with _$SurfaceDTO {
  const factory SurfaceDTO({
    @JsonKey(name: "White") ColorShadeValueDTO? white,
    @JsonKey(name: "Black") ColorShadeValueDTO? black,
    @JsonKey(name: "Background") ColorShadeValueDTO? background,
  }) = _SurfaceDTO;

  factory SurfaceDTO.fromJson(Map<String, dynamic> json) =>
      _$SurfaceDTOFromJson(json);
}
