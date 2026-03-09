use sqlx::PgPool;

use crate::shared::AppError;

/// Format a number range result into a padded document number string.
pub fn format_number(prefix: &str, current_number: i64, pad_length: i32) -> String {
    let num_str = format!("{:0>width$}", current_number, width = pad_length as usize);
    format!("{}{}", prefix, num_str)
}

pub async fn next_number(pool: &PgPool, object_type: &str) -> Result<String, AppError> {
    let row: (String, i64, i32) = sqlx::query_as(
        "UPDATE number_ranges SET current_number = current_number + 1 WHERE object_type = $1 RETURNING prefix, current_number, pad_length"
    )
    .bind(object_type)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::Internal(format!("Number range not found for {}", object_type)))?;

    Ok(format_number(&row.0, row.1, row.2))
}

pub async fn next_number_on_conn(
    conn: &mut sqlx::PgConnection,
    object_type: &str,
) -> Result<String, AppError> {
    let row: (String, i64, i32) = sqlx::query_as(
        "UPDATE number_ranges SET current_number = current_number + 1 WHERE object_type = $1 RETURNING prefix, current_number, pad_length"
    )
    .bind(object_type)
    .fetch_optional(&mut *conn)
    .await?
    .ok_or_else(|| AppError::Internal(format!("Number range not found for {}", object_type)))?;

    Ok(format_number(&row.0, row.1, row.2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_material_number() {
        assert_eq!(format_number("MAT", 1, 10), "MAT0000000001");
    }

    #[test]
    fn format_po_number() {
        assert_eq!(format_number("PO", 42, 8), "PO00000042");
    }

    #[test]
    fn format_so_number() {
        assert_eq!(format_number("SO", 100, 8), "SO00000100");
    }

    #[test]
    fn format_large_number_no_padding() {
        assert_eq!(format_number("X", 12345678, 8), "X12345678");
    }

    #[test]
    fn format_number_exceeds_pad_length() {
        // If the number exceeds pad length, it should still show the full number
        assert_eq!(format_number("N", 123456789, 5), "N123456789");
    }

    #[test]
    fn format_empty_prefix() {
        assert_eq!(format_number("", 7, 4), "0007");
    }

    #[test]
    fn format_customer_number() {
        assert_eq!(format_number("CUST", 1, 10), "CUST0000000001");
    }

    #[test]
    fn format_vendor_number() {
        assert_eq!(format_number("VEND", 99, 8), "VEND00000099");
    }
}
