class ApiException implements Exception {
  final String message;
  final int? statusCode;
  final dynamic data;

  const ApiException({
    required this.message,
    this.statusCode,
    this.data,
  });

  @override
  String toString() => 'ApiException($statusCode): $message';
}

class UnauthorizedException extends ApiException {
  const UnauthorizedException({String message = 'Unauthorized'})
      : super(message: message, statusCode: 401);
}

class ForbiddenException extends ApiException {
  const ForbiddenException({String message = 'Forbidden'})
      : super(message: message, statusCode: 403);
}

class NotFoundException extends ApiException {
  const NotFoundException({String message = 'Not Found'})
      : super(message: message, statusCode: 404);
}

class ServerException extends ApiException {
  const ServerException({String message = 'Internal Server Error'})
      : super(message: message, statusCode: 500);
}

class NetworkException extends ApiException {
  const NetworkException({String message = 'Network Error'})
      : super(message: message);
}

class ConnectionTimeoutException extends ApiException {
  const ConnectionTimeoutException({String message = 'Request Timeout'})
      : super(message: message);
}
