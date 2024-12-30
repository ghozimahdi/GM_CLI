import 'package:flutter/material.dart';
import 'package:fluttertoast/fluttertoast.dart';
import 'package:resource/resource.dart';

/// AppToast
///
/// A mixin providing a utility method to show toast messages using the GetX package.
///
/// This mixin can be used to display brief messages in the form of a snack bar to the user.

mixin AppToast {
  static void showMessage(
    BuildContext context,
    String message, {
    ToastType toastType = ToastType.defaults,
    IconData iconData = Icons.info,
  }) {
    final fToast = FToast();
    fToast.init(context);
    fToast.showToast(
      child: Container(
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(8.r),
          border: Border.fromBorderSide(
            BorderSide(
              color: toastType.borderColor,
            ),
          ),
          color: toastType.backgroundColor,
        ),
        padding: EdgeInsets.all(10.w),
        child: Row(
          children: [
            Icon(
              iconData,
              color: toastType.iconColor,
            ),
            6.horizontalSpace,
            Expanded(
              child: Text(
                message,
                style: res.textTheme.bodyLargeMedium?.copyWith(
                  color: toastType.foregroundColor,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

enum ToastType { error, success, warning, defaults }

extension SnackBarTypeExt on ToastType {
  Color get backgroundColor {
    switch (this) {
      case ToastType.error:
        return colors.semanticError.shade100;
      case ToastType.success:
        return colors.semanticSuccess.shade100;
      case ToastType.warning:
        return colors.semanticWarning.shade100;
      case ToastType.defaults:
        return colors.surface.white;
    }
  }

  Color get borderColor {
    switch (this) {
      case ToastType.error:
        return colors.semanticError.shade300;
      case ToastType.success:
        return colors.semanticSuccess.shade300;
      case ToastType.warning:
        return colors.semanticWarning.shade300;
      case ToastType.defaults:
        return colors.outline.main;
    }
  }

  Color get foregroundColor {
    switch (this) {
      case ToastType.error:
        return colors.semanticError.shade1000;
      case ToastType.success:
        return colors.semanticSuccess.shade1000;
      case ToastType.warning:
        return colors.semanticWarning.shade1000;
      case ToastType.defaults:
        return colors.neural.shade500;
    }
  }

  Color get iconColor {
    switch (this) {
      case ToastType.error:
        return colors.semanticError.shade800;
      case ToastType.success:
        return colors.semanticSuccess.shade800;
      case ToastType.warning:
        return colors.semanticWarning.shade800;
      case ToastType.defaults:
        return colors.neural.shade500;
    }
  }
}
