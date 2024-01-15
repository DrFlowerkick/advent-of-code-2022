//!lib.rs

pub mod days;

use anyhow::Result;

pub fn run() -> Result<()> {
    days::day_01::day_01()?;
    days::day_02::day_02()?;
    days::day_03::day_03()?;
    days::day_04::day_04()?;
    days::day_05::day_05()?;
    days::day_06::day_06()?;
    days::day_07::day_07()?;

    Ok(())
}
