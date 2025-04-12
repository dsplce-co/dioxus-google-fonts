use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ExprArray, ExprTuple, ExprAssign, Expr, ExprLit, Lit};

#[proc_macro]
pub fn google_fonts_url(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExprArray);
    let mut families = Vec::new();

    for tuple in input.elems {
        let Expr::Tuple(ExprTuple { elems, .. }) = tuple else {
            panic!("Expected tuple: (\"Font Name\", wght = [...], ital = [...])");
        };

        let mut iter = elems.into_iter();
        let name = match iter.next().expect("Font name required") {
            Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => s.value().replace(' ', "+"),
            _ => panic!("Font name must be a string literal"),
        };

        let mut wghts = Vec::new();
        let mut itals = Vec::new();

        for arg in iter {
            if let Expr::Assign(ExprAssign { left, right, .. }) = arg {
                let attr = match *left {
                    Expr::Path(p) => p.path.segments[0].ident.to_string(),
                    _ => panic!("Expected identifier (wght or ital)"),
                };

                match attr.as_str() {
                    "wght" => {
                        if let Expr::Array(arr) = *right {
                            for e in arr.elems {
                                match e {
                                    Expr::Lit(ExprLit { lit: Lit::Int(i), .. }) => {
                                        wghts.push(i.base10_digits().to_string());
                                    }
                                    Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => {
                                        wghts.push(s.value());
                                    }
                                    _ => panic!("Invalid weight value"),
                                }
                            }
                        }
                    }
                    "ital" => {
                        if let Expr::Array(arr) = *right {
                            for e in arr.elems {
                                match e {
                                    Expr::Tuple(tup) if tup.elems.len() == 2 => {
                                        let ital_val = &tup.elems[0];
                                        let wght_val = &tup.elems[1];
                                        match (ital_val, wght_val) {
                                            (
                                                Expr::Lit(ExprLit { lit: Lit::Int(i1), .. }),
                                                Expr::Lit(ExprLit { lit: Lit::Int(i2), .. }),
                                            ) => {
                                                itals.push(format!("{},{}", i1.base10_digits(), i2.base10_digits()));
                                            }
                                            _ => panic!("Invalid ital tuple"),
                                        }
                                    }
                                    _ => panic!("Invalid ital value"),
                                }
                            }
                        }
                    }
                    _ => panic!("Unknown attribute: {}", attr),
                }
            }
        }

        let style = if !itals.is_empty() {
            format!("ital,wght@{}", itals.join(";"))
        } else if !wghts.is_empty() {
            format!("wght@{}", wghts.join(";"))
        } else {
            String::new()
        };

        let family = if style.is_empty() {
            format!("family={}", name)
        } else {
            format!("family={}:{style}", name)
        };

        families.push(family);
    }

    let final_url = format!(
        "https://fonts.googleapis.com/css2?{}&display=swap",
        families.join("&")
    );

    TokenStream::from(quote! {
        #final_url
    })
}

#[proc_macro]
pub fn google_fonts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);

    let output = quote! {
        rsx! {
            document::Link {
                rel: "stylesheet",
                href: {
                    dioxus_google_fonts::google_fonts_url!(#input2)
                }
            }
        }
    };

    output.into()
}
