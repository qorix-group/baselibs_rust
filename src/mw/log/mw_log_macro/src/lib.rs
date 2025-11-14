// All errors should result in compilation error.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use mw_log_fmt::{Alignment, DebugAsHex, FormatSpec, Sign};
use quote::{ToTokens, quote};
use syn::punctuated::{IntoIter, Punctuated};
use syn::token::Comma;
use syn::{Error, Expr, ExprLit, ExprPath, Ident, Lit, Path, PathSegment, parse_macro_input};

/// Parse error containing reason.
/// - Functions with access to tokens should return `syn::Error`
/// - Other functions should return `ParseError` containing explanation.
#[derive(Debug)]
struct ParseError(pub String);

#[derive(Debug)]
enum Argument {
    Position,
    Index(usize),
    Name(String),
}

/// Parse left side of the placeholder (`{*arg*:spec}`).
fn parse_argument(s: &str) -> Result<Argument, ParseError> {
    let arg = if s.is_empty() {
        Argument::Position
    } else if let Ok(v) = s.parse::<usize>() {
        Argument::Index(v)
    } else {
        Argument::Name(s.to_string())
    };
    Ok(arg)
}

// Not the same enum as `mw_log_fmt::fmt_spec::DisplayHint`.
// This one should be aligned, but also include `Debug`.
#[derive(Debug)]
enum DisplayHint {
    // `{}` or `{:}`.
    NoHint,
    // `{:?}`, `{:x?}`, or `{:X?}`.
    Debug,
    // `{:o}`.
    Octal,
    // `{:x}`.
    LowerHex,
    // `{:X}`.
    UpperHex,
    // `{:p}`.
    Pointer,
    // `{:b}`.
    Binary,
    // `{:e}`.
    LowerExp,
    // `{:E}`.
    UpperExp,
}

/// Get alignment based on provided character.
fn get_alignment(c: &char) -> Result<Alignment, ParseError> {
    match c {
        '<' => Ok(Alignment::Left),
        '>' => Ok(Alignment::Right),
        '^' => Ok(Alignment::Center),
        _ => Err(ParseError(format!(
            "unknown alignment character provided: {c}"
        ))),
    }
}

/// Get sign based on provided character.
fn get_sign(c: &char) -> Result<Sign, ParseError> {
    match c {
        '+' => Ok(Sign::Plus),
        '-' => Ok(Sign::Minus),
        _ => Err(ParseError(format!("unknown sign character provided: {c}"))),
    }
}

