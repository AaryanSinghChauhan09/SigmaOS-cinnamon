use ::cinnamon::{cinnamon, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = cinnamon::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-cinnamon operational");
    Ok(())
}
