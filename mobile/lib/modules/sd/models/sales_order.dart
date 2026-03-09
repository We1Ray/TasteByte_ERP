class SalesOrder {
  final String id;
  final String orderNumber;
  final String customerId;
  final String customerName;
  final String status;
  final DateTime orderDate;
  final DateTime? deliveryDate;
  final double totalAmount;
  final String currency;
  final List<SalesOrderItem> items;
  final DateTime? createdAt;

  const SalesOrder({
    required this.id,
    required this.orderNumber,
    required this.customerId,
    required this.customerName,
    required this.status,
    required this.orderDate,
    this.deliveryDate,
    required this.totalAmount,
    this.currency = 'TWD',
    this.items = const [],
    this.createdAt,
  });

  factory SalesOrder.fromJson(Map<String, dynamic> json) {
    return SalesOrder(
      id: json['id']?.toString() ?? '',
      orderNumber: json['order_number'] as String? ?? '',
      customerId: json['customer_id']?.toString() ?? '',
      customerName: json['customer_name'] as String? ?? '',
      status: json['status'] as String? ?? 'draft',
      orderDate: DateTime.tryParse(json['order_date'] as String? ?? '') ??
          DateTime.now(),
      deliveryDate: json['delivery_date'] != null
          ? DateTime.tryParse(json['delivery_date'] as String)
          : null,
      totalAmount: (json['total_amount'] as num?)?.toDouble() ?? 0,
      currency: json['currency'] as String? ?? 'TWD',
      items: (json['items'] as List<dynamic>?)
              ?.map((e) => SalesOrderItem.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
      createdAt: json['created_at'] != null
          ? DateTime.tryParse(json['created_at'] as String)
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'order_number': orderNumber,
      'customer_id': customerId,
      'customer_name': customerName,
      'status': status,
      'order_date': orderDate.toIso8601String(),
      'delivery_date': deliveryDate?.toIso8601String(),
      'total_amount': totalAmount,
      'currency': currency,
      'items': items.map((e) => e.toJson()).toList(),
    };
  }
}

class SalesOrderItem {
  final String id;
  final String materialId;
  final String materialNumber;
  final String description;
  final double quantity;
  final String unitOfMeasure;
  final double unitPrice;
  final double totalPrice;

  const SalesOrderItem({
    required this.id,
    required this.materialId,
    required this.materialNumber,
    required this.description,
    required this.quantity,
    this.unitOfMeasure = 'EA',
    required this.unitPrice,
    required this.totalPrice,
  });

  factory SalesOrderItem.fromJson(Map<String, dynamic> json) {
    return SalesOrderItem(
      id: json['id']?.toString() ?? '',
      materialId: json['material_id']?.toString() ?? '',
      materialNumber: json['material_number'] as String? ?? '',
      description: json['description'] as String? ?? '',
      quantity: (json['quantity'] as num?)?.toDouble() ?? 0,
      unitOfMeasure: json['unit_of_measure'] as String? ?? 'EA',
      unitPrice: (json['unit_price'] as num?)?.toDouble() ?? 0,
      totalPrice: (json['total_price'] as num?)?.toDouble() ?? 0,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'material_id': materialId,
      'material_number': materialNumber,
      'description': description,
      'quantity': quantity,
      'unit_of_measure': unitOfMeasure,
      'unit_price': unitPrice,
      'total_price': totalPrice,
    };
  }
}
