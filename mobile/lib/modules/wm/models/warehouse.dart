class Warehouse {
  final String id;
  final String warehouseNumber;
  final String description;
  final String address;
  final String type;
  final bool isActive;
  final int totalCapacity;
  final int usedCapacity;

  const Warehouse({
    required this.id,
    required this.warehouseNumber,
    required this.description,
    this.address = '',
    this.type = 'standard',
    this.isActive = true,
    this.totalCapacity = 0,
    this.usedCapacity = 0,
  });

  double get utilizationPercent =>
      totalCapacity > 0 ? (usedCapacity / totalCapacity) * 100 : 0;

  factory Warehouse.fromJson(Map<String, dynamic> json) {
    return Warehouse(
      id: json['id']?.toString() ?? '',
      warehouseNumber: json['warehouse_number'] as String? ?? '',
      description: json['description'] as String? ?? '',
      address: json['address'] as String? ?? '',
      type: json['type'] as String? ?? 'standard',
      isActive: json['is_active'] as bool? ?? true,
      totalCapacity: json['total_capacity'] as int? ?? 0,
      usedCapacity: json['used_capacity'] as int? ?? 0,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'warehouse_number': warehouseNumber,
      'description': description,
      'address': address,
      'type': type,
      'is_active': isActive,
      'total_capacity': totalCapacity,
      'used_capacity': usedCapacity,
    };
  }
}
