use std::{ops::RangeInclusive, path::Path};

use clap::{Args, Parser, Subcommand, value_parser};
use miette::miette;

#[derive(Debug, Parser)]
#[command(author, about, version)]
pub struct Cli {
    #[arg(short, long, default_value = "0")]
    /// The Linux GPIB board index.
    pub bord_index: i32,
    #[arg(value_parser(value_parser!(u8).range(0..=30)))]
    /// GPIB address of device to dump data from.
    pub gpib_address: u8,
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    /// Use on older revisions of HP3457A. Memory range: 0x40-0x01FF.
    Old(ConstRangeArgs),
    /// Use on newer revisions of HP3457A (8-68A/8-68B and 8-67/8-68). Memory range: 0x5000-0x57FF.
    New(ConstRangeArgs),
    /// Use this to read any user defined range of memory. Memory range: 0-32767.
    User(UserRangeArgs),
}

#[derive(Debug, Args)]
pub struct ConstRangeArgs {
    /// Path to generated binary file.
    pub output_file: Box<Path>,
}

#[derive(Debug, Args)]
pub struct UserRangeArgs {
    #[arg(value_parser(value_parser!(u16).range(0x0000..=0x7FFF)))]
    /// Start address of address range to dump.
    pub start: u16,
    #[arg(value_parser(value_parser!(u16).range(0x0000..=0x7FFF)))]
    /// End address of address range to dump. Note that memory data is dumped two bytes at a time.
    pub end: u16,
    /// Path to generated binary file.
    pub output_file: Box<Path>,
}

impl UserRangeArgs {
    pub fn mem_range(&self) -> miette::Result<RangeInclusive<u16>> {
        if self.start <= self.end {
            Ok(self.start..=self.end)
        } else {
            Err(miette!("Start of range needs to be less than end of range"))
        }
    }
}
