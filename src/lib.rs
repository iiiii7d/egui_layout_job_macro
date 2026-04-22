use std::collections::HashMap;

use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{
    Expr, ExprAssign, ExprLit, ExprPath, Lit, LitByteStr, Token, bracketed, parenthesized,
    parse::{End, Parse, ParseBuffer, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
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
            _ => return None,
        })
    }
    fn process_colour(input: &Punctuated<Expr, Token![,]>) -> TokenStream {
        match input.iter().count() {
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
            _ => panic!(),
        }
    }
    fn process_float(input: &Expr) -> TokenStream {
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
                    _ => return Err(syn::Error::new(f.args.span(), "")),
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
                Self::process_colour(&f.args),
            ),
            "coords" => (f.key.clone(), {
                let values = f.args.iter().map(|kv| {
                    let Expr::Assign(ExprAssign { left, right, .. }) = kv else {
                        panic!();
                    };
                    let right = Self::process_float(right);
                    expr2ident(left).map_or_else(
                        || quote! { (#left, #right) },
                        |left| {
                            let left = LitByteStr::new(left.to_string().as_bytes(), left.span());
                            quote! { (#left, #right) }
                        },
                    )
                });
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
                    _ => return Err(syn::Error::new(f.args.span(), "")),
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
                        Self::process_colour(&it.cloned().collect())
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
                let default = &self.default;
                quote! { egui::FontId::new(#size, #default.font_id.family.clone()) }
            }),
            "font_family" => (Ident::new("font_id", f.key.span()), {
                let family = Self::process_font_family(f.get_one_arg()?);
                let default = &self.default;
                quote! { egui::FontId::new(#default.font_id.size, #family) }
            }),
            "prop" | "proportional" => (Ident::new("font_id", f.key.span()), {
                let default = &self.default;
                quote! { egui::FontId::new(#default.font_id.size, egui::FontFamily::Proportional) }
            }),
            "mono" | "monospace" => (Ident::new("font_id", f.key.span()), {
                let default = &self.default;
                quote! { egui::FontId::new(#default.font_id.size, egui::FontFamily::Monospace) }
            }),
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
                    return Err(syn::Error::new(f.args.span(), ""));
                }
            }
        };
        self.attrs.insert(k, v);
        Ok(())
    }
}

struct InputLayoutJob {
    #[expect(dead_code)]
    pub in_tok: Token![in],
    pub contents: Expr,
    #[expect(dead_code)]
    pub colon_tok: Token![:],
}
impl Parse for InputLayoutJob {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            in_tok: input.parse()?,
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
                return Err(content.error(""));
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
        self.args
            .iter()
            .exactly_one()
            .map_err(|_e| syn::Error::new(self.args.span(), ""))
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
    Text(Expr),
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
            })
        }
        if !(input.peek(Token![@])) {
            return Ok(Self::Text(input.parse::<Expr>()?));
        }

        let at_tok = input.parse()?;
        if input.peek(Bracket) {
            let content;
            let brackets = bracketed!(content in input);
            let expr = content.parse()?;
            if !content.peek(End) {
                return Err(content.error(""));
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
            Self::Text(expr) => tokens.append_all(quote! {
                layout_job.append(
                    &(#expr).to_string(),
                    1.0,
                    #text_format,
                );
            }),
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
        let input_layout_job = if input.peek(Token![in]) {
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

#[proc_macro]
pub fn layout_job(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let InputArgs {
        input_layout_job,
        segments,
    } = parse_macro_input!(tokens as InputArgs);
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
        segment.tokens(&mut segment_tokens, &text_format).unwrap();
    }
    quote! {{
        let mut layout_job = #layout_job_tokens;
        #segment_tokens
        layout_job
    }}
    .into()
}

#[proc_macro]
pub fn text_format(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attrs = parse_macro_input!(tokens with Punctuated<FormatAttr, Token![,]>::parse_terminated);
    let mut tf = TextFormat::new();
    for attr in attrs {
        tf.insert(&attr).unwrap();
    }
    tf.to_token_stream().into()
}
