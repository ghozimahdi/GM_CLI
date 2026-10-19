import 'package:domain_common/domain_common.dart';

abstract class AuthRepository {
  Future<Failure> login();

  Future<bool> logout();

  bool getIsLogin();

  Future<Failure> register();
}
