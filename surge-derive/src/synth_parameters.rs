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

crate::ix!();

pub fn _synth_parameters(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let input = parse_macro_input!(item as Item);

    let generated = match input {
        Item::Enum(ref e) => generate_synth_parameters(&e.ident, e),
        _ => return TokenStream::from(quote! {
            compile_error!("`synth_parameters` can only be used with enums");
        }),
    };

    let result = quote! {
        #input
        #generated
    };

    TokenStream::from(result)
}

fn generate_synth_parameters(ident: &Ident, e: &ItemEnum) -> proc_macro2::TokenStream {

    let array_type_name    = quote::format_ident!("{}Array", ident);
    let rt_type_name       = quote::format_ident!("{}RT", ident);
    let array_rt_type_name = quote::format_ident!("{}ArrayRT", ident);

    let match_arms = generate_match_arms(ident, &rt_type_name, &e.variants);

    let runtime_impl = quote! {
        impl RuntimeDataFactory for #ident {
            type ParamArrayRT = #array_rt_type_name;

            #[inline]
            fn new_runtime() -> Self::ParamArrayRT {
                Self::ParamArrayRT::new_with(|x| match x {
                    #match_arms
                })
            }
        }
    };

    let parameter_interface_impl = quote! {
        impl ParameterInterface for #ident {}
    };

    let array_type_impl = generate_array_type(ident, &array_type_name);
    let rt_type_impl = generate_rt_types(ident, &rt_type_name, &array_type_name, &array_rt_type_name);

    let count_method  = generate_count_method(ident, &e.variants);
    let try_from_impl = generate_try_from_impl(ident, &e.variants);

    quote! {
        #runtime_impl
        #parameter_interface_impl
        #array_type_impl
        #rt_type_impl
        #count_method
        #try_from_impl
    }
}

fn generate_count_method(enum_ident: &Ident, variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>) -> proc_macro2::TokenStream {
    let variant_count = variants.len();

    quote! {
        impl #enum_ident {
            pub const fn count() -> usize {
                #variant_count
            }
        }
    }
}

fn generate_try_from_impl(enum_ident: &Ident, variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>) -> proc_macro2::TokenStream {
    let from_arms = variants.iter().enumerate().map(|(i, variant)| {
        let var_ident = &variant.ident;
        quote! {
            #i => Ok(#enum_ident::#var_ident),
        }
    });

    quote! {
        impl std::convert::TryFrom<usize> for #enum_ident {
            type Error = &'static str;

            fn try_from(value: usize) -> Result<Self, Self::Error> {
                match value {
                    #(#from_arms)*
                    _ => Err("Index out of bounds"),
                }
            }
        }

        impl std::convert::From<#enum_ident> for usize {
            fn from(param: #enum_ident) -> Self {
                param as usize
            }
        }
    }
}

fn generate_match_arms(
    enum_ident: &Ident,
    rt_type_name: &Ident,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>
) -> proc_macro2::TokenStream {
    let arms = variants.iter().map(|variant| {
        let var_ident = &variant.ident;
        quote! {
            #enum_ident::#var_ident => #rt_type_name::new(#enum_ident::#var_ident),
        }
    });

    quote! {
        #(#arms)*
    }
}

