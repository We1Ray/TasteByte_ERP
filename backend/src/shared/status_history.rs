use uuid::Uuid;

use super::status::DocumentType;
use super::AppError;

impl DocumentType {
    pub fn db_key(&self) -> &'static str {
        match self {
            DocumentType::SalesOrder => "SALES_ORDER",
            DocumentType::PurchaseOrder => "PURCHASE_ORDER",
            DocumentType::ProductionOrder => "PRODUCTION_ORDER",
            DocumentType::JournalEntry => "JOURNAL_ENTRY",
            DocumentType::Delivery => "DELIVERY",
            DocumentType::Invoice => "INVOICE",
        }
    }
}

pub async fn record_transition(
    executor: impl sqlx::PgExecutor<'_>,
    doc_type: &DocumentType,
    doc_id: Uuid,
    from: Option<&str>,
    to: &str,
    user_id: Uuid,
    reason: Option<&str>,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO document_status_history (document_type, document_id, from_status, to_status, changed_by, change_reason) \
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(doc_type.db_key())
    .bind(doc_id)
    .bind(from)
    .bind(to)
    .bind(user_id)
    .bind(reason)
    .execute(executor)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_key_sales_order() {
        assert_eq!(DocumentType::SalesOrder.db_key(), "SALES_ORDER");
    }

    #[test]
    fn db_key_purchase_order() {
        assert_eq!(DocumentType::PurchaseOrder.db_key(), "PURCHASE_ORDER");
    }

    #[test]
    fn db_key_production_order() {
        assert_eq!(DocumentType::ProductionOrder.db_key(), "PRODUCTION_ORDER");
    }

    #[test]
    fn db_key_journal_entry() {
        assert_eq!(DocumentType::JournalEntry.db_key(), "JOURNAL_ENTRY");
    }

    #[test]
    fn db_key_delivery() {
        assert_eq!(DocumentType::Delivery.db_key(), "DELIVERY");
    }

    #[test]
    fn db_key_invoice() {
        assert_eq!(DocumentType::Invoice.db_key(), "INVOICE");
    }
}
