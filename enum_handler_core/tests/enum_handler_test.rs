#![allow(unused_variables)]

use enum_handler::*;

pub struct PauseData;

pub struct CargoExpandStart;

#[derive(EnumHandler)]
#[enum_handler(
//    trait_name = "MyTrait", //
//    trait_suffix = "Processor", 
//    default_implementation = true,
//    handler_prefix = "handle_",
//    return_type = "i32",
//    default_return_value = "0",
//    visibility = "pub(self)",
//    is_async = true
    mock_name = "TestEvent",
)]
pub enum Event1 {
    Unit,
    EmptyTuple(),
    Tuple(String, i32),
    Start(String),
    Stop {},
    Pause(PauseData),
    Resume { var1: String, var2: i32 },
}
