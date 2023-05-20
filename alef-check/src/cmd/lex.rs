use alef_parser::{lex, source::MemoryBuffer};
use clap::{Parser, AppSettings};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[derive(Parser, Debug)]
#[clap(about = "Lex (tokenize) an Alef source file", version, author)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]
pub struct LexCommand {
    /// Enable tracing
    #[clap(long)]
    pub trace: bool,

    /// Enable debug messages
    #[clap(long)]
    pub debug: bool,

    /// Suppress output 
    #[clap(short, long)]
    pub suppress_output: bool,

    /// Input file
    #[clap(parse(from_os_str))]
    pub input: std::path::PathBuf,
}

impl LexCommand {
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

        let mbuf = MemoryBuffer::from_file(in_path.to_string_lossy().into())?;
        let mut lex = lex::scan::Scanner::new(Box::new(mbuf), None);
        let mut tok = lex.tok();
        
        while !tok.is_end() {
            if !self.suppress_output {
                println!("{}", tok);
            }
            tok = lex.tok();
        }
        Ok(())
    }
}
