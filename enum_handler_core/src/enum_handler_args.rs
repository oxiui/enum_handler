use darling::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_str, Expr, Ident, Type, Visibility};

use crate::{Error, Result};

const DEFAULT_TRAIT_SUFFIX: &str = "Handler";
const DEFAULT_HANDLER_NAME: &str = "on";
const DEFAULT_RETURN_TYPE: &str = "()";

#[derive(Debug, Clone, Default, FromDeriveInput)]
#[darling(default, attributes(enum_handler))]
pub struct EnumHandlerArgs {
    trait_suffix: String,
    trait_name: String,
    handler_name: String,
    return_type: String,
    default_return_value: String,
    is_async: bool,
    default_implementation: bool,
    visibility: String,
    no_async_trait_macro: bool,
    mock_name: String,
    pass_args_by_ref: bool,
}

impl EnumHandlerArgs {
    pub fn get_trait_suffix(&self) -> String {
        if self
            .trait_suffix
            .is_empty()
        {
            return DEFAULT_TRAIT_SUFFIX.to_string();
        }
        self.trait_suffix
            .clone()
    }

    pub fn get_trait_name(
        &self,
        enum_name: &Ident,
    ) -> Ident {
        if self
            .trait_name
            .is_empty()
        {
            format_ident!("{}{}", enum_name, self.get_trait_suffix())
        } else {
            format_ident!("{}", self.trait_name)
        }
    }

    pub fn visibility(
        &self,
        enum_visibility: &Visibility,
    ) -> Result<Visibility> {
        if self
            .visibility
            .is_empty()
        {
            Ok(enum_visibility.clone())
        } else {
            parse_str::<Visibility>(&self.visibility).map_err(Error::from)
        }
    }

    pub fn get_handler_name(&self) -> Ident {
        if self
            .handler_name
            .is_empty()
        {
            return format_ident!("{}", DEFAULT_HANDLER_NAME);
        }
        format_ident!("{}", self.handler_name)
    }

    pub fn get_return_type(&self) -> Result<Type> {
        if self
            .return_type
            .is_empty()
        {
            return parse_str::<Type>(DEFAULT_RETURN_TYPE).map_err(Error::from);
        }
        parse_str::<Type>(&self.return_type).map_err(Error::from)
    }

    pub fn get_return_value(&self) -> Result<TokenStream> {
        if self
            .default_return_value
            .is_empty()
            || self.default_return_value == "()"
        {
            return Ok(quote! {});
        }
        Ok(parse_str::<Expr>(&self.default_return_value)?.to_token_stream())
    }

    pub fn is_generate_mock(&self) -> bool {
        !self
            .mock_name
            .is_empty()
    }

    pub fn get_mock_name(&self) -> Result<Ident> {
        parse_str::<Ident>(&self.mock_name).map_err(Error::from)
    }

    pub fn is_default_implementation(&self) -> bool {
        self.default_implementation
    }

    pub fn is_pass_args_by_ref(&self) -> bool {
        self.pass_args_by_ref
    }

    pub fn is_move_arguments(&self) -> bool {
        !self.is_pass_args_by_ref()
    }

    pub fn use_async_trait_macro(&self) -> bool {
        self.is_async && !self.no_async_trait_macro
    }

    pub fn is_async(&self) -> bool {
        self.is_async
    }

    pub fn clone_for_mock(&self) -> Self {
        EnumHandlerArgs {
            default_implementation: false,
            ..self.clone()
        }
    }
}
