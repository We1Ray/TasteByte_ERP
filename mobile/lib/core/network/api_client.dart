import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../constants.dart';
import '../storage/secure_storage.dart';
import 'api_exceptions.dart';

final dioProvider = Provider<Dio>((ref) {
  final dio = Dio(BaseOptions(
    baseUrl: AppConstants.apiBaseUrl,
    connectTimeout: AppConstants.connectTimeout,
    receiveTimeout: AppConstants.receiveTimeout,
    headers: {'Content-Type': 'application/json'},
  ));

  dio.interceptors.add(AuthInterceptor(ref));
  dio.interceptors.add(LogInterceptor(
    requestBody: true,
    responseBody: true,
    logPrint: (obj) => print('[API] $obj'),
  ));

  return dio;
});

class AuthInterceptor extends Interceptor {
  final Ref _ref;

  AuthInterceptor(this._ref);

  @override
  void onRequest(
      RequestOptions options, RequestInterceptorHandler handler) async {
    final storage = _ref.read(secureStorageProvider);
    final token = await storage.getToken();
    if (token != null) {
      options.headers['Authorization'] = 'Bearer $token';
    }
    handler.next(options);
  }

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) {
    final exception = _mapDioException(err);
    handler.reject(DioException(
      requestOptions: err.requestOptions,
      error: exception,
      type: err.type,
      response: err.response,
    ));
  }

  ApiException _mapDioException(DioException err) {
    switch (err.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.sendTimeout:
      case DioExceptionType.receiveTimeout:
        return const ConnectionTimeoutException();
      case DioExceptionType.connectionError:
        return const NetworkException();
      case DioExceptionType.badResponse:
        return _mapStatusCode(err.response);
      default:
        return ApiException(
          message: err.message ?? 'Unknown error occurred',
        );
    }
  }

  ApiException _mapStatusCode(Response? response) {
    final statusCode = response?.statusCode;
    final data = response?.data;
    final message =
        data is Map ? (data['detail'] ?? data['message'] ?? 'Error') : 'Error';

    switch (statusCode) {
      case 401:
        return UnauthorizedException(message: message.toString());
      case 403:
        return ForbiddenException(message: message.toString());
      case 404:
        return NotFoundException(message: message.toString());
      case 500:
        return ServerException(message: message.toString());
      default:
        return ApiException(
          message: message.toString(),
          statusCode: statusCode,
          data: data,
        );
    }
  }
}
