//!
//! # Examples
//!
//! ```
//! extern crate unidiff;
//!
//! use unidiff::PatchSet;
//!
//! fn main() {
//!     let diff_str = "diff --git a/added_file b/added_file
//! new file mode 100644
//! index 0000000..9b710f3
//! --- /dev/null
//! +++ b/added_file
//! @@ -0,0 +1,4 @@
//! +This was missing!
//! +Adding it now.
//! +
//! +Only for testing purposes.";
//!     let mut patch = PatchSet::new();
//!     patch.parse(diff_str).ok().expect("Error parsing diff");
//! }
//! ```
///
/// You can iterate over it to get ``Line``s.
///
/// You can iterate over it to get ``Hunk``s.
    /// Initialize a new PatchedFile instance
    /// Initialize a new PatchedFile instance with hunks
///
/// You can iterate over it to get ``PatchedFile``s.
///
/// ```ignore
/// let mut patch = PatchSet::new();
/// patch.parse("some diff");
/// for patched_file in patch {
///   // do something with patched_file
///   for hunk in patched_file {
///       // do something with hunk
///       for line in hunk {
///           // do something with line
///       }
///   }
/// }
/// ```
    /// Initialize a new PatchSet instance
    /// Initialize a new PatchedSet instance with encoding
    /// Initialize a new PatchedSet instance with encoding(string form)
    /// Parse diff from bytes
    /// Parse diff from string