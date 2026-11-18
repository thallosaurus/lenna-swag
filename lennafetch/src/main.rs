use std::{io, thread::sleep, time::Duration};

use ansi_to_tui::IntoText;
use colored::Colorize;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Flex, Layout},
    symbols::{self, border},
    text::Text,
    widgets::{Block, Paragraph},
};
use std::io::Write;

const FW_VERSION: &str = "v1998.10-rc1";
const SN: &str = "AC1DB4B3-420-1998";
const HOSTNAME: &str = "lenna@momosoft";
const CLOCK_FREQ: &str = "2.75Hz";
const ORIGIN: &str = "Terra/Sol III";
const MOTTO: &str = "Powered By Emotion™";
const LOGO: &str = include_str!("../cat.txt");

const SHORT_DUR_MS: u64 = 250;
const LONG_DUR_MS: u64 = 50;

fn main() -> std::io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let logo = Paragraph::new(Text::from(LOGO));

    //let logo = writeln!(buffer, "{}", LOGO)?;

    let mut text = Vec::new();
    write_info(&mut text).unwrap();
    let ansi_text = Paragraph::new(text.to_text().unwrap());

    let width = 35;

    let [left, middle, right] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .flex(Flex::Center)
    .areas(frame.area());

    let block = Block::bordered().padding(ratatui::widgets::Padding { left: 2, right: 2, top: 2, bottom: 2 });
    let inner = block.inner(middle);
    frame.render_widget(block, middle);

    let [logo_area, text_area] =
        Layout::vertical([Constraint::Length(9), Constraint::Min(0)]).areas(inner);

    /*let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(9), Constraint::Min(0)])
        .flex(ratatui::layout::Flex::Center)
        .split(outer_layout[0]);*/

    frame.render_widget(logo, logo_area);
    frame.render_widget(ansi_text, text_area);

    /*let outer = Paragraph::new(ansi_text)
    .block(Block::bordered().padding(ratatui::widgets::Padding {
        left: 3,
        right: 3,
        top: 3,
        bottom: 3,
    }))
    .centered()
    .alignment(ratatui::layout::HorizontalAlignment::Center);*/
}

fn write_info(buffer: &mut Vec<u8>) -> io::Result<()> {
    //clearscreen::clear().expect("faied to clear screen");
    //write!(&mut text, "trans rights").expect("failed to write info text to buffer");
    //writeln!(buffer, "{}", LOGO)?;
    writeln!(buffer, "\n{} - {}\n", "こけし".bold().red(), "v2.0".bold())?;
    writeln!(buffer, "{}: {}", "Firmware".bold().blue(), FW_VERSION)?;
    writeln!(buffer, "{}: {}", "S/N".bold().blue(), SN)?;
    writeln!(buffer, "{}: {}", "Hostname".bold().blue(), HOSTNAME)?;
    writeln!(buffer, "{}: {}", "ClockFreq".bold().blue(), CLOCK_FREQ)?;
    writeln!(buffer, "{}: {}", "Origin".bold().blue(), ORIGIN)?;
    writeln!(buffer, "\n{}", MOTTO.bold().red())?;
    Ok(())
}
