import 'package:flutter/material.dart';

class OutlineShades {
  final Color main;

  OutlineShades({
    required this.main,
  });

  factory OutlineShades.defaults() => OutlineShades(
        main: Colors.white,
      );
}
