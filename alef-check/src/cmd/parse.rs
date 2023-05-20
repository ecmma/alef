use alef_parser::{parse, source::MemoryBuffer};
use clap::{Parser, AppSettings};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[derive(Parser, Debug)]
#[clap(about = "Parse an Alef source file", version, author)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]
pub struct ParseCommand {
    /// Enable tracing
    #[clap(long)]
    pub trace: bool,

    /// Enable debug messages
    #[clap(long)]
    pub debug: bool,

    /// Enable type checking
    #[clap(short, long)]
    pub typecheck: bool,

    /// Input file
    #[clap(parse(from_os_str))]
    pub input: std::path::PathBuf,

    /// Output file
    #[clap(short, long, parse(from_os_str))]
    pub output: Option<std::path::PathBuf>,
}

impl ParseCommand {
    pub fn execute(&self) -> anyhow::Result<()> {
        if self.trace {
            let sl = SimpleLogger::new();
            let sl = SimpleLogger::with_level(sl, LevelFilter::max());
            sl.init()?;
        }

        if self.debug & !self.trace {
            let sl = SimpleLogger::new();
            sl.init()?;
        }

        let in_path = self.input.as_path();
        let out_path: std::path::PathBuf;

        if let Some(ref op) = self.output {
            out_path = op.clone();
        } else {
            out_path = in_path.with_extension(".o");
        }

        let mbuf = MemoryBuffer::from_file(in_path.to_string_lossy().into())?;
        let mut parser = parse::Parser::new(Box::new(mbuf), None);
        let program = parser.parse();
        println!("{:?}", program);
        Ok(())
    }
}
