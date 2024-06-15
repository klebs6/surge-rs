crate::ix!();

/*
impl RuntimeDataFactory for AdsrParam {
    #[inline] pub fn new_runtime() -> AdsrParamArrayRT {
        AdsrParamArrayRT::new_with( 
            |x| match x {
                AdsrParam::Attack       => AdsrParamRT::new(AdsrParam::Attack),
                AdsrParam::Decay        => AdsrParamRT::new(AdsrParam::Decay),
                AdsrParam::Sustain      => AdsrParamRT::new(AdsrParam::Sustain),
                AdsrParam::Release      => AdsrParamRT::new(AdsrParam::Release),
                AdsrParam::AttackShape  => AdsrParamRT::new(AdsrParam::AttackShape),
                AdsrParam::DecayShape   => AdsrParamRT::new(AdsrParam::DecayShape),
                AdsrParam::ReleaseShape => AdsrParamRT::new(AdsrParam::ReleaseShape),
                AdsrParam::Mode         => AdsrParamRT::new(AdsrParam::Mode),
            }
        )
    }
}
impl ParameterInterface for AdsrParam {}
*/
pub(crate) fn _synth_parameter_with_runtime(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    let generated = match input {
        Item::Enum(ref e) => generate_synth_parameter_with_runtime(&e.ident, &e),
        _ => return TokenStream::from(quote! {
            compile_error!("`synth_parameter_with_runtime` can only be used with enums");
        }),
    };

    let result = quote! {
        #input
        #generated
    };

    TokenStream::from(result)
}

fn generate_synth_parameter_with_runtime(ident: &syn::Ident, e: &syn::ItemEnum) 
    -> proc_macro2::TokenStream 
{
    let match_arms = generate_match_arms(ident, &e.variants);

    let runtime_impl = quote! {
        impl RuntimeDataFactory for #ident {

            type ParamArrayRT = #ident ArrayRT;

            #[inline]
            pub fn new_runtime() -> Self::ParamArrayRT {
                Self::ParamArrayRT::new_with(|x| match x {
                    #match_arms
                })
            }
        }
    };

    let parameter_interface_impl = quote! {
        impl ParameterInterface for #ident {}
    };

    quote! {
        #runtime_impl
        #parameter_interface_impl
    }
}

fn generate_match_arms(enum_ident: &syn::Ident, variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>) -> proc_macro2::TokenStream {
    let arms = variants.iter().map(|variant| {
        let var_ident = &variant.ident;
        quote! {
            #enum_ident::#var_ident => #enum_ident RT::new(#enum_ident::#var_ident),
        }
    });

    quote! {
        #(#arms)*
    }
}
