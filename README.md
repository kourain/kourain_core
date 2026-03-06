# Kourain Core Extensions

This module provides useful trait extensions for common data types and operations in Rust.

## Overview

The `extensions` module includes:
- **Bitwise**: Trait for bit manipulation operations
- **String**: Trait for advanced string operations with Vietnamese character support

---

## Bitwise Extension

### Description
The `Bitwise` trait provides convenient methods for manipulating individual bits in numeric types and byte collections.

### Trait Methods

#### `is_bit_active(bit_position: u32) -> bool`
Checks if a bit at the specified position is active (set to 1).

**Parameters:**
- `bit_position`: The position of the bit to check (0-indexed from the right)

**Returns:** `true` if the bit is active, `false` otherwise

**Panics:** If `bit_position` exceeds the number of bits available in the type

**Example:**
```rust
let num: u8 = 0b1010_0101; // 165 in decimal
assert!(num.is_bit_active(0));     // true
assert!(!num.is_bit_active(1));    // false
```

#### `active_bit(bit_position: u32) -> bool`
Activates a bit at the specified position (sets it to 1).

**Parameters:**
- `bit_position`: The position of the bit to activate

**Returns:** `true` if the bit was successfully activated (was previously inactive), `false` if it was already active

**Panics:** If `bit_position` exceeds the number of bits available in the type

**Example:**
```rust
let mut num: u8 = 0b1010_0101;
assert!(num.active_bit(1));     // true (bit was activated)
assert!(!num.active_bit(1));    // false (bit is already active)
assert_eq!(num, 0b1010_0111);   // 167 in decimal
```

#### `deactive_bit(bit_position: u32) -> bool`
Deactivates a bit at the specified position (sets it to 0).

**Parameters:**
- `bit_position`: The position of the bit to deactivate

**Returns:** `true` if the bit was successfully deactivated (was previously active), `false` if it was already inactive

**Panics:** If `bit_position` exceeds the number of bits available in the type

**Example:**
```rust
let mut num: u8 = 0b1010_0101;
assert!(num.deactive_bit(0));    // true (bit was deactivated)
assert!(!num.deactive_bit(0));   // false (bit is already inactive)
assert_eq!(num, 0b1010_0100);    // 164 in decimal
```

### Supported Types

The `Bitwise` trait is implemented for:
- Integer types: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`
- Byte slices: `[u8]`
- Byte vectors: `Vec<u8>`

### Usage Example

```rust
use kourain_core::extensions::Bitwise;

// Integer types
let mut value: u32 = 0;
value.active_bit(5);          // Activate bit 5
assert!(value.is_bit_active(5));

// Byte slices
let mut bytes = vec![0u8; 2];
bytes.active_bit(10);         // Activate bit 10 across the byte array
assert!(bytes.is_bit_active(10));
```

---

## String Extension (ToSlug)

### Description
The `ToSlug` trait provides methods for converting strings to URL-friendly slugs with comprehensive Vietnamese character support.

### Trait Methods

#### `to_slug() -> String`
Converts a string to a slug using default settings.

**Default behavior:**
- Converts to lowercase
- Removes Vietnamese diacritics (á, à, ả, ạ, ã, etc.)
- Replaces whitespace with hyphens
- Removes special characters

**Returns:** A URL-friendly slug string

**Example:**
```rust
assert_eq!("Xin Chào Thế Giới".to_slug(), "xin-chao-the-gioi");
```

#### `to_slug_with_spliter(spliter: char) -> String`
Converts a string to a slug with a custom separator character instead of the default hyphen.

**Parameters:**
- `spliter`: The character to use as a word separator

**Example:**
```rust
assert_eq!("Hello World".to_slug_with_spliter('_'), "hello_world");
```

#### `to_slug_with_cv_spec(cv_spec: bool) -> String`
Converts a string to a slug with the option to handle special characters.

**Parameters:**
- `cv_spec`: If `true`, special characters are converted to underscores instead of being removed

**Example:**
```rust
// With cv_spec = false (default)
assert_eq!("Hello@World!".to_slug_with_cv_spec(false), "helloworld");

// With cv_spec = true
assert_eq!("Hello@World!".to_slug_with_cv_spec(true), "hello_world_");
```

#### `to_slug_full(spliter: char, cv_spec: bool) -> String`
Converts a string to a slug with full customization of both separator and special character handling.

**Parameters:**
- `spliter`: The character to use as a word separator
- `cv_spec`: If `true`, special characters are converted to underscores

**Example:**
```rust
assert_eq!(
    "Xin Chào Thế Giới@2024!".to_slug_full('_', true),
    "xin_chao_the_gioi_2024_"
);
```

#### `sub_string(start: usize, size: usize) -> String`
Extracts a substring starting at the given position with the specified length.

**Parameters:**
- `start`: The starting character index (0-indexed)
- `size`: The number of characters to extract

**Returns:** A new string containing the extracted characters

**Note:** This method handles UTF-8 characters correctly, unlike byte-based slicing

**Example:**
```rust
let text = "Xin Chào";
assert_eq!(text.sub_string(0, 3), "Xin");
assert_eq!(text.sub_string(4, 4), "Chào");
```

### Vietnamese Character Support

The `ToSlug` trait includes comprehensive support for Vietnamese diacritical marks:

| Character | Converted to |
|-----------|-------------|
| á, à, ả, ạ, ã, ă, ắ, ằ, ẳ, ẵ, ặ, â, ấ, ầ, ẩ, ẫ, ậ | a |
| é, è, ẻ, ẽ, ẹ, ê, ế, ề, ể, ễ, ệ | e |
| i, í, ì, ỉ, ĩ, ị | i |
| ó, ò, ỏ, õ, ọ, ô, ố, ồ, ổ, ỗ, ộ, ơ, ớ, ờ, ở, ỡ, ợ | o |
| ú, ù, ủ, ũ, ụ, ư, ứ, ừ, ử, ữ, ự | u |
| ý, ỳ, ỷ, ỹ, ỵ | y |
| đ | d |

### Supported Types

The `ToSlug` trait is implemented for:
- String references: `&str`
- Owned strings: `String`

### Usage Example

```rust
use kourain_core::extensions::ToSlug;

// Basic slug conversion
let title = "Bài Viết Về Lập Trình Rust";
assert_eq!(title.to_slug(), "bai-viet-ve-lap-trinh-rust");

// Custom separator
let hashtag = title.to_slug_with_spliter('_');
assert_eq!(hashtag, "bai_viet_ve_lap_trinh_rust");

// With special character handling
let description = "Hello! This is a test @ 2024.";
assert_eq!(
    description.to_slug_with_cv_spec(true),
    "hello_this_is_a_test_2024_"
);

// Extract substring
let name = "Kourain";
assert_eq!(name.sub_string(0, 3), "Kou");
```

---

## Dependencies

The extensions module uses:
- `regex`: For pattern matching and text transformation in the `ToSlug` trait

---

## Testing

Both extensions include comprehensive test suites:
- Bitwise tests: [tests/bitwise_test.rs](../../tests/bitwise_test.rs)
- String tests: [tests/string_test.rs](../../tests/string_test.rs)

Run tests with:
```bash
cargo test
```

---

## Performance Considerations

### Bitwise Extension
- **O(1) complexity** for all operations
- No allocations required
- Suitable for performance-critical code

### String Extension
- Uses compiled regex patterns for character replacements
- **O(n) complexity** where n is the string length
- Allocates new string for output
- Regex patterns are compiled during the function call, consider caching if used frequently in loops

---

