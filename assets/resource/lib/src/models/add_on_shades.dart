import 'package:flutter/material.dart';

class AddOnShades {
  final Color info;
  final Color success;
  final Color warning;
  final Color notification;
  final Color error;
  final Color gold;
  final Color grey;

  AddOnShades({
    required this.info,
    required this.success,
    required this.warning,
    required this.notification,
    required this.error,
    required this.gold,
    required this.grey,
  });

  factory AddOnShades.defaults() => AddOnShades(
        info: Colors.white,
        success: Colors.white,
        warning: Colors.white,
        notification: Colors.white,
        error: Colors.white,
        gold: Colors.white,
        grey: Colors.white,
      );
}
