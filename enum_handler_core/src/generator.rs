use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    enum_handler_args::EnumHandlerArgs,
    model::{Enum, EnumVariant, EnumVariantType},
    Result,
};

impl Enum {
    pub fn generate_trait(
        &self,
        args: &EnumHandlerArgs,
    ) -> Result<TokenStream> {
        let visibility = args.visibility(&self.vis)?;
        let trait_name = args.get_trait_name(&self.name);
        let enum_name = &self.name;

        let handler_name = args.get_handler_name();
        let return_type = args.get_return_type()?;

        let handlers = self.generate_handlers(args)?;

        let match_arms = self
            .variants
            .iter()
            .map(|v| v.generate_match_arm(args))
            .collect::<Vec<_>>();

        let async_trait = if args.use_async_trait_macro() {
            quote! {#[async_trait::async_trait]}
        } else {
            quote! {}
        };
        let async_fn = if args.is_async() {
            quote! {async}
        } else {
            quote! {}
        };

        let move_or_borrow = if args.is_move_arguments() {
            quote! {}
        } else {
            quote! {&}
        };

        let output = quote! {
            #async_trait
            #visibility trait #trait_name {
                #async_fn fn #handler_name(&self, e: #move_or_borrow #enum_name) -> #return_type {
                    match (e) {
                        #(#match_arms)*
                    }
                }
                #(#handlers)*
            }
        };
        // println!("{}", output);
        Ok(output)
    }

    pub fn generate_mock(
        &self,
        args: &EnumHandlerArgs,
    ) -> Result<TokenStream> {
        let mock_args = args.clone_for_mock();
        let args = &mock_args;

        let visibility = args.visibility(&self.vis)?;
        let trait_name = args.get_trait_name(&self.name);

        let handlers = self.generate_handlers(args)?;

        let mock_name = args.get_mock_name()?;

        let async_trait = if args.use_async_trait_macro() {
            quote! {#[async_trait::async_trait]}
        } else {
            quote! {}
        };

        let output = quote! {
            #[cfg(test)]
            mockall::mock! {
                #visibility  #mock_name {}
                #async_trait
                impl #trait_name for #mock_name {
                    #(#handlers)*
                }
            }
        };
        // println!("{}", output);
        Ok(output)
    }

    pub fn generate_handlers(
        &self,
        args: &EnumHandlerArgs,
    ) -> Result<Vec<TokenStream>> {
        self.variants
            .iter()
            .map(|v| v.generate_handler(args))
            .collect()
    }
}

impl EnumVariant {
    pub(crate) fn generate_handler(
        &self,
        args: &EnumHandlerArgs,
    ) -> Result<TokenStream> {
        let handler_name = self.get_variant_handler_name(args);
        let return_type = &self.return_type;
        let mut params = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, (name, ty))| {
                let name = self.parameter_name(name, i);
                let ty_string = quote! {#ty}.to_string();
                if args.is_move_arguments() {
                    quote! {
                        #name: #ty
                    }
                } else if ty_string == "String" {
                    quote! {
                        #name: &str
                    }
                } else {
                    quote! {
                        #name: &#ty
                    }
                }
            })
            .collect::<Vec<_>>();

        params.insert(0, quote! {&self});

        let async_fn = if args.is_async() {
            quote! {async}
        } else {
            quote! {}
        };

        if args.is_default_implementation() {
            let return_value = args.get_return_value()?;
            Ok(quote! {
                #async_fn fn #handler_name(#(#params),*) -> #return_type {
                    #return_value
                }
            })
        } else {
            Ok(quote! {
                #async_fn fn #handler_name(#(#params),*) -> #return_type;
            })
        }
    }

    pub(crate) fn generate_match_arm(
        &self,
        args: &EnumHandlerArgs,
    ) -> TokenStream {
        let enum_name = &self.enum_name;
        let variant_name = &self.name;
        let parameters = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, (name, _))| {
                let name = self.parameter_name(name, i);
                quote! { #name }
            })
            .collect::<Vec<_>>();
        let handler_name = self.get_variant_handler_name(args);
        let await_fn = if args.is_async() {
            quote! {.await}
        } else {
            quote! {}
        };

        match self.variant_type {
            EnumVariantType::Unit => quote! {
                #enum_name::#variant_name => { self.#handler_name() #await_fn }
            },
            EnumVariantType::Struct => quote! {
                #enum_name::#variant_name { #(#parameters),* } => { self.#handler_name( #(#parameters),* ) #await_fn }
            },
            EnumVariantType::Tuple => quote! {
                #enum_name::#variant_name( #(#parameters),* ) => { self.#handler_name( #(#parameters),* ) #await_fn }
            },
        }
    }
}
