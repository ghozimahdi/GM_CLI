import 'dart:io';

import 'package:app/firebase/notification_helper_ext.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:injectable/injectable.dart';
import 'package:resource/resource.dart';

@singleton
class NotificationHelper {
  late FlutterLocalNotificationsPlugin _flutterLocalNotificationsPlugin;

  static const androidNotificationChannel = AndroidNotificationChannel(
    'App', // TODO: change id to your app name
    'App Notification', // TODO: change name to your app name
    description: 'This channel is used for important notifications.',
    importance: Importance.max,
  );

  NotificationHelper() {
    _flutterLocalNotificationsPlugin = FlutterLocalNotificationsPlugin();
  }

  /// Initialize notification settings for Android and iOS.
  /// TODO: Change the default icon to your app’s icon.
  ///
  /// For Android notifications, use a **white icon** because:
  /// 1. Notifications in Android are typically shown on dark backgrounds, so white icons stand out well.
  /// 2. The icon color will be automatically adapted to your theme,
  /// and a white icon will look better when combined with the primary color set in `AndroidNotificationDetails`.
  Future initialize() async {
    const initSettings = InitializationSettings(
      // TODO: Replace with your app's icon
      android: AndroidInitializationSettings('@drawable/ic_logo'),
      iOS: DarwinInitializationSettings(),
    );

    await _flutterLocalNotificationsPlugin.initialize(
      initSettings,
      onDidReceiveNotificationResponse: (NotificationResponse response) async {
        handleNotificationClick(payload: response.payload);
      },
    );

    if (Platform.isIOS) {
      await _flutterLocalNotificationsPlugin
          .resolvePlatformSpecificImplementation<
              IOSFlutterLocalNotificationsPlugin>()
          ?.requestPermissions();
    }
    if (Platform.isAndroid) {
      await _flutterLocalNotificationsPlugin
          .resolvePlatformSpecificImplementation<
              AndroidFlutterLocalNotificationsPlugin>()
          ?.createNotificationChannel(androidNotificationChannel);
    }
  }

  /// Show a local notification with the provided details.
  /// TODO: Ensure the color is set according to your app's theme, such as `colors.primary`.
  Future showLocalNotification({
    int? id,
    required String? title,
    required String? body,
    required String? payload,
    String? image,
    List<AndroidNotificationAction>? actions,
  }) async {
    await _flutterLocalNotificationsPlugin.show(
      id ?? hashCode,
      title,
      body,
      NotificationDetails(
        android: AndroidNotificationDetails(
          androidNotificationChannel.id,
          androidNotificationChannel.name,
          channelDescription: androidNotificationChannel.description,
          importance: Importance.max,
          actions: actions,
          color: colors
              .primary, // TODO: Ensure this color matches your app's primary theme
          // Explanation: The icon color should be white because the notification will be displayed on dark backgrounds
          // and white icons provide better visibility and contrast. Additionally, it integrates well with the primary theme color.
        ),
        iOS: const DarwinNotificationDetails(
          presentAlert: true,
          presentBadge: true,
          presentSound: true,
        ),
      ),
      payload: payload,
    );
  }
}
