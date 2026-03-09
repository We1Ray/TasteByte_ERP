class PlantStock {
  final String id;
  final String materialId;
  final String materialNumber;
  final String materialDescription;
  final String warehouseId;
  final String warehouseName;
  final double quantity;
  final String unitOfMeasure;
  final double? minStock;
  final double? maxStock;
  final DateTime? lastUpdated;

  const PlantStock({
    required this.id,
    required this.materialId,
    required this.materialNumber,
    required this.materialDescription,
    required this.warehouseId,
    required this.warehouseName,
    required this.quantity,
    this.unitOfMeasure = 'EA',
    this.minStock,
    this.maxStock,
    this.lastUpdated,
  });

  bool get isBelowMin => minStock != null && quantity < minStock!;
  bool get isAboveMax => maxStock != null && quantity > maxStock!;

  factory PlantStock.fromJson(Map<String, dynamic> json) {
    return PlantStock(
      id: json['id']?.toString() ?? '',
      materialId: json['material_id']?.toString() ?? '',
      materialNumber: json['material_number'] as String? ?? '',
      materialDescription: json['material_description'] as String? ?? '',
      warehouseId: json['warehouse_id']?.toString() ?? '',
      warehouseName: json['warehouse_name'] as String? ?? '',
      quantity: (json['quantity'] as num?)?.toDouble() ?? 0,
      unitOfMeasure: json['unit_of_measure'] as String? ?? 'EA',
      minStock: (json['min_stock'] as num?)?.toDouble(),
      maxStock: (json['max_stock'] as num?)?.toDouble(),
      lastUpdated: json['last_updated'] != null
          ? DateTime.tryParse(json['last_updated'] as String)
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'material_id': materialId,
      'material_number': materialNumber,
      'material_description': materialDescription,
      'warehouse_id': warehouseId,
      'warehouse_name': warehouseName,
      'quantity': quantity,
      'unit_of_measure': unitOfMeasure,
      'min_stock': minStock,
      'max_stock': maxStock,
    };
  }
}
