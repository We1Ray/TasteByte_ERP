-- FK indexes (PostgreSQL does NOT auto-create indexes for foreign keys)
CREATE INDEX IF NOT EXISTS idx_mm_po_items_po_id ON mm_purchase_order_items(purchase_order_id);
CREATE INDEX IF NOT EXISTS idx_mm_po_items_material ON mm_purchase_order_items(material_id);
CREATE INDEX IF NOT EXISTS idx_mm_movements_material ON mm_material_movements(material_id);
CREATE INDEX IF NOT EXISTS idx_mm_movements_warehouse ON mm_material_movements(warehouse_id);
CREATE INDEX IF NOT EXISTS idx_mm_stock_material ON mm_plant_stock(material_id);
CREATE INDEX IF NOT EXISTS idx_mm_stock_warehouse ON mm_plant_stock(warehouse_id);

CREATE INDEX IF NOT EXISTS idx_sd_so_items_order ON sd_sales_order_items(sales_order_id);
CREATE INDEX IF NOT EXISTS idx_sd_so_items_material ON sd_sales_order_items(material_id);
CREATE INDEX IF NOT EXISTS idx_sd_deliveries_order ON sd_deliveries(sales_order_id);
CREATE INDEX IF NOT EXISTS idx_sd_invoices_order ON sd_invoices(sales_order_id);

CREATE INDEX IF NOT EXISTS idx_pp_orders_material ON pp_production_orders(material_id);
CREATE INDEX IF NOT EXISTS idx_pp_bom_items_bom ON pp_bom_items(bom_id);
CREATE INDEX IF NOT EXISTS idx_pp_bom_items_material ON pp_bom_items(component_material_id);

CREATE INDEX IF NOT EXISTS idx_fi_je_items_entry ON fi_journal_items(journal_entry_id);
CREATE INDEX IF NOT EXISTS idx_fi_je_items_account ON fi_journal_items(account_id);

CREATE INDEX IF NOT EXISTS idx_qm_lots_material ON qm_inspection_lots(material_id);
CREATE INDEX IF NOT EXISTS idx_qm_results_lot ON qm_inspection_results(inspection_lot_id);

-- Composite query indexes
CREATE INDEX IF NOT EXISTS idx_audit_entity ON audit_log(table_name, record_id);
CREATE INDEX IF NOT EXISTS idx_status_history_doc ON document_status_history(document_type, document_id);
