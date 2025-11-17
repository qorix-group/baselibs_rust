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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    /// Align to left (`<`).
    Left,
    /// Align to right (`<`).
    Right,
    /// Align to center (`<`).
    Center,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    /// Always show sign (`+`).
    Plus,
    /// Unused (`-`).
    Minus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugAsHex {
    /// Format integer values to lower hex.
    Lower,
    /// Format integer values to upper hex.
    Upper,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayHint {
    /// `{}` or `{:}`.
    NoHint,
    /// `{:o}`.
    Octal,
    /// `{:x}`.
    LowerHex,
    /// `{:X}`.
    UpperHex,
    /// `{:p}`.
    Pointer,
    /// `{:b}`.
    Binary,
    /// `{:e}`.
    LowerExp,
    /// `{:E}`.
    UpperExp,
}

/// Format spec.
///
/// format_spec := [[fill]align][sign]['#']['0'][width]['.' precision][type]
/// fill := character
/// align := '<' | '^' | '>'
/// sign := '+' | '-'
/// width := count
/// precision := count | '*'
/// type := '?' | 'x?' | 'X?' | 'o' | 'x' | 'X' | 'p' | 'b' | 'e' | 'E'
/// parameter := argument '$'
#[derive(Clone, Debug)]
pub struct FormatSpec {
    display_hint: DisplayHint,
    fill: char,
    align: Option<Alignment>,
    sign: Option<Sign>,
    alternate: bool,
    zero_pad: bool,
    debug_as_hex: Option<DebugAsHex>,
    width: Option<u16>,
    precision: Option<u16>,
}

impl FormatSpec {
    pub fn new() -> Self {
        Self {
            display_hint: DisplayHint::NoHint,
            fill: ' ',
            align: None,
            sign: None,
            alternate: false,
            zero_pad: false,
            debug_as_hex: None,
            width: None,
            precision: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_params(
        display_hint: DisplayHint,
        fill: char,
        align: Option<Alignment>,
        sign: Option<Sign>,
        alternate: bool,
        zero_pad: bool,
        debug_as_hex: Option<DebugAsHex>,
        width: Option<u16>,
        precision: Option<u16>,
    ) -> Self {
        Self {
            display_hint,
            fill,
            align,
            sign,
            alternate,
            zero_pad,
            debug_as_hex,
            width,
            precision,
        }
    }

    pub fn display_hint(&mut self, display_hint: DisplayHint) -> &mut Self {
        self.display_hint = display_hint;
        self
    }

    pub fn fill(&mut self, fill: char) -> &mut Self {
        self.fill = fill;
        self
    }

    pub fn align(&mut self, align: Option<Alignment>) -> &mut Self {
        self.align = align;
        self
    }

    pub fn sign(&mut self, sign: Option<Sign>) -> &mut Self {
        self.sign = sign;
        self
    }

    pub fn alternate(&mut self, alternate: bool) -> &mut Self {
        self.alternate = alternate;
        self
    }

    pub fn zero_pad(&mut self, zero_pad: bool) -> &mut Self {
        self.zero_pad = zero_pad;
        self
    }

    pub fn debug_as_hex(&mut self, debug_as_hex: Option<DebugAsHex>) -> &mut Self {
        self.debug_as_hex = debug_as_hex;
        self
    }

    pub fn width(&mut self, width: Option<u16>) -> &mut Self {
        self.width = width;
        self
    }

    pub fn precision(&mut self, precision: Option<u16>) -> &mut Self {
        self.precision = precision;
        self
    }

    pub fn get_display_hint(&self) -> DisplayHint {
        self.display_hint
    }

    pub fn get_fill(&self) -> char {
        self.fill
    }

    pub fn get_align(&self) -> Option<Alignment> {
        self.align
    }

    pub fn get_sign(&self) -> Option<Sign> {
        self.sign
    }

    pub fn get_alternate(&self) -> bool {
        self.alternate
    }

    pub fn get_zero_pad(&self) -> bool {
        self.zero_pad
    }

    pub fn get_debug_as_hex(&self) -> Option<DebugAsHex> {
        self.debug_as_hex
    }

    pub fn get_width(&self) -> Option<u16> {
        self.width
    }

    pub fn get_precision(&self) -> Option<u16> {
        self.precision
    }
}

impl Default for FormatSpec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Alignment, DebugAsHex, DisplayHint, FormatSpec, Sign};

    #[test]
    fn test_new() {
        let format_spec = FormatSpec::new();

        assert_eq!(format_spec.get_display_hint(), DisplayHint::NoHint);
        assert_eq!(format_spec.get_fill(), ' ');
        assert_eq!(format_spec.get_align(), None);
        assert_eq!(format_spec.get_sign(), None);
        assert!(!format_spec.get_alternate());
        assert!(!format_spec.get_zero_pad());
        assert_eq!(format_spec.get_debug_as_hex(), None);
        assert_eq!(format_spec.get_width(), None);
        assert_eq!(format_spec.get_precision(), None);
    }

    #[test]
    fn test_default() {
        let spec_default = FormatSpec::default();
        let spec_new = FormatSpec::new();

        assert_eq!(spec_default.get_display_hint(), spec_new.get_display_hint());
        assert_eq!(spec_default.get_fill(), spec_new.get_fill());
        assert_eq!(spec_default.get_align(), spec_new.get_align());
        assert_eq!(spec_default.get_sign(), spec_new.get_sign());
        assert_eq!(spec_default.get_alternate(), spec_new.get_alternate());
        assert_eq!(spec_default.get_zero_pad(), spec_new.get_zero_pad());
        assert_eq!(spec_default.get_debug_as_hex(), spec_new.get_debug_as_hex());
        assert_eq!(spec_default.get_width(), spec_new.get_width());
        assert_eq!(spec_default.get_precision(), spec_new.get_precision());
    }

    #[test]
    fn test_from_params() {
        let display_hint = DisplayHint::Binary;
        let fill = 'Z';
        let align = Some(Alignment::Right);
        let sign = Some(Sign::Plus);
        let alternate = true;
        let zero_pad = true;
        let debug_as_hex = Some(DebugAsHex::Upper);
        let width = Some(1234);
        let precision = Some(5);

        let format_spec = FormatSpec::from_params(display_hint, fill, align, sign, alternate, zero_pad, debug_as_hex, width, precision);

        assert_eq!(format_spec.get_display_hint(), display_hint);
        assert_eq!(format_spec.get_fill(), fill);
        assert_eq!(format_spec.get_align(), align);
        assert_eq!(format_spec.get_sign(), sign);
        assert_eq!(format_spec.get_alternate(), alternate);
        assert_eq!(format_spec.get_zero_pad(), zero_pad);
        assert_eq!(format_spec.get_debug_as_hex(), debug_as_hex);
        assert_eq!(format_spec.get_width(), width);
        assert_eq!(format_spec.get_precision(), precision);
    }

    #[test]
    fn test_display_hint() {
        let mut format_spec = FormatSpec::new();
        assert_eq!(format_spec.get_display_hint(), DisplayHint::NoHint);
        format_spec.display_hint(DisplayHint::LowerExp);
        assert_eq!(format_spec.get_display_hint(), DisplayHint::LowerExp);
    }

    #[test]
    fn test_fill() {
        let mut format_spec = FormatSpec::new();
        assert_eq!(format_spec.get_fill(), ' ');
        format_spec.fill('c');
        assert_eq!(format_spec.get_fill(), 'c');
    }

    #[test]
    fn test_align() {
        let mut format_spec = FormatSpec::new();
        assert_eq!(format_spec.get_align(), None);
        format_spec.align(Some(Alignment::Center));
        assert_eq!(format_spec.get_align(), Some(Alignment::Center));
    }

    #[test]
    fn test_sign() {
        let mut format_spec = FormatSpec::new();
        assert_eq!(format_spec.get_sign(), None);
        format_spec.sign(Some(Sign::Minus));
        assert_eq!(format_spec.get_sign(), Some(Sign::Minus));
    }

    #[test]
    fn test_alternate() {
        let mut format_spec = FormatSpec::new();
        assert!(!format_spec.get_alternate());
        format_spec.alternate(true);
        assert!(format_spec.get_alternate());
    }

    #[test]
    fn test_zero_pad() {
        let mut format_spec = FormatSpec::new();
        assert!(!format_spec.get_zero_pad());
        format_spec.zero_pad(true);
        assert!(format_spec.get_zero_pad());
    }

    #[test]
    fn test_debug_as_hex() {
        let mut format_spec = FormatSpec::new();
        assert_eq!(format_spec.get_debug_as_hex(), None);
        format_spec.debug_as_hex(Some(DebugAsHex::Lower));
        assert_eq!(format_spec.get_debug_as_hex(), Some(DebugAsHex::Lower));
    }

    #[test]
    fn test_width() {
        let mut format_spec = FormatSpec::new();
        assert_eq!(format_spec.get_width(), None);
        format_spec.width(Some(12345));
        assert_eq!(format_spec.get_width(), Some(12345));
    }

    #[test]
    fn test_precision() {
        let mut format_spec = FormatSpec::new();
        assert_eq!(format_spec.get_precision(), None);
        format_spec.precision(Some(54321));
        assert_eq!(format_spec.get_precision(), Some(54321));
    }
}