/// Parse right side of the placeholder `{arg:*spec*}`.
fn parse_spec(s: &str) -> Result<(FormatSpec, DisplayHint), ParseError> {
    let mut chars = s.chars().peekable();

    // Parse fill and alignment ([[fill]align]).
    let mut fill = ' ';
    let mut align = None;
    {
        if let (Some(a), Some(b)) = (chars.next(), chars.peek()) {
            const ALIGN_CHARS: [char; 3] = ['<', '^', '>'];
            // `[[fill]align]`
            if ALIGN_CHARS.contains(b) {
                fill = a;
                align = Some(get_alignment(b)?);
                chars.next();
            }
            // `[align]`
            else if ALIGN_CHARS.contains(&a) {
                align = Some(get_alignment(&a)?);
            }
        }

        // `align` not set (`[]`) - reset `chars` position.
        if align.is_none() {
            chars = s.chars().peekable();
        }
    }

    // Parse sign ([sign]).
    let mut sign = None;
    {
        if let Some(c) = chars.peek() {
            const SIGN_CHARS: [char; 2] = ['+', '-'];
            if SIGN_CHARS.contains(c) {
                sign = Some(get_sign(c)?);
            }
        }

        if sign.is_some() {
            chars.next();
        }
    }

    // Parse alternate (['#']).
    let mut alternate = false;
    {
        if let Some(c) = chars.peek()
            && *c == '#'
        {
            alternate = true;
            chars.next();
        }
    }

    // Parse zero pad (['0']).
    let mut zero_pad = false;
    {
        if let Some(c) = chars.peek()
            && *c == '0'
        {
            zero_pad = true;
            chars.next();
        }
    }

    // Parse width ([width]).
    let mut width: Option<u16> = None;
    {
        let mut width_str = String::new();
        while let Some(c) = chars.peek() {
            if c.is_ascii_digit() {
                width_str.push(*c);
                chars.next();
            } else {
                break;
            }
        }
        if !width_str.is_empty() {
            width = match width_str.parse() {
                Ok(v) => Some(v),
                Err(_) => return Err(ParseError("unable to parse width".to_string())),
            };
        }
    }

    // Parse precision (['.' precision]).
    let mut precision: Option<u16> = None;
    {
        if let Some(c) = chars.peek()
            && *c == '.'
        {
            chars.next();

            let mut precision_str = String::new();
            while let Some(c) = chars.peek() {
                if c.is_ascii_digit() {
                    precision_str.push(*c);
                    chars.next();
                } else {
                    break;
                }
            }
            if !precision_str.is_empty() {
                precision = match precision_str.parse() {
                    Ok(v) => Some(v),
                    Err(_) => return Err(ParseError("unable to parse precision".to_string())),
                };
            }
        }
    }

    // Parse formatting trait ([type]).
    let display_hint;
    let mut debug_as_hex = None;
    {
        let remainder = chars.collect::<String>();
        display_hint = match remainder.as_str() {
            "" => DisplayHint::NoHint,
            "?" => DisplayHint::Debug,
            "x?" => {
                debug_as_hex = Some(DebugAsHex::Lower);
                DisplayHint::Debug
            }
            "X?" => {
                debug_as_hex = Some(DebugAsHex::Upper);
                DisplayHint::Debug
            }
            "o" => DisplayHint::Octal,
            "x" => DisplayHint::LowerHex,
            "X" => DisplayHint::UpperHex,
            "p" => DisplayHint::Pointer,
            "b" => DisplayHint::Binary,
            "e" => DisplayHint::LowerExp,
            "E" => DisplayHint::UpperExp,
            _ => return Err(ParseError(format!("unknown display hint: {remainder}"))),
        };
    }

    // Construct format spec.
    let mut spec = FormatSpec::new();
    spec.fill(fill)
        .align(align)
        .sign(sign)
        .alternate(alternate)
        .zero_pad(zero_pad)
        .debug_as_hex(debug_as_hex)
        .width(width)
        .precision(precision);

    Ok((spec, display_hint))
}

