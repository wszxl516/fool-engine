use clap::Parser;
use log::LevelFilter;
use packtool::ResourcePackage;
use prettytable::{Attr, Cell, Row, Table, color, row};
use std::str::FromStr;
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct PackArgs {
    /// assets used for packaging
    #[arg(short = 'i', long, default_value = "./assets")]
    input_assets_dir: String,
    /// packed Assets output
    #[arg(short = 'o', long, default_value = "./assets.pak")]
    output: String,
    /// compress Assets ?
    #[arg(short = 'c', long, default_value_t = true)]
    compress: bool,
    /// compress level
    #[arg(short = 'p', long, default_value_t = 10)]
    compress_level: u32,
    /// off, error, warn, info, debug, trace,
    #[arg(short = 'l', long, default_value = "info")]
    log_level: String,
    /// log to file
    #[arg(short = 'f', long, default_value = "./log.log")]
    file_log: String,
    /// The log is output to the console
    #[arg(short = 'v', long, default_value_t = false)]
    verbose: bool,
}
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct UnPackArgs {
    /// output dir
    #[arg(short = 'o', long, default_value = "./output")]
    out_put: String,
    /// packed Assets input
    #[arg(short = 'i', long, default_value = "./assets.pak")]
    input: String,
    /// off, error, warn, info, debug, trace,
    #[arg(short = 'l', long, default_value = "info")]
    log_level: String,
    /// log to file
    #[arg(short = 'f', long, default_value = "./log.log")]
    file_log: String,
    /// The log is output to the console
    #[arg(short = 'v', long, default_value_t = false)]
    verbose: bool,
    /// do not unpack only get info
    #[arg(short = 's', long, default_value_t = false)]
    show: bool,
}
#[derive(Parser, Debug)]
#[allow(non_camel_case_types)]
pub enum Args {
    pack(PackArgs),
    unpack(UnPackArgs),
}
fn main() -> anyhow::Result<()> {
    match Args::parse() {
        Args::pack(args) => {
            let level = LevelFilter::from_str(args.log_level.as_str())
                .unwrap_or_else(|_| LevelFilter::Info);
            rolllog::log_init(level, args.verbose, &args.file_log, &["packtool"])?;
            let mut gp = ResourcePackage::create_pak(
                args.input_assets_dir,
                args.output,
                args.compress,
                args.compress_level as i32,
            );
            gp.pack()?;
            dump_info(&gp);
        }
        Args::unpack(args) => {
            let level = LevelFilter::from_str(args.log_level.as_str())
                .unwrap_or_else(|_| LevelFilter::Info);
            rolllog::log_init(level, args.verbose, &args.file_log, &["packtool"])?;
            if args.show {
                let gp = ResourcePackage::from_pak(args.input)?;
                dump_info(&gp);
                dump_files(&gp);
            } else {
                let gp = ResourcePackage::from_pak(&args.input)?;
                gp.unpack2dir(args.out_put)?;
            }
        }
    }
    Ok(())
}

pub fn dump_info(gp: &ResourcePackage) {
    let byte = byte_unit::Byte::from_u64(gp.total_size);
    let adjusted_byte = byte.get_appropriate_unit(byte_unit::UnitType::Binary);
    let info = gp.info();
    let mut table = Table::new();
    table.set_titles(row![
        "version",
        "file count",
        "compressed",
        "level",
        "size",
        "date",
        "id"
    ]);
    table.add_row(Row::new(vec![
        Cell::new(info.version_string().as_str()).with_style(Attr::ForegroundColor(color::WHITE)),
        Cell::new(format!("{}", info.file_count).as_str())
            .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
        Cell::new(format!("{}", gp.header.compress).as_str())
            .with_style(Attr::ForegroundColor(color::BRIGHT_YELLOW)),
        Cell::new(format!("{}", gp.header.compress_level).as_str())
            .with_style(Attr::ForegroundColor(color::BRIGHT_YELLOW)),
        Cell::new(format!("{:0.2}", adjusted_byte).as_str())
            .with_style(Attr::ForegroundColor(color::BRIGHT_YELLOW)),
        Cell::new(
            format!(
                "{}",
                chrono::DateTime::<chrono::Utc>::from(info.timestamp.clone())
                    .with_timezone(&chrono::Local)
                    .format("%Y-%m-%d %H:%M:%S")
            )
            .as_str(),
        )
        .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE)),
        Cell::new(info.resource_id.as_str())
            .with_style(Attr::ForegroundColor(color::BRIGHT_YELLOW)),
    ]));
    table.printstd();
}

pub fn dump_files(gp: &ResourcePackage) {
    let mut table = Table::new();
    table.set_titles(row!["path", "length", "sha256"]);
    for entry in &gp.entrys {
        let byte = byte_unit::Byte::from_u64(entry.data_length);
        let adjusted_byte = byte.get_appropriate_unit(byte_unit::UnitType::Binary);
        table.add_row(Row::new(vec![
            Cell::new(entry.path.as_str()).with_style(Attr::ForegroundColor(color::WHITE)),
            Cell::new(format!("{:0.2}", adjusted_byte).as_str())
                .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
            Cell::new(format!("{}", hex::encode(entry.hash)).as_str())
                .with_style(Attr::ForegroundColor(color::BRIGHT_YELLOW)),
        ]));
    }
    table.printstd();
}