fn generate_array_type(enum_ident: &Ident, array_type_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        pub struct #array_type_name<T> {
            data: [T; #enum_ident::count()]
        }

        impl<T> #array_type_name<T> {
            pub fn new(initial_value: T) -> Self where T: Clone {
                Self::new_with(|_| initial_value.clone())
            }

            pub fn new_with<F>(initial_value: F) -> Self
                where F: Fn(#enum_ident) -> T
            {
                use std::convert::TryFrom;
                use std::mem::{MaybeUninit, forget, replace};
                let mut data: [T; #enum_ident::count()] = unsafe {
                    MaybeUninit::uninit().assume_init()
                };
                for (idx, elem) in data.iter_mut().enumerate() {
                    forget(replace(elem, initial_value(#enum_ident::try_from(idx).unwrap())));
                }
                Self { data }
            }

            pub const fn len(&self) -> usize { #enum_ident::count() }

            pub const fn is_empty(&self) -> bool { self.len() == 0 }

            pub fn iter(&self) -> impl Iterator<Item=&T> {
                self.data.iter()
            }

            pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
                self.data.iter_mut()
            }

            pub fn iter_enumerate(&self) -> impl Iterator<Item=(#enum_ident, &T)> {
                use std::convert::TryFrom;
                self.data.iter().enumerate().map(|(idx, v)| (#enum_ident::try_from(idx).unwrap(), v))
            }

            pub fn iter_mut_enumerate(&mut self) -> impl Iterator<Item=(#enum_ident, &mut T)> {
                use std::convert::TryFrom;
                self.data.iter_mut().enumerate().map(|(idx, v)| (#enum_ident::try_from(idx).unwrap(), v))
            }

            pub fn map<F, Q>(&self, f: F) -> #array_type_name<Q> where F: Fn(&T) -> Q {
                #array_type_name::new_with(|x| f(&self[x]))
            }

            pub fn contains(&self, x: &T) -> bool where T: PartialEq<T> {
                self.data.contains(x)
            }
        }

        impl<T> std::ops::Index<#enum_ident> for #array_type_name<T> {
            type Output = T;
            fn index(&self, x: #enum_ident) -> &Self::Output {
                &self.data[x as usize]
            }
        }

        impl<T> std::ops::IndexMut<#enum_ident> for #array_type_name<T> {
            fn index_mut(&mut self, x: #enum_ident) -> &mut Self::Output {
                &mut self.data[x as usize]
            }
        }

        impl<'a, T> IntoIterator for &'a #array_type_name<T> {
            type Item = &'a T;
            type IntoIter = std::slice::Iter<'a, T>;
            fn into_iter(self) -> std::slice::Iter<'a, T> {
                self.data.iter()
            }
        }

        impl<'a, T> IntoIterator for &'a mut #array_type_name<T> {
            type Item = &'a mut T;
            type IntoIter = std::slice::IterMut<'a, T>;
            fn into_iter(self) -> std::slice::IterMut<'a, T> {
                self.data.iter_mut()
            }
        }

        impl<T> Copy for #array_type_name<T> where T: Copy {}

        impl<T> Clone for #array_type_name<T> where T: Clone {
            fn clone(&self) -> Self {
                Self { data: self.data.clone() }
            }
        }

        impl<T> std::fmt::Debug for #array_type_name<T> where T: std::fmt::Debug {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.data.fmt(f)
            }
        }

        impl<T> PartialEq for #array_type_name<T> where T: PartialEq {
            fn eq(&self, other: &Self) -> bool {
                self.data == other.data
            }
        }

        impl<T> Eq for #array_type_name<T> where T: Eq {}

        impl<T> PartialOrd for #array_type_name<T> where T: PartialOrd {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.data.partial_cmp(&other.data)
            }
        }

        impl<T> Ord for #array_type_name<T> where T: Ord {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.data.cmp(&other.data)
            }
        }

        impl<T> std::hash::Hash for #array_type_name<T> where T: std::hash::Hash {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.data.hash(state);
            }
        }

        impl<T> Default for #array_type_name<T> where T: Default {
            fn default() -> Self {
                Self::new_with(|_| T::default())
            }
        }
    }
}

fn generate_rt_types(enum_ident: &Ident, rt_type_name: &Ident, array_type_name: &Ident, array_rt_type_name: &Ident) -> proc_macro2::TokenStream {
    paste! {
        quote! {
            pub type #rt_type_name = ParamRT<#enum_ident>;
            pub type #array_rt_type_name = #array_type_name<#rt_type_name>;
        }
    }
}
