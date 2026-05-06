//! Macros for [egui](https://github.com/emilk/egui) `LayoutJob` and `TextFormat`.
//!
//! Documentation below assumes `egui` is installed and available, as well as the following imports:
//! ```
//! use egui_layout_job_macro::{layout_job, text_format};
//! use egui::text::LayoutJob;
//! use egui::TextFormat;
//! ```
//! ## Simple
//! Text is passed into [`layout_job`] as literals or expressions with **no commas** between them. Each literal/expression must have a `.to_string()` method.
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!("LayoutJob consisting of one segment");
//!
//! let num_segments = 3;
//! layout_job!("LayoutJob consisting of " num_segments " segments");
//! ```
//!
//! ## From pre-existing `LayoutJob`
//! Useful if your `LayoutJob` requires settings or already has segments in it. Syntax for defining fields for `LayoutJob` directly in the macro is planned.
//! ```
//! # use egui_layout_job_macro::layout_job;
//! # use egui::text::LayoutJob;
//! use egui::text::TextWrapping;
//! let lj = layout_job!(use LayoutJob {
//!     wrap: TextWrapping::wrap_at_width(10.0),
//!     ..LayoutJob::default()
//! }: "Pre-defined LayoutJob");
//! ```
//!
//! ## Formatting
//! Formatting syntax was inspired by LaTeX and Typst.
//!
//! In [`layout_job`], formatting functions have a `@` prefix, optionally take arguments inside square brackets `[]` and are applied to segments inside parentheses `()`.
//! Each function's arguments inside `[]` may have their own syntax. Formatting functions can be nested.
//! ```custom
//! @format(...) <- functions with no arguments
//! @format[...](...) <- functions with arguments
//! ```
//!
//! Arguments accept either literals, identifiers or expressions dpeending on the function command.
//! For float fields, integers and floats will be automatically cast to `f32`, and everything else is treated as an expression and will not be cast automatically to f32. (Note that negative integers currently do not work and will be fixed in a future release)
//! Some fields have identifiers as possible values (e.g. colours, backgrounds). Any unrecognised identifier is assumed to be an expression.
//! To force an argument to be an expression, wrap it in parentheses `()`.
//! ```custom
//! // expand_bg accepts one float
//! @expand_bg[2.0](...) <- OK
//! @expand_bg[2](...) <- OK
//! @expand_bg[(2.0f32)](...) <- OK
//! @expand_bg[float_variable](...) <- OK if variable exists
//! @expand_bg["hello"](...) <- NOT OK
//!
//! // col (1 argument) accepts one Color32
//! @col[red](...) <- OK
//! @col[egui::Color32::RED](...) <- OK
//! @col[(red)](...) <- OK, variable `red` will be used
//! @col[magic](...) <- OK if variable `magic` does not exist, NOT OK if not
//! ```
//!
//! To get a `TextFormat` of just the formatting (no segments or text to format), use [`text_format`] and list the functions without the `@` prefix and **separated by commas**:
//! ```
//! # use egui_layout_job_macro::text_format;
//! let tf = text_format!(red, u[2]);
//! ```
//!
//! ### Foreground & Background Colour
//! The full form is
//! ```ignore
//! layout_job!(@color[args...](...) @background[args...](...));
//! ```
//!
//! For pre-defined colours in `egui::Color32`, just write the name of the colour in the args in snake case.
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@color[light_green]("light green text") @background[light_green]("light green background"));
//! ```
//!
//! Pre-defined colours have a shorter form. For background colours prepend `bg_`:
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@light_green("light green text") @bg_light_green("light green background"));
//! ```
//!
//! For custom colours, you can do so using the hex value (requires `egui` feature `color-hex`), RGB or RGBA:
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@color["90ee90"]("this is ") @color[144, 238, 144]("all ") @color[144, 238, 144, 255]("light green"));
//! ```
//!
//! `color` and `background` have shorthands:
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@color[red]("these all ") @colour[red]("specify ") @col[red]("colour"));
//! layout_job!(@background[red]("and these ") @bg[red]("specify background"));
//! ```
//!
//! ### Font ID (Family/Size)
//! A `FontID` is made of a font size (float) and a font family (see below). However, you can specify only the size and the font family.
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@font_id[20, mono]("Monospace with size 20"));
//! layout_job!(@size[20]("Size 20 only"));
//! layout_job!(@family[mono]("Monospace only"));
//! ```
//!
//! For font families, the three options are `mono`, `prop` and custom family (any other expressions or literals)
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@family[mono]("Monospace") @family[prop]("Proportional") @family["custom"]("Custom font family"));
//! ```
//!
//! `mono` and `prop` have shorthands:
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@mono("Monospace") @prop("Proportional"));
//! ```
//!
//! ### Italics
//! Most basic is just `italics` or `i`:
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@italics("italics") @i("also italics"));
//! ```
//!
//! It accepts one optional boolean argument to toggle italics (for e.g. italics off in the middle of italicised text)
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@i[false]("not italics"));
//! ```
//!
//! ### Underline / Strikethrough
//! Most basic is just `underline`/`strikethrough` or `u`/`s`, defaulting to width 1.0 and same colour as the text:
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@underline("underline") @u("also underline"));
//! layout_job!(@strikethrough("strikethrough") @s("also strikethrough"));
//! ```
//!
//! Both accept optional arguments. The first is the width of the line (float):
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@u[2]("thick underline") @s[2]("thick strikethrough"));
//! ```
//!
//! The second argument onwards is the colour of the line. See "Foreground & Background Colour" for the syntax.
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@u[2, red]("red underline") @s[2, 255, 0, 0]("red strikethrough"));
//! ```
//!
//! ### Extra Letter Spacing / Expand Background
//! Both accept one optional float argument.
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@extra_letter_spacing[2]("extra letter spacing") @expand_bg[2]("expand background"));
//! ```
//!
//! ### Line Height
//! Similar to the above, but any non-literal expression passed to it must be an `Option`.
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@line_height[2]("line height 2") @line_height[None]("default line height"));
//! ```
//!
//! ### Vertical Alignment
//! Possible options are `min`, `centre`/`center`, `max`, `top`, `bottom` and any custom literal/expression that is an `egui::Align`.
//! A `super` and `sub` function that combines this with `size` is planned for the future.
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@valign[top]("superscript") @valign[bottom]("subscript"));
//! layout_job!(@top("these are ") @bottom("shorthands"));
//! ```
//!
//! ### Coords
//! I have no clue what this does, but I have implemented a syntax anyway. The special format for arguments is `key=value`,... where `key` is a bytestring that can be specified with or without byte quotes (`b""`), and `value` is a float.
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!(@coords[wght=500,b"wdth"=75]("weight=500, width=75"));
//! ```
//!
//! ### Custom `TextFormat`
//! The syntax is `@[text_format_expr](...)`. Useful for pre-defining `TextFormat` combinations and using them throughout your project.
//! ```
//! # use egui_layout_job_macro::{layout_job, text_format};
//! let impt = text_format!(white, bg_red, u, i);
//! layout_job!(@[impt]("this is very important"));
//! ```
//!
//! ## Leading Space
//! The default leading space before each segment is 0.0. You can specify a custom leading space with the syntax `~number` before any segment.
//! ```
//! # use egui_layout_job_macro::layout_job;
//! layout_job!("This leaves" ~10 "a lot of space");
//! ```
//!
//! Note that this cannot be done before a *formatting function*.
//! ```compile_fail
//! # use egui_layout_job_macro::layout_job;
//! layout_job!("This should not" ~10 @red("be done"));
//! ```
//!
//! ## Custom Segment
//! Fundamentally, appending a segment requires 1. a segment of text, 2. a leading space, 3. a `TextFormat`. You can pass all 3 directly as a tuple of `(text, leading space, text format)` with `#expression`:
//! ```
//! # use egui_layout_job_macro::layout_job;
//! # use egui::TextFormat;
//! let segment = ("custom segment", 1.0, TextFormat::default());
//! layout_job!("This is a " #segment);
//! ```

