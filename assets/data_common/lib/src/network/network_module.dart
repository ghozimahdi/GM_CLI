import 'package:chucker_flutter/chucker_flutter.dart';
import 'package:data_common/src/network/dio_error_interceptor.dart';
import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import 'package:injectable/injectable.dart';

@module
abstract class NetworkModule {
  @Singleton(env: [Environment.prod])
  Dio get dioProd {
    final dio = Dio(
      BaseOptions(
        connectTimeout: const Duration(milliseconds: 20000),
        sendTimeout: const Duration(milliseconds: 30000),
        receiveTimeout: const Duration(milliseconds: 30000),
      ),
    );

    dio.interceptors.add(DioErrorInterceptor());

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

  @Singleton(env: [Environment.dev, Environment.test])
  Dio get dio {
    final dio = Dio(
      BaseOptions(
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
      DioErrorInterceptor(),
    ]);
    return dio;
  }
}
