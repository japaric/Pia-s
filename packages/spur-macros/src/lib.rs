use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Ident, parse_macro_input};
use syntax::Subscriptions;

mod codegen;
mod syntax;

#[proc_macro_derive(Message)]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input as DeriveInput);

    let krate = crate_name();

    quote!(
        impl #krate::Message for #ident {}
    )
    .into()
}

#[proc_macro_attribute]
pub fn subscriptions(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    if !attr_args.is_empty() {
        return quote!(compile_error!(
            "`#[subscriptions]` attribute takes no arguments"
        ))
        .into();
    }

    let subscriptions = parse_macro_input!(item as Subscriptions);

    codegen::subscriptions(&subscriptions)
}

fn crate_name() -> Ident {
    let suffixed_crate_name = env!("CARGO_CRATE_NAME");
    let crate_name = suffixed_crate_name
        .strip_suffix("_macros")
        .expect("crate name does not end in '_macros'");
    format_ident!("{}", crate_name)
}
