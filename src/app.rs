use ratatui::crossterm::event::{self, Event, KeyCode};
use crate::stats::DashboardData;
use crate::widgets::Panel;

pub struct App {
    pub data: DashboardData,
    pub should_quit: bool,
    pub focused_panel: Panel,
}

impl App {
    pub fn new(data: DashboardData) -> Self {
        Self { data, should_quit: false, focused_panel: Panel::Daily }
    }

    pub fn handle_event(&mut self) {
        let Ok(event) = event::read() else { return };
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => self.should_quit = true,
                KeyCode::Char('1') => self.focused_panel = Panel::Daily,
                KeyCode::Char('2') => self.focused_panel = Panel::Heatmap,
                KeyCode::Char('3') => self.focused_panel = Panel::Languages,
                KeyCode::Char('4') => self.focused_panel = Panel::Repos,
                _ => {}
            }
        }
    }
}

pub fn run(data: DashboardData) {
    let mut terminal = ratatui::init();
    let mut app = App::new(data);
    while !app.should_quit {
        terminal.draw(|f| crate::ui::render(f, &app)).unwrap();
        app.handle_event();
    }
    ratatui::restore();
}