use std::collections::HashMap;

use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{
    Expr, ExprAssign, ExprLit, ExprPath, Lit, LitByteStr, Token, bracketed, parenthesized,
    parse::{End, Parse, ParseBuffer, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Bracket, Paren},
};

fn expr2ident(expr: &Expr) -> Option<&Ident> {
    let Expr::Path(ExprPath { path, .. }) = expr else {
        return None;
    };
    path.segments.iter().exactly_one().ok().map(|a| &a.ident)
}

#[derive(Clone)]
struct TextFormat {
    pub attrs: HashMap<Ident, TokenStream>,
    pub default: TokenStream,
}
impl TextFormat {
    pub fn new() -> Self {
        Self {
            attrs: HashMap::new(),
            default: quote! { egui::text::TextFormat::default() },
        }
    }
    pub fn new_with_default(default: TokenStream) -> Self {
        Self {
            attrs: HashMap::new(),
            default,
        }
    }
}
impl ToTokens for TextFormat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.attrs.is_empty() {
            tokens.append_all(self.default.to_token_stream());
            return;
        }
        let contents = self.attrs.iter().map(|(k, v)| quote! { #k: #v });
        let default = &self.default;
        tokens.append_all(quote! {
            egui::text::TextFormat {
                #(#contents),*,
                ..#default
            }
        });
    }
}
impl TextFormat {
    fn process_font_family(input: &Expr) -> TokenStream {
        #[expect(clippy::option_if_let_else)]
        if let Some(input) = expr2ident(input) {
            match &*input.to_string() {
                "mono" | "monospace" => quote! { egui::FontFamily::Monospace },
                "prop" | "proportional" => quote! { egui::FontFamily::Proportional },
                _ => input.to_token_stream(),
            }
        } else if let Expr::Lit(ExprLit {
            lit: Lit::Str(input),
            ..
        }) = input
        {
            quote! { egui::FontFamily::Name((#input).into()) }
        } else {
            input.to_token_stream()
        }
    }
    fn colour2tokens(input: &str) -> Option<TokenStream> {
        Some(match input {
            "transparent" => quote! { egui::Color32::TRANSPARENT },
            "black" => quote! { egui::Color32::BLACK },
            "dark_gray" | "dark_grey" => quote! { egui::Color32::DARK_GRAY },
            "gray" | "grey" => quote! { egui::Color32::GRAY },
            "light_gray" | "light_grey" => quote! { egui::Color32::LIGHT_GRAY },
            "white" => quote! { egui::Color32::WHITE },
            "brown" => quote! { egui::Color32::BROWN },
            "dark_red" => quote! { egui::Color32::DARK_RED },
            "red" => quote! { egui::Color32::RED },
            "light_red" => quote! { egui::Color32::LIGHT_RED },
            "cyan" => quote! { egui::Color32::CYAN },
            "magenta" => quote! { egui::Color32::MAGENTA },
            "yellow" => quote! { egui::Color32::YELLOW },
            "orange" => quote! { egui::Color32::ORANGE },
            "light_yellow" => quote! { egui::Color32::LIGHT_YELLOW },
            "khaki" => quote! { egui::Color32::KHAKI },
            "dark_green" => quote! { egui::Color32::DARK_GREEN },
            "green" => quote! { egui::Color32::GREEN },
            "light_green" => quote! { egui::Color32::LIGHT_GREEN },
            "dark_blue" => quote! { egui::Color32::DARK_BLUE },
            "blue" => quote! { egui::Color32::BLUE },
            "light_blue" => quote! { egui::Color32::LIGHT_BLUE },
            "purple" => quote! { egui::Color32::PURPLE },
            "gold" => quote! { egui::Color32::GOLD },
            "debug_color" | "debug" | "debug_colour" => {
                quote! { egui::Color32::DEBUG_COLOR }
            }
            "placeholder" => quote! { egui::Color32::PLACEHOLDER },
            _ => return None,
        })
    }
    fn align2tokens(input: &str) -> Option<TokenStream> {
        Some(match input {
            "min" => quote! { egui::Align::Min },
            "center" | "centre" => quote! { egui::Align::Center },
            "max" => quote! { egui::Align::Max },
            "top" => quote! { egui::Align::TOP },
            "bottom" => quote! { egui::Align::BOTTOM },
            _ => return None,
        })
    }
    fn process_colour(input: &Punctuated<Expr, Token![,]>) -> syn::Result<TokenStream> {
        Ok(match input.iter().count() {
            1 => {
                let value = input.get(0).unwrap();
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(value),
                    ..
                }) = value
                {
                    quote! { egui::hex_color!(#value) }
                } else {
                    expr2ident(value)
                        .and_then(|value| Self::colour2tokens(&value.to_string()))
                        .unwrap_or_else(|| value.to_token_stream())
                }
            }
            3 => {
                let (r, g, b) = input.iter().collect_tuple().unwrap();
                quote! { egui::Color32::from_rgb(#r, #g, #b) }
            }
            4 => {
                let (r, g, b, a) = input.iter().collect_tuple().unwrap();
                quote! { egui::Color32::from_rgba_unmultiplied(#r, #g, #b, #a) }
            }
            count => {
                return Err(syn::Error::new_spanned(
                    input,
                    format!("Did not expect {count} args for colour"),
                ));
            }
        })
    }
    pub(crate) fn process_float(input: &Expr) -> TokenStream {
        match input {
            Expr::Lit(ExprLit {
                lit: Lit::Float(value),
                ..
            }) => quote! { #value as f32 },
            Expr::Lit(ExprLit {
                lit: Lit::Int(value),
                ..
            }) => quote! { #value as f32 },
            value => value.to_token_stream(),
        }
    }
    #[expect(clippy::too_many_lines)]
    fn insert(&mut self, f: &FormatAttr) -> syn::Result<()> {
        let (k, v) = match &*f.key_string() {
            "font_id" => (f.key.clone(), {
                match f.args_count() {
                    1 => f.get_arg(0).to_token_stream(),
                    2 => {
                        let size = Self::process_float(f.get_arg(0));
                        let family = Self::process_font_family(f.get_arg(1));
                        quote! { egui::FontId::new(#size, #family) }
                    }
                    count => {
                        return Err(syn::Error::new_spanned(
                            &f.args,
                            format!("Did not expect {count} args"),
                        ));
                    }
                }
            }),
            "extra_letter_spacing" | "expand_bg" => {
                (f.key.clone(), Self::process_float(f.get_one_arg()?))
            }
            "line_height" => (
                f.key.clone(),
                match f.get_one_arg()? {
                    Expr::Lit(ExprLit {
                        lit: Lit::Float(value),
                        ..
                    }) => quote! { Some(#value as f32) },
                    Expr::Lit(ExprLit {
                        lit: Lit::Int(value),
                        ..
                    }) => quote! { Some(#value as f32) },
                    value => value.to_token_stream(),
                },
            ),
            key @ ("color" | "colour" | "col" | "background" | "bg") => (
                match key {
                    "colour" | "col" => Ident::new("color", f.key.span()),
                    "bg" => Ident::new("background", f.key.span()),
                    _ => f.key.clone(),
                },
                Self::process_colour(&f.args)?,
            ),
            "coords" => (f.key.clone(), {
                let values = f
                    .args
                    .iter()
                    .map(|kv| {
                        let Expr::Assign(ExprAssign { left, right, .. }) = kv else {
                            return Err(syn::Error::new_spanned(
                                kv,
                                "Not key-value pair, e.g. `key=value`",
                            ));
                        };
                        let right = Self::process_float(right);
                        Ok(expr2ident(left).map_or_else(
                            || quote! { (#left, #right) },
                            |left| {
                                let left =
                                    LitByteStr::new(left.to_string().as_bytes(), left.span());
                                quote! { (#left, #right) }
                            },
                        ))
                    })
                    .collect::<syn::Result<Vec<_>>>()?;
                quote! { egui::epaint::text::VariationCoords::new([#(#values),*]) }
            }),
            key @ ("italics" | "i") => (
                if key == "i" {
                    Ident::new("italics", f.key.span())
                } else {
                    f.key.clone()
                },
                match f.args_count() {
                    0 => quote!(true),
                    1 => f.get_arg(0).to_token_stream(),
                    count => {
                        return Err(syn::Error::new_spanned(
                            &f.args,
                            format!("Did not expect {count} args"),
                        ));
                    }
                },
            ),
            key @ ("underline" | "u" | "strikethrough" | "s") => (
                match key {
                    "u" => Ident::new("underline", f.key.span()),
                    "s" => Ident::new("strikethrough", f.key.span()),
                    _ => f.key.clone(),
                },
                {
                    let mut it = f.args.iter().peekable();
                    let width = it
                        .next()
                        .map_or_else(|| quote!(1.0f32), Self::process_float);
                    let colour = if it.peek().is_some() {
                        Self::process_colour(&it.cloned().collect())?
                    } else if let Some((_, colour)) = self.attrs.iter().find(|(k, _)| *k == "color")
                    {
                        colour.clone()
                    } else {
                        let default = &self.default;
                        quote! { #default.color }
                    };
                    quote! { egui::Stroke::new(#width, #colour) }
                },
            ),
            "valign" => (f.key.clone(), {
                let value = f.get_one_arg()?;
                expr2ident(value)
                    .and_then(|value| Self::align2tokens(&value.to_string()))
                    .unwrap_or_else(|| value.to_token_stream())
            }),

            "size" => (Ident::new("font_id", f.key.span()), {
                let size = Self::process_float(f.get_one_arg()?);
                let family = if let Some((_, font_id)) =
                    self.attrs.iter().find(|(key, _)| *key == "font_id")
                {
                    quote! { #font_id.family.clone() }
                } else if let Some((_, family)) =
                    self.attrs.iter().find(|(key, _)| *key == "family")
                {
                    quote! { #family.clone() }
                } else {
                    let default = &self.default;
                    quote! { #default.font_id.family.clone() }
                };
                quote! { egui::FontId::new(#size, #family) }
            }),
            key @ ("family" | "prop" | "proportional" | "mono" | "monospace") => {
                (Ident::new("font_id", f.key.span()), {
                    let family = match key {
                        "prop" | "proportional" => quote! { egui::FontFamily::Proportional },
                        "mono" | "monospace" => quote! { egui::FontFamily::Monospace },
                        "family" => Self::process_font_family(f.get_one_arg()?),
                        _ => unreachable!(),
                    };
                    let size = if let Some((_, font_id)) =
                        self.attrs.iter().find(|(key, _)| *key == "font_id")
                    {
                        quote! { #font_id.size }
                    } else if let Some((_, size)) =
                        self.attrs.iter().find(|(key, _)| *key == "size")
                    {
                        size.clone()
                    } else {
                        let default = &self.default;
                        quote! { #default.font_id.size }
                    };
                    quote! { egui::FontId::new(#size, #family) }
                })
            }
            key => {
                if let Ok(value) = f.get_one_arg() {
                    (f.key.clone(), value.to_token_stream())
                } else if f.args_count() == 0 {
                    #[expect(clippy::option_if_let_else)]
                    if let Some(colour) =
                        Self::colour2tokens(key.strip_prefix("bg_").unwrap_or(key))
                    {
                        let new_key = if key.starts_with("bg_") {
                            "background"
                        } else {
                            "color"
                        };
                        (Ident::new(new_key, f.key.span()), colour)
                    } else if let Some(valign) = Self::align2tokens(key) {
                        (Ident::new("valign", f.key.span()), valign)
                    } else {
                        (f.key.clone(), quote! { Default::default() })
                    }
                } else {
                    return Err(syn::Error::new_spanned(&f.key, "Unknown key"));
                }
            }
        };
        self.attrs.insert(k, v);
        Ok(())
    }
}

struct InputLayoutJob {
    #[expect(dead_code)]
    pub use_tok: Token![use],
    pub contents: Expr,
    #[expect(dead_code)]
    pub colon_tok: Token![:],
}
impl Parse for InputLayoutJob {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            use_tok: input.parse()?,
            contents: input.parse()?,
            colon_tok: input.parse()?,
        })
    }
}

struct FormatAttr {
    pub key: Ident,
    #[expect(dead_code)]
    pub brackets: Option<Bracket>,
    pub args: Punctuated<Expr, Token![,]>,
}
impl Parse for FormatAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let (brackets, args) = if input.peek(Bracket) {
            let content;
            let brackets = bracketed!(content in input);
            let argument = content.parse_terminated(ParseBuffer::parse, Token![,])?;
            if !content.peek(End) {
                return Err(content.error("Expected end of bracketed content"));
            }
            (Some(brackets), argument)
        } else {
            (None, Punctuated::new())
        };
        Ok(Self {
            key,
            brackets,
            args,
        })
    }
}
impl FormatAttr {
    pub fn key_string(&self) -> String {
        self.key.to_string()
    }
    pub fn args_count(&self) -> usize {
        self.args.iter().count()
    }
    pub fn get_arg(&self, index: usize) -> &Expr {
        self.args.get(index).unwrap_or_else(|| unreachable!())
    }
    pub fn get_one_arg(&self) -> syn::Result<&Expr> {
        self.args.iter().exactly_one().map_err(|e| {
            syn::Error::new_spanned(&self.args, format!("Did not get one argument: {e}"))
        })
    }
}

enum InputSegment {
    FormatAttr {
        #[expect(dead_code)]
        at_tok: Token![@],
        attr: FormatAttr,
        #[expect(dead_code)]
        parentheses: Paren,
        segments: Vec<Self>,
    },
    TextFormat {
        #[expect(dead_code)]
        at_tok: Token![@],
        #[expect(dead_code)]
        brackets: Bracket,
        expr: Expr,
        #[expect(dead_code)]
        parentheses: Paren,
        segments: Vec<Self>,
    },
    Text {
        text: Expr,
        #[expect(dead_code)]
        leading_space_tilde_tok: Option<Token![~]>,
        leading_space: Option<Expr>,
    },
    Raw {
        #[expect(dead_code)]
        pound_tok: Token![#],
        tuple: Expr,
    },
}
impl Parse for InputSegment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![#]) {
            return Ok(Self::Raw {
                pound_tok: input.parse()?,
                tuple: input.parse()?,
            });
        }
        if input.peek(Token![~]) {
            return Ok(Self::Text {
                leading_space_tilde_tok: Some(input.parse()?),
                leading_space: Some(input.parse()?),
                text: input.parse()?,
            });
        }
        if !input.peek(Token![@]) {
            return Ok(Self::Text {
                text: input.parse()?,
                leading_space_tilde_tok: None,
                leading_space: None,
            });
        }

        let at_tok = input.parse()?;
        if input.peek(Bracket) {
            let content;
            let brackets = bracketed!(content in input);
            let expr = content.parse()?;
            if !content.peek(End) {
                return Err(content.error("Expected end of bracketed content"));
            }

            let content;
            let parentheses = parenthesized!(content in input);
            let mut segments = Vec::new();
            while !content.peek(End) {
                segments.push(content.parse()?);
            }
            Ok(Self::TextFormat {
                at_tok,
                brackets,
                expr,
                parentheses,
                segments,
            })
        } else {
            let function = input.parse()?;

            let content;
            let parentheses = parenthesized!(content in input);
            let mut segments = Vec::new();
            while !content.peek(End) {
                segments.push(content.parse()?);
            }
            Ok(Self::FormatAttr {
                at_tok,
                attr: function,
                parentheses,
                segments,
            })
        }
    }
}
impl InputSegment {
    fn tokens(&self, tokens: &mut TokenStream, text_format: &TextFormat) -> syn::Result<()> {
        match self {
            Self::Raw { tuple, .. } => tokens.append_all(quote! {
                let (string, leading_space, text_format) = #tuple;
                layout_job.append(string, leading_space, text_format);
            }),
            Self::Text {
                text,
                leading_space,
                ..
            } => {
                let leading_space = leading_space
                    .as_ref()
                    .map_or_else(|| quote!(0.0), TextFormat::process_float);
                tokens.append_all(quote! {
                    layout_job.append(
                        &(#text).to_string(),
                        #leading_space,
                        #text_format,
                    );
                });
            }
            Self::TextFormat { expr, segments, .. } => {
                for segment in segments {
                    let text_format2 = TextFormat::new_with_default(quote! { (#expr).clone() });
                    segment.tokens(tokens, &text_format2)?;
                }
            }
            Self::FormatAttr {
                attr: function,
                segments,
                ..
            } => {
                for segment in segments {
                    let mut text_format2 = text_format.to_owned();
                    text_format2.insert(function)?;
                    segment.tokens(tokens, &text_format2)?;
                }
            }
        }
        Ok(())
    }
}

struct InputArgs {
    pub input_layout_job: Option<InputLayoutJob>,
    pub segments: Vec<InputSegment>,
}
impl Parse for InputArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_layout_job = if input.peek(Token![use]) {
            Some(InputLayoutJob::parse(input)?)
        } else {
            None
        };
        let mut segments = Vec::new();
        while !input.peek(End) {
            segments.push(input.parse()?);
        }
        Ok(Self {
            input_layout_job,
            segments,
        })
    }
}

#[cfg_attr(test, no_panic::no_panic)]
#[proc_macro]
pub fn layout_job(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    layout_job_(parse_macro_input!(tokens as InputArgs))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn layout_job_(
    InputArgs {
        input_layout_job,
        segments,
    }: InputArgs,
) -> syn::Result<TokenStream> {
    let layout_job_tokens = if let Some(InputLayoutJob {
        contents: input_layout_job,
        ..
    }) = input_layout_job
    {
        input_layout_job.to_token_stream()
    } else {
        quote! { egui::text::LayoutJob::default() }
    };
    let mut segment_tokens = TokenStream::new();
    let text_format = TextFormat::new();
    for segment in segments {
        segment.tokens(&mut segment_tokens, &text_format)?;
    }
    Ok(quote! {{
        let mut layout_job = #layout_job_tokens;
        #segment_tokens
        layout_job
    }})
}

#[cfg_attr(test, no_panic::no_panic)]
#[proc_macro]
pub fn text_format(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    text_format_(
        parse_macro_input!(tokens with Punctuated<FormatAttr, Token![,]>::parse_terminated),
    )
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

fn text_format_(attrs: Punctuated<FormatAttr, Token![,]>) -> syn::Result<TokenStream> {
    let mut tf = TextFormat::new();
    for attr in attrs {
        tf.insert(&attr)?;
    }
    Ok(tf.to_token_stream())
}
