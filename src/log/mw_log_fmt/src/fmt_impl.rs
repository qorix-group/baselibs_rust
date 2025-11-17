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
