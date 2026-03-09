import 'dart:convert';
import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/network/api_client.dart';
import '../../../core/storage/secure_storage.dart';
import '../models/login_request.dart';
import '../models/token_response.dart';
import '../models/user.dart';

enum AuthStatus { initial, loading, authenticated, unauthenticated, error }

class AuthState {
  final AuthStatus status;
  final User? user;
  final String? errorMessage;

  const AuthState({
    this.status = AuthStatus.initial,
    this.user,
    this.errorMessage,
  });

  AuthState copyWith({
    AuthStatus? status,
    User? user,
    String? errorMessage,
  }) {
    return AuthState(
      status: status ?? this.status,
      user: user ?? this.user,
      errorMessage: errorMessage,
    );
  }
}

class AuthNotifier extends StateNotifier<AuthState> {
  final Dio _dio;
  final SecureStorage _storage;

  AuthNotifier(this._dio, this._storage) : super(const AuthState());

  Future<void> checkAuth() async {
    state = state.copyWith(status: AuthStatus.loading);
    try {
      final token = await _storage.getToken();
      if (token == null) {
        state = state.copyWith(status: AuthStatus.unauthenticated);
        return;
      }

      final userJson = await _storage.getUser();
      if (userJson != null) {
        final user = User.fromJson(jsonDecode(userJson) as Map<String, dynamic>);
        state = state.copyWith(
          status: AuthStatus.authenticated,
          user: user,
        );
      } else {
        state = state.copyWith(status: AuthStatus.unauthenticated);
      }
    } catch (_) {
      state = state.copyWith(status: AuthStatus.unauthenticated);
    }
  }

  Future<void> login(LoginRequest request) async {
    state = state.copyWith(status: AuthStatus.loading, errorMessage: null);
    try {
      final response = await _dio.post(
        '/auth/login',
        data: FormData.fromMap({
          'username': request.username,
          'password': request.password,
        }),
        options: Options(
          contentType: Headers.formUrlEncodedContentType,
        ),
      );

      final tokenResponse =
          TokenResponse.fromJson(response.data as Map<String, dynamic>);
      await _storage.saveToken(tokenResponse.accessToken);
      if (tokenResponse.refreshToken != null) {
        await _storage.saveRefreshToken(tokenResponse.refreshToken!);
      }

      final userResponse = await _dio.get(
        '/auth/me',
        options: Options(
          headers: {
            'Authorization': 'Bearer ${tokenResponse.accessToken}',
          },
        ),
      );

      final user = User.fromJson(userResponse.data as Map<String, dynamic>);
      await _storage.saveUser(jsonEncode(user.toJson()));

      state = state.copyWith(
        status: AuthStatus.authenticated,
        user: user,
      );
    } on DioException catch (e) {
      final message = e.response?.data is Map
          ? (e.response?.data['detail'] ?? 'Login failed').toString()
          : 'Login failed. Please check your credentials.';
      state = state.copyWith(
        status: AuthStatus.error,
        errorMessage: message,
      );
    } catch (e) {
      state = state.copyWith(
        status: AuthStatus.error,
        errorMessage: 'An unexpected error occurred.',
      );
    }
  }

  Future<void> logout() async {
    await _storage.clearAll();
    state = const AuthState(status: AuthStatus.unauthenticated);
  }
}

final authProvider = StateNotifierProvider<AuthNotifier, AuthState>((ref) {
  final dio = ref.watch(dioProvider);
  final storage = ref.watch(secureStorageProvider);
  return AuthNotifier(dio, storage);
});

final isAuthenticatedProvider = Provider<bool>((ref) {
  return ref.watch(authProvider).status == AuthStatus.authenticated;
});
