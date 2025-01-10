import 'package:flutter/material.dart';

class SurfaceShades {
  final Color white;
  final Color black;
  final Color background;

  SurfaceShades({
    required this.white,
    required this.black,
    required this.background,
  });

  factory SurfaceShades.defaults() => SurfaceShades(
        white: Colors.white,
        black: Colors.black,
        background: Colors.white,
      );
}
