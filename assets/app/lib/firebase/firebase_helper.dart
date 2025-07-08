import 'dart:convert';
import 'dart:io';

import 'package:app/firebase/firebase_options.dart';
import 'package:app/firebase/messaging_helper.dart';
import 'package:app/firebase/notification_helper.dart';
import 'package:app/injector.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flutter/cupertino.dart';

mixin FirebaseHelper {
  static Future<void> initialize() async {
    await Firebase.initializeApp(
      options: DefaultFirebaseOptions.currentPlatform,
    );

    await FirebaseMessaging.instance.requestPermission();

    final notificationHelper = getIt<NotificationHelper>();
    await notificationHelper.initialize();

    await getIt<MessagingHelper>().initialize(
      onMessage: (message) {
        final notification = message.notification;

        if (notification?.title != null && notification?.body != null) {
          notificationHelper.showLocalNotification(
            id: notification.hashCode,
            title: notification?.title,
            body: notification?.body,
            payload: json.encode(message.data),
          );
        }
      },
    );
  }

  static Future<String> getToken() async {
    String? token = await FirebaseMessaging.instance.getToken();
    String? apnsToken = await FirebaseMessaging.instance.getAPNSToken();
    debugPrint('Token Firebase: ${Platform.isAndroid ? token : apnsToken}');
    return (Platform.isAndroid ? token : apnsToken) ?? '';
  }
}
