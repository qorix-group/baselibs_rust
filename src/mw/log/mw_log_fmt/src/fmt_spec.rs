#[derive(Clone, Copy, Debug)]
pub enum Alignment {
    /// Align to left (`<`).
    Left,
    /// Align to right (`<`).
    Right,
    /// Align to center (`<`).
    Center,
}

#[derive(Clone, Copy, Debug)]
pub enum Sign {
    /// Always show sign (`+`).
    Plus,
    /// Unused (`-`).
    Minus,
}

#[derive(Clone, Copy, Debug)]
pub enum DebugAsHex {
    /// Format integer values to lower hex.
    Lower,
    /// Format integer values to upper hex.
    Upper,
}

#[derive(Clone, Copy, Debug)]
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
