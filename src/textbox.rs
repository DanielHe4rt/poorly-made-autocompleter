use crate::{App, Component, FocusState, Tabs};
use ratatui::crossterm::event;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use regex::Regex;

static QUERY_REGEX: &str = r#"^SELECT\s+(DISTINCT\s+)?(\*|[\w,\s]+)\s+FROM\s+\w+(\s+WHERE\s+[\w\s=<>!\'"(),]+)?(\s+GROUP\s+BY\s+[\w\s,]+)?(\s+ORDER\s+BY\s+[\w\s,]+)?(\s+PER\s+PARTITION\s+LIMIT\s+(\d+|\?))?(\s+LIMIT\s+(\d+|\?))?(\s+ALLOW\s+FILTERING)?(\s+BYPASS\s+CACHE)?(\s+USING\s+TIMEOUT\s+\d+)?\s*;?$"#;

#[derive(Debug, Default, PartialEq)]
pub enum QueryState {
    #[default]
    Waiting,
    Check,
    Error,
}
#[derive(Debug, Default)]
pub struct TextBox {
    pub current_query: String,
    pub character_index: usize,
    pub query_state: QueryState,
}

#[derive(PartialEq, Clone)]
pub enum Message {
    NewCharacter(char),
    Validate,
    Exit,
    DeleteCharacter,
    AutoComplete,
}

impl TextBox {
    pub fn reset_state(&mut self) {
        if self.query_state != QueryState::Waiting {
            self.query_state = QueryState::Waiting;
        }
    }
}
impl Component for TextBox {
    fn handle_key(key: event::KeyEvent) -> Option<Tabs> {
        let message = match key.code {
            KeyCode::Esc => Some(Message::Exit),
            KeyCode::Char(to_insert) => Some(Message::NewCharacter(to_insert)),
            KeyCode::Tab => Some(Message::AutoComplete),
            KeyCode::Enter => Some(Message::Validate),
            KeyCode::Backspace => Some(Message::DeleteCharacter),
            _ => None,
        };

        Some(crate::Tabs::TextBox(message?))
    }

    fn render(model: &App, area: Rect, frame: &mut Frame) {
        let style = Style::default().bg(Color::Black);

        let block = if model.focus_state == FocusState::Editing {
            let border_color = match model.text_box.query_state {
                QueryState::Waiting => Color::Yellow,
                QueryState::Check => Color::Green,
                QueryState::Error => Color::Red,
            };
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(border_color))
                .title("Editing")
        } else {
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("StandBy")
        };

        let input = Paragraph::new(format!("> {}", model.text_box.current_query))
            .style(style)
            .block(block);

        frame.render_widget(input, area);
        if model.focus_state == FocusState::Editing {
            frame.set_cursor_position(Position::new(
                area.x + model.text_box.character_index as u16 + 3,
                area.y + 1,
            ))
        }
    }

    fn update(model: &mut App, msg: Tabs) -> Option<Tabs> {
        match msg {
            Tabs::TextBox(Message::Exit) => {
                model.focus_state = FocusState::StandBy;
            }
            Tabs::TextBox(Message::NewCharacter(char)) => {
                model
                    .text_box
                    .current_query
                    .insert(model.text_box.character_index, char);
                model.text_box.character_index += 1;
                model.update_autocomplete();
                model.text_box.reset_state();
            }
            Tabs::TextBox(Message::DeleteCharacter) => {
                if model.text_box.character_index > 0 {
                    model
                        .text_box
                        .current_query
                        .remove(model.text_box.character_index - 1);
                    model.text_box.character_index -= 1;
                    model.update_autocomplete();
                    model.text_box.reset_state();
                }
            }
            Tabs::TextBox(Message::AutoComplete) => {
                model.text_box.reset_state();
                let possible_completion = model.update_autocomplete()?;

                let (length, completion) = possible_completion;
                for _ in 0..length {
                    model
                        .text_box
                        .current_query
                        .remove(model.text_box.character_index - 1);
                    model.text_box.character_index -= 1;
                }

                model
                    .text_box
                    .current_query
                    .insert_str(model.text_box.character_index, completion.as_str());
                model.text_box.character_index += completion.len();
            }
            Tabs::TextBox(Message::Validate) => {
                let query_regex = Regex::new(QUERY_REGEX).unwrap();

                if query_regex.is_match(model.text_box.current_query.as_str()) {
                    model.text_box.query_state = QueryState::Check
                } else {
                    model.text_box.query_state = QueryState::Error
                }
            }
            _ => {}
        };
        None
    }
}
