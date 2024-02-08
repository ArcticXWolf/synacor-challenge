use crate::{viewer::app::App, vm::REGISTER_ADDRESS_START};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::fmt::Write;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(40), Constraint::Min(1)])
        .split(f.size());

    render_cpu_state(app, f, layout[0]);
    render_output(app, f, layout[1]);
}

pub fn render_cpu_state(app: &mut App, f: &mut Frame, size: Rect) {
    let mut registers = String::new();
    for (i, r) in app.last_update.registers.iter().enumerate() {
        write!(
            registers,
            "{} | {}: {}\n",
            REGISTER_ADDRESS_START as usize + i,
            i,
            r
        )
        .unwrap();
    }
    let mut stack = String::new();
    for (i, s) in app.last_update.stack.iter().enumerate() {
        write!(stack, "{}: {}\n", i, s).unwrap();
    }

    let mut widget = Paragraph::new(format!(
        "Cycle: {}
------ Execution -------
PC: {}
Instruction: {}
------ Registers -------
{}
-------- Stack ---------
{}
",
        app.last_update.cycle,
        app.last_update.current_program_counter,
        app.last_update.current_instruction,
        registers,
        stack,
    ));

    widget = widget.block(
        Block::default()
            .title("CPU State")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    widget = widget.style(Style::default().fg(Color::White));

    f.render_widget(widget, size);
}

pub fn render_output(app: &mut App, f: &mut Frame, size: Rect) {
    let mut widget = Paragraph::new(format!("{}", app.last_update.output_buffer,));

    widget = widget.block(
        Block::default()
            .title("VM Output")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    widget = widget.style(Style::default().fg(Color::White));

    f.render_widget(widget, size);
}
