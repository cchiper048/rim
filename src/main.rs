pub mod engine;

use std::cmp;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Layout, Position};
use ratatui::widgets::Paragraph;
use crate::engine::line::Line;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut cursor_line_index: usize = 0;
    let mut cursor_character_index: usize = 0;

    let mut content: Vec<Line> = Vec::new();
    content.push(Line::new());

    loop {
        terminal.draw(|frame| render(
            frame,
            &content,
            cursor_character_index,
            cursor_line_index
        ))?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                    content[cursor_line_index].insert_char(c, cursor_character_index);
                    cursor_character_index += 1;
                }

                KeyCode::Left => {
                    if cursor_character_index > 0 {
                        cursor_character_index -= 1;
                    } else if (cursor_line_index > 0) {
                        cursor_line_index -= 1;
                        cursor_character_index = content[cursor_line_index].len();
                    }
                }

                KeyCode::Right => {
                    cursor_character_index += 1;
                    if cursor_character_index > content[cursor_line_index].len() {
                        cursor_character_index = 0;
                        cursor_line_index += 1;
                        if (content.len() == cursor_line_index) {
                            content.push(Line::new());
                        }
                    }
                }

                KeyCode::Up => {
                    if cursor_line_index == 0 {
                        cursor_line_index = 0;
                        cursor_character_index = 0;
                    } else {
                        cursor_line_index -= 1;
                    }

                    if cursor_character_index >= content[cursor_line_index].len() {
                        cursor_character_index = content[cursor_line_index].len();
                    }
                }

                KeyCode::Down => {
                    if cursor_line_index + 1 >= content.len() {
                        content.push(Line::new());
                        cursor_character_index = 0;
                    }

                    cursor_line_index += 1;

                    if cursor_character_index >= content[cursor_line_index].len() {
                        cursor_character_index = content[cursor_line_index].len();
                    }
                }

                KeyCode::Enter => {
                    content.insert((cursor_line_index + 1), Line::new());
                    let mut line_data_copy: String = content[cursor_line_index].as_str().to_string();
                    let (left, right) =
                        line_data_copy
                            .split_at_mut(cursor_character_index);

                    content[cursor_line_index].set_str(left);
                    content[(cursor_line_index + 1)].set_str(right);

                    cursor_line_index += 1;
                    cursor_character_index = 0;
                }

                // TODO: Finish backspace logic
                KeyCode::Backspace => {
                    content[cursor_line_index].remove_char_at((cursor_character_index - 1));
                    cursor_character_index -= 1;
                }

                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, content: &Vec<Line>, cursor_position_x: usize, cursor_position_y: usize) {
    let mut left_offset: usize = 0;
    let mut top_offset: usize = 0;

    if cursor_position_x < left_offset {
        left_offset = cursor_position_x;
    }
    if cursor_position_y < top_offset {
        top_offset = cursor_position_y;
    }
    if cursor_position_x - left_offset > (frame.area().width - 3) as usize {
        left_offset = cursor_position_x - (frame.area().width - 3) as usize;
    }

    let editor_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            content
                .iter()
                .map(|_| Constraint::Length(1))
                .collect::<Vec<_>>(),
        )
        .split(frame.area());


    for (i, (line, &area)) in content.iter().skip(top_offset).zip(editor_layout.iter()).enumerate() {
        let line_layout =
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Length(3),
                    Constraint::Fill(1),
                ])
                .split(area);

        frame.render_widget(Paragraph::new(i.to_string()), line_layout[0]);
        frame.render_widget(Paragraph::new(&line.as_str()[left_offset..]), line_layout[1]);
    }

    frame.set_cursor_position(Position::new(
        (cursor_position_x - left_offset + 3) as u16,
        (cursor_position_y - top_offset) as u16
    ));
}

