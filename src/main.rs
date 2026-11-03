use anyhow::Result;
use clap::Parser;
use cinnamon::cinnamon;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_integration: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = cinnamon::Config { 
        desktop_integration: !cli.no_integration, 
        ..Default::default() 
    };
    let mut component = cinnamon::new(config);
    
    component.initialize()?;
    component.launch()?;
    
    println!("SigmaOS-cinnamon ready");
    Ok(())
}
