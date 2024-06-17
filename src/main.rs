//! TODO

// TODO: remove me.
#![allow(unused_crate_dependencies)]

//---------------------------------------------------------------------------------------------------- Mod
mod book;
mod cli;
mod config;
mod constants;
mod disk;
mod download;
mod free;
mod launch;
mod panic;
mod regex;
mod statics;
mod tui;
mod update;

//---------------------------------------------------------------------------------------------------- Use
use std::process::{Command, Stdio};

//---------------------------------------------------------------------------------------------------- Main
fn main() {
    let mut cmd = Command::new("path/to/monero-wallet-cli")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
