#[cfg(test)]
mod tests {
    use crate::models::{NewNoteInput, UpdateNoteInput, PaginationParams};
    use validator::Validate;

    #[test]
    fn test_valid_note_input() {
        let input = NewNoteInput {
            content: "Valid note content".to_string(),
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn test_empty_note_input() {
        let input = NewNoteInput {
            content: String::new(),
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_note_input_too_long() {
        let input = NewNoteInput {
            content: "x".repeat(5001),
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_note_input_at_max_length() {
        let input = NewNoteInput {
            content: "x".repeat(5000),
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn test_update_input_validation() {
        let input = UpdateNoteInput {
            content: "Updated content".to_string(),
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn test_update_input_empty() {
        let input = UpdateNoteInput {
            content: String::new(),
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_update_input_too_long() {
        let input = UpdateNoteInput {
            content: "x".repeat(5001),
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_pagination_params_defaults() {
        let params: PaginationParams = serde_json::from_str("{}").unwrap();
        assert_eq!(params.limit, 20);
        assert_eq!(params.offset, 0);
    }

    #[test]
    fn test_pagination_params_custom() {
        let params: PaginationParams =
            serde_json::from_str(r#"{"limit": 50, "offset": 100}"#).unwrap();
        assert_eq!(params.limit, 50);
        assert_eq!(params.offset, 100);
    }

    #[test]
    fn test_pagination_params_partial() {
        let params: PaginationParams =
            serde_json::from_str(r#"{"limit": 10}"#).unwrap();
        assert_eq!(params.limit, 10);
        assert_eq!(params.offset, 0);
    }
}
