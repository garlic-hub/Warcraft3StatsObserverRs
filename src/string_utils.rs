use std::borrow::Cow;
use std::fmt::{Debug, Display};

/// Fixed-size, optionally NULL-terminated UTF-8 string read from the observer
/// memory map.
///
/// The backing buffer is always `SIZE` bytes wide. Logical string content runs
/// from byte 0 up to (but excluding) the first `0x00` byte, or to the end of
/// the buffer if no NUL byte is present.
///
/// `PaddedString` has alignment 1, so it can safely be borrowed by reference
/// even when nested inside a `#[repr(C, packed)]` struct.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaddedString<const SIZE: usize> {
    array: [u8; SIZE],
}

impl<const SIZE: usize> PaddedString<SIZE> {
    /// Returns the bytes up to (but excluding) the first NUL byte.
    ///
    /// If the buffer contains no NUL byte, all `SIZE` bytes are returned.
    pub fn as_bytes(&self) -> &[u8] {
        &self.array[..self.len()]
    }

    /// Lossy UTF-8 view of [`Self::as_bytes`].
    ///
    /// Invalid UTF-8 sequences are replaced with `U+FFFD`.
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.as_bytes())
    }

    /// Number of logical bytes in the string — the position of the first NULL
    /// byte, or `SIZE` if the buffer is not NULL-terminated.
    pub fn len(&self) -> usize {
        self.array.iter().position(|&b| b == 0).unwrap_or(SIZE)
    }

    /// Returns `true` if the first byte is NUL (i.e. the logical length is 0).
    pub fn is_empty(&self) -> bool {
        self.array.first() == Some(&0)
    }
}

impl<const SIZE: usize> Display for PaddedString<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

impl<const SIZE: usize> Debug for PaddedString<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.to_string_lossy(), f)
    }
}

impl<const SIZE: usize> AsRef<[u8]> for PaddedString<SIZE> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
