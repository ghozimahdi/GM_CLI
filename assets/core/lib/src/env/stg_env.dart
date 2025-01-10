import 'package:core/src/env/app_env_fields.dart';
import 'package:envied/envied.dart';

part 'stg_env.g.dart';

@Envied(
  name: 'Env',
  path: '.env_staging',
  obfuscate: true,
  useConstantCase: true,
)
final class StgEnv implements AppEnvFields {
  StgEnv();

  @override
  @EnviedField()
  final String baseUrl = _Env.baseUrl;

  @override
  @EnviedField()
  final String androidFirebaseApiKey = _Env.androidFirebaseApiKey;

  @override
  @EnviedField()
  final String androidFirebaseAppId = _Env.androidFirebaseAppId;

  @override
  @EnviedField()
  final String androidFirebaseMessagingSenderId =
      _Env.androidFirebaseMessagingSenderId;

  @override
  @EnviedField()
  final String androidFirebaseProjectId = _Env.androidFirebaseProjectId;

  @override
  @EnviedField()
  final String androidFirebaseStorageBucket = _Env.androidFirebaseStorageBucket;

  @override
  @EnviedField()
  final String iosFirebaseApiKey = _Env.iosFirebaseApiKey;

  @override
  @EnviedField()
  final String iosFirebaseAppId = _Env.iosFirebaseAppId;

  @override
  @EnviedField()
  final String iosFirebaseBundleId = _Env.iosFirebaseBundleId;

  @override
  @EnviedField()
  final String iosFirebaseMessagingSenderId = _Env.iosFirebaseMessagingSenderId;

  @override
  @EnviedField()
  final String iosFirebaseProjectId = _Env.iosFirebaseProjectId;

  @override
  @EnviedField()
  final String iosFirebaseStorageBucket = _Env.iosFirebaseStorageBucket;

  @override
  @EnviedField()
  final String iosFirebaseClientId = _Env.iosFirebaseClientId;
}
