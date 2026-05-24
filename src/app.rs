use ratatui::crossterm::event::{self, Event, KeyCode};
use std::path::PathBuf;
use crate::stats::{self, DashboardData};
use crate::theme::Theme;
use crate::widgets::Panel;
use crate::scanner;

pub struct App {
    pub data: DashboardData,
    pub should_quit: bool,
    pub focused_panel: Panel,
    pub theme: Theme,
    pub scroll_offset: usize,
    pub scan_paths: Vec<PathBuf>,
    pub days: u32,
    needs_refresh: bool,
}

impl App {
    pub fn new(data: DashboardData, scan_paths: Vec<PathBuf>, days: u32, theme: Theme) -> Self {
        Self {
            data,
            should_quit: false,
            focused_panel: Panel::Daily,
            theme,
            scroll_offset: 0,
            scan_paths,
            days,
            needs_refresh: false,
        }
    }

    pub fn handle_event(&mut self) {
        let Ok(event) = event::read() else { return };
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => self.should_quit = true,

                KeyCode::Char('1') => {
                    self.focused_panel = Panel::Daily;
                    self.scroll_offset = 0;
                }
                KeyCode::Char('2') => {
                    self.focused_panel = Panel::Heatmap;
                    self.scroll_offset = 0;
                }
                KeyCode::Char('3') => {
                    self.focused_panel = Panel::Languages;
                    self.scroll_offset = 0;
                }
                KeyCode::Char('4') => {
                    self.focused_panel = Panel::Repos;
                    self.scroll_offset = 0;
                }
                KeyCode::Char('5') => {
                    self.focused_panel = Panel::Authors;
                    self.scroll_offset = 0;
                }

                KeyCode::Char('t') => {
                    self.theme = self.theme.next();
                }
                KeyCode::Char('T') => {
                    self.theme = self.theme.prev();
                }

                KeyCode::Char('r') => {
                    self.needs_refresh = true;
                }

                KeyCode::Down | KeyCode::Char('j') => {
                    self.scroll_offset += 1;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(1);
                }
                KeyCode::PageDown => {
                    self.scroll_offset += 10;
                }
                KeyCode::PageUp => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(10);
                }
                KeyCode::Home => {
                    self.scroll_offset = 0;
                }
                KeyCode::End => {
                    self.scroll_offset = usize::MAX / 2;
                }

                _ => {}
            }
        }
    }

    pub fn try_refresh(&mut self) {
        if self.needs_refresh {
            let paths: Vec<PathBuf> = if !self.scan_paths.is_empty() {
                scanner::discover_all(&self.scan_paths)
            } else {
                scanner::discover_repos(&PathBuf::from("."))
            };
            if !paths.is_empty() {
                self.data = stats::compute(&paths, self.days);
                self.scan_paths = paths;
            }
            self.needs_refresh = false;
            self.scroll_offset = 0;
        }
    }
}

pub fn run(data: DashboardData, scan_paths: Vec<PathBuf>, days: u32, theme: Theme) {
    let mut terminal = ratatui::init();
    let mut app = App::new(data, scan_paths, days, theme);
    while !app.should_quit {
        terminal.draw(|f| crate::ui::render(f, &app)).unwrap();
        app.handle_event();
        app.try_refresh();
    }
    ratatui::restore();
}
