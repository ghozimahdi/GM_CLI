import 'package:chucker_flutter/chucker_flutter.dart';
import 'package:core/core.dart';
import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import 'package:injectable/injectable.dart';

@module
abstract class NetworkModule {
  @LazySingleton(env: [Environment.prod])
  Dio get dioProd {
    final dio = Dio(
      BaseOptions(
        baseUrl: AppEnv.env.baseUrl,
        connectTimeout: const Duration(milliseconds: 20000),
        sendTimeout: const Duration(milliseconds: 30000),
        receiveTimeout: const Duration(milliseconds: 30000),
      ),
    );

    if (kDebugMode) {
      dio.interceptors.addAll(
        [
          LogInterceptor(
            requestBody: true,
            responseBody: true,
          ),
          ChuckerDioInterceptor(),
        ],
      );
    }
    return dio;
  }

  @LazySingleton(env: [Environment.dev, Environment.test])
  Dio get dio {
    final dio = Dio(
      BaseOptions(
        baseUrl: AppEnv.env.baseUrl,
        connectTimeout: const Duration(milliseconds: 20000),
        sendTimeout: const Duration(milliseconds: 30000),
        receiveTimeout: const Duration(milliseconds: 30000),
      ),
    );

    dio.interceptors.addAll([
      LogInterceptor(
        requestBody: true,
        responseBody: true,
      ),
      ChuckerDioInterceptor(),
    ]);
    return dio;
  }
}
