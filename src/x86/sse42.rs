#[cfg(test)]
use stdsimd_test::assert_instr;

use v128::*;
use x86::__m128i;

/// String contains unsigned 8-bit characters *(Default)*
pub const _SIDD_UBYTE_OPS: i8 = 0b00000000;
/// String contains unsigned 16-bit characters
pub const _SIDD_UWORD_OPS: i8 = 0b00000001;
/// String contains signed 8-bit characters
pub const _SIDD_SBYTE_OPS: i8 = 0b00000010;
/// String contains unsigned 16-bit characters
pub const _SIDD_SWORD_OPS: i8 = 0b00000011;

/// For each character in `a`, find if it is in `b` *(Default)*
pub const _SIDD_CMP_EQUAL_ANY: i8 = 0b00000000;
/// For each character in `a`, determine if `b[0] <= c <= b[1] or b[1] <= c <=
/// b[2]...`
pub const _SIDD_CMP_RANGES: i8 = 0b00000100;
/// The strings defined by `a` and `b` are equal
pub const _SIDD_CMP_EQUAL_EACH: i8 = 0b00001000;
/// Search for the defined substring in the target
pub const _SIDD_CMP_EQUAL_ORDERED: i8 = 0b00001100;

/// Do not negate results *(Default)*
pub const _SIDD_POSITIVE_POLARITY: i8 = 0b00000000;
/// Negate results
pub const _SIDD_NEGATIVE_POLARITY: i8 = 0b00010000;
/// Do not negate results before the end of the string
pub const _SIDD_MASKED_POSITIVE_POLARITY: i8 = 0b00100000;
/// Negate results only before the end of the string
pub const _SIDD_MASKED_NEGATIVE_POLARITY: i8 = 0b00110000;

/// **Index only**: return the least significant bit *(Default)*
pub const _SIDD_LEAST_SIGNIFICANT: i8 = 0b00000000;
/// **Index only**: return the most significant bit
pub const _SIDD_MOST_SIGNIFICANT: i8 = 0b01000000;

