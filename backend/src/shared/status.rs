use crate::shared::AppError;

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
        ),
        DocumentType::PurchaseOrder => matches!(
            (from, to),
            ("DRAFT", "RELEASED")
                | ("RELEASED", "PARTIALLY_RECEIVED")
                | ("RELEASED", "RECEIVED")
                | ("PARTIALLY_RECEIVED", "RECEIVED")
                | ("RECEIVED", "CLOSED")
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
}
