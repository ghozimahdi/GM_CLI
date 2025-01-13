import 'dart:convert';

import 'package:data_common/src/network/app_exception.dart';
import 'package:dio/dio.dart';

class DioErrorInterceptor extends Interceptor {
  @override
  void onError(DioException err, ErrorInterceptorHandler handler) {
    final customException = _mapDioExceptionToAppException(err);
    handler.reject(
      DioException(
        requestOptions: err.requestOptions,
        error: customException,
      ),
    );
  }

  AppException _mapDioExceptionToAppException(DioException exception) {
    switch (exception.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.sendTimeout:
      case DioExceptionType.receiveTimeout:
        return TimeoutException();

      case DioExceptionType.connectionError:
        return NoConnectionException();

      case DioExceptionType.badResponse:
        final statusCode = exception.response?.statusCode ?? 0;
        final errorMessage = _extractErrorMessage(exception.response?.data) ??
            'An unknown error occurred';

        if (statusCode == 401) {
          return UnauthorizedException();
        } else if (statusCode >= 400 && statusCode < 500) {
          return ClientException(errorMessage);
        } else if (statusCode >= 500) {
          return ServerException();
        }

        return AppException(
          errorMessage,
          statusCode: statusCode,
        );

      case DioExceptionType.badCertificate:
        return AppException('SSL certificate validation failed');

      case DioExceptionType.cancel:
        return AppException('Request was cancelled');

      case DioExceptionType.unknown:
        return AppException(exception.message ?? 'An unknown error occurred');
    }
  }

  String? _extractErrorMessage(Object? data) {
    try {
      if (data is Map<String, dynamic>) {
        return data['message'] as String?;
      } else if (data is String) {
        final jsonData = jsonDecode(data) as Map<String, dynamic>;
        return jsonData['message'] as String?;
      }
    } catch (e) {
      return null;
    }
    return null;
  }
}
