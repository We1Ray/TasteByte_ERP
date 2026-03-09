class User {
  final String id;
  final String username;
  final String email;
  final String fullName;
  final String role;
  final bool isActive;

  const User({
    required this.id,
    required this.username,
    required this.email,
    required this.fullName,
    required this.role,
    this.isActive = true,
  });

  factory User.fromJson(Map<String, dynamic> json) {
    return User(
      id: json['id']?.toString() ?? '',
      username: json['username'] as String? ?? '',
      email: json['email'] as String? ?? '',
      fullName: json['full_name'] as String? ?? json['fullName'] as String? ?? '',
      role: json['role'] as String? ?? 'user',
      isActive: json['is_active'] as bool? ?? true,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'username': username,
      'email': email,
      'full_name': fullName,
      'role': role,
      'is_active': isActive,
    };
  }
}
