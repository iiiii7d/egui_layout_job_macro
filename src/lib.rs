use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Expr, FieldValue, LitStr, Token, braced, parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, token::{Brace, DotDot}, bracketed, parenthesized};
use syn::parse::End;
use syn::token::{Bracket, Paren, Token};

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
        argument: Option<(Bracket, Expr)>,
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
            let argument = if input.peek(Bracket) {
                let content;
                let bracket = bracketed!(content in input);
                let argument = content.parse::<Expr>()?;
                if !content.peek(End) {
                    return Err(content.error(""));
                }
                Some((bracket, argument))
            } else {
                None
            };
            let content;
            let parentheses = parenthesized!(content in input);
            let mut segments = Vec::new();
            while !input.peek(End) {
                segments.push(input.parse()?);
            }
            Ok(Self::Formatting {
                at_tok,
                keyword,
                argument,
                parentheses,
                segments
            })
        } else {
            Ok(Self::Text(input.parse::<Expr>()?))
        }
    }
}
impl InputSegment {
    fn tokens(&self, token_stream: &mut TokenStream) {
        match self {
            Self::Text(expr) => token_stream.append_all(quote! {
                layout_job.append(
                    &(#expr).to_string(),
                    0.0,
                    egui::text::TextFormat::default(),
                );
            }),
            Self::Formatting { keyword, argument, segments, .. } => for segment in segments {
                segment.tokens(token_stream);
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
        let input_layout_job = if input.peek2(Token![in]) {
            Some(InputLayoutJob::parse(input)?)
        } else {
            None
        };
        let mut segments = Vec::new();
        while !input.peek(End) {
            segments.push(input.parse()?);
        }
        Ok(Self {
            input_layout_job, segments
        })
    }
}

#[proc_macro]
pub fn layout_job(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let InputArgs {input_layout_job, segments } = parse_macro_input!(token_stream as InputArgs);
    let layout_job_tokens = if let Some(InputLayoutJob {input_layout_job, ..}) = input_layout_job {
        input_layout_job.to_token_stream()
    } else {
        quote! { egui::text::LayoutJob::default() }
    };
    let mut segment_tokens = TokenStream::new();
    for segment in segments {
        segment.tokens(&mut segment_tokens);
    };
    quote! {{
        let mut layout_job = #layout_job_tokens;
        #segment_tokens
        layout_job
    }}.into()
}
