//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

mod chain_spec;
mod eth;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

#[allow(clippy::result_large_err)]
fn main() -> sc_cli::Result<()> {
    command::run()
}
