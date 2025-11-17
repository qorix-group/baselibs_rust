use crate::FormatSpec;
use core::marker::PhantomData;
use core::ptr::NonNull;

pub type Result = core::result::Result<(), Error>;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error;

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
