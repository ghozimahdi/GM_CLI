import 'dart:io';

import 'package:app/firebase/firebase_options.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flutter/cupertino.dart';
import 'package:injectable/injectable.dart';

@singleton
class MessagingHelper {
  static String? _consumedMessageId;

  Future initialize({
    required Function(RemoteMessage event) onMessage,
  }) async {
    if (Platform.isIOS) {
      await FirebaseMessaging.instance.requestPermission();

      await FirebaseMessaging.instance
          .setForegroundNotificationPresentationOptions(
        alert: true,
        badge: true,
        sound: true,
      );
    }

    FirebaseMessaging.onMessage.listen(onMessage);

    FirebaseMessaging.onBackgroundMessage(_onBackgroundMessage);
  }

  @pragma('vm:entry-point')
  static Future<void> _onBackgroundMessage(RemoteMessage message) async {
    try {
      await Firebase.initializeApp(
        options: DefaultFirebaseOptions.currentPlatform,
      );

      final notification = message.notification;
      if (notification?.title != null && notification?.body != null) {
        debugPrint('Background Message data: ${message.data}');
        // TODO: Update your database local as you need
      }
    } on Exception catch (e) {
      debugPrint(e.toString());
    }
  }

  /// Configures notification handling when the app is opened from a terminated
  /// or background state due to a notification click.
  ///
  /// This method should be called in the `initState` of the `HomePage` to handle
  /// notification clicks and navigate to the appropriate page based on the notification data.
  ///
  /// [callback] is a function that takes a `Map<String, dynamic>` containing the notification
  /// data and performs the necessary actions, such as navigating to a specific page.
  ///
  /// Usage:
  ///
  /// ```dart
  /// class HomePage extends StatefulWidget {
  ///   @override
  ///   _HomePageState createState() => _HomePageState();
  /// }
  ///
  /// class _HomePageState extends State<HomePage> {
  ///   @override
  ///   void initState() {
  ///     super.initState();
  ///     messagingHelper.configureOpenNotification((data) {
  ///       // Handle the data from the notification
  ///       // Navigate to the appropriate page based on the data
  ///       navigateToNotificationPage(data);
  ///     });
  ///   }
  ///
  ///   void navigateToNotificationPage(Map<String, dynamic> data) {
  ///     // Your navigation logic based on notification data
  ///   }
  ///
  ///   @override
  ///   Widget build(BuildContext context) {
  ///     return Scaffold(
  ///       appBar: AppBar(
  ///         title: Text('Home Page'),
  ///       ),
  ///       body: Center(
  ///         child: Text('Home Page Content'),
  ///       ),
  ///     );
  ///   }
  /// }
  /// ```
  ///
  /// This ensures that the app can handle notifications properly, whether it is opened
  /// from a terminated state or the background.
  Future<void> configureOpenNotification(
    Function(Map<String, dynamic>) callback,
  ) async {
    final initialMessage = await FirebaseMessaging.instance.getInitialMessage();

    if (initialMessage != null &&
        _consumedMessageId != initialMessage.messageId) {
      callback(initialMessage.data);
      _consumedMessageId = initialMessage.messageId;
    }

    FirebaseMessaging.onMessageOpenedApp.listen((RemoteMessage message) {
      if (_consumedMessageId != message.messageId) {
        callback(message.data);
        _consumedMessageId = message.messageId;
      }
    });
  }
}
