abstract interface class AppEnvFields {
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
