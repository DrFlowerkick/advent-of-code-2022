//!lib.rs

pub mod days;

use anyhow::Result;

pub fn run() -> Result<()> {
    days::day_01::day_01()?;

    Ok(())
}
