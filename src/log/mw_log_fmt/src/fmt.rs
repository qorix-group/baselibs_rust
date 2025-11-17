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

use crate::FormatSpec;
use core::fmt::Error as CoreFmtError;
use core::marker::PhantomData;
use core::ptr::NonNull;

pub type Result = core::result::Result<(), Error>;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error;

impl From<CoreFmtError> for Error {
    fn from(_value: CoreFmtError) -> Self {
        Self
    }
}

pub trait ScoreWrite {
    fn write_bool(&mut self, v: &bool, spec: &FormatSpec) -> Result;
    fn write_f32(&mut self, v: &f32, spec: &FormatSpec) -> Result;
    fn write_f64(&mut self, v: &f64, spec: &FormatSpec) -> Result;
    fn write_i8(&mut self, v: &i8, spec: &FormatSpec) -> Result;
    fn write_i16(&mut self, v: &i16, spec: &FormatSpec) -> Result;
    fn write_i32(&mut self, v: &i32, spec: &FormatSpec) -> Result;
    fn write_i64(&mut self, v: &i64, spec: &FormatSpec) -> Result;
    fn write_u8(&mut self, v: &u8, spec: &FormatSpec) -> Result;
    fn write_u16(&mut self, v: &u16, spec: &FormatSpec) -> Result;
    fn write_u32(&mut self, v: &u32, spec: &FormatSpec) -> Result;
    fn write_u64(&mut self, v: &u64, spec: &FormatSpec) -> Result;
    fn write_str(&mut self, v: &str, spec: &FormatSpec) -> Result;
}

#[derive(Debug)]
pub struct Placeholder<'a> {
    value: NonNull<()>,
    formatter: fn(NonNull<()>, &mut dyn ScoreWrite, &FormatSpec) -> Result,
    spec: FormatSpec,
    _lifetime: PhantomData<&'a ()>,
}

macro_rules! new_format {
    ($name:ident, $trait:ident) => {
        pub const fn $name<T: $trait>(value: &T, spec: FormatSpec) -> Self {
            let value = NonNull::from_ref(value).cast();
            let formatter = |v: NonNull<()>, f: &mut dyn ScoreWrite, spec: &FormatSpec| {
                let typed = unsafe { v.cast::<T>().as_ref() };
                typed.fmt(f, spec)
            };
            Self {
                value,
                formatter,
                spec,
                _lifetime: PhantomData,
            }
        }
    };
}

impl<'a> Placeholder<'a> {
    new_format!(new_debug, ScoreDebug);
    new_format!(new_display, ScoreDisplay);

    pub fn fmt(&self, f: &mut dyn ScoreWrite, spec: &FormatSpec) -> Result {
        (self.formatter)(self.value, f, spec)
    }
}

#[derive(Debug)]
pub enum Fragment<'a> {
    Literal(&'a str),
    Placeholder(Placeholder<'a>),
}

#[derive(Copy, Clone, Debug)]
pub struct Arguments<'a>(pub &'a [Fragment<'a>]);

impl ScoreDebug for Arguments<'_> {
    fn fmt(&self, f: &mut dyn ScoreWrite, spec: &FormatSpec) -> Result {
        ScoreDisplay::fmt(self, f, spec)
    }
}

impl ScoreDisplay for Arguments<'_> {
    fn fmt(&self, f: &mut dyn ScoreWrite, _spec: &FormatSpec) -> Result {
        write(f, *self)
    }
}

pub trait ScoreDebug {
    fn fmt(&self, f: &mut dyn ScoreWrite, spec: &FormatSpec) -> Result;
}

pub trait ScoreDisplay {
    fn fmt(&self, f: &mut dyn ScoreWrite, spec: &FormatSpec) -> Result;
}

