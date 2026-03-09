import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/constants.dart';

class StockCountScreen extends ConsumerStatefulWidget {
  const StockCountScreen({super.key});

  @override
  ConsumerState<StockCountScreen> createState() => _StockCountScreenState();
}

class _StockCountScreenState extends ConsumerState<StockCountScreen> {
  final _materialController = TextEditingController();
  final _quantityController = TextEditingController();
  String _selectedWarehouse = 'WH-01';
  final List<_CountEntry> _entries = [];

  @override
  void dispose() {
    _materialController.dispose();
    _quantityController.dispose();
    super.dispose();
  }

  void _addEntry() {
    if (_materialController.text.isNotEmpty &&
        _quantityController.text.isNotEmpty) {
      setState(() {
        _entries.add(_CountEntry(
          materialNumber: _materialController.text.trim(),
          quantity: double.tryParse(_quantityController.text) ?? 0,
          warehouse: _selectedWarehouse,
          timestamp: DateTime.now(),
        ));
        _materialController.clear();
        _quantityController.clear();
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Stock Count'),
        actions: [
          if (_entries.isNotEmpty)
            TextButton.icon(
              onPressed: _submitCount,
              icon: const Icon(Icons.cloud_upload, color: AppColors.onPrimary),
              label: const Text(
                'Submit',
                style: TextStyle(color: AppColors.onPrimary),
              ),
            ),
        ],
      ),
      body: Column(
        children: [
          // Input form
          Card(
            margin: const EdgeInsets.all(AppSpacing.md),
            child: Padding(
              padding: const EdgeInsets.all(AppSpacing.md),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'Add Count Entry',
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(height: 12),
                  DropdownButtonFormField<String>(
                    initialValue: _selectedWarehouse,
                    decoration: const InputDecoration(
                      labelText: 'Warehouse',
                      prefixIcon: Icon(Icons.warehouse_outlined),
                    ),
                    items: const [
                      DropdownMenuItem(
                        value: 'WH-01',
                        child: Text('WH-01 Main Warehouse'),
                      ),
                      DropdownMenuItem(
                        value: 'WH-02',
                        child: Text('WH-02 Cold Storage'),
                      ),
                      DropdownMenuItem(
                        value: 'WH-03',
                        child: Text('WH-03 Distribution Center'),
                      ),
                    ],
                    onChanged: (value) {
                      setState(() {
                        _selectedWarehouse = value!;
                      });
                    },
                  ),
                  const SizedBox(height: 12),
                  TextField(
                    controller: _materialController,
                    decoration: const InputDecoration(
                      labelText: 'Material Number',
                      hintText: 'e.g. MAT-001',
                      prefixIcon: Icon(Icons.inventory_2_outlined),
                    ),
                  ),
                  const SizedBox(height: 12),
                  TextField(
                    controller: _quantityController,
                    decoration: const InputDecoration(
                      labelText: 'Quantity',
                      hintText: 'Enter counted quantity',
                      prefixIcon: Icon(Icons.numbers),
                    ),
                    keyboardType:
                        const TextInputType.numberWithOptions(decimal: true),
                  ),
                  const SizedBox(height: 12),
                  SizedBox(
                    width: double.infinity,
                    child: ElevatedButton.icon(
                      onPressed: _addEntry,
                      icon: const Icon(Icons.add),
                      label: const Text('Add Entry'),
                    ),
                  ),
                ],
              ),
            ),
          ),

          // Entries list
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: AppSpacing.md),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  'Entries (${_entries.length})',
                  style: const TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                if (_entries.isNotEmpty)
                  TextButton(
                    onPressed: () => setState(() => _entries.clear()),
                    child: const Text(
                      'Clear All',
                      style: TextStyle(color: AppColors.error),
                    ),
                  ),
              ],
            ),
          ),
          Expanded(
            child: _entries.isEmpty
                ? const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.playlist_add,
                          size: 48,
                          color: AppColors.textSecondary,
                        ),
                        SizedBox(height: 8),
                        Text(
                          'No entries yet',
                          style: TextStyle(color: AppColors.textSecondary),
                        ),
                        Text(
                          'Add materials above to start counting',
                          style: TextStyle(
                            fontSize: 12,
                            color: AppColors.textSecondary,
                          ),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: _entries.length,
                    padding:
                        const EdgeInsets.symmetric(horizontal: AppSpacing.md),
                    itemBuilder: (context, index) {
                      final entry = _entries[index];
                      return Card(
                        margin: const EdgeInsets.only(bottom: 8),
                        child: ListTile(
                          leading: CircleAvatar(
                            backgroundColor:
                                AppColors.success.withValues(alpha: 0.1),
                            child: const Icon(
                              Icons.check,
                              color: AppColors.success,
                            ),
                          ),
                          title: Text(
                            entry.materialNumber,
                            style: const TextStyle(
                                fontWeight: FontWeight.w500),
                          ),
                          subtitle: Text(
                            '${entry.warehouse} | Counted: ${entry.quantity}',
                            style: const TextStyle(fontSize: 12),
                          ),
                          trailing: IconButton(
                            icon: const Icon(Icons.delete_outline,
                                color: AppColors.error, size: 20),
                            onPressed: () {
                              setState(() => _entries.removeAt(index));
                            },
                          ),
                        ),
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }

  void _submitCount() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Submit Stock Count'),
        content: Text(
            'Submit ${_entries.length} count entries? This action cannot be undone.'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () {
              Navigator.pop(context);
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(
                  content: Text(
                      '${_entries.length} entries submitted successfully'),
                  backgroundColor: AppColors.success,
                ),
              );
              setState(() => _entries.clear());
            },
            child: const Text('Submit'),
          ),
        ],
      ),
    );
  }
}

class _CountEntry {
  final String materialNumber;
  final double quantity;
  final String warehouse;
  final DateTime timestamp;

  const _CountEntry({
    required this.materialNumber,
    required this.quantity,
    required this.warehouse,
    required this.timestamp,
  });
}