/// Tokenize format spec constructor.
fn tokenize_spec(spec: &FormatSpec) -> proc_macro2::TokenStream {
    // TODO: instead of repackaging - generate `TokenStream` in the first place?

    // Additional helpers are required to properly tokenize enums and options.
    fn tokenize_display_hint(display_hint: mw_log_fmt::DisplayHint) -> proc_macro2::TokenStream {
        match display_hint {
            mw_log_fmt::DisplayHint::NoHint => quote! { DisplayHint::NoHint },
            mw_log_fmt::DisplayHint::Octal => quote! { DisplayHint::Octal },
            mw_log_fmt::DisplayHint::LowerHex => quote! { DisplayHint::LowerHex },
            mw_log_fmt::DisplayHint::UpperHex => quote! { DisplayHint::UpperHex },
            mw_log_fmt::DisplayHint::Pointer => quote! { DisplayHint::Pointer },
            mw_log_fmt::DisplayHint::Binary => quote! { DisplayHint::Binary },
            mw_log_fmt::DisplayHint::LowerExp => quote! { DisplayHint::LowerExp },
            mw_log_fmt::DisplayHint::UpperExp => quote! { DisplayHint::UpperExp },
        }
    }

    fn tokenize_alignment(align: Option<Alignment>) -> proc_macro2::TokenStream {
        match align {
            Some(v) => match v {
                Alignment::Left => quote! { Some(Alignment::Left) },
                Alignment::Right => quote! { Some(Alignment::Right) },
                Alignment::Center => quote! { Some(Alignment::Center) },
            },
            None => quote! { None },
        }
    }

    fn tokenize_sign(sign: Option<Sign>) -> proc_macro2::TokenStream {
        match sign {
            Some(v) => match v {
                Sign::Plus => quote! { Some(Sign::Plus) },
                Sign::Minus => quote! { Some(Sign::Minus) },
            },
            None => quote! { None },
        }
    }

    fn tokenize_debug_as_hex(debug_as_hex: Option<DebugAsHex>) -> proc_macro2::TokenStream {
        match debug_as_hex {
            Some(v) => match v {
                DebugAsHex::Lower => quote! { Some(DebugAsHex::Lower) },
                DebugAsHex::Upper => quote! { Some(DebugAsHex::Upper) },
            },
            None => quote! { None },
        }
    }

    fn tokenize_option_u16(o: Option<u16>) -> proc_macro2::TokenStream {
        match o {
            Some(v) => quote! { Some(#v) },
            None => quote! { None },
        }
    }

    let display_hint = tokenize_display_hint(spec.get_display_hint());
    let fill = spec.get_fill();
    let align = tokenize_alignment(spec.get_align());
    let sign = tokenize_sign(spec.get_sign());
    let alternate = spec.get_alternate();
    let zero_pad = spec.get_zero_pad();
    let debug_as_hex = tokenize_debug_as_hex(spec.get_debug_as_hex());
    let width = tokenize_option_u16(spec.get_width());
    let precision = tokenize_option_u16(spec.get_precision());

    quote! {{
        FormatSpec::from_params(
            #display_hint,
            #fill,
            #align,
            #sign,
            #alternate,
            #zero_pad,
            #debug_as_hex,
            #width,
            #precision
        )
    }}
}

#[derive(Debug)]
struct Placeholder {
    argument: Argument,
    display_hint: DisplayHint,
    spec: FormatSpec,
}

impl Placeholder {
    fn from(s: &str) -> Result<Self, ParseError> {
        // Strip surrounding "{}", trim whitespace.
        let s = s
            .strip_prefix('{')
            .ok_or(ParseError("failed to strip placeholder prefix".to_string()))?
            .strip_suffix('}')
            .ok_or(ParseError("failed to strip placeholder suffix".to_string()))?
            .trim();

        // Check placeholder is empty: `{}`.
        if s.is_empty() {
            return Ok(Placeholder {
                argument: Argument::Position,
                display_hint: DisplayHint::NoHint,
                spec: FormatSpec::default(),
            });
        }

        // Split by `:`.
        let (arg, spec) = match s.split_once(':') {
            Some((arg, spec)) => (arg, Some(spec)),
            None => (s, None),
        };

        // Parse argument.
        let argument = parse_argument(arg)?;

        // Parse format spec.
        let (spec, display_hint) = match spec {
            Some(s) => parse_spec(s)?,
            None => (FormatSpec::default(), DisplayHint::NoHint),
        };

        Ok(Placeholder {
            argument,
            display_hint,
            spec,
        })
    }
}

#[derive(Debug)]
enum Spec {
    Literal(String),
    Placeholder(Placeholder),
}

fn process_format_string(format_string: &str) -> Result<Vec<Spec>, ParseError> {
    // Find braces locations.
    #[derive(PartialEq)]
    enum Brace {
        SingleLeft,
        DoubleLeft,
        SingleRight,
        DoubleRight,
    }

    let mut chars = format_string.chars().enumerate().peekable();
    let mut braces = Vec::new();
    while let Some((i, c)) = chars.next() {
        let peek = chars.peek();

        // Check double left.
        if c == '{' && peek.is_some_and(|(_, c)| *c == '{') {
            chars.next();
            braces.push((i, Brace::DoubleLeft));
        }
        // Check single left.
        else if c == '{' && peek.is_none_or(|(_, c)| *c != '{') {
            braces.push((i, Brace::SingleLeft));
        }
        // Check double right.
        else if c == '}' && peek.is_some_and(|(_, c)| *c == '}') {
            chars.next();
            braces.push((i, Brace::DoubleRight));
        }
        // Check single right.
        else if c == '}' && peek.is_none_or(|(_, c)| *c != '}') {
            braces.push((i, Brace::SingleRight));
        }
    }

    // Process braces locations.
    // - Process placeholder locations (must start with left and end with right brace).
    // - Detect dangling braces.
    // - Detect escaped braces inside placeholders.
    let mut placeholders = Vec::new();
    let mut braces_it = braces.into_iter().peekable();
    while let Some((i, brace)) = braces_it.next() {
        match brace {
            // Single left brace might start placeholder.
            Brace::SingleLeft => {
                let (pi, pb) = match braces_it.peek() {
                    Some(x) => x,
                    None => {
                        return Err(ParseError("dangling left brace".to_string()));
                    }
                };
                match pb {
                    Brace::SingleLeft => {
                        return Err(ParseError("dangling left brace".to_string()));
                    }
                    Brace::SingleRight => {
                        // Inclusive range cannot be used.
                        // `Range` and `RangeInclusive` are not compatible.
                        placeholders.push(i..*pi + 1);
                        braces_it.next();
                    }
                    Brace::DoubleLeft | Brace::DoubleRight => {
                        return Err(ParseError(
                            "escaped characters inside placeholder".to_string(),
                        ));
                    }
                }
            }
            // Dangling right brace.
            Brace::SingleRight => {
                return Err(ParseError("dangling right brace".to_string()));
            }
            // Escaped characters are ignored.
            Brace::DoubleLeft | Brace::DoubleRight => continue,
        }
    }

    // Get ranges of string literals - inverted `placeholders`.
    let mut literals = Vec::new();
    let mut prev_end = 0;
    let format_string_len = format_string.len();
    for range in placeholders.clone() {
        if range.start > prev_end {
            literals.push(prev_end..range.start);
        }
        prev_end = range.end;
    }
    if prev_end < format_string_len {
        literals.push(prev_end..format_string_len);
    }

    // Merge literals and placeholders with correct order.
    let mut types_and_ranges = Vec::new();
    types_and_ranges.extend(literals.iter().map(|r| (false, r.clone())));
    types_and_ranges.extend(placeholders.iter().map(|r| (true, r.clone())));
    types_and_ranges.sort_by_key(|(_, r)| r.start);

    // Create output - list of specs containing strings.
    let mut specs = Vec::new();
    for (is_placeholder, range) in types_and_ranges {
        let spec = if is_placeholder {
            Spec::Placeholder(Placeholder::from(&format_string[range])?)
        } else {
            Spec::Literal(format_string[range].to_string())
        };
        specs.push(spec);
    }

    Ok(specs)
}

/// Check valid expression types are used.
/// Named expressions must come after all positional expressions.
fn validate_args(args: &[Expr]) -> Result<(), Error> {
    let mut named_found = false;
    for arg in args.iter() {
        match arg {
            Expr::Assign(_) => named_found = true,
            // TODO: list of allowed expression types might not be complete!
            Expr::Lit(_) | Expr::Path(_) | Expr::Unary(_) => {
                if named_found {
                    return Err(Error::new_spanned(
                        arg,
                        "positional arguments must be before named arguments",
                    ));
                }
            }
            _ => return Err(Error::new_spanned(arg, "invalid expression type")),
        }
    }

    Ok(())
}

/// Create path expression to a variable defined using name.
///
/// NOTE: this functions is not fully functional yet.
/// It's not easily possible to gain access to original caller site to properly generate path.
fn create_expr_path(name: &str) -> Expr {
    let ident = Ident::new(name, proc_macro2::Span::mixed_site());
    let path = Path {
        leading_colon: None,
        segments: {
            let mut segments = Punctuated::new();
            segments.push(PathSegment {
                ident,
                arguments: syn::PathArguments::None,
            });
            segments
        },
    };
    Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path,
    })
}

