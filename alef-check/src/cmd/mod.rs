use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}


pub mod parse; 
pub mod generate; 
pub mod lex; 
use parse::ParseCommand;
use generate::GenerateCommand;
use lex::LexCommand; 

#[derive(Subcommand, Debug)]
pub enum Command {
    Parse(ParseCommand),
    Generate(GenerateCommand),
    Lex(LexCommand),
}


