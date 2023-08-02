use std::env;

use clap::Parser;
use yt_as_storage::cmd::CmdArgs;

fn main() {
    CmdArgs::parse_from(env::args_os());
}
