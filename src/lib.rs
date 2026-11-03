//! SigmaOS-cinnamon - SigmaOS Component
//!
//! Desktop and productivity component for SigmaOS.

use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub enabled: bool,
    pub desktop_integration: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { enabled: true, desktop_integration: true }
    }
}

pub struct cinnamon {
    config: Config,
}

impl cinnamon {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn initialize(&mut self) -> Result<()> {
        info!("Initializing SigmaOS-cinnamon");
        Ok(())
    }

    pub fn launch(&mut self) -> Result<()> {
        info!("Launching SigmaOS-cinnamon");
        Ok(())
    }
}