/// Select argument with name.
///
/// Following cases are supported:
/// - Name provided by spec and `args` - get argument expression from `args`.
///   E.g., `mw_log_format_args!("{arg}", arg)`.
/// - Name provided by spec, but aliased by `args` - get assigned argument expression from `args`.
///   E.g., `mw_log_format_args!("{arg}", arg=other_value)`.
///
/// Not yet supported:
/// - Name provided by spec, but not `args` - create argument expression.
///   E.g., `mw_log_format_args!("{arg}")`.
fn select_arg_with_name(args: &[Expr], name: &str) -> Result<Expr, Error> {
    // Find all arguments that match. Either zero or one are allowed.
    let mut found: Vec<Expr> = Vec::new();
    for arg in args.iter() {
        let (arg_expr, alias_expr) = match arg {
            Expr::Assign(expr_assign) => (
                expr_assign.left.as_ref().clone(),
                Some(expr_assign.right.as_ref().clone()),
            ),
            // TODO: list of allowed expression types might not be complete!
            Expr::Lit(_) | Expr::Path(_) | Expr::Unary(_) => (arg.clone(), None),
            _ => return Err(Error::new_spanned(arg, "invalid expression type")),
        };

        if arg_expr.to_token_stream().to_string() == name {
            if let Some(alias_expr) = alias_expr {
                found.push(alias_expr);
            } else {
                found.push(arg_expr);
            }
        }
    }

    match found.len() {
        // No matching args found - create argument expression.
        0 => Ok(create_expr_path(name)),
        // Matching arg found.
        1 => Ok(found[0].clone()),
        // Multiple matching args found - invalid.
        _ => Err(Error::new(
            proc_macro2::Span::call_site(),
            "multiple matching arguments found",
        )),
    }
}

