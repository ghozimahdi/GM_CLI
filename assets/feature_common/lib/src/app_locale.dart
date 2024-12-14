import 'package:flutter/material.dart';

enum AppLocale {
  indonesian,
  english;

  static String get path => 'packages/resource/assets/translations';

  Locale get toLocale {
    switch (this) {
      case AppLocale.english:
        return const Locale('en', 'US');
      case AppLocale.indonesian:
        return const Locale('id', 'ID');
    }
  }

  String get toLanguageTag {
    switch (this) {
      case AppLocale.english:
        return 'en-US';
      case AppLocale.indonesian:
        return 'id-ID';
    }
  }

  String get code {
    switch (this) {
      case AppLocale.english:
        return 'en';
      case AppLocale.indonesian:
        return 'id';
    }
  }
}
