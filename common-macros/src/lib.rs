use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{ItemFn, LitInt, LitStr, Token, parse_macro_input};

#[proc_macro_attribute]
pub fn timed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<LitStr, Token![,]>::parse_terminated);
    let func = parse_macro_input!(item as ItemFn);

    if args.len() > 1 {
        return syn::Error::new_spanned(
            &func.sig.ident,
            "timed expects max 1 argument: #[timed(\"Name\")]",
        )
        .to_compile_error()
        .into();
    }

    let (attrs, vis, sig, block) = (&func.attrs, &func.vis, &func.sig, &func.block);
    let label = match args.first() {
        Some(lit) => lit.value(),
        None => sig.ident.to_string(),
    };

    quote! {
        #(#attrs)* #vis #sig {
            let __start = std::time::Instant::now();
            let __result = (move || #block)();
            println!("{} took {:.0?}", #label, __start.elapsed());
            __result
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn aoc_input(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<LitInt, Token![,]>::parse_terminated);

    let func = parse_macro_input!(item as ItemFn);

    if args.len() != 2 {
        return syn::Error::new_spanned(
            &func.sig.ident,
            "aoc_input expects two arguments: #[aoc_input(year, day)]",
        )
        .to_compile_error()
        .into();
    }

    if func.sig.inputs.len() != 1 {
        return syn::Error::new_spanned(
            &func.sig.ident,
            "aoc_input expects one parameter, e.g. fn main(input: String)",
        )
        .to_compile_error()
        .into();
    }

    let (year, day) = (&args[0], &args[1]);
    let attrs = &func.attrs;
    let param = &func.sig.inputs;
    let output = &func.sig.output;
    let block = &func.block;

    quote! {
        #(#attrs)* fn main() #output {
            let __input = ::aoc_client::input(#year, #day);
            (move |#param| #block)(__input)
        }
    }
    .into()
}
