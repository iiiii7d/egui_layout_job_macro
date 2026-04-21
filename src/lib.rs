use std::{borrow::Cow, collections::HashMap};

use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{
    Expr, ExprAssign, ExprLit, ExprPath, FieldValue, Lit, LitByteStr, LitStr, Token, braced,
    bracketed, parenthesized,
    parse::{End, Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Bracket, DotDot, Paren, Token},
};

fn expr2ident(expr: &Expr) -> Option<&Ident> {
    let Expr::Path(ExprPath { path, .. }) = expr else {
        return None;
    };
    path.segments.iter().exactly_one().ok().map(|a| &a.ident)
}

#[derive(Clone, Default)]
struct TextFormat(pub HashMap<Ident, TokenStream>);
impl ToTokens for TextFormat {
    fn to_tokens(&self, token_stream: &mut TokenStream) {
        if self.0.is_empty() {
            token_stream.append_all(quote! { egui::text::TextFormat::default() });
            return;
        }
        let contents = self.0.iter().map(|(k, v)| quote! { #k: #v });
        token_stream.append_all(quote! {
            egui::text::TextFormat {
                #(#contents),*,
                ..egui::text::TextFormat::default()
            }
        })
    }
}
impl TextFormat {
    fn process_font_family(input: &Expr) -> TokenStream {
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
    fn process_colour(input: &Punctuated<Expr, Token![,]>) -> TokenStream {
        match input.iter().count() {
            1 => {
                let value = input.iter().exactly_one().map_err(|_| ()).unwrap();
                if let Some(value) = expr2ident(value) {
                    match &*value.to_string() {
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
                        _ => value.to_token_stream(),
                    }
                } else {
                    value.to_token_stream()
                }
            }
            3 => {
                let mut it = input.iter();
                let r = it.next().expect("");
                let g = it.next().expect("");
                let b = it.next().expect("");
                quote! { egui::Color32::from_rgb(#r, #g, #b) }
            }
            4 => {
                let mut it = input.iter();
                let r = it.next().expect("");
                let g = it.next().expect("");
                let b = it.next().expect("");
                let a = it.next().expect("");
                quote! { egui::Color32::from_rgba_unmultiplied(#r, #g, #b, #a) }
            }
            _ => panic!(),
        }
    }
    fn process_float(input: &Expr) -> TokenStream {
        if let Expr::Lit(ExprLit {
            lit: Lit::Float(value),
            ..
        }) = input
        {
            quote! { #value }
        } else if let Expr::Lit(ExprLit {
            lit: Lit::Int(value),
            ..
        }) = input
        {
            quote! { #value as f32 }
        } else {
            input.to_token_stream()
        }
    }
    fn insert(&mut self, keyword: &Ident, argument: Option<&Punctuated<Expr, Token![,]>>) {
        let (k, v) = match (&*keyword.to_string(), argument) {
            ("font_id", Some(value)) => (keyword.clone(), {
                match value.iter().count() {
                    1 => value.to_token_stream(),
                    2 => {
                        let mut it = value.iter();
                        let size = Self::process_float(it.next().expect(""));
                        let family = Self::process_font_family(it.next().expect(""));
                        quote! { egui::FontId::new(#size, #family) }
                    }
                    _ => panic!(),
                }
            }),
            ("extra_letter_spacing" | "expand_bg", Some(value)) => (
                keyword.clone(),
                Self::process_float(value.iter().exactly_one().map_err(|_| ()).unwrap()),
            ),
            ("line_height", Some(value)) => (keyword.clone(), {
                let value = value.iter().exactly_one().map_err(|_| ()).unwrap();
                if let Expr::Lit(ExprLit {
                    lit: Lit::Float(value),
                    ..
                }) = value
                {
                    quote! { Some(#value) }
                } else if let Expr::Lit(ExprLit {
                    lit: Lit::Int(value),
                    ..
                }) = value
                {
                    quote! { Some(#value as f32) }
                } else {
                    value.to_token_stream()
                }
            }),
            ("color" | "background", Some(value)) => (keyword.clone(), Self::process_colour(value)),
            ("coords", Some(value)) => (keyword.clone(), {
                let values = value.iter().map(|kv| {
                    let Expr::Assign(ExprAssign { left, right, .. }) = kv else {
                        panic!();
                    };
                    let right = Self::process_float(right);
                    if let Some(left) = expr2ident(left) {
                        let left = LitByteStr::new(left.to_string().as_bytes(), left.span());
                        quote! { (#left, #right) }
                    } else {
                        quote! { (#left, #right) }
                    }
                });
                quote! { egui::epaint::text::VariationCoords::new([#(#values),*]) }
            }),
            ("italics", value) => (
                keyword.clone(),
                if let Some(value) = value {
                    value
                        .iter()
                        .exactly_one()
                        .map_err(|_| ())
                        .unwrap()
                        .to_token_stream()
                } else {
                    quote!(true)
                },
            ),
            ("underline" | "strikethrough", Some(value)) => (keyword.clone(), {
                let mut it = value.iter();
                let width = Self::process_float(it.next().expect(""));
                let colour = Self::process_colour(&Punctuated::from_iter(it.cloned()));
                quote! { egui::Stroke::new(#width, #colour) }
            }),
            ("valign", Some(value)) => (keyword.clone(), {
                let value = value.iter().exactly_one().map_err(|_| ()).unwrap();
                if let Some(value) = expr2ident(value) {
                    match &*value.to_string() {
                        "min" => quote! { egui::Align::Min },
                        "center" | "centre" => quote! { egui::Align::Center },
                        "max" => quote! { egui::Align::Max },
                        _ => value.to_token_stream(),
                    }
                } else {
                    value.to_token_stream()
                }
            }),
            (_, None) => panic!(),
            (_, Some(value)) => (keyword.clone(), quote! {(#value)}),
        };
        self.0.insert(k, v);
    }
}

struct InputLayoutJob {
    pub in_tok: Token![in],
    pub input_layout_job: Expr,
    pub colon_tok: Token![:],
}
impl Parse for InputLayoutJob {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            in_tok: input.parse()?,
            input_layout_job: input.parse()?,
            colon_tok: input.parse()?,
        })
    }
}

enum InputSegment {
    Formatting {
        at_tok: Token![@],
        keyword: Ident,
        brackets: Option<Bracket>,
        argument: Option<Punctuated<Expr, Token![,]>>,
        parentheses: Paren,
        segments: Vec<InputSegment>,
    },
    Text(Expr),
}
impl Parse for InputSegment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![@]) {
            let at_tok = input.parse()?;
            let keyword = input.parse()?;
            let (brackets, argument) = if input.peek(Bracket) {
                let content;
                let brackets = bracketed!(content in input);
                let argument = content.parse_terminated(|s| s.parse(), Token![,])?;
                if !content.peek(End) {
                    return Err(content.error(""));
                }
                (Some(brackets), Some(argument))
            } else {
                (None, None)
            };
            let content;
            let parentheses = parenthesized!(content in input);
            let mut segments = Vec::new();
            while !content.peek(End) {
                segments.push(content.parse()?);
            }
            Ok(Self::Formatting {
                at_tok,
                keyword,
                brackets,
                argument,
                parentheses,
                segments,
            })
        } else {
            Ok(Self::Text(input.parse::<Expr>()?))
        }
    }
}
impl InputSegment {
    fn tokens(&self, token_stream: &mut TokenStream, text_format: &TextFormat) {
        match self {
            Self::Text(expr) => token_stream.append_all(quote! {
                layout_job.append(
                    &(#expr).to_string(),
                    0.0,
                    #text_format,
                );
            }),
            Self::Formatting {
                keyword,
                argument,
                segments,
                ..
            } => {
                for segment in segments {
                    let mut text_format2 = text_format.to_owned();
                    text_format2.insert(keyword, argument.as_ref());
                    segment.tokens(token_stream, &text_format2);
                }
            }
        }
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
pub fn layout_job(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let InputArgs {
        input_layout_job,
        segments,
    } = parse_macro_input!(token_stream as InputArgs);
    let layout_job_tokens = if let Some(InputLayoutJob {
        input_layout_job, ..
    }) = input_layout_job
    {
        input_layout_job.to_token_stream()
    } else {
        quote! { egui::text::LayoutJob::default() }
    };
    let mut segment_tokens = TokenStream::new();
    let text_format = TextFormat::default();
    for segment in segments {
        segment.tokens(&mut segment_tokens, &text_format);
    }
    quote! {{
        let mut layout_job = #layout_job_tokens;
        #segment_tokens
        layout_job
    }}
    .into()
}
