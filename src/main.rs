use std::{ops::RangeInclusive, path::Path};

use clap::Parser;
use cli::Cli;
use indicatif::ProgressIterator;
use miette::IntoDiagnostic;
use rusty_gpib::{Device, EoS, EoSModeFlags};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

mod cli;

const HP3457_CAL_OLD_ADDRS: RangeInclusive<u16> = 0x0040..=0x01FF;
const HP3457_CAL_NEW_ADDRS: RangeInclusive<u16> = 0x5000..=0x57FF;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::CliCommand::Old(args) => {
            read_hp3457_cal_data(
                cli.bord_index,
                cli.gpib_address,
                HP3457_CAL_OLD_ADDRS,
                &args.output_file,
            )
            .await?;
        }
        cli::CliCommand::New(args) => {
            read_hp3457_cal_data(
                cli.bord_index,
                cli.gpib_address,
                HP3457_CAL_NEW_ADDRS,
                &args.output_file,
            )
            .await?;
        }
        cli::CliCommand::User(args) => {
            read_hp3457_cal_data(
                cli.bord_index,
                cli.gpib_address,
                args.mem_range()?,
                &args.output_file,
            )
            .await?;
        }
    }

    Ok(())
}

async fn read_hp3457_cal_data<R>(
    board_id: i32,
    gpib_address: u8,
    range: R,
    output_file: &Path,
) -> miette::Result<()>
where
    R: ExactSizeIterator<Item = u16>,
{
    let device = Device::new(
        board_id,
        gpib_address as i32,
        None,
        20,
        true,
        Some(EoS::new(EoSModeFlags::REOS, b'\n')),
    )
    .into_diagnostic()?;

    let mut file = BufWriter::new(File::create(output_file).await.into_diagnostic()?);

    println!("Send 'TRIG 4'");
    device.write(b"TRIG 4").into_diagnostic()?;

    println!("Reading data from device");
    let mut len = 0;
    let mut buf: [u8; 16] = [0; 16];

    for addr in range.step_by(2).progress() {
        device
            .write(format!("PEEK {}", addr).as_bytes())
            .into_diagnostic()?;

        device.read(&mut buf).into_diagnostic()?;

        let val_str = String::from_utf8(buf.to_vec()).into_diagnostic()?;
        let val_str = val_str
            .strip_suffix("\r\n")
            .unwrap_or(&val_str)
            .trim_start();
        let val = val_str.parse::<f32>().into_diagnostic()? as i32;
        let val = (0x0000ffff & val) as u16;

        len += file.write(&val.to_be_bytes()).await.into_diagnostic()?;
    }

    println!("{len} bytes written to disk");

    file.flush().await.into_diagnostic()?;

    Ok(())
}
