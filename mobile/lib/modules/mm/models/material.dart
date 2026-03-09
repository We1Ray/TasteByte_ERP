class Material {
  final String id;
  final String materialNumber;
  final String description;
  final String materialType;
  final String unitOfMeasure;
  final String materialGroup;
  final double? weight;
  final String? weightUnit;
  final bool isActive;
  final DateTime? createdAt;

  const Material({
    required this.id,
    required this.materialNumber,
    required this.description,
    required this.materialType,
    this.unitOfMeasure = 'EA',
    this.materialGroup = '',
    this.weight,
    this.weightUnit,
    this.isActive = true,
    this.createdAt,
  });

  factory Material.fromJson(Map<String, dynamic> json) {
    return Material(
      id: json['id']?.toString() ?? '',
      materialNumber: json['material_number'] as String? ?? '',
      description: json['description'] as String? ?? '',
      materialType: json['material_type'] as String? ?? '',
      unitOfMeasure: json['unit_of_measure'] as String? ?? 'EA',
      materialGroup: json['material_group'] as String? ?? '',
      weight: (json['weight'] as num?)?.toDouble(),
      weightUnit: json['weight_unit'] as String?,
      isActive: json['is_active'] as bool? ?? true,
      createdAt: json['created_at'] != null
          ? DateTime.tryParse(json['created_at'] as String)
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'material_number': materialNumber,
      'description': description,
      'material_type': materialType,
      'unit_of_measure': unitOfMeasure,
      'material_group': materialGroup,
      'weight': weight,
      'weight_unit': weightUnit,
      'is_active': isActive,
    };
  }
}
