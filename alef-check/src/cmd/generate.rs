use clap::{App, AppSettings, IntoApp, Parser, ValueHint};
use clap_complete::{generate, Generator, Shell};
use super::Cli; 

#[derive(Parser, Debug)]
#[clap(about = "Generate shell completion", version, author)]
pub struct GenerateCommand {
    /// The shell to generate completions for. 
    #[clap(arg_enum)]
    pub shell: Shell
}


impl GenerateCommand {
    pub fn execute(&self) -> anyhow::Result<()> {
        let shell = self.shell; 
        let app = Cli::into_app(); 
        let app_name = app.get_name().to_string(); 
        let mut app = Cli::into_app(); 
        generate(shell, &mut app, app_name, &mut std::io::stdout());

        Ok(())
    }
}
