# Rust Enum Handler Derive Macro

This project provides a macro for handling Rust enums in a convenient and efficient way. 

The `#[derive(EnumHandler)]` macro simplifies the process of matching and handling different enum variants, 
allowing for cleaner and more readable code.

It generates a trait with a common handler method with match statements for each variant of the enum, allowing you to easily handle each case separately. 

```rust
// You declare the CounterEvent enum:

use enum_handler::EnumHandler;

#[derive(EnumHandler)]
pub enum CounterEvent {
    Increment,
    Decrement,
    Reset,
    Set(i32),
}

// and the enum_handler macro will generate the following code for you behind the scenes:

pub trait CounterEventHandler {
    fn on(&self, e: CounterEvent) -> () {
        match (e) {
            CounterEvent::Increment => self.on_increment(),
            CounterEvent::Decrement => self.on_decrement(),
            CounterEvent::Reset => self.on_reset(),
            CounterEvent::Set(arg) => self.on_set(arg),
        }
    }
    fn on_increment(&self) -> ();
    fn on_decrement(&self) -> ();
    fn on_reset(&self) -> ();
    fn on_set(&self, set: i32) -> ();
}
```

The generated code can be highly customized to suit your specific needs.

Eg. you can specify:

- Names for the generated trait and methods
- async or sync methods
- pass arguments by value (default, changes ownership) or by reference
- generate default implementations for the methods
    - specify a return value 
- specify a common return type for each method
- specify visibility for the generated trait and methods
- output the generated code to a file for debug purposes 
- mock all support

## Project Structure

The project is structured as follows:

| Crate | Description |
|-------|-------------|
| `enum_handler`        | The library crate that exposes the `#[enum_handler()]` attribute macro. This is the crate that you will include in your project! |
| `enum_handler_derive` | The procedural macro crate that implements the `#[derive(EnumHandler)]` macro. |
| `enum_handler_core`   | The core crate that contains almost all the logic for the macro. This crate is shared between the library and the derive crate. You can see examples is the tests directory and the tests.rs file in the enum_handler_core crate. |

## Configuration

With the `#[enum_handler()]` attribute macro, you can customize the generated code by specifying the following options:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `trait_suffix` | `String` | `"Handler"` | Specifies the suffix for the generated trait name which will be appended to the enum name. |
| `trait_name` | `String` | `""` | If specified, the generated trait will have this name instead of the default one. |
| `handler_name` | `String` | `on` | Specifies the name of the common handler method. This is also used as a prefix for the generated method names (separator is `_`). |
| `return_type` | `String` | `()` | Specifies the common return type for each method. |
| `default_return_value` | `String` | `()` | Specifies the common return value for each method if the default implementations are generated. |
| `is_async` | `bool` | `false` | Specifies whether the generated methods should be asynchronous (`true`) or synchronous (`false`). |
| `default_implementation` | `bool` | `true` | Specifies whether default implementations should be generated for the methods (`true`) or not (`false`). |
| `visibility` | `String` | `""` | Specifies the visibility for the generated trait and methods. If not specified, the visibility of the enum is used. |
| `no_async_trait_macro` | `bool` | `false` | Specifies whether to use the `#[async_trait::async_trait]` macro (`false`) or not (`true`). This is only relevant if `is_async` is `true`. The `async_trait` crate must be included in the \[dependencies\]. |
| `mock_name` | `String` | `""` | If specified, a mockall trait will be generated with this name. The `mockall` crate must be included in the \[dev-dependencies\]. |
| `pass_args_by_ref` | `bool` | `false` | Specifies whether the arguments should be passed by reference (`true`) or by value (`false`). |

## Examples

Here are a few examples to demonstrate the usage of the `#[derive(EnumHandler)]` macro:

### Async Methods

```rust
use enum_handler::EnumHandler;

#[derive(EnumHandler)]
#[enum_handler(is_async = true)]
pub enum CounterEvent {
    Increment,
    Decrement,
    Reset,
    Set(i32),
}
```

### Custom Trait Name

```rust
use enum_handler::EnumHandler;

#[derive(EnumHandler)]
#[enum_handler(trait_name = "CounterHandler")]

pub enum CounterEvent {
    Increment,
    Decrement,
    Reset,
    Set(i32),
}
```
### Custom Handler Method Name

```rust
use enum_handler::EnumHandler;

#[derive(EnumHandler)]
#[enum_handler(handler_name = "handle")]
pub enum CounterEvent {
    Increment,
    Decrement,
    Reset,
    Set(i32),
}
```


### Custom Return Type

```rust
use enum_handler::EnumHandler;

#[derive(EnumHandler)]
#[enum_handler(return_type = "i32")]
pub enum CounterEvent {
    Increment,
    Decrement,
    Reset,
    Set(i32),
}
```

### Mockall Support

```rust 
use enum_handler::EnumHandler;

#[derive(EnumHandler)]
#[enum_handler(mock_name = "MockCounterHandler")]
pub enum CounterEvent {
    Increment,
    Decrement,
    Reset,
    Set(i32),
}
```

## Write generated code to a file

You can set the environment variable `ENUM_HANDLER_DEBUG` to write the generated code to a file. 

```bash
ENUM_HANDLER_DEBUG=/tmp/enum_handler.rs cargo build
```

- If the file is greater than 128kB, it will be truncated.
- This is only available for debug purposes and should not be used in production code.

If you want to format the generated code with `rustfmt`, you can set the environment variable `ENUM_HANDLER_DEBUG_FORMAT` to `1`.

```bash
ENUM_HANDLER_DEBUG=/tmp/enum_handler.rs ENUM_HANDLER_DEBUG_FORMAT=1 cargo build
```

- The file will be formatted with `rustfmt` if it is available.

You can also use cargo-expand to see the generated code:

```bash
cargo install cargo-expand
cargo expand --help
```

## Known Issues

- This crate is under heavy development and may have braking changes in the future.
- The `#[async_trait::async_trait]` macro usage will be improved in a future release.
- The `rustfmt` formatting will be customizable in a future release.
- The names of the tuple variant parameters will be changed in a future release.
- The minimum supported Rust version (MSRV) is currently set to 1.80.1. This will be reviewed in a future release.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request on the [GitHub repository](https://github.com/oxiui/enum_handler).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
