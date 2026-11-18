use std::{thread::sleep, time::Duration};

use colored::Colorize;

const FW_VERSION: &str = "v1998.10-rc1";
const SN: &str = "AC1DB4B3-420-1998";
const HOSTNAME: &str = "lenna@momosoft";
const CLOCK_FREQ: &str = "2.75Hz";
const ORIGIN: &str = "Terra/Sol III";
const MOTTO: &str = "Powered By Emotion™";
const LOGO: &str = include_str!("../cat.txt");

const SHORT_DUR_MS: u64 = 250;
const LONG_DUR_MS: u64 = 50;

fn main() {
    clearscreen::clear().expect("faied to clear screen");
    println!("{}", LOGO);
    sleep(Duration::from_millis(SHORT_DUR_MS));
    println!("\n{} - {}\n", "こけし".bold().red(), "v2.0".bold());
    sleep(Duration::from_millis(LONG_DUR_MS));
    println!("{}: {}", "Firmware".bold().blue(), FW_VERSION);
    sleep(Duration::from_millis(LONG_DUR_MS));
    println!("{}: {}", "S/N".bold().blue(), SN);
    sleep(Duration::from_millis(LONG_DUR_MS));
    println!("{}: {}", "Hostname".bold().blue(), HOSTNAME);
    sleep(Duration::from_millis(LONG_DUR_MS));
    println!("{}: {}", "ClockFreq".bold().blue(), CLOCK_FREQ);
    sleep(Duration::from_millis(LONG_DUR_MS));
    println!("{}: {}", "Origin".bold().blue(), ORIGIN);
    sleep(Duration::from_millis(LONG_DUR_MS));
    println!("\n{}", MOTTO.bold().red());
}
