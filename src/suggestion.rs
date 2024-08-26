use crate::{App, Component, FocusState, Tabs};
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

#[derive(Debug, Default)]
pub struct Suggestion {
    pub available_completions: Vec<String>,
    pub suggested_completions: Vec<String>,
}

impl Suggestion {
    pub fn new() -> Self {
        Self {
            available_completions: vec![
                "SELECT".to_string(),
                "FROM".to_string(),
                "WHERE".to_string(),
                "AGGREGATORS".to_string(),
            ],
            suggested_completions: vec![],
        }
    }
}

impl Component for Suggestion {
    fn handle_key(_: KeyEvent) -> Option<Tabs> {
        None
    }

    fn render(model: &App, area: Rect, frame: &mut Frame) {
        if model.focus_state != FocusState::Editing {
            return;
        }
        let style = Style::default().bg(Color::Black);
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow))
            .title("Suggestions");

        let suggestion_text = model.suggestion_box.suggested_completions.join(" ");

        let suggestions = Paragraph::new(format!("-> {}", suggestion_text))
            .style(style)
            .block(block);

        frame.render_widget(suggestions, area);
    }

    fn update(_: &mut App, _: crate::Tabs) -> Option<Tabs> {
        None
    }
}
