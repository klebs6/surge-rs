crate::ix!();

/*
   impl<P: ParameterInterface + ?Sized> ParameterInterface for ParamRT<P> { }
   */
pub(crate) fn _parameter_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let input = parse_macro_input!(item as Item);

    let generated = match input {

        Item::Struct(ref s) => generate_parameter_interface_for_struct(&s.ident, &s.generics),
        Item::Enum(ref e)   => generate_parameter_interface_for_enum(&e.ident, &e.generics),

        _ => return TokenStream::from(quote! {
            compile_error!("`parameter_interface` can only be used with structs or enums");
        }),
    };

    let result = quote! {
        #input
        #generated
    };

    TokenStream::from(result)
}

fn generate_parameter_interface_for_struct(ident: &syn::Ident, generics: &Generics) -> proc_macro2::TokenStream {
    // Extract the generics for the impl block
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate the impl block for the ParameterInterface trait
    quote! {
        impl #impl_generics ParameterInterface for #ident #ty_generics #where_clause {
            // Implement trait methods here
        }
    }
}

fn generate_parameter_interface_for_enum(ident: &syn::Ident, generics: &Generics) -> proc_macro2::TokenStream {
    // Extract the generics for the impl block
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate the impl block for the ParameterInterface trait
    quote! {
        impl #impl_generics ParameterInterface for #ident #ty_generics #where_clause {
            // Implement trait methods here
        }
    }
}
