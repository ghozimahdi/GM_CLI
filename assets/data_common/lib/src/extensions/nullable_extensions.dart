extension StringExtensions on String? {
  String orEmpty() => this ?? '';
}

extension ListExtensions<T> on List<T>? {
  List<T> orEmpty() => this ?? [];
}

extension IntExtensions on int? {
  int orZero() => this ?? 0;
}

extension DoubleExtensions on double? {
  double orZero() => this ?? 0.0;
}

extension BoolExtensions on bool? {
  bool orFalse() => this ?? false;
}

extension StringOrNullExtensions on String {
  String? orNull() => isNotEmpty ? this : null;
}

extension ListOrNullExtensions<T> on List<T> {
  List<T>? orNull() => isNotEmpty ? this : null;
}

extension IntOrNullExtensions on int {
  int? orNull() => this == 0 ? null : this;
}

extension DoubleOrNullExtensions on double {
  double? orNull() => this == 0 ? null : this;
}
