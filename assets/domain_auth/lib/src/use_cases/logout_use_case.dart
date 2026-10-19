import 'package:domain_auth/domain_auth.dart';
import 'package:injectable/injectable.dart';

@lazySingleton
class LogoutUseCase {
  final AuthRepository _authRepository;

  LogoutUseCase(this._authRepository);

  Future<void> call() async {
    await _authRepository.logout();
  }
}
