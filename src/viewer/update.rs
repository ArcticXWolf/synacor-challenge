use crossterm::event::{KeyCode, KeyEvent};

use crate::viewer::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Tab => app.toggle_page(),
        KeyCode::Up => app.memory_page_scroll = app.memory_page_scroll.saturating_sub(1),
        KeyCode::Down => app.memory_page_scroll = app.memory_page_scroll.saturating_add(1),
        KeyCode::F(5) => app.next_tick_to_send.save_state = true,
        KeyCode::F(6) => app.next_tick_to_send.write_history = true,
        KeyCode::F(8) => app.next_tick_to_send.step_once = true,
        KeyCode::F(9) => app.next_tick_to_send.load_state = true,
        KeyCode::Enter => {
            if app.current_input.starts_with("!") {
                handle_command(app, app.current_input.clone());
            } else {
                let mut result = app.current_input.clone();
                result.push('\n');
                app.next_tick_to_send.additional_stdin = result;
            }
            app.current_input = String::default();
        }
        KeyCode::Backspace => {
            app.current_input.pop();
        }
        KeyCode::Char(c) => {
            app.current_input.push(c);
        }
        _ => {}
    };
}

pub fn handle_command(app: &mut App, input: String) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.first() {
        Some(&"!pause") => app.next_tick_to_send.toggle_pause = true,
        Some(&"!setr") => {
            if let Some(&register_idx_str) = parts.iter().nth(1) {
                if let Ok(register_idx) = register_idx_str.parse::<usize>() {
                    if let Some(&register_value_str) = parts.iter().nth(2) {
                        if let Ok(register_value) = register_value_str.parse::<u16>() {
                            app.next_tick_to_send.set_register_id = Some(register_idx);
                            app.next_tick_to_send.set_register_value = register_value;
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
