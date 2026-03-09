class TokenResponse {
  final String accessToken;
  final String tokenType;
  final String? refreshToken;

  const TokenResponse({
    required this.accessToken,
    this.tokenType = 'bearer',
    this.refreshToken,
  });

  factory TokenResponse.fromJson(Map<String, dynamic> json) {
    return TokenResponse(
      accessToken: json['access_token'] as String? ?? '',
      tokenType: json['token_type'] as String? ?? 'bearer',
      refreshToken: json['refresh_token'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'access_token': accessToken,
      'token_type': tokenType,
      if (refreshToken != null) 'refresh_token': refreshToken,
    };
  }
}
