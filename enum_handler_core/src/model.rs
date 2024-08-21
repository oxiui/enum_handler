use crate::{enum_handler_args::EnumHandlerArgs, Error, Result};
use heck::ToSnakeCase;
use quote::format_ident;
use syn::{Data, DeriveInput, Fields, Ident, Type, Variant, Visibility};

pub struct Enum {
    pub vis: Visibility,
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

impl Enum {
    pub fn new(
        derive_input: &DeriveInput,
        args: &EnumHandlerArgs,
    ) -> Result<Self> {
        let data_enum = match &derive_input.data {
            Data::Enum(data_enum) => data_enum,
            _ => return Err(Error::NotEnum),
        };

        let vis = derive_input
            .vis
            .clone();
        let name = derive_input
            .ident
            .clone();
        let variants: Result<Vec<EnumVariant>> = data_enum
            .variants
            .iter()
            .map(|v| EnumVariant::new(v, name.clone(), args))
            .collect();

        match variants {
            Ok(variants) => Ok(Enum {
                vis,
                name,
                variants,
            }),
            Err(e) => Err(e),
        }
    }
}

pub enum EnumVariantType {
    Unit,
    Tuple,
    Struct,
}

pub struct EnumVariant {
    pub enum_name: Ident,
    pub name: Ident,
    pub variant_type: EnumVariantType,
    pub return_type: Type,
    pub fields: Vec<(Option<Ident>, Type)>,
}

impl EnumVariant {
    pub fn new(
        variant: &Variant,
        enum_name: Ident,
        args: &EnumHandlerArgs,
    ) -> Result<Self> {
        let name = variant
            .ident
            .clone();
        let fields = variant
            .fields
            .iter()
            .map(|field| {
                let name = field
                    .ident
                    .clone();
                let ty = field
                    .ty
                    .clone();
                (name, ty)
            })
            .collect();
        let return_type = args.get_return_type()?;
        let variant_type = match variant.fields {
            Fields::Unit => EnumVariantType::Unit,
            Fields::Named(_) => EnumVariantType::Struct,
            Fields::Unnamed(_) => EnumVariantType::Tuple,
        };
        Ok(EnumVariant {
            enum_name,
            name,
            variant_type,
            return_type,
            fields,
        })
    }

    pub fn get_variant_handler_name(
        &self,
        args: &EnumHandlerArgs,
    ) -> Ident {
        format_ident!(
            "{}_{}",
            args.get_handler_name(),
            self.name
                .to_string()
                .to_snake_case()
        )
    }

    pub fn parameter_name(
        &self,
        field_name: &Option<Ident>,
        index: usize,
    ) -> Ident {
        if let Some(name) = field_name {
            return name.clone();
        }

        if self.is_single_field() {
            format_ident!("arg")
        } else {
            format_ident!("arg{}", index)
        }
    }

    fn is_single_field(&self) -> bool {
        self.fields
            .len()
            == 1
    }
}
