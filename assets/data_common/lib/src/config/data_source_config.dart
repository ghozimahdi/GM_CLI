import 'package:injectable/injectable.dart';

abstract interface class DataSourceConfig {
  abstract final String baseUrl;

  // Firebase keys
  abstract final String androidFirebaseApiKey;
  abstract final String androidFirebaseAppId;
  abstract final String androidFirebaseMessagingSenderId;
  abstract final String androidFirebaseProjectId;
  abstract final String androidFirebaseStorageBucket;

  abstract final String iosFirebaseApiKey;
  abstract final String iosFirebaseAppId;
  abstract final String iosFirebaseMessagingSenderId;
  abstract final String iosFirebaseProjectId;
  abstract final String iosFirebaseStorageBucket;
  abstract final String iosFirebaseBundleId;
  abstract final String iosFirebaseClientId;
}

@LazySingleton(
  as: DataSourceConfig,
  env: [
    Environment.dev,
    Environment.test,
  ],
)
class DevDataSourceConfig implements DataSourceConfig {
  @override
  String get baseUrl => 'https://www.dev.ghozimahdi.com';

  @override
  // TODO: implement androidFirebaseApiKey
  String get androidFirebaseApiKey => throw UnimplementedError();

  @override
  // TODO: implement androidFirebaseAppId
  String get androidFirebaseAppId => throw UnimplementedError();

  @override
  // TODO: implement androidFirebaseMessagingSenderId
  String get androidFirebaseMessagingSenderId => throw UnimplementedError();

  @override
  // TODO: implement androidFirebaseProjectId
  String get androidFirebaseProjectId => throw UnimplementedError();

  @override
  // TODO: implement androidFirebaseStorageBucket
  String get androidFirebaseStorageBucket => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseApiKey
  String get iosFirebaseApiKey => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseAppId
  String get iosFirebaseAppId => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseBundleId
  String get iosFirebaseBundleId => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseClientId
  String get iosFirebaseClientId => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseMessagingSenderId
  String get iosFirebaseMessagingSenderId => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseProjectId
  String get iosFirebaseProjectId => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseStorageBucket
  String get iosFirebaseStorageBucket => throw UnimplementedError();
}

@LazySingleton(as: DataSourceConfig, env: [Environment.prod])
class ProdDataSourceConfig implements DataSourceConfig {
  @override
  String get baseUrl => 'https://www.ghozimahdi.com';

  @override
  // TODO: implement androidFirebaseApiKey
  String get androidFirebaseApiKey => throw UnimplementedError();

  @override
  // TODO: implement androidFirebaseAppId
  String get androidFirebaseAppId => throw UnimplementedError();

  @override
  // TODO: implement androidFirebaseMessagingSenderId
  String get androidFirebaseMessagingSenderId => throw UnimplementedError();

  @override
  // TODO: implement androidFirebaseProjectId
  String get androidFirebaseProjectId => throw UnimplementedError();

  @override
  // TODO: implement androidFirebaseStorageBucket
  String get androidFirebaseStorageBucket => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseApiKey
  String get iosFirebaseApiKey => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseAppId
  String get iosFirebaseAppId => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseBundleId
  String get iosFirebaseBundleId => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseClientId
  String get iosFirebaseClientId => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseMessagingSenderId
  String get iosFirebaseMessagingSenderId => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseProjectId
  String get iosFirebaseProjectId => throw UnimplementedError();

  @override
  // TODO: implement iosFirebaseStorageBucket
  String get iosFirebaseStorageBucket => throw UnimplementedError();
}
