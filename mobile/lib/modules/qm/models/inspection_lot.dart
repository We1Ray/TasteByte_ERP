class InspectionLot {
  final String id;
  final String lotNumber;
  final String materialId;
  final String materialNumber;
  final String materialDescription;
  final String inspectionType;
  final String status;
  final double quantity;
  final String unitOfMeasure;
  final DateTime createdAt;
  final DateTime? completedAt;
  final String? result;
  final String? inspector;
  final List<InspectionResult> results;

  const InspectionLot({
    required this.id,
    required this.lotNumber,
    required this.materialId,
    required this.materialNumber,
    required this.materialDescription,
    required this.inspectionType,
    required this.status,
    required this.quantity,
    this.unitOfMeasure = 'EA',
    required this.createdAt,
    this.completedAt,
    this.result,
    this.inspector,
    this.results = const [],
  });

  factory InspectionLot.fromJson(Map<String, dynamic> json) {
    return InspectionLot(
      id: json['id']?.toString() ?? '',
      lotNumber: json['lot_number'] as String? ?? '',
      materialId: json['material_id']?.toString() ?? '',
      materialNumber: json['material_number'] as String? ?? '',
      materialDescription: json['material_description'] as String? ?? '',
      inspectionType: json['inspection_type'] as String? ?? '',
      status: json['status'] as String? ?? 'created',
      quantity: (json['quantity'] as num?)?.toDouble() ?? 0,
      unitOfMeasure: json['unit_of_measure'] as String? ?? 'EA',
      createdAt:
          DateTime.tryParse(json['created_at'] as String? ?? '') ??
              DateTime.now(),
      completedAt: json['completed_at'] != null
          ? DateTime.tryParse(json['completed_at'] as String)
          : null,
      result: json['result'] as String?,
      inspector: json['inspector'] as String?,
      results: (json['results'] as List<dynamic>?)
              ?.map(
                  (e) => InspectionResult.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'lot_number': lotNumber,
      'material_id': materialId,
      'material_number': materialNumber,
      'material_description': materialDescription,
      'inspection_type': inspectionType,
      'status': status,
      'quantity': quantity,
      'unit_of_measure': unitOfMeasure,
      'created_at': createdAt.toIso8601String(),
      'completed_at': completedAt?.toIso8601String(),
      'result': result,
      'inspector': inspector,
      'results': results.map((e) => e.toJson()).toList(),
    };
  }
}

class InspectionResult {
  final String id;
  final String characteristic;
  final String targetValue;
  final String actualValue;
  final bool passed;
  final String? notes;

  const InspectionResult({
    required this.id,
    required this.characteristic,
    required this.targetValue,
    required this.actualValue,
    required this.passed,
    this.notes,
  });

  factory InspectionResult.fromJson(Map<String, dynamic> json) {
    return InspectionResult(
      id: json['id']?.toString() ?? '',
      characteristic: json['characteristic'] as String? ?? '',
      targetValue: json['target_value'] as String? ?? '',
      actualValue: json['actual_value'] as String? ?? '',
      passed: json['passed'] as bool? ?? false,
      notes: json['notes'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'characteristic': characteristic,
      'target_value': targetValue,
      'actual_value': actualValue,
      'passed': passed,
      'notes': notes,
    };
  }
}
