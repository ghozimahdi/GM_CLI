import 'package:flutter/material.dart';
import 'package:resource/resource.dart';
import 'package:shimmer/shimmer.dart';

class AppShimmer extends StatelessWidget {
  final double borderRadius;

  const AppShimmer({
    super.key,
    this.borderRadius = 6,
  });

  @override
  Widget build(BuildContext context) {
    return Shimmer.fromColors(
      baseColor: colors.addOn.grey,
      highlightColor: colors.surface.white,
      child: ClipRRect(
        borderRadius: BorderRadius.circular(
          borderRadius,
        ),
        child: Container(
          decoration: BoxDecoration(
            color: colors.addOn.grey,
          ),
        ),
      ),
    );
  }
}
