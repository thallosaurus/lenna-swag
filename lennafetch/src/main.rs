use colored::Colorize;

const FW_VERSION: &str = "v1998.10-rc1";
const SN: &str = "AC1DB4B3-420-1998";
const HOSTNAME: &str = "lenna@momosoft";
const CLOCK_FREQ: &str = "2.75Hz";
const ORIGIN: &str = "Terra/Sol III";
const MOTTO: &str = "Powered By Emotion™";
const LOGO: &str = include_str!("../cat.txt");


fn main() {
    clearscreen::clear().expect("faied to clear screen");
    println!("{}", LOGO);
    println!("\n{} - {}\n", "こけし".bold().red(), "v2.0".bold());
    println!("{}: {}", "Firmware".bold().blue(), FW_VERSION);
    println!("{}: {}", "S/N".bold().blue(), SN);
    println!("{}: {}", "Hostname".bold().blue(), HOSTNAME);
    println!("{}: {}", "ClockFreq".bold().blue(), CLOCK_FREQ);
    println!("{}: {}", "Origin".bold().blue(), ORIGIN);
    println!("\n{}", MOTTO.bold().red());
}
