use crate::shared::error::AppError;

/// Check whether a document of the given type can have its child items modified
/// at the current status. The `doc_type` parameter is a plain string identifier
/// (e.g. "PURCHASE_ORDER", "BOM") so callers do not need to convert into an enum.
pub fn is_mutable_status(doc_type: &str, status: &str) -> bool {
    match doc_type {
        "PURCHASE_ORDER" => status == "DRAFT",
        "SALES_ORDER" => status == "DRAFT",
        "JOURNAL_ENTRY" => status == "DRAFT",
        "GOODS_RECEIPT" => status == "DRAFT",
        "DELIVERY" => status == "CREATED",
        "BOM" | "ROUTING" => true, // master data — always mutable
        "STOCK_COUNT" => status == "PLANNED",
        "INSPECTION_LOT" => status == "CREATED",
        "PAYROLL_RUN" => status == "DRAFT",
        _ => false,
    }
}

/// Convenience wrapper: returns `Ok(())` when child-item modifications are
/// allowed, or an `AppError::Validation` explaining why they are not.
pub fn ensure_mutable(doc_type: &str, status: &str) -> Result<(), AppError> {
    if is_mutable_status(doc_type, status) {
        Ok(())
    } else {
        Err(AppError::Validation(format!(
            "Cannot modify items: document status is {status}"
        )))
    }
}

/// Document types for status machine validation
pub enum DocumentType {
    SalesOrder,
    PurchaseOrder,
    ProductionOrder,
    JournalEntry,
    Delivery,
    Invoice,
}

