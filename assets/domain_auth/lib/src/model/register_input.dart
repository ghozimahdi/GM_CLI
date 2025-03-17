import 'package:freezed_annotation/freezed_annotation.dart';

part 'register_input.freezed.dart';

@freezed
class RegisterInput with _$RegisterInput {
  const RegisterInput._();

  const factory RegisterInput({
    required String name,
    required String password,
    required String email,
    required String mobile,
    required bool hasAgreedToPDPA,
  }) = _RegisterInput;
}
