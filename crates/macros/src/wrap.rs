use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{
        Error,
        LitInt,
        LitStr,
        Result,
        Token,
        parse::{Parse, ParseStream},
        parse_macro_input,
    },
};

struct Input {
    text: LitStr,
    width: Option<LitInt>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let text: LitStr = input.parse()?;

        let width = if input.is_empty() {
            None
        } else {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        };

        Ok(Self { text, width })
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let Input { text, width } = parse_macro_input!(input as Input);

    let max_width = match width {
        Some(w) => {
            match w.base10_parse::<usize>() {
                Ok(0) => {
                    return Error::new_spanned(w, "width must be greater than 0")
                        .to_compile_error()
                        .into();
                }

                Ok(v) => v,

                Err(e) => {
                    return Error::new_spanned(w, format!("invalid width: {e}"))
                        .to_compile_error()
                        .into();
                }
            }
        }
        
        None => 80,
    };

    let source = text.value();
    let mut out = String::with_capacity(source.len());

    for (i, paragraph) in source.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }

        let mut line_len = 0usize;

        for word in paragraph.split_whitespace() {
            let word_len = word.len();

            if line_len == 0 {
                out.push_str(word);
                line_len = word_len;
            } else if line_len + 1 + word_len > max_width {
                out.push('\n');
                out.push_str(word);
                line_len = word_len;
            } else {
                out.push(' ');
                out.push_str(word);
                line_len += 1 + word_len;
            }
        }
    }

    let lit = LitStr::new(&out, text.span());
    quote!(#lit).into()
}
