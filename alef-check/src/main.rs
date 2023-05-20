#![allow(unused_assignments, dead_code, unused_imports, unused_variables)]
mod cmd;
use clap::Parser;
use crate::cmd::{Cli, Command, parse::ParseCommand, generate::GenerateCommand, lex::LexCommand};

fn main() -> anyhow::Result<()> {
    let cmd = Cli::parse();
    
    match cmd.command {
        Command::Parse(p) => p.execute()?, 
        Command::Generate(g) => g.execute()?,
        Command::Lex(l) => l.execute()?,
    }

    Ok(())
}