fn parse_fragments(
    punctuated_it: &mut IntoIter<Expr>,
) -> Result<Vec<proc_macro2::TokenStream>, Error> {
    // Get first argument - format string.
    // Must be a string literal.
    let format_string_expr = match punctuated_it.next() {
        Some(Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        })) => s,
        Some(expr) => {
            return Err(Error::new_spanned(
                expr,
                "first argument must be a string literal",
            ));
        }
        None => {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "expected a string literal",
            ));
        }
    };

    // Process format string and create list of specs.
    let format_string = format_string_expr.value();
    let specs = process_format_string(&format_string)
        .map_err(|e| Error::new_spanned(format_string_expr.clone(), e.0))?;

    // Process specs and match them to provided args.
    let args: Vec<Expr> = punctuated_it.collect();
    validate_args(&args)?;
    let mut fragments = Vec::new();
    // Iterator is used for positional arguments.
    let mut args_it = args.iter();
    for spec in specs.into_iter() {
        match spec {
            Spec::Literal(s) => fragments.push(quote! {{
                Fragment::Literal(#s)
            }}),
            Spec::Placeholder(placeholder) => {
                // Select argument based on provided argument.
                let arg = match placeholder.argument {
                    Argument::Position => match args_it.next() {
                        Some(arg) => arg,
                        None => {
                            return Err(Error::new_spanned(
                                format_string_expr,
                                "argument with provided position not found",
                            ));
                        }
                    },
                    Argument::Index(i) => &args[i],
                    Argument::Name(name) => &select_arg_with_name(&args, &name)?,
                };

                // Select implementation based on provided format spec.
                let placeholder_ctor = match placeholder.display_hint {
                    DisplayHint::Debug => quote! { Placeholder::new_debug },
                    _ => quote! { Placeholder::new_display },
                };

                let spec_ctor = tokenize_spec(&placeholder.spec);

                fragments.push(quote! {{
                    Fragment::Placeholder(#placeholder_ctor(&#arg, #spec_ctor))
                }});
            }
        }
    }

    Ok(fragments)
}

#[proc_macro]
pub fn mw_log_format_args(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Collect expressions separated by comma.
    // NOTE: `parse_macro_input!` can't be build if function return type is not `TokenStream`.
    // This prevents moving it to a separate function.
    let punctuated = parse_macro_input!(input with Punctuated<Expr, Comma>::parse_terminated);
    let mut punctuated_it = punctuated.into_iter();

    // Parse string format into fragments.
    let fragments = match parse_fragments(&mut punctuated_it) {
        Ok(f) => f,
        Err(e) => return e.to_compile_error().into(),
    };

    quote! { Arguments(&[#(#fragments),*]) }.into()
}

#[proc_macro]
pub fn mw_log_format_args_nl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Collect expressions separated by comma.
    // NOTE: `parse_macro_input!` can't be build if function return type is not `TokenStream`.
    // This prevents moving it to a separate function.
    let punctuated = parse_macro_input!(input with Punctuated<Expr, Comma>::parse_terminated);
    let mut punctuated_it = punctuated.into_iter();

    // Parse string format into fragments.
    let mut fragments = match parse_fragments(&mut punctuated_it) {
        Ok(f) => f,
        Err(e) => return e.to_compile_error().into(),
    };

    // Add newline at the end.
    fragments.push(quote! {{
        Fragment::Literal("\n")
    }});

    quote! { Arguments(&[#(#fragments),*]) }.into()
}
