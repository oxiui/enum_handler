use quote::quote;

use crate::Error;

use super::*;

fn assert_tokens_eq(
    expected: &TokenStream,
    actual: &TokenStream,
) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if expected != actual {
        println!(
            "{}",
            colored_diff::PrettyDifference {
                expected: &expected,
                actual: &actual,
            }
        );
        println!("expected: {}", &expected);
        println!("actual  : {}", &actual);
        panic!("expected != actual");
    }
}
#[test]
fn test_visibility_default_module() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        enum Visibility {
            Unit,
        }
    })
    .unwrap();

    let expected = quote! {
        trait VisibilityHandler {
            fn on(&self, e: Visibility) -> () {
                match (e) {
                    Visibility::Unit => {
                        self.on_unit()
                    }
                }
            }
            fn on_unit(&self) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_visibility_default_pub() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        pub enum Visibility {
            Unit,
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait VisibilityHandler {
            fn on(&self, e: Visibility) -> () {
                match (e) {
                    Visibility::Unit => {
                        self.on_unit()
                    }
                }
            }
            fn on_unit(&self) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_visibility_override() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        #[enum_handler(visibility = "pub(crate)")]
        pub enum Visibility {
            Unit,
        }
    })
    .unwrap();

    let expected = quote! {
        pub(crate) trait VisibilityHandler {
            fn on(&self, e: Visibility) -> () {
                match (e) {
                    Visibility::Unit => {
                        self.on_unit()
                    }
                }
            }
            fn on_unit(&self) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_trait_name_suffix_override() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        #[enum_handler(trait_suffix = "Processor")]
        pub enum TraitName {
            Unit,
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait TraitNameProcessor {
            fn on(&self, e: TraitName) -> () {
                match (e) {
                    TraitName::Unit => {
                        self.on_unit()
                    }
                }
            }
            fn on_unit(&self) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_trait_name_override() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        #[enum_handler(trait_name = "NewTraitName", trait_suffix = "Processor")]
        pub enum TraitName {
            Unit,
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait NewTraitName {
            fn on(&self, e: TraitName) -> () {
                match (e) {
                    TraitName::Unit => {
                        self.on_unit()
                    }
                }
            }
            fn on_unit(&self) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_handler_name_override() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        #[enum_handler(handler_name = "handle")]
        pub enum HandlerName {
            Unit,
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait HandlerNameHandler {
            fn handle(&self, e: HandlerName) -> () {
                match (e) {
                    HandlerName::Unit => {
                        self.handle_unit()
                    }
                }
            }
            fn handle_unit(&self) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_return_type() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        #[enum_handler(return_type = "i32")]
        pub enum ReturnType {
            Unit,
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait ReturnTypeHandler {
            fn on(&self, e: ReturnType) -> i32 {
                match (e) {
                    ReturnType::Unit => {
                        self.on_unit()
                    }
                }
            }
            fn on_unit(&self) -> i32;
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_return_value() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        #[enum_handler(return_type = "i32", default_return_value = "42", default_implementation = true)]
        pub enum ReturnValue {
            Unit,
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait ReturnValueHandler {
            fn on(&self, e: ReturnValue) -> i32 {
                match (e) {
                    ReturnValue::Unit => {
                        self.on_unit()
                    }
                }
            }
            fn on_unit(&self) -> i32 {
                42
            }
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_unit_variant() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        pub enum UnitVariant {
            Unit,
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait UnitVariantHandler {
            fn on(&self, e: UnitVariant) -> () {
                match (e) {
                    UnitVariant::Unit => {
                        self.on_unit()
                    }
                }
            }
            fn on_unit(&self) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_empty_tuple_variant() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        pub enum EmptyTupleVariant {
            EmptyTuple(),
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait EmptyTupleVariantHandler {
            fn on(&self, e: EmptyTupleVariant) -> () {
                match (e) {
                    EmptyTupleVariant::EmptyTuple() => {
                        self.on_empty_tuple()
                    }
                }
            }
            fn on_empty_tuple(&self) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_one_tuple_variant() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        pub enum OneTupleVariant {
            OneTuple(String),
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait OneTupleVariantHandler {
            fn on(&self, e: OneTupleVariant) -> () {
                match (e) {
                    OneTupleVariant::OneTuple(arg) => {
                        self.on_one_tuple(arg)
                    }
                }
            }
            fn on_one_tuple(&self, arg: String) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_two_tuple_variant() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        pub enum TwoTupleVariant {
            TwoTuple(String, i32),
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait TwoTupleVariantHandler {
            fn on(&self, e: TwoTupleVariant) -> () {
                match (e) {
                    TwoTupleVariant::TwoTuple(arg0, arg1) => {
                        self.on_two_tuple(arg0, arg1)
                    }
                }
            }
            fn on_two_tuple(&self, arg0: String, arg1: i32) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_empty_struct_variant() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        pub enum EmptyStructVariant {
            EmptyStruct{ },
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait EmptyStructVariantHandler {
            fn on(&self, e: EmptyStructVariant) -> () {
                match (e) {
                    EmptyStructVariant::EmptyStruct {} => {
                        self.on_empty_struct()
                    }
                }
            }
            fn on_empty_struct(&self) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_one_struct_variant() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        pub enum OneStructVariant {
            OneStruct{ var1: String },
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait OneStructVariantHandler {
            fn on(&self, e: OneStructVariant) -> () {
                match (e) {
                    OneStructVariant::OneStruct { var1 } => {
                        self.on_one_struct(var1)
                    }
                }
            }
            fn on_one_struct(&self, var1: String) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_two_struct_variant() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        pub enum TwoStructVariant {
            TwoStruct{ var1: String, var2: i32 },
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait TwoStructVariantHandler {
            fn on(&self, e: TwoStructVariant) -> () {
                match (e) {
                    TwoStructVariant::TwoStruct { var1, var2 } => {
                        self.on_two_struct(var1, var2)
                    }
                }
            }
            fn on_two_struct(&self, var1: String, var2: i32) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_all_variants() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        pub enum AllVariants {
            Unit,
            EmptyTuple(),
            OneTuple(String),
            TwoTuple(String, i32),
            EmptyStruct {},
            OneStruct { var1: String },
            TwoStruct { var1: String, var2: i32 },
        }
    })
    .unwrap();
    let expected = quote! {
        pub trait AllVariantsHandler {
            fn on(&self, e: AllVariants) -> () {
                match (e) {
                    AllVariants::Unit => {
                        self.on_unit()
                    }
                    AllVariants::EmptyTuple() => {
                        self.on_empty_tuple()
                    }
                    AllVariants::OneTuple(arg) => {
                        self.on_one_tuple(arg)
                    }
                    AllVariants::TwoTuple(arg0, arg1) => {
                        self.on_two_tuple(arg0, arg1)
                    }
                    AllVariants::EmptyStruct {} => {
                        self.on_empty_struct()
                    }
                    AllVariants::OneStruct { var1 } => {
                        self.on_one_struct(var1)
                    }
                    AllVariants::TwoStruct { var1, var2 } => {
                        self.on_two_struct(var1, var2)
                    }
                }
            }
            fn on_unit(&self) -> ();
            fn on_empty_tuple(&self) -> ();
            fn on_one_tuple(&self, arg: String) -> ();
            fn on_two_tuple(&self, arg0: String, arg1: i32) -> ();
            fn on_empty_struct(&self) -> ();
            fn on_one_struct(&self, var1: String) -> ();
            fn on_two_struct(&self, var1: String, var2: i32) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_async() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        #[enum_handler(is_async = true, no_async_trait_macro = true)]
        enum Async {
            OneTuple(String),
        }
    })
    .unwrap();

    let expected = quote! {
        trait AsyncHandler {
            async fn on(&self, e: Async) -> () {
                match (e) {
                    Async::OneTuple(arg) => {
                        self.on_one_tuple(arg).await
                    }
                }
            }
            async fn on_one_tuple(&self, arg: String) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_pass_args_by_ref() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        #[enum_handler(pass_args_by_ref = true)]
        pub enum OneTupleVariant {
            OneTuple(String),
        }
    })
    .unwrap();

    let expected = quote! {
        pub trait OneTupleVariantHandler {
            fn on(&self, e: &OneTupleVariant) -> () {
                match (e) {
                    OneTupleVariant::OneTuple(arg) => {
                        self.on_one_tuple(arg)
                    }
                }
            }
            fn on_one_tuple(&self, arg: &str) -> ();
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_mock() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        #[enum_handler(mock_name = "MockHandler")]
        enum Event {
            OneTuple(String),
        }
    })
    .unwrap();

    let expected = quote! {
        trait EventHandler {
            fn on(&self, e: Event) -> () {
                match (e) {
                    Event::OneTuple(arg) => {
                        self.on_one_tuple(arg)
                    }
                }
            }
            fn on_one_tuple(&self, arg: String) -> ();
        }
        #[cfg (test)]
        mockall::mock! {
            MockHandler { }
            impl EventHandler for MockHandler {
                fn on_one_tuple(&self, arg: String) -> ();
            }
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_async_mock() {
    let actual = enum_handler_core(quote! {
        #[derive(EnumHandler)]
        #[enum_handler(mock_name = "MockHandler", is_async = true)]
        enum Event {
            OneTuple(String),
        }
    })
    .unwrap();

    let expected = quote! {
        #[async_trait :: async_trait]
        trait EventHandler {
            async fn on(&self, e: Event) -> () {
                match (e) {
                    Event::OneTuple(arg) => {
                        self.on_one_tuple(arg).await
                    }
                }
            }
            async fn on_one_tuple(&self, arg: String) -> ();
        }
        #[cfg (test)]
        mockall::mock! {
            MockHandler { }
            #[async_trait :: async_trait]
            impl EventHandler for MockHandler {
                async fn on_one_tuple(&self, arg: String) -> ();
            }
        }
    };
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn test_panic_if_no_variants() {
    assert_eq!(
        enum_handler_core(quote! {
            #[derive(EnumHandler)]
            enum NoVariants {}
        })
        .unwrap_err()
        .to_string(),
        Error::NoVariants.to_string(),
    );
}

#[test]
fn test_panic_if_struct() {
    assert_eq!(
        enum_handler_core(quote! {
            #[derive(EnumHandler)]
            struct NotEnum {}
        })
        .unwrap_err()
        .to_string(),
        Error::NotEnum.to_string(),
    );
}
