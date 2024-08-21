#![doc = include_str!("../README.md")]

pub mod enum_handler_args;
mod error;
mod generator;
mod model;

#[cfg(test)]
mod tests;

use std::{env, fs::OpenOptions, io::Write, process};

use darling::FromDeriveInput;
use enum_handler_args::EnumHandlerArgs;
use model::Enum;
use proc_macro2::TokenStream;
use syn::*;

pub use error::{Error, Result};

// https://astexplorer.net/

pub fn enum_handler_core(input: TokenStream) -> Result<TokenStream> {
    let derive_input: DeriveInput = syn::parse2::<DeriveInput>(input.clone())?;
    let args = EnumHandlerArgs::from_derive_input(&derive_input)?;

    let e = Enum::new(&derive_input, &args)?;

    if e.variants
        .is_empty()
    {
        return Err(Error::NoVariants);
    }

    let mut output = e.generate_trait(&args)?;

    if args.is_generate_mock() {
        let mock = e.generate_mock(&args);
        output.extend(mock);
    }

    write_debug_file(&input, &output)?;
    Ok(output)
}

// FIXME: concurrent writes to the debug file
fn write_debug_file(
    input: &TokenStream,
    output: &TokenStream,
) -> Result<()> {
    static DEBUG_FILE_ENV: &str = "ENUM_HANDLER_DEBUG";
    static FORMAT_DEBUG_FILE_ENV: &str = "ENUM_HANDLER_DEBUG_FORMAT";

    static MAX_FILE_SIZE: u64 = 1024 * 128;
    let debug_filename = env::var(DEBUG_FILE_ENV).unwrap_or_default();
    let format_debug_file = !env::var(FORMAT_DEBUG_FILE_ENV)
        .unwrap_or_default()
        .is_empty();
    if !debug_filename.is_empty() {
        let mut append = true;
        if let Ok(metadata) = std::fs::metadata(&debug_filename) {
            if metadata.len() > MAX_FILE_SIZE {
                append = false;
            }
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(append)
            .open(&debug_filename)
            .map_err(|_| Error::CannotWriteDebugFile(debug_filename.clone(), DEBUG_FILE_ENV.to_string()))?;

        let trace = format!(
            r#"

// ************************************************************
// Given input:
{}
// ------------------------------------------------------------
// Generated output:
{}
// ************************************************************

"#,
            input, output
        );

        file.write(trace.as_bytes())
            .map_err(|_| Error::CannotWriteDebugFile(debug_filename.clone(), DEBUG_FILE_ENV.to_string()))?;

        if format_debug_file {
            let _ = process::Command::new("rustfmt")
                .arg("--edition=2021")
                .arg(&debug_filename)
                .output();
        }
    }
    Ok(())
}
