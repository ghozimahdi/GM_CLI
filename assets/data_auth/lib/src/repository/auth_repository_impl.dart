import 'package:data_auth/src/data_source/auth_data_source.dart';
import 'package:data_common/data_common.dart';
import 'package:domain_auth/domain_auth.dart';
import 'package:domain_common/domain_common.dart';
import 'package:injectable/injectable.dart';

@LazySingleton(as: AuthRepository)
class AuthRepositoryImpl extends AuthRepository with FailureHandlerMixin {
  final AuthDataSource _authDataSource;
  final SharedPreferenceDataSource _preferenceDataSource;

  AuthRepositoryImpl(this._authDataSource, this._preferenceDataSource);

  @override
  Future<Failure> login() async {
    try {
      await _authDataSource.login(
        action: 'login',
        username: 'input.username',
        password: 'input.password',
      );

      await _preferenceDataSource.setIsLogin(true);
      return const Failure.noFailure();
    } catch (e) {
      final failure = mapToFailure(e);
      return failure;
    }
  }

  @override
  Future<Failure> register() async {
    try {
      await _authDataSource.createNewAccount(
        action: 'createNewAccount',
        name: 'input.name',
        password: 'input.password',
        email: 'input.email',
        mobile: 'input.mobile',
        hasAgreedToPDPA: true,
      );

      return const Failure.noFailure();
    } catch (e) {
      final failure = mapToFailure(e);
      return failure;
    }
  }

  @override
  Future<bool> logout() {
    return _preferenceDataSource.clearAllData();
  }

  @override
  bool getIsLogin() {
    return _preferenceDataSource.getIsLogin();
  }
}
