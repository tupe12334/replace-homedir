# replace-homedir

Replace user home directory in a path with another string. Useful for tildifying a path.

This is a Rust alternative to the npm package [replace-homedir](https://www.npmjs.com/package/replace-homedir).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
replace-homedir = "0.1"
```

## Usage

```rust
use replace_homedir::replace_homedir;

let short_path = replace_homedir("/Users/phated/myProject", "~");
// short_path == "~/myProject"
```

## API

### `replace_homedir(path, replacement) -> String`

Takes a path string and a replacement string. If the path is absolute and begins with the user's home directory, the home directory portion is replaced with the replacement string.

#### Parameters

- `path` (`&str`): The file path to process
- `replacement` (`&str`): The string to substitute for the home directory

#### Returns

A `String` with the home directory replaced, or the original path if it doesn't start with the home directory.

#### Example

```rust
use replace_homedir::replace_homedir;

// Assuming home directory is /home/user
let result = replace_homedir("/home/user/projects/myapp", "~");
assert_eq!(result, "~/projects/myapp");

// Non-home paths are returned unchanged
let result = replace_homedir("/tmp/something", "~");
assert_eq!(result, "/tmp/something");
```

### `replace_homedir_with(path, replacement_fn) -> String`

Takes a path string and a closure for dynamic replacement logic. The closure receives the home directory as a `&Path` and returns the replacement string.

#### Parameters

- `path` (`&str`): The file path to process
- `replacement_fn` (`FnOnce(&Path) -> String`): A closure that receives the home directory and returns the replacement

#### Example

```rust
use replace_homedir::replace_homedir_with;

let result = replace_homedir_with("/home/user/docs", |_home| "HOME".to_string());
// result == "HOME/docs"
```

## License

MIT
