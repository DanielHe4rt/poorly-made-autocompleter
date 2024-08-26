mod suggestion;
mod textbox;

use crate::suggestion::Suggestion;
use crate::textbox::TextBox;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Frame, Terminal,
};
use std::cmp::PartialEq;
use std::io::{stdout, Result};
use std::time::Duration;

pub trait Component {
    fn handle_key(key: event::KeyEvent) -> Option<Tabs>;

    fn render(model: &App, area: Rect, frame: &mut Frame);

    fn update(model: &mut App, msg: Tabs) -> Option<Tabs>;
}

#[derive(Debug, Default)]
pub struct App {
    pub text_box: TextBox,
    pub focus_state: FocusState,
    pub suggestion_box: Suggestion,
}

impl App {
    pub fn new() -> Self {
        Self {
            text_box: TextBox::default(),
            focus_state: FocusState::Editing,
            suggestion_box: Suggestion::new(),
        }
    }

    pub fn update_autocomplete(&mut self) -> Option<(usize, String)> {
        let last_word = self.text_box.current_query.clone();
        let last_word = last_word.split(" ").last()?;

        if last_word.is_empty() {
            self.suggestion_box.suggested_completions.clear();
            return None;
        }

        let possible_completions = self
            .suggestion_box
            .available_completions
            .iter()
            .filter(|c| c.contains(last_word))
            .map(String::from)
            .collect::<Vec<String>>();

        if possible_completions.is_empty() {
            return None;
        }

        self.suggestion_box.suggested_completions = possible_completions.clone();

        let result = possible_completions.first()?;
        let result = String::from(result);
        Some((last_word.len(), result))
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum FocusState {
    #[default]
    StandBy,
    Editing,
    Done,
}

pub enum Tabs {
    TextBox(textbox::Message),
    SuggestionsBox,
}

fn view(model: &mut App, frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);
    let [input_area, suggestion_area, _] = vertical.areas(frame.area());

    TextBox::render(model, input_area, frame);
    Suggestion::render(model, suggestion_area, frame);
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut model = App::new();

    while model.focus_state != FocusState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let current_msg = handle_event(&mut model).unwrap();

        // Process updates as long as they return a non-None message
        if current_msg.is_none() {
            continue;
        }

        let current_message = current_msg.unwrap();

        match current_message {
            Tabs::TextBox(_) => TextBox::update(&mut model, current_message),
            _ => None,
        };
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn handle_event(app: &mut App) -> color_eyre::Result<Option<Tabs>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(app, key));
            }
        }
    }
    Ok(None)
}

fn handle_key(app: &mut App, key: event::KeyEvent) -> Option<Tabs> {
    match app.focus_state {
        FocusState::StandBy => match key.code {
            KeyCode::Enter => {
                app.focus_state = FocusState::Editing;
                None
            }
            KeyCode::Esc => {
                app.focus_state = FocusState::Done;
                None
            }
            _ => None,
        },
        FocusState::Editing => TextBox::handle_key(key),
        _ => None,
    }
}
