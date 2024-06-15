crate::ix!();

pub(crate) fn _name(attr: TokenStream, item: TokenStream) -> TokenStream {

    let input       = parse_macro_input!(item as Item);
    let attr_parser = Punctuated::<Lit, Comma>::parse_terminated;

    let attr        = match attr_parser.parse(attr) {
        Ok(attrs) => attrs,
        Err(_) => return TokenStream::from(quote! {
            compile_error!("Expected a string literal as the first argument to the `name` attribute");
        }),
    };

    let name = match attr.first() {
        Some(Lit::Str(lit_str)) => lit_str.value(),
        _ => return TokenStream::from(quote! {
            compile_error!("Expected a string literal as the first argument to the `name` attribute");
        }),
    };

    let generated = match input {
        Item::Struct(ref s) => generate_named_for_struct(&s.ident, &name),
        Item::Enum(ref e) => generate_named_for_enum(&e.ident, &name),
        _ => return TokenStream::from(quote! {
            compile_error!("`name` attribute can only be used with structs or enums");
        }),
    };

    let result = quote! {
        #input
        #generated
    };

    TokenStream::from(result)
}

pub(crate) fn generate_named_for_struct(ident: &syn::Ident, name: &str) -> proc_macro2::TokenStream {
    quote! {
        impl Named for #ident {
            const NAME: &'static str = #name;
        }
    }
}

pub(crate) fn generate_named_for_enum(ident: &syn::Ident, name: &str) -> proc_macro2::TokenStream {
    quote! {
        impl Named for #ident {
            const NAME: &'static str = #name;
        }
    }
}
