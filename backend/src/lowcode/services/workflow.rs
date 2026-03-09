use crate::shared::AppError;

/// Release status state machine
/// DRAFT -> SUBMITTED -> TESTING -> APPROVED -> RELEASED
///                    -> REJECTED (can resubmit from REJECTED -> SUBMITTED)
pub fn validate_release_transition(from: &str, to: &str) -> Result<(), AppError> {
    let valid = matches!(
        (from, to),
        ("DRAFT", "SUBMITTED")
            | ("SUBMITTED", "TESTING")
            | ("TESTING", "APPROVED")
            | ("SUBMITTED", "APPROVED")
            | ("SUBMITTED", "REJECTED")
            | ("APPROVED", "RELEASED")
            | ("REJECTED", "SUBMITTED")
    );

    if valid {
        Ok(())
    } else {
        Err(AppError::Validation(format!(
            "Invalid release status transition from '{from}' to '{to}'"
        )))
    }
}

/// Feedback status state machine
pub fn validate_feedback_transition(from: &str, to: &str) -> Result<(), AppError> {
    let valid = matches!(
        (from, to),
        ("OPEN", "IN_PROGRESS")
            | ("IN_PROGRESS", "RESOLVED")
            | ("IN_PROGRESS", "OPEN")
            | ("RESOLVED", "CLOSED")
            | ("RESOLVED", "OPEN")
            | ("OPEN", "CLOSED")
    );

    if valid {
        Ok(())
    } else {
        Err(AppError::Validation(format!(
            "Invalid feedback status transition from '{from}' to '{to}'"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Release state machine tests ---

    #[test]
    fn release_draft_to_submitted() {
        assert!(validate_release_transition("DRAFT", "SUBMITTED").is_ok());
    }

    #[test]
    fn release_submitted_to_testing() {
        assert!(validate_release_transition("SUBMITTED", "TESTING").is_ok());
    }

    #[test]
    fn release_approved_to_released() {
        assert!(validate_release_transition("APPROVED", "RELEASED").is_ok());
    }

    #[test]
    fn release_rejected_to_submitted() {
        assert!(validate_release_transition("REJECTED", "SUBMITTED").is_ok());
    }

    #[test]
    fn release_invalid_draft_to_released() {
        assert!(validate_release_transition("DRAFT", "RELEASED").is_err());
    }

    // --- Feedback state machine tests ---

    #[test]
    fn feedback_open_to_in_progress() {
        assert!(validate_feedback_transition("OPEN", "IN_PROGRESS").is_ok());
    }

    #[test]
    fn feedback_resolved_to_closed() {
        assert!(validate_feedback_transition("RESOLVED", "CLOSED").is_ok());
    }

    #[test]
    fn feedback_invalid_closed_to_open() {
        assert!(validate_feedback_transition("CLOSED", "OPEN").is_err());
    }
}