/// Validate that a status transition is allowed for the given document type.
/// Returns Ok(()) if valid, Err with a message if invalid.
pub fn validate_transition(doc_type: DocumentType, from: &str, to: &str) -> Result<(), AppError> {
    let valid = match doc_type {
        DocumentType::SalesOrder => matches!(
            (from, to),
            ("DRAFT", "CONFIRMED")
                | ("CONFIRMED", "PARTIALLY_DELIVERED")
                | ("CONFIRMED", "DELIVERED")
                | ("PARTIALLY_DELIVERED", "DELIVERED")
                | ("DELIVERED", "INVOICED")
                | ("PARTIALLY_DELIVERED", "INVOICED")
                | ("INVOICED", "CLOSED")
                | ("DELIVERED", "CLOSED")
                | ("DRAFT", "CANCELLED")
                | ("CONFIRMED", "CANCELLED")
        ),
        DocumentType::PurchaseOrder => matches!(
            (from, to),
            ("DRAFT", "RELEASED")
                | ("RELEASED", "PARTIALLY_RECEIVED")
                | ("RELEASED", "RECEIVED")
                | ("PARTIALLY_RECEIVED", "RECEIVED")
                | ("RECEIVED", "CLOSED")
                | ("PARTIALLY_RECEIVED", "CLOSED")
                | ("DRAFT", "CANCELLED")
                | ("RELEASED", "CANCELLED")
        ),
        DocumentType::ProductionOrder => matches!(
            (from, to),
            ("CREATED", "RELEASED")
                | ("RELEASED", "IN_PROGRESS")
                | ("IN_PROGRESS", "COMPLETED")
                | ("COMPLETED", "CLOSED")
                | ("CREATED", "CANCELLED")
                | ("RELEASED", "CANCELLED")
        ),
        DocumentType::JournalEntry => matches!((from, to), ("DRAFT", "POSTED")),
        DocumentType::Delivery => matches!(
            (from, to),
            ("CREATED", "SHIPPED") | ("SHIPPED", "DELIVERED") | ("CREATED", "CANCELLED")
        ),
        DocumentType::Invoice => matches!(
            (from, to),
            ("CREATED", "POSTED") | ("POSTED", "PAID") | ("CREATED", "CANCELLED")
        ),
    };

    if valid {
        Ok(())
    } else {
        Err(AppError::Validation(format!(
            "Invalid status transition from '{}' to '{}'",
            from, to
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn so_draft_to_confirmed() {
        assert!(validate_transition(DocumentType::SalesOrder, "DRAFT", "CONFIRMED").is_ok());
    }

    #[test]
    fn so_invalid_draft_to_closed() {
        assert!(validate_transition(DocumentType::SalesOrder, "DRAFT", "CLOSED").is_err());
    }

    #[test]
    fn po_draft_to_released() {
        assert!(validate_transition(DocumentType::PurchaseOrder, "DRAFT", "RELEASED").is_ok());
    }

    #[test]
    fn po_released_to_cancelled() {
        assert!(validate_transition(DocumentType::PurchaseOrder, "RELEASED", "CANCELLED").is_ok());
    }

    #[test]
    fn prod_order_created_to_released() {
        assert!(validate_transition(DocumentType::ProductionOrder, "CREATED", "RELEASED").is_ok());
    }

    #[test]
    fn prod_order_invalid_completed_to_released() {
        assert!(
            validate_transition(DocumentType::ProductionOrder, "COMPLETED", "RELEASED").is_err()
        );
    }

    #[test]
    fn je_draft_to_posted() {
        assert!(validate_transition(DocumentType::JournalEntry, "DRAFT", "POSTED").is_ok());
    }

    #[test]
    fn delivery_shipped_to_delivered() {
        assert!(validate_transition(DocumentType::Delivery, "SHIPPED", "DELIVERED").is_ok());
    }

    // --- Additional coverage for all document types ---

    #[test]
    fn so_confirmed_to_delivered() {
        assert!(validate_transition(DocumentType::SalesOrder, "CONFIRMED", "DELIVERED").is_ok());
    }

    #[test]
    fn so_confirmed_to_partially_delivered() {
        assert!(
            validate_transition(DocumentType::SalesOrder, "CONFIRMED", "PARTIALLY_DELIVERED")
                .is_ok()
        );
    }

    #[test]
    fn so_partially_delivered_to_delivered() {
        assert!(
            validate_transition(DocumentType::SalesOrder, "PARTIALLY_DELIVERED", "DELIVERED")
                .is_ok()
        );
    }

    #[test]
    fn so_delivered_to_closed() {
        assert!(validate_transition(DocumentType::SalesOrder, "DELIVERED", "CLOSED").is_ok());
    }

    #[test]
    fn so_draft_to_cancelled() {
        assert!(validate_transition(DocumentType::SalesOrder, "DRAFT", "CANCELLED").is_ok());
    }

    #[test]
    fn so_invalid_confirmed_to_draft() {
        assert!(validate_transition(DocumentType::SalesOrder, "CONFIRMED", "DRAFT").is_err());
    }

    #[test]
    fn po_released_to_partially_received() {
        assert!(validate_transition(
            DocumentType::PurchaseOrder,
            "RELEASED",
            "PARTIALLY_RECEIVED"
        )
        .is_ok());
    }

    #[test]
    fn po_partially_received_to_received() {
        assert!(validate_transition(
            DocumentType::PurchaseOrder,
            "PARTIALLY_RECEIVED",
            "RECEIVED"
        )
        .is_ok());
    }

    #[test]
    fn po_received_to_closed() {
        assert!(validate_transition(DocumentType::PurchaseOrder, "RECEIVED", "CLOSED").is_ok());
    }

    #[test]
    fn po_draft_to_cancelled() {
        assert!(validate_transition(DocumentType::PurchaseOrder, "DRAFT", "CANCELLED").is_ok());
    }

    #[test]
    fn po_invalid_received_to_released() {
        assert!(validate_transition(DocumentType::PurchaseOrder, "RECEIVED", "RELEASED").is_err());
    }

    #[test]
    fn prod_released_to_in_progress() {
        assert!(
            validate_transition(DocumentType::ProductionOrder, "RELEASED", "IN_PROGRESS").is_ok()
        );
    }

    #[test]
    fn prod_in_progress_to_completed() {
        assert!(
            validate_transition(DocumentType::ProductionOrder, "IN_PROGRESS", "COMPLETED").is_ok()
        );
    }

    #[test]
    fn prod_completed_to_closed() {
        assert!(validate_transition(DocumentType::ProductionOrder, "COMPLETED", "CLOSED").is_ok());
    }

    #[test]
    fn prod_created_to_cancelled() {
        assert!(validate_transition(DocumentType::ProductionOrder, "CREATED", "CANCELLED").is_ok());
    }

    #[test]
    fn je_invalid_posted_to_draft() {
        assert!(validate_transition(DocumentType::JournalEntry, "POSTED", "DRAFT").is_err());
    }

    #[test]
    fn delivery_created_to_shipped() {
        assert!(validate_transition(DocumentType::Delivery, "CREATED", "SHIPPED").is_ok());
    }

    #[test]
    fn delivery_created_to_cancelled() {
        assert!(validate_transition(DocumentType::Delivery, "CREATED", "CANCELLED").is_ok());
    }

    #[test]
    fn delivery_invalid_shipped_to_created() {
        assert!(validate_transition(DocumentType::Delivery, "SHIPPED", "CREATED").is_err());
    }

    #[test]
    fn invoice_created_to_posted() {
        assert!(validate_transition(DocumentType::Invoice, "CREATED", "POSTED").is_ok());
    }

    #[test]
    fn invoice_posted_to_paid() {
        assert!(validate_transition(DocumentType::Invoice, "POSTED", "PAID").is_ok());
    }

    #[test]
    fn invoice_created_to_cancelled() {
        assert!(validate_transition(DocumentType::Invoice, "CREATED", "CANCELLED").is_ok());
    }

    #[test]
    fn invoice_invalid_paid_to_created() {
        assert!(validate_transition(DocumentType::Invoice, "PAID", "CREATED").is_err());
    }

    // --- GAP 10: SO INVOICED status transitions ---

    #[test]
    fn so_delivered_to_invoiced() {
        assert!(validate_transition(DocumentType::SalesOrder, "DELIVERED", "INVOICED").is_ok());
    }

    #[test]
    fn so_partially_delivered_to_invoiced() {
        assert!(
            validate_transition(DocumentType::SalesOrder, "PARTIALLY_DELIVERED", "INVOICED")
                .is_ok()
        );
    }

    #[test]
    fn so_invoiced_to_closed() {
        assert!(validate_transition(DocumentType::SalesOrder, "INVOICED", "CLOSED").is_ok());
    }

    // --- is_mutable_status / ensure_mutable tests ---

    #[test]
    fn po_draft_is_mutable() {
        assert!(is_mutable_status("PURCHASE_ORDER", "DRAFT"));
    }

    #[test]
    fn po_released_is_not_mutable() {
        assert!(!is_mutable_status("PURCHASE_ORDER", "RELEASED"));
    }

    #[test]
    fn so_draft_is_mutable() {
        assert!(is_mutable_status("SALES_ORDER", "DRAFT"));
    }

    #[test]
    fn so_confirmed_is_not_mutable() {
        assert!(!is_mutable_status("SALES_ORDER", "CONFIRMED"));
    }

    #[test]
    fn je_draft_is_mutable() {
        assert!(is_mutable_status("JOURNAL_ENTRY", "DRAFT"));
    }

    #[test]
    fn je_posted_is_not_mutable() {
        assert!(!is_mutable_status("JOURNAL_ENTRY", "POSTED"));
    }

    #[test]
    fn goods_receipt_draft_is_mutable() {
        assert!(is_mutable_status("GOODS_RECEIPT", "DRAFT"));
    }

    #[test]
    fn delivery_created_is_mutable() {
        assert!(is_mutable_status("DELIVERY", "CREATED"));
    }

    #[test]
    fn delivery_shipped_is_not_mutable() {
        assert!(!is_mutable_status("DELIVERY", "SHIPPED"));
    }

    #[test]
    fn bom_always_mutable() {
        assert!(is_mutable_status("BOM", "ACTIVE"));
        assert!(is_mutable_status("BOM", "ANYTHING"));
    }

    #[test]
    fn routing_always_mutable() {
        assert!(is_mutable_status("ROUTING", "ACTIVE"));
    }

    #[test]
    fn stock_count_planned_is_mutable() {
        assert!(is_mutable_status("STOCK_COUNT", "PLANNED"));
    }

    #[test]
    fn stock_count_in_progress_is_not_mutable() {
        assert!(!is_mutable_status("STOCK_COUNT", "IN_PROGRESS"));
    }

    #[test]
    fn inspection_lot_created_is_mutable() {
        assert!(is_mutable_status("INSPECTION_LOT", "CREATED"));
    }

    #[test]
    fn payroll_run_draft_is_mutable() {
        assert!(is_mutable_status("PAYROLL_RUN", "DRAFT"));
    }

    #[test]
    fn unknown_doc_type_is_not_mutable() {
        assert!(!is_mutable_status("UNKNOWN", "DRAFT"));
    }

    #[test]
    fn ensure_mutable_ok() {
        assert!(ensure_mutable("PURCHASE_ORDER", "DRAFT").is_ok());
    }

    #[test]
    fn ensure_mutable_err() {
        let err = ensure_mutable("PURCHASE_ORDER", "RELEASED").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Validation error: Cannot modify items: document status is RELEASED"
        );
    }
}
