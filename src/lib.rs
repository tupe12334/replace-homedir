//! A Rust library to replace the user's home directory in a path with another string.
//!
//! This is a Rust alternative to the npm package [replace-homedir](https://www.npmjs.com/package/replace-homedir).
//!
//! # Example
//!
//! ```
//! use replace_homedir::replace_homedir;
//!
//! // Assuming home directory is /home/user
//! let path = "/home/user/projects/myapp";
//! let short_path = replace_homedir(path, "~");
//! // short_path might be "~/projects/myapp" if home dir matches
//! ```

use std::path::Path;

/// Replaces the user's home directory in a path with the given replacement string.
///
/// If the path starts with the user's home directory, that portion is replaced
/// with the replacement string. Otherwise, the original path is returned unchanged.
///
/// # Arguments
///
/// * `path` - The path to process
/// * `replacement` - The string to replace the home directory with (e.g., "~")
///
/// # Returns
///
/// A new string with the home directory replaced, or the original path if it
/// doesn't start with the home directory.
///
/// # Example
///
/// ```
/// use replace_homedir::replace_homedir;
///
/// // Replace home directory with ~
/// let result = replace_homedir("/home/user/documents", "~");
/// ```
pub fn replace_homedir(path: &str, replacement: &str) -> String {
    let Some(home_dir) = dirs::home_dir() else {
        return path.to_string();
    };

    let path_obj = Path::new(path);

    // Check if the path starts with the home directory
    if let Ok(stripped) = path_obj.strip_prefix(&home_dir) {
        let remainder = stripped.to_string_lossy();
        if remainder.is_empty() {
            replacement.to_string()
        } else {
            format!("{}/{}", replacement, remainder)
        }
    } else {
        path.to_string()
    }
}

/// Replaces the user's home directory in a path, using a closure for the replacement.
///
/// This is useful when you need dynamic replacement logic.
///
/// # Arguments
///
/// * `path` - The path to process
/// * `replacement_fn` - A closure that receives the home directory and returns the replacement
///
/// # Example
///
/// ```
/// use replace_homedir::replace_homedir_with;
///
/// let result = replace_homedir_with("/home/user/docs", |_home| "~".to_string());
/// ```
pub fn replace_homedir_with<F>(path: &str, replacement_fn: F) -> String
where
    F: FnOnce(&Path) -> String,
{
    let Some(home_dir) = dirs::home_dir() else {
        return path.to_string();
    };

    let path_obj = Path::new(path);

    if let Ok(stripped) = path_obj.strip_prefix(&home_dir) {
        let replacement = replacement_fn(&home_dir);
        let remainder = stripped.to_string_lossy();
        if remainder.is_empty() {
            replacement
        } else {
            format!("{}/{}", replacement, remainder)
        }
    } else {
        path.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_homedir_with_home_path() {
        let home = dirs::home_dir().expect("Should have home dir");
        let test_path = format!("{}/projects/myapp", home.display());
        let result = replace_homedir(&test_path, "~");
        assert_eq!(result, "~/projects/myapp");
    }

    #[test]
    fn test_replace_homedir_exact_home() {
        let home = dirs::home_dir().expect("Should have home dir");
        let test_path = home.to_string_lossy().to_string();
        let result = replace_homedir(&test_path, "~");
        assert_eq!(result, "~");
    }

    #[test]
    fn test_replace_homedir_non_home_path() {
        let result = replace_homedir("/tmp/something", "~");
        assert_eq!(result, "/tmp/something");
    }

    #[test]
    fn test_replace_homedir_empty_path() {
        let result = replace_homedir("", "~");
        assert_eq!(result, "");
    }

    #[test]
    fn test_replace_homedir_relative_path() {
        let result = replace_homedir("relative/path", "~");
        assert_eq!(result, "relative/path");
    }

    #[test]
    fn test_replace_homedir_with_closure() {
        let home = dirs::home_dir().expect("Should have home dir");
        let test_path = format!("{}/docs", home.display());
        let result = replace_homedir_with(&test_path, |_| "HOME".to_string());
        assert_eq!(result, "HOME/docs");
    }

    #[test]
    fn test_replace_homedir_nested_path() {
        let home = dirs::home_dir().expect("Should have home dir");
        let test_path = format!("{}/a/b/c/d", home.display());
        let result = replace_homedir(&test_path, "~");
        assert_eq!(result, "~/a/b/c/d");
    }
}
