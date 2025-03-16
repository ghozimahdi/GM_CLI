import 'package:feature_common/feature_common.dart';
import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';

mixin AppDialog {
  Future<void> showCustomDialog(
    BuildContext context, {
    required Widget child,
    bool barrierDismissible = true,
    EdgeInsetsGeometry? padding,
  }) async {
    await showDialog(
      barrierDismissible: barrierDismissible,
      context: context,
      builder: (_) {
        return Dialog(
          surfaceTintColor: colors.white,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8.r),
          ),
          backgroundColor: colors.white,
          child: Padding(
            padding: padding ?? EdgeInsets.all(16.w),
            child: child,
          ),
        );
      },
    );
  }

  Future<void> showAlertDialog(
    BuildContext context, {
    required Widget title,
    required Widget content,
    required List<TextButton> actions,
  }) async {
    return showDialog<void>(
      context: context,
      barrierDismissible: false,
      builder: (BuildContext context) {
        return AlertDialog(
          title: title,
          content: content,
          actions: actions,
        );
      },
    );
  }
}
