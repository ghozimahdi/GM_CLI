import 'package:app/firebase/notification_helper.dart';
import 'package:flutter/foundation.dart';

extension NotificationHelperExt on NotificationHelper {
  Future<void> handleNotificationClick({required String? payload}) async {
    debugPrint('Notification opened with data: $payload');
    if (payload == null) return;

    // TODO: Parsing your payload
    // TODO: Then implement your business logic here
  }
}
