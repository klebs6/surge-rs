crate::ix!();

pub(crate) fn _modulation_source_control(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let input = parse_macro_input!(item as Item);

    let generated = match input {
        Item::Struct(ref s) => generate_modulation_source_control_for_struct(&s.ident),
        Item::Enum(ref e)   => generate_modulation_source_control_for_enum(&e.ident),
        _ => return TokenStream::from(quote! {
            compile_error!("`modulation_source_control` can only be used with structs or enums");
        }),
    };

    let result = quote! {
        #input
        #generated
    };

    TokenStream::from(result)
}

pub(crate) fn generate_modulation_source_control_for_struct(ident: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl ModulationSourceControl for #ident {
            // Implement trait methods here
        }
    }
}

pub(crate) fn generate_modulation_source_control_for_enum(ident: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl ModulationSourceControl for #ident {
            // Implement trait methods here
        }
    }
}
