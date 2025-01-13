class AppException implements Exception {
  final String message;
  final int? statusCode;

  AppException(this.message, {this.statusCode});

  @override
  String toString() {
    return 'AppException: $message (statusCode: $statusCode)';
  }
}

class UnauthorizedException extends AppException {
  UnauthorizedException() : super('Unauthorized access', statusCode: 401);
}

class NoConnectionException extends AppException {
  NoConnectionException() : super('No internet connection');
}

class TimeoutException extends AppException {
  TimeoutException() : super('Request timed out');
}

class ServerException extends AppException {
  ServerException() : super('Internal server error', statusCode: 500);
}

class ClientException extends AppException {
  ClientException(super.message, {super.statusCode});
}
