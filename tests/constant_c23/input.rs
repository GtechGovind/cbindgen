pub const SMALL: u8 = 255;
pub const LARGE: u64 = 18446744073709551615;
pub const NEGATIVE: i32 = -42;
pub const MASK: u32 = !0u32;
pub const ARITHMETIC: usize = (4 + 2) * 3;
pub const SHIFT: u32 = 1 << 31;
pub const SUM: u32 = 2147483647 + 1;
pub const BITWISE: u32 = 5 | 2;
pub const ENABLED: bool = true;
pub const CHARACTER: char = 'x';
pub const FLOAT: f32 = 0.1;
pub const DOUBLE: f64 = 0.3333333333333333;
pub const FLOAT_DIVISION: f64 = 1.0 / 3.0;
pub const NESTED_NEGATION: i64 = -(!2147483647);
pub const SATURATING_CAST: f64 = (1e100 as i32) as f64;
pub const BOUND: u32 = u32::MAX;
pub const MIN8: u8 = u8::MIN;
pub const MIN16: u16 = u16::MIN;
pub const MIN32: u32 = u32::MIN;
pub const MIN64: u64 = u64::MIN;
pub const BIG_FLOAT: f64 = 18446744073709551616f64;
pub const BIG_FLOAT_SIGNED: f64 = 9223372036854775808f64;
pub const SIGNED_MIN_LITERAL: i64 = -9223372036854775808i64;
pub const LATIN: char = 'é';
pub const OFF: bool = !true;
pub const OFF_COMPARE: bool = !(1 == 1);

// These are deliberately left as macros. In particular, alphabetical sorting
// puts A_REFERENCE before its dependency, which is harmless for macros.
pub const A_REFERENCE: u32 = Z_VALUE + 1;
pub const Z_VALUE: u32 = 41;
pub const FLOAT_TO_INTEGER: u32 = (1.5 + 2.5) as u32;
pub const DOUBLE_CAST: u32 = 1 as f32 as u32;
pub const POINTER: *const u8 = 1 as *const u8;
pub const NULL_POINTER: *const u8 = 0 as *const u8;
pub const ARRAY: [u8; 2] = [1, 2];
pub const STRING: &str = "text";

#[repr(C)]
pub struct Aggregate {
    pub value: u32,
}

pub const AGGREGATE: Aggregate = Aggregate { value: 4 };
pub const FIELD: u32 = AGGREGATE.value;

pub type Alias = u32;
pub const ALIAS: Alias = 5;

impl Aggregate {
    pub const ASSOCIATED: u32 = 6;
}

#[no_mangle]
pub extern "C" fn use_types(_: Aggregate, _: Alias) {}
