//
// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// <https://www.apache.org/licenses/LICENSE-2.0>
//
// SPDX-License-Identifier: Apache-2.0
//

use crate::fmt;
use crate::fmt::*;
use crate::fmt_spec::FormatSpec;

macro_rules! impl_fmt_for_t {
    ($t:ty, $fn:ident, $($fmt:ident),*) => {
        $(
        impl $fmt for $t {
            fn fmt(&self, f: &mut dyn ScoreWrite, spec: &FormatSpec) -> fmt::Result {
                f.$fn(self, &spec)
            }
        }
        )*
    };
}

impl_fmt_for_t!(bool, write_bool, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(f32, write_f32, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(f64, write_f64, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(i8, write_i8, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(i16, write_i16, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(i32, write_i32, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(i64, write_i64, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(u8, write_u8, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(u16, write_u16, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(u32, write_u32, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(u64, write_u64, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(&str, write_str, ScoreDebug, ScoreDisplay);
impl_fmt_for_t!(String, write_str, ScoreDebug, ScoreDisplay);

#[cfg(test)]
mod tests {
    use crate::{FormatSpec, Result, ScoreDebug, ScoreDisplay, ScoreWrite};
    use core::fmt::Write;

    struct StringWriter {
        buf: String,
    }

    impl StringWriter {
        pub fn new() -> Self {
            Self { buf: String::new() }
        }

        pub fn get(&self) -> &str {
            self.buf.as_str()
        }
    }

    impl ScoreWrite for StringWriter {
        fn write_bool(&mut self, v: &bool, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_f32(&mut self, v: &f32, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_f64(&mut self, v: &f64, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_i8(&mut self, v: &i8, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_i16(&mut self, v: &i16, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_i32(&mut self, v: &i32, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_i64(&mut self, v: &i64, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_u8(&mut self, v: &u8, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_u16(&mut self, v: &u16, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_u32(&mut self, v: &u32, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_u64(&mut self, v: &u64, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }

        fn write_str(&mut self, v: &str, _spec: &FormatSpec) -> Result {
            Ok(write!(self.buf, "{}", v)?)
        }
    }

    macro_rules! common_test {
        ($t:ty, $v:expr, $fmt:ident) => {
            let v: $t = $v;
            let mut w = StringWriter::new();
            $fmt::fmt(&v, &mut w, &FormatSpec::new()).unwrap();
            assert_eq!(w.get(), format!("{v}"));
        };
    }

    #[test]
    fn test_bool_display() {
        common_test!(bool, true, ScoreDisplay);
    }

    #[test]
    fn test_bool_debug() {
        common_test!(bool, true, ScoreDebug);
    }

    #[test]
    fn test_f32_display() {
        common_test!(f32, 123.4, ScoreDisplay);
    }

    #[test]
    fn test_f32_debug() {
        common_test!(f32, 123.4, ScoreDebug);
    }

    #[test]
    fn test_f64_display() {
        common_test!(f64, 123.4, ScoreDisplay);
    }

    #[test]
    fn test_f64_debug() {
        common_test!(f64, 123.4, ScoreDebug);
    }

    #[test]
    fn test_i8_display() {
        common_test!(i8, -123, ScoreDisplay);
    }

    #[test]
    fn test_i8_debug() {
        common_test!(i8, -123, ScoreDebug);
    }

    #[test]
    fn test_i16_display() {
        common_test!(i16, -1234, ScoreDisplay);
    }

    #[test]
    fn test_i16_debug() {
        common_test!(i16, -1234, ScoreDebug);
    }

    #[test]
    fn test_i32_display() {
        common_test!(i32, -123456, ScoreDisplay);
    }

    #[test]
    fn test_i32_debug() {
        common_test!(i32, -123456, ScoreDebug);
    }

    #[test]
    fn test_i64_display() {
        common_test!(i64, -1200000000000000000, ScoreDisplay);
    }

    #[test]
    fn test_i64_debug() {
        common_test!(i64, -1200000000000000000, ScoreDebug);
    }

    #[test]
    fn test_u8_display() {
        common_test!(u8, 123, ScoreDisplay);
    }

    #[test]
    fn test_u8_debug() {
        common_test!(u8, 123, ScoreDebug);
    }

    #[test]
    fn test_u16_display() {
        common_test!(u16, 1234, ScoreDisplay);
    }

    #[test]
    fn test_u16_debug() {
        common_test!(u16, 1234, ScoreDebug);
    }

    #[test]
    fn test_u32_display() {
        common_test!(u32, 123456, ScoreDisplay);
    }

    #[test]
    fn test_u32_debug() {
        common_test!(u32, 123456, ScoreDebug);
    }

    #[test]
    fn test_u64_display() {
        common_test!(u64, 1200000000000000000, ScoreDisplay);
    }

    #[test]
    fn test_u64_debug() {
        common_test!(u64, 1200000000000000000, ScoreDebug);
    }

    #[test]
    fn test_str_display() {
        common_test!(&str, "test", ScoreDisplay);
    }

    #[test]
    fn test_str_debug() {
        common_test!(&str, "test", ScoreDebug);
    }

    #[test]
    fn test_string_display() {
        common_test!(String, String::from("test"), ScoreDisplay);
    }

    #[test]
    fn test_string_debug() {
        common_test!(String, String::from("test"), ScoreDebug);
    }
}