pub fn write(output: &mut dyn ScoreWrite, args: Arguments<'_>) -> Result {
    for fragment in args.0 {
        match fragment {
            Fragment::Literal(s) => output.write_str(s, &FormatSpec::new()),
            Fragment::Placeholder(ph) => ph.fmt(output, &ph.spec),
        }?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{write, Arguments, Error, FormatSpec, Fragment, Placeholder, Result, ScoreDebug, ScoreDisplay, ScoreWrite};
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

    #[test]
    fn test_error_from_core_fmt_error_ok() {
        let core_fmt_error = core::fmt::Error;
        let _crate_error: Error = core_fmt_error.into();
    }

    #[test]
    fn test_arguments_display() {
        let mut w = StringWriter::new();
        let args = Arguments(&[
            Fragment::Literal("test_"),
            Fragment::Placeholder(Placeholder::new_display(&true, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&123.4f32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&432.2f64, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-100i8, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-1234i16, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-123456i32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-1200000000000000000i64, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&123u8, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&1234u16, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&123456u32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&1200000000000000000u64, FormatSpec::new())),
            Fragment::Literal("_string"),
        ]);

        let result = ScoreDisplay::fmt(&args, &mut w, &FormatSpec::new());
        assert_eq!(result, Ok(()));
        assert_eq!(
            w.get(),
            "test_true123.4432.2-100-1234-123456-120000000000000000012312341234561200000000000000000_string"
        )
    }

    #[test]
    fn test_arguments_debug() {
        let mut w = StringWriter::new();
        let args = Arguments(&[
            Fragment::Literal("test_"),
            Fragment::Placeholder(Placeholder::new_display(&true, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&123.4f32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&432.2f64, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-100i8, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-1234i16, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-123456i32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-1200000000000000000i64, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&123u8, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&1234u16, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&123456u32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&1200000000000000000u64, FormatSpec::new())),
            Fragment::Literal("_string"),
        ]);

        let result = ScoreDebug::fmt(&args, &mut w, &FormatSpec::new());
        assert_eq!(result, Ok(()));
        assert_eq!(
            w.get(),
            "test_true123.4432.2-100-1234-123456-120000000000000000012312341234561200000000000000000_string"
        )
    }

    #[test]
    fn test_write_empty() {
        let mut w = StringWriter::new();
        let args = Arguments(&[]);
        assert_eq!(write(&mut w, args), Ok(()));
    }

    #[test]
    fn test_write_literals_only() {
        let mut w = StringWriter::new();
        let args = Arguments(&[Fragment::Literal("test_"), Fragment::Literal("string")]);
        assert_eq!(write(&mut w, args), Ok(()));
        assert_eq!(w.get(), "test_string");
    }

    #[test]
    fn test_write_placeholders_only() {
        let mut w = StringWriter::new();
        let string_value = String::from("xyz");
        let args = Arguments(&[
            Fragment::Placeholder(Placeholder::new_display(&true, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&123.4f32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&432.2f64, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-100i8, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-1234i16, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-123456i32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&-1200000000000000000i64, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&123u8, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&1234u16, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&123456u32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&1200000000000000000u64, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&"test", FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_display(&string_value, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&true, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&123.4f32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&432.2f64, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&-100i8, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&-1234i16, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&-123456i32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&-1200000000000000000i64, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&123u8, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&1234u16, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&123456u32, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&1200000000000000000u64, FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&"test", FormatSpec::new())),
            Fragment::Placeholder(Placeholder::new_debug(&string_value, FormatSpec::new())),
        ]);
        assert_eq!(write(&mut w, args), Ok(()));

        let exp_pattern = "true123.4432.2-100-1234-123456-120000000000000000012312341234561200000000000000000testxyz";
        assert_eq!(w.get(), format!("{0}{0}", exp_pattern));
    }

    #[test]
    fn test_write_mixed() {
        let mut w = StringWriter::new();
        let args = Arguments(&[
            Fragment::Literal("test_"),
            Fragment::Placeholder(Placeholder::new_display(&123i8, FormatSpec::new())),
            Fragment::Literal("_string"),
        ]);
        assert_eq!(write(&mut w, args), Ok(()));
        assert_eq!(w.get(), "test_123_string");
    }
}
