import 'package:freezed_annotation/freezed_annotation.dart';

part 'login_input.freezed.dart';

@freezed
class LoginInput with _$LoginInput {
  const LoginInput._();

  const factory LoginInput({
    @Default('') String username,
    @Default('') String password,
    @Default(0) int postalCode,
    @Default(0) int mobile,
  }) = _LoginInput;
}
