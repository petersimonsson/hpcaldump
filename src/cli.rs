use std::{ops::RangeInclusive, path::Path};

use clap::{Args, Parser, Subcommand, value_parser};
use miette::miette;

#[derive(Debug, Parser)]
#[command(author, about, version)]
pub struct Cli {
    #[arg(value_parser(value_parser!(u8).range(0..=30)))]
    pub gpib_address: u8,
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    A(ConstRangeArgs),
    B(ConstRangeArgs),
    Range(RangeArgs),
}

#[derive(Debug, Args)]
pub struct ConstRangeArgs {
    pub output_file: Box<Path>,
}

#[derive(Debug, Args)]
pub struct RangeArgs {
    #[arg(value_parser(value_parser!(u16).range(0..32768)))]
    pub start: u16,
    #[arg(value_parser(value_parser!(u16).range(0..32768)))]
    pub end: u16,
    pub output_file: Box<Path>,
}

impl RangeArgs {
    pub fn mem_range(&self) -> miette::Result<RangeInclusive<u16>> {
        if self.start <= self.end {
            Ok(self.start..=self.end)
        } else {
            Err(miette!("Start of range needs to be less than end of range"))
        }
    }
}
