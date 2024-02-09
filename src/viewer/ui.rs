use crate::{
    viewer::app::App,
    vm::memory::{HEAP_SIZE, REGISTER_ADDRESS_START},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Frame,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};
use std::fmt::Write;

use super::app::Page;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout_main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(40), Constraint::Min(1)])
        .split(f.size());

    let layout_output = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(layout_main[1]);

    render_cpu_state(app, f, layout_main[0]);

    match app.active_page {
        Page::Output => {
            render_output(app, f, layout_output[0]);
            render_input(app, f, layout_output[1]);
        }
        Page::MemoryView => {
            render_memory(app, f, layout_main[1]);
        }
    }
}

pub fn render_cpu_state(app: &mut App, f: &mut Frame, size: Rect) {
    let mut registers = String::new();
    for (i, r) in app
        .last_update
        .savestate
        .memory
        .registers
        .iter()
        .enumerate()
    {
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
    for (i, (sv, sc)) in app.last_update.savestate.memory.stack.iter().enumerate() {
        if let Some(call) = sc {
            write!(stack, "{:5}: {:6} | CALL {:5}\n", i, sv, call).unwrap();
        } else {
            write!(stack, "{:5}: {:6}\n", i, sv).unwrap();
        }
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
        app.last_update.savestate.cycle,
        app.last_update.savestate.program_counter,
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
    let mut widget = Paragraph::new(format!("{}", app.last_update.savestate.output_buffer,));
    widget = widget.wrap(Wrap { trim: true });
    widget = widget.block(
        Block::default()
            .title("VM Output")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    widget = widget.style(Style::default().fg(Color::White));

    // autoscroll in ratatui paragraph
    let line_count = widget.line_count(size.width);
    let scroll_y = (line_count as u16).saturating_sub(size.height - 2);
    widget = widget.scroll((scroll_y, 0));

    f.render_widget(widget, size);
}

pub fn render_input(app: &mut App, f: &mut Frame, size: Rect) {
    let mut widget = Paragraph::new(format!("{}", app.current_input));

    widget = widget.block(
        Block::default()
            .title("Input")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    widget = widget.style(Style::default().fg(Color::White));

    f.render_widget(widget, size);
}

pub fn render_memory(app: &mut App, f: &mut Frame, size: Rect) {
    let line_width = (size.width - 2) as usize;
    let widget_height = (size.height - 2) as usize;
    let memory_group_size = 7 * 5 + 2;
    let memory_values_per_line = ((line_width - 6) / memory_group_size) * 5;
    let memory_values_total = widget_height * memory_values_per_line;
    if (app.memory_page_scroll * memory_values_per_line + memory_values_total) > HEAP_SIZE {
        app.memory_page_scroll = (HEAP_SIZE - memory_values_total) / memory_values_per_line;
    }

    let memory_page_start = app.memory_page_scroll * memory_values_per_line;
    let memory_page_end = memory_page_start + memory_values_total;

    let mut text = String::new();
    for (i, chunk) in app.last_update.savestate.memory.heap[memory_page_start..memory_page_end]
        .chunks(memory_values_per_line)
        .enumerate()
    {
        write!(
            text,
            "{:5} ",
            (app.memory_page_scroll + i) * memory_values_per_line
        )
        .unwrap();
        for memory_group in chunk.chunks(5) {
            write!(text, " |").unwrap();
            for memval in memory_group {
                write!(text, " {:6}", memval).unwrap();
            }
        }
        writeln!(text).unwrap();
    }
    let mut widget = Paragraph::new(format!("{}", text));
    widget = widget.block(
        Block::default()
            .title("Memory View")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    widget = widget.style(Style::default().fg(Color::White));

    f.render_widget(widget, size);
}