/// **Mask only**: return the bit mask
pub const _SIDD_BIT_MASK: i8 = 0b00000000;
/// **Mask only**: return the byte mask
pub const _SIDD_UNIT_MASK: i8 = 0b01000000;

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return the generated mask.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpistrm, imm8 = 0))]
pub unsafe fn _mm_cmpistrm(a: __m128i, b: __m128i, imm8: i8) -> u8x16 {
    macro_rules! call {
        ($imm8:expr) => { pcmpistrm128(a, b, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return the generated index. Similar to
/// [`_mm_cmpestri`] with the excception that [`_mm_cmpestri`] requires the
/// lengths of `a` and `b` to be explicitly specified.
///
/// # Control modes
///
/// The control specified by `imm8` may be one or more of the following.
///
/// ## Data size and signedness
///
///  - [`_SIDD_UBYTE_OPS`] - Default
///  - [`_SIDD_UWORD_OPS`]
///  - [`_SIDD_SBYTE_OPS`]
///  - [`_SIDD_SWORD_OPS`]
///
/// ## Comparison options
///  - [`_SIDD_CMP_EQUAL_ANY`] - Default
///  - [`_SIDD_CMP_RANGES`]
///  - [`_SIDD_CMP_EQUAL_EACH`]
///  - [`_SIDD_CMP_EQUAL_ORDERED`]
///
/// ## Result polarity
///  - [`_SIDD_POSITIVE_POLARITY`] - Default
///  - [`_SIDD_NEGATIVE_POLARITY`]
///
/// ## Bit returned
///  - [`_SIDD_LEAST_SIGNIFICANT`] - Default
///  - [`_SIDD_MOST_SIGNIFICANT`]
///
/// # Examples
///
/// Find a substring using [`_SIDD_CMP_EQUAL_ORDERED`]
///
/// ```
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// #
/// # #[macro_use] extern crate stdsimd;
/// #
/// # fn main() {
/// #     if cfg_feature_enabled!("sse4.2") {
/// #         #[target_feature = "+sse4.2"]
/// #         fn worker() {
///
/// use stdsimd::simd::u8x16;
/// use stdsimd::vendor::{__m128i, _mm_cmpistri, _SIDD_CMP_EQUAL_ORDERED};
///
/// let haystack = b"This is a long string of text data\r\n\tthat extends
/// multiple lines";
/// let needle = b"\r\n\t\0\0\0\0\0\0\0\0\0\0\0\0\0";
///
/// let a = __m128i::from(u8x16::load(needle, 0));
/// let hop = 16;
/// let mut indexes = Vec::new();
///
/// // Chunk the haystack into 16 byte chunks and find
/// // the first "\r\n\t" in the chunk.
/// for (i, chunk) in haystack.chunks(hop).enumerate() {
///     let b = __m128i::from(u8x16::load(chunk, 0));
///     let idx = unsafe {
///         _mm_cmpistri(a, b, _SIDD_CMP_EQUAL_ORDERED)
///     };
///     if idx != 16 {
///        indexes.push((idx as usize) + (i * hop));
///     }
/// }
/// assert_eq!(indexes, vec![34]);
/// #         }
/// #         worker();
/// #     }
/// # }
/// ```
///
/// The `_mm_cmpistri` intrinsic may also be used to find the existance of
/// one or more of a given set of characters in the haystack.
///
/// ```
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// #
/// # #[macro_use] extern crate stdsimd;
/// #
/// # fn main() {
/// #     if cfg_feature_enabled!("sse4.2") {
/// #         #[target_feature = "+sse4.2"]
/// #         fn worker() {
/// use stdsimd::simd::u8x16;
/// use stdsimd::vendor::{__m128i, _mm_cmpistri, _SIDD_CMP_EQUAL_ANY};
///
/// // Ensure your input is 16 byte aligned
/// let password = b"hunter2\0\0\0\0\0\0\0\0\0";
/// let special_chars = b"!@#$%^&*()[]:;<>";
///
/// // Load the input
/// let a = __m128i::from(u8x16::load(special_chars, 0));
/// let b = __m128i::from(u8x16::load(password, 0));
///
/// // Use _SIDD_CMP_EQUAL_ANY to find the index of any bytes in b
/// let idx = unsafe {
///     _mm_cmpistri(a, b, _SIDD_CMP_EQUAL_ANY)
/// };
///
/// if idx < 16 {
///     println!("Congrats! Your password contains a special character");
///     # panic!("{:?} does not contain a special character", password);
/// } else {
///     println!("Your password should contain a special character");
/// }
/// #         }
/// #         worker();
/// #     }
/// # }
/// ```
///
/// Find the index of the first character in the haystack that is within a
/// range of characters.
///
/// ```
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// #
/// # #[macro_use] extern crate stdsimd;
/// #
/// # fn main() {
/// #     if cfg_feature_enabled!("sse4.2") {
/// #         #[target_feature = "+sse4.2"]
/// #         fn worker() {
/// use stdsimd::simd::u8x16;
/// use stdsimd::vendor::{__m128i, _mm_cmpistri, _SIDD_CMP_RANGES};
/// # let b = __m128i::from(u8x16::load(b":;<=>?@[\\]^_`abc", 0));
///
/// // Specify the ranges of values to be searched for [A-Za-z0-9].
/// let a = __m128i::from(u8x16::load(b"AZaz09\0\0\0\0\0\0\0\0\0\0", 0));
///
/// // Use _SIDD_CMP_RANGES to find the index of first byte in ranges.
/// // Which in this case will be the first alpha numeric byte found
/// // in the string.
/// let idx = unsafe {
///     _mm_cmpistri(a, b, _SIDD_CMP_RANGES)
/// };
///
///
/// if idx < 16 {
///     println!("Found an alpha numeric character");
///     # assert_eq!(idx, 13);
/// } else {
///     println!("Did not find an alpha numeric character");
/// }
/// #         }
/// #         worker();
/// #     }
/// # }
/// ```
///
/// Working with 16-bit characters.
///
/// ```
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// #
/// # #[macro_use] extern crate stdsimd;
/// #
/// # fn main() {
/// #     if cfg_feature_enabled!("sse4.2") {
/// #         #[target_feature = "+sse4.2"]
/// #         fn worker() {
/// use stdsimd::simd::u16x8;
/// use stdsimd::vendor::{__m128i, _mm_cmpistri};
/// use stdsimd::vendor::{_SIDD_UWORD_OPS, _SIDD_CMP_EQUAL_EACH};
///
/// # let mut some_utf16_words = [0u16; 8];
/// # let mut more_utf16_words = [0u16; 8];
/// # '❤'.encode_utf16(&mut some_utf16_words);
/// # '𝕊'.encode_utf16(&mut more_utf16_words);
/// // Load the input
/// let a = __m128i::from(u16x8::load(&some_utf16_words, 0));
/// let b = __m128i::from(u16x8::load(&more_utf16_words, 0));
///
/// // Specify _SIDD_UWORD_OPS to compare words instead of bytes, and
/// // use _SIDD_CMP_EQUAL_EACH to compare the two strings.
/// let idx = unsafe {
///     _mm_cmpistri(a, b, _SIDD_UWORD_OPS | _SIDD_CMP_EQUAL_EACH)
/// };
///
/// if idx == 0 {
///     println!("16-bit unicode strings were equal!");
///     # panic!("Strings should not be equal!")
/// } else {
///     println!("16-bit unicode strings were not equal!");
/// }
/// #         }
/// #         worker();
/// #     }
/// # }
/// ```
///
/// [`_SIDD_UBYTE_OPS`]: constant._SIDD_UBYTE_OPS.html
/// [`_SIDD_UWORD_OPS`]: constant._SIDD_UWORD_OPS.html
/// [`_SIDD_SBYTE_OPS`]: constant._SIDD_SBYTE_OPS.html
/// [`_SIDD_SWORD_OPS`]: constant._SIDD_SWORD_OPS.html
/// [`_SIDD_CMP_EQUAL_ANY`]: constant._SIDD_CMP_EQUAL_ANY.html
/// [`_SIDD_CMP_RANGES`]: constant._SIDD_CMP_RANGES.html
/// [`_SIDD_CMP_EQUAL_EACH`]: constant._SIDD_CMP_EQUAL_EACH.html
/// [`_SIDD_CMP_EQUAL_ORDERED`]: constant._SIDD_CMP_EQUAL_ORDERED.html
/// [`_SIDD_POSITIVE_POLARITY`]: constant._SIDD_POSITIVE_POLARITY.html
/// [`_SIDD_NEGATIVE_POLARITY`]: constant._SIDD_NEGATIVE_POLARITY.html
/// [`_SIDD_LEAST_SIGNIFICANT`]: constant._SIDD_LEAST_SIGNIFICANT.html
/// [`_SIDD_MOST_SIGNIFICANT`]: constant._SIDD_MOST_SIGNIFICANT.html
/// [`_mm_cmpestri`]: fn._mm_cmpestri.html
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
pub unsafe fn _mm_cmpistri(a: __m128i, b: __m128i, imm8: i8) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpistri128(a, b, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return `1` if any character in `b` was null.
/// and `0` otherwise.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
pub unsafe fn _mm_cmpistrz(a: __m128i, b: __m128i, imm8: i8) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpistriz128(a, b, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return `1` if the resulting mask was non-zero,
/// and `0` otherwise.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
pub unsafe fn _mm_cmpistrc(a: __m128i, b: __m128i, imm8: i8) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpistric128(a, b, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and returns `1` if any character in `a` was null,
/// and `0` otherwise.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
pub unsafe fn _mm_cmpistrs(a: __m128i, b: __m128i, imm8: i8) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpistris128(a, b, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return bit `0` of the resulting bit mask.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
pub unsafe fn _mm_cmpistro(a: __m128i, b: __m128i, imm8: i8) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpistrio128(a, b, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings with implicit lengths in `a` and `b` using the
/// control in `imm8`, and return `1` if `b` did not contain a null
/// character and the resulting mask was zero, and `0` otherwise.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpistri, imm8 = 0))]
pub unsafe fn _mm_cmpistra(a: __m128i, b: __m128i, imm8: i8) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpistria128(a, b, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return the generated mask.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpestrm, imm8 = 0))]
pub unsafe fn _mm_cmpestrm(
    a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
) -> u8x16 {
    macro_rules! call {
        ($imm8:expr) => { pcmpestrm128(a, la, b, lb, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings `a` and `b` with lengths `la` and `lb` using the
/// control in `imm8`, and return the generated index. Similar to
/// [`_mm_cmpistri`] with the excception that [`_mm_cmpistri`] implicityly
/// determines the length of `a` and `b`.
///
/// # Control modes
///
/// The control specified by `imm8` may be one or more of the following.
///
/// ## Data size and signedness
///
///  - [`_SIDD_UBYTE_OPS`] - Default
///  - [`_SIDD_UWORD_OPS`]
///  - [`_SIDD_SBYTE_OPS`]
///  - [`_SIDD_SWORD_OPS`]
///
/// ## Comparison options
///  - [`_SIDD_CMP_EQUAL_ANY`] - Default
///  - [`_SIDD_CMP_RANGES`]
///  - [`_SIDD_CMP_EQUAL_EACH`]
///  - [`_SIDD_CMP_EQUAL_ORDERED`]
///
/// ## Result polarity
///  - [`_SIDD_POSITIVE_POLARITY`] - Default
///  - [`_SIDD_NEGATIVE_POLARITY`]
///
/// ## Bit returned
///  - [`_SIDD_LEAST_SIGNIFICANT`] - Default
///  - [`_SIDD_MOST_SIGNIFICANT`]
///
/// # Examples
///
/// ```
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// #
/// # #[macro_use] extern crate stdsimd;
/// #
/// # fn main() {
/// #     if cfg_feature_enabled!("sse4.2") {
/// #         #[target_feature = "+sse4.2"]
/// #         fn worker() {
///
/// use stdsimd::simd::u8x16;
/// use stdsimd::vendor::{__m128i, _mm_cmpestri, _SIDD_CMP_EQUAL_ORDERED};
///
/// // The string we want to find a substring in
/// let haystack = b"Split \r\n\t line  ";
///
/// // The string we want to search for with some
/// // extra bytes we do not want to search for.
/// let needle = b"\r\n\t ignore this ";
///
/// let a = __m128i::from(u8x16::load(needle, 0));
/// let b = __m128i::from(u8x16::load(haystack, 0));
///
/// // Note: We explicitly specify we only want to search `b` for the
/// // first 3 characters of a.
/// let idx = unsafe {
///     _mm_cmpestri(a, 3, b, 15, _SIDD_CMP_EQUAL_ORDERED)
/// };
///
/// assert_eq!(idx, 6);
/// #         }
/// #         worker();
/// #     }
/// # }
/// ```
///
/// [`_SIDD_UBYTE_OPS`]: constant._SIDD_UBYTE_OPS.html
/// [`_SIDD_UWORD_OPS`]: constant._SIDD_UWORD_OPS.html
/// [`_SIDD_SBYTE_OPS`]: constant._SIDD_SBYTE_OPS.html
/// [`_SIDD_SWORD_OPS`]: constant._SIDD_SWORD_OPS.html
/// [`_SIDD_CMP_EQUAL_ANY`]: constant._SIDD_CMP_EQUAL_ANY.html
/// [`_SIDD_CMP_RANGES`]: constant._SIDD_CMP_RANGES.html
/// [`_SIDD_CMP_EQUAL_EACH`]: constant._SIDD_CMP_EQUAL_EACH.html
/// [`_SIDD_CMP_EQUAL_ORDERED`]: constant._SIDD_CMP_EQUAL_ORDERED.html
/// [`_SIDD_POSITIVE_POLARITY`]: constant._SIDD_POSITIVE_POLARITY.html
/// [`_SIDD_NEGATIVE_POLARITY`]: constant._SIDD_NEGATIVE_POLARITY.html
/// [`_SIDD_LEAST_SIGNIFICANT`]: constant._SIDD_LEAST_SIGNIFICANT.html
/// [`_SIDD_MOST_SIGNIFICANT`]: constant._SIDD_MOST_SIGNIFICANT.html
/// [`_mm_cmpistri`]: fn._mm_cmpistri.html
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
pub unsafe fn _mm_cmpestri(
    a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpestri128(a, la, b, lb, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return `1` if any character in
/// `b` was null, and `0` otherwise.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
pub unsafe fn _mm_cmpestrz(
    a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpestriz128(a, la, b, lb, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return `1` if the resulting mask
/// was non-zero, and `0` otherwise.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
pub unsafe fn _mm_cmpestrc(
    a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpestric128(a, la, b, lb, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return `1` if any character in
/// a was null, and `0` otherwise.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
pub unsafe fn _mm_cmpestrs(
    a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpestris128(a, la, b, lb, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return bit `0` of the resulting
/// bit mask.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
pub unsafe fn _mm_cmpestro(
    a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpestrio128(a, la, b, lb, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Compare packed strings in `a` and `b` with lengths `la` and `lb`
/// using the control in `imm8`, and return `1` if `b` did not
/// contain a null character and the resulting mask was zero, and `0`
/// otherwise.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpestri, imm8 = 0))]
pub unsafe fn _mm_cmpestra(
    a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
) -> i32 {
    macro_rules! call {
        ($imm8:expr) => { pcmpestria128(a, la, b, lb, $imm8) }
    }
    constify_imm8!(imm8, call)
}

/// Starting with the initial value in `crc`, return the accumulated
/// CRC32 value for unsigned 8-bit integer `v`.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(crc32))]
pub unsafe fn _mm_crc32_u8(crc: u32, v: u8) -> u32 {
    crc32_32_8(crc, v)
}

/// Starting with the initial value in `crc`, return the accumulated
/// CRC32 value for unsigned 16-bit integer `v`.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(crc32))]
pub unsafe fn _mm_crc32_u16(crc: u32, v: u16) -> u32 {
    crc32_32_16(crc, v)
}

/// Starting with the initial value in `crc`, return the accumulated
/// CRC32 value for unsigned 32-bit integer `v`.
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(crc32))]
pub unsafe fn _mm_crc32_u32(crc: u32, v: u32) -> u32 {
    crc32_32_32(crc, v)
}

/// Starting with the initial value in `crc`, return the accumulated
/// CRC32 value for unsigned 64-bit integer `v`.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(crc32))]
pub unsafe fn _mm_crc32_u64(crc: u64, v: u64) -> u64 {
    crc32_64_64(crc, v)
}

/// Compare packed 64-bit integers in `a` and `b` for greater-than,
/// return the results.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[target_feature = "+sse4.2"]
#[cfg_attr(test, assert_instr(pcmpgtq))]
pub unsafe fn _mm_cmpgt_epi64(a: i64x2, b: i64x2) -> i64x2 {
    a.gt(b)
}

#[allow(improper_ctypes)]
extern "C" {
    // SSE 4.2 string and text comparison ops
    #[link_name = "llvm.x86.sse42.pcmpestrm128"]
    fn pcmpestrm128(
        a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
    ) -> u8x16;
    #[link_name = "llvm.x86.sse42.pcmpestri128"]
    fn pcmpestri128(a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8)
        -> i32;
    #[link_name = "llvm.x86.sse42.pcmpestriz128"]
    fn pcmpestriz128(
        a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
    ) -> i32;
    #[link_name = "llvm.x86.sse42.pcmpestric128"]
    fn pcmpestric128(
        a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
    ) -> i32;
    #[link_name = "llvm.x86.sse42.pcmpestris128"]
    fn pcmpestris128(
        a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
    ) -> i32;
    #[link_name = "llvm.x86.sse42.pcmpestrio128"]
    fn pcmpestrio128(
        a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
    ) -> i32;
    #[link_name = "llvm.x86.sse42.pcmpestria128"]
    fn pcmpestria128(
        a: __m128i, la: i32, b: __m128i, lb: i32, imm8: i8
    ) -> i32;
    #[link_name = "llvm.x86.sse42.pcmpistrm128"]
    fn pcmpistrm128(a: __m128i, b: __m128i, imm8: i8) -> u8x16;
    #[link_name = "llvm.x86.sse42.pcmpistri128"]
    fn pcmpistri128(a: __m128i, b: __m128i, imm8: i8) -> i32;
    #[link_name = "llvm.x86.sse42.pcmpistriz128"]
    fn pcmpistriz128(a: __m128i, b: __m128i, imm8: i8) -> i32;
    #[link_name = "llvm.x86.sse42.pcmpistric128"]
    fn pcmpistric128(a: __m128i, b: __m128i, imm8: i8) -> i32;
    #[link_name = "llvm.x86.sse42.pcmpistris128"]
    fn pcmpistris128(a: __m128i, b: __m128i, imm8: i8) -> i32;
    #[link_name = "llvm.x86.sse42.pcmpistrio128"]
    fn pcmpistrio128(a: __m128i, b: __m128i, imm8: i8) -> i32;
    #[link_name = "llvm.x86.sse42.pcmpistria128"]
    fn pcmpistria128(a: __m128i, b: __m128i, imm8: i8) -> i32;
    // SSE 4.2 CRC instructions
    #[link_name = "llvm.x86.sse42.crc32.32.8"]
    fn crc32_32_8(crc: u32, v: u8) -> u32;
    #[link_name = "llvm.x86.sse42.crc32.32.16"]
    fn crc32_32_16(crc: u32, v: u16) -> u32;
    #[link_name = "llvm.x86.sse42.crc32.32.32"]
    fn crc32_32_32(crc: u32, v: u32) -> u32;
    #[link_name = "llvm.x86.sse42.crc32.64.64"]
    fn crc32_64_64(crc: u64, v: u64) -> u64;
}

#[cfg(test)]
mod tests {
    use stdsimd_test::simd_test;

    use std::ptr;
    use v128::*;
    use x86::{__m128i, sse42};

    // Currently one cannot `load` a &[u8] that is is less than 16
    // in length. This makes loading strings less than 16 in length
    // a bit difficult. Rather than `load` and mutate the __m128i,
    // it is easier to memcpy the given string to a local slice with
    // length 16 and `load` the local slice.
    #[target_feature = "+sse4.2"]
    unsafe fn str_to_m128i(s: &[u8]) -> __m128i {
        assert!(s.len() <= 16);
        let slice = &mut [0u8; 16];
        ptr::copy_nonoverlapping(
            s.get_unchecked(0) as *const u8 as *const u8,
            slice.get_unchecked_mut(0) as *mut u8 as *mut u8,
            s.len(),
        );
        __m128i::from(u8x16::load(slice, 0))
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpistrm() {
        let a = str_to_m128i(b"Hello! Good-Bye!");
        let b = str_to_m128i(b"hello! good-bye!");
        let i = sse42::_mm_cmpistrm(a, b, sse42::_SIDD_UNIT_MASK);
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let res = u8x16::new(
            0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00,
            0xff, 0xff, 0xff, 0xff, 0x00, 0xff, 0xff, 0xff,
        );
        assert_eq!(i, res);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpistri() {
        let a = str_to_m128i(b"Hello");
        let b = str_to_m128i(b"   Hello        ");
        let i = sse42::_mm_cmpistri(a, b, sse42::_SIDD_CMP_EQUAL_ORDERED);
        assert_eq!(3, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpistrz() {
        let a = str_to_m128i(b"");
        let b = str_to_m128i(b"Hello");
        let i = sse42::_mm_cmpistrz(a, b, sse42::_SIDD_CMP_EQUAL_ORDERED);
        assert_eq!(1, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpistrc() {
        let a = str_to_m128i(b"                ");
        let b = str_to_m128i(b"       !        ");
        let i = sse42::_mm_cmpistrc(a, b, sse42::_SIDD_UNIT_MASK);
        assert_eq!(1, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpistrs() {
        let a = str_to_m128i(b"Hello");
        let b = str_to_m128i(b"");
        let i = sse42::_mm_cmpistrs(a, b, sse42::_SIDD_CMP_EQUAL_ORDERED);
        assert_eq!(1, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpistro() {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let a_bytes = u8x16::new(
            0x00, 0x47, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c,
            0x00, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        );
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let b_bytes = u8x16::new(
            0x00, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c,
            0x00, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        );
        let a = __m128i::from(a_bytes);
        let b = __m128i::from(b_bytes);
        let i = sse42::_mm_cmpistro(
            a,
            b,
            sse42::_SIDD_UWORD_OPS | sse42::_SIDD_UNIT_MASK,
        );
        assert_eq!(0, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpistra() {
        let a = str_to_m128i(b"");
        let b = str_to_m128i(b"Hello!!!!!!!!!!!");
        let i = sse42::_mm_cmpistra(a, b, sse42::_SIDD_UNIT_MASK);
        assert_eq!(1, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpestrm() {
        let a = str_to_m128i(b"Hello!");
        let b = str_to_m128i(b"Hello.");
        let i = sse42::_mm_cmpestrm(a, 5, b, 5, sse42::_SIDD_UNIT_MASK);
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let r = u8x16::new(
            0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        );
        assert_eq!(i, r);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpestri() {
        let a = str_to_m128i(b"bar - garbage");
        let b = str_to_m128i(b"foobar");
        let i =
            sse42::_mm_cmpestri(a, 3, b, 6, sse42::_SIDD_CMP_EQUAL_ORDERED);
        assert_eq!(3, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpestrz() {
        let a = str_to_m128i(b"");
        let b = str_to_m128i(b"Hello");
        let i =
            sse42::_mm_cmpestrz(a, 16, b, 6, sse42::_SIDD_CMP_EQUAL_ORDERED);
        assert_eq!(1, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpestrc() {
        let va = str_to_m128i(b"!!!!!!!!");
        let vb = str_to_m128i(b"        ");
        let i = sse42::_mm_cmpestrc(va, 7, vb, 7, sse42::_SIDD_UNIT_MASK);
        assert_eq!(0, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpestrs() {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let a_bytes = u8x16::new(
            0x00, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c,
            0x00, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        );
        let a = __m128i::from(a_bytes);
        let b = __m128i::from(u8x16::splat(0x00));
        let i = sse42::_mm_cmpestrs(a, 8, b, 0, sse42::_SIDD_UWORD_OPS);
        assert_eq!(0, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpestro() {
        let a = str_to_m128i(b"Hello");
        let b = str_to_m128i(b"World");
        let i = sse42::_mm_cmpestro(a, 5, b, 5, sse42::_SIDD_UBYTE_OPS);
        assert_eq!(0, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpestra() {
        let a = str_to_m128i(b"Cannot match a");
        let b = str_to_m128i(b"Null after 14");
        let i = sse42::_mm_cmpestra(
            a,
            14,
            b,
            16,
            sse42::_SIDD_CMP_EQUAL_EACH | sse42::_SIDD_UNIT_MASK,
        );
        assert_eq!(1, i);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_crc32_u8() {
        let crc = 0x2aa1e72b;
        let v = 0x2a;
        let i = sse42::_mm_crc32_u8(crc, v);
        assert_eq!(i, 0xf24122e4);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_crc32_u16() {
        let crc = 0x8ecec3b5;
        let v = 0x22b;
        let i = sse42::_mm_crc32_u16(crc, v);
        assert_eq!(i, 0x13bb2fb);
    }

    #[simd_test = "sse4.2"]
    unsafe fn _mm_crc32_u32() {
        let crc = 0xae2912c8;
        let v = 0x845fed;
        let i = sse42::_mm_crc32_u32(crc, v);
        assert_eq!(i, 0xffae2ed1);
    }

    #[cfg(target_arch = "x86_64")]
    #[simd_test = "sse4.2"]
    unsafe fn _mm_crc32_u64() {
        let crc = 0x7819dccd3e824;
        let v = 0x2a22b845fed;
        let i = sse42::_mm_crc32_u64(crc, v);
        assert_eq!(i, 0xbb6cdc6c);
    }

    #[cfg(target_arch = "x86_64")]
    #[simd_test = "sse4.2"]
    unsafe fn _mm_cmpgt_epi64() {
        let a = i64x2::splat(0x00).replace(1, 0x2a);
        let b = i64x2::splat(0x00);
        let i = sse42::_mm_cmpgt_epi64(a, b);
        assert_eq!(i, i64x2::new(0x00, 0xffffffffffffffffu64 as i64));
    }
}
