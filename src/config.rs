//! Configurable properties for brute forcing.
//!
//! All these constants must be set to correct values for your brute force attempt, before you
//! start using the tool.

/// What type of code we want to crack. Should likely be a pattern.
///
/// Any of:
/// - `crate::Code::Pattern`
/// - `crate::Code::Pin`
pub const CODE_TYPE: crate::Code = crate::Code::Pin;

/// Pattern:
/// The dots we should generate patterns on for brute forcing.
///
/// The indices are row-by-row based, for example:
///
/// ```ignore
/// 0 1 2    00 01 02 03    00 01 02 03 04
/// 3 4 5    04 05 06 07    05 06 07 08 09
/// 6 7 8    08 09 10 11    10 11 12 13 14
///          12 13 14 15    15 16 17 18 19
///                         20 21 22 23 24
/// ```
///
/// You should make the list of dots as small as possible to greatly improve brute force speed.
/// Here are some examples:
///
/// ```rust
/// // Full 3x3 grid
/// pub const DOTS: [u16; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
///
/// // Full 4x4 grid
/// pub const DOTS: [u16; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
///
/// // Full 5x5 grid
/// pub const DOTS: [u16; 25] = [
///     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
///     14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
/// ];
///
/// // Only right two columns in 3x3 grid
/// pub const DOTS: [u16; 6] = [1, 2, 4, 5, 7, 8];
///
/// // Only top two rows in 4x4 grid
/// pub const DOTS: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
///
/// // Only 3x3 top-left dots in 4x4 grid
/// pub const DOTS: [u16; 9] = [0, 1, 2, 4, 5, 6, 8, 9, 10];
///
/// // Only 9 center dots in 5x5 pattern
/// pub const DOTS: [u16; 9] = [6, 7, 8, 11, 12, 13, 16, 17, 18];
/// ```
pub const DOTS: [u16; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

/// Pattern:
/// The size of the pattern grid, probably 3, 4 or 5.
pub const GRID_SIZE: u16 = 3;

/// Pattern:
/// The minimum length of patterns to attempt.
pub const PATTERN_LEN_MIN: u16 = 4;

/// Pattern:
/// The maximum length of patterns to attempt.
pub const PATTERN_LEN_MAX: u16 = 5;

/// Pattern:
/// The maximum distance between dots in a pattern, must be `>= 1`.
///
/// A value of 1 means connected dots are always next to each other (may be diagonal).
///
/// Imagine the following distances for each dot, where `X` is the center:
///
/// ```
/// 2 2 2 2 2
/// 2 1 1 1 2
/// 2 1 X 1 2
/// 2 1 1 1 2
/// 2 2 2 2 2
/// ```
pub const PATTERN_DISTANCE_MAX: u16 = 1;

/// The number of milliseconds to wait after each attempt.
///
/// This should be >=10000 because TWRP has a decrypt attempt timeout of about 10 seconds, any new
/// attempts within that time frame fail with no warning.
pub const ATTEMPT_TIMEOUT: u64 = 10_500;
