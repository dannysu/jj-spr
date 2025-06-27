/*
 * Utilities for handling revision parameters and ranges
 */

use crate::error::{Error, Result};

/// Parse revision parameter and determine if it should be treated as a range
/// Returns (use_range_mode, base_rev, target_rev)
pub fn parse_revision_and_range(
    revision_opt: Option<&str>,
    all_mode: bool,
    base_opt: Option<&str>,
) -> Result<(bool, String, String)> {
    let revision = revision_opt.unwrap_or("@-");

    if revision.contains("..") {
        // Range specified in revision (e.g., "main..@") - this overrides --all mode
        let parts: Vec<&str> = revision.split("..").collect();
        if parts.len() == 2 {
            Ok((true, parts[0].to_string(), parts[1].to_string()))
        } else {
            Err(Error::new(format!(
                "Invalid revision range format: {}. Use 'base..target' format",
                revision
            )))
        }
    } else if all_mode {
        // Explicit --all mode
        let base = base_opt.unwrap_or("trunk()");
        Ok((true, base.to_string(), revision.to_string()))
    } else {
        // Single revision
        Ok((false, String::new(), revision.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_revision_is_at_minus() {
        // Test that when no revision is specified, it defaults to "@-"
        let (use_range_mode, base_rev, target_rev) =
            parse_revision_and_range(None, false, None).unwrap();

        assert!(!use_range_mode);
        assert_eq!(base_rev, "");
        assert_eq!(target_rev, "@-");
    }

    #[test]
    fn test_explicit_revision_overrides_default() {
        // Test that when a revision is explicitly specified, it overrides the default
        let (use_range_mode, base_rev, target_rev) =
            parse_revision_and_range(Some("@"), false, None).unwrap();

        assert!(!use_range_mode);
        assert_eq!(base_rev, "");
        assert_eq!(target_rev, "@");
    }

    #[test]
    fn test_range_revision_detection() {
        // Test that range revision syntax is correctly detected
        let (use_range_mode, base_rev, target_rev) =
            parse_revision_and_range(Some("main..@"), false, None).unwrap();

        assert!(use_range_mode);
        assert_eq!(base_rev, "main");
        assert_eq!(target_rev, "@");
    }

    #[test]
    fn test_all_mode_with_default_revision() {
        // Test that --all mode works with default revision
        let (use_range_mode, base_rev, target_rev) =
            parse_revision_and_range(None, true, None).unwrap();

        assert!(use_range_mode);
        assert_eq!(base_rev, "trunk()");
        assert_eq!(target_rev, "@-");
    }

    #[test]
    fn test_all_mode_with_custom_base() {
        // Test that --all mode works with custom base
        let (use_range_mode, base_rev, target_rev) =
            parse_revision_and_range(None, true, Some("main")).unwrap();

        assert!(use_range_mode);
        assert_eq!(base_rev, "main");
        assert_eq!(target_rev, "@-");
    }

    #[test]
    fn test_invalid_range_format() {
        // Test that invalid range format produces an error
        let result = parse_revision_and_range(Some("invalid..range..format"), false, None);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid revision range format"));
    }

    #[test]
    fn test_range_overrides_all_mode() {
        // Test that when both range syntax and --all are specified, range takes precedence
        let (use_range_mode, base_rev, target_rev) =
            parse_revision_and_range(Some("feature..@"), true, Some("trunk()")).unwrap();

        assert!(use_range_mode);
        assert_eq!(base_rev, "feature");
        assert_eq!(target_rev, "@");
    }
}
