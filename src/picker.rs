use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::crossterm::event::{self, Event, KeyCode};
use std::path::{Path, PathBuf};
use std::fs;

pub fn run_picker(start_dir: &Path) -> Option<PathBuf> {
    if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
        eprintln!("devibe: no git repos in current directory, and no terminal available for picker.");
        eprintln!("Hint: use --scan <dir> or --repo <dir> to specify a path.");
        return None;
    }
    let mut terminal = ratatui::init();
    let result = picker_loop(&mut terminal, start_dir);
    ratatui::restore();
    result
}

fn picker_loop(terminal: &mut ratatui::DefaultTerminal, start_dir: &Path) -> Option<PathBuf> {
    let mut current_dir = fs::canonicalize(start_dir).unwrap_or_else(|_| start_dir.to_path_buf());
    let mut state = ListState::default();
    let mut scroll_offset: usize = 0;

    loop {
        let entries = list_dirs(&current_dir);

        if entries.is_empty() {
            state.select(None);
        } else {
            let idx = state.selected().unwrap_or(0);
            if idx >= entries.len() {
                state.select(Some(entries.len().saturating_sub(1)));
            }
        }

        // Keep selected entry visible
        if let Some(sel) = state.selected() {
            if sel < scroll_offset {
                scroll_offset = sel;
            }
            // Estimate visible area: ~15 items in list area
            let visible = 15usize;
            if sel >= scroll_offset + visible {
                scroll_offset = sel - visible + 1;
            }
        }

        terminal.draw(|f| render_picker(f, &current_dir, &entries, &mut state, scroll_offset)).unwrap();

        let Ok(event) = event::read() else { continue };
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return None,
                KeyCode::Up | KeyCode::Char('k') => {
                    let idx = state.selected().unwrap_or(0);
                    let new_idx = idx.saturating_sub(1);
                    if new_idx < scroll_offset && scroll_offset > 0 {
                        scroll_offset = new_idx;
                    }
                    state.select(Some(new_idx));
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !entries.is_empty() {
                        let idx = state.selected().unwrap_or(0);
                        let new_idx = (idx + 1).min(entries.len() - 1);
                        if new_idx >= scroll_offset + 15 {
                            scroll_offset = new_idx - 14;
                        }
                        state.select(Some(new_idx));
                    }
                }
                KeyCode::Enter => {
                    let Some(idx) = state.selected() else { continue };
                    if idx >= entries.len() {
                        continue;
                    }
                    let chosen = &entries[idx];
                    if chosen.name == ".." {
                        if let Some(parent) = current_dir.parent() {
                            current_dir = parent.to_path_buf();
                            state.select(Some(0));
                            scroll_offset = 0;
                        }
                    } else {
                        let new_dir = current_dir.join(&chosen.name);
                        if new_dir.join(".git").exists() {
                            return Some(new_dir);
                        }
                        if new_dir.is_dir() {
                            current_dir = new_dir;
                            state.select(Some(0));
                            scroll_offset = 0;
                        }
                    }
                }
                KeyCode::Backspace => {
                    if let Some(parent) = current_dir.parent() {
                        current_dir = parent.to_path_buf();
                        state.select(Some(0));
                        scroll_offset = 0;
                    }
                }
                KeyCode::Home => {
                    if let Some(home) = dirs_home() {
                        current_dir = home;
                        state.select(Some(0));
                        scroll_offset = 0;
                    }
                }
                KeyCode::PageDown => {
                    if !entries.is_empty() {
                        let idx = state.selected().unwrap_or(0);
                        let new_idx = (idx + 15).min(entries.len() - 1);
                        scroll_offset = (scroll_offset + 15).min(entries.len().saturating_sub(1));
                        state.select(Some(new_idx));
                    }
                }
                KeyCode::PageUp => {
                    let idx = state.selected().unwrap_or(0);
                    let new_idx = idx.saturating_sub(15);
                    scroll_offset = scroll_offset.saturating_sub(15);
                    state.select(Some(new_idx));
                }
                _ => {}
            }
        }
    }
}

struct DirEntry {
    name: String,
    is_git_repo: bool,
}

fn list_dirs(dir: &Path) -> Vec<DirEntry> {
    let mut entries = Vec::new();

    if dir.parent().is_some() {
        entries.push(DirEntry { name: "..".into(), is_git_repo: false });
    }

    let Ok(read) = fs::read_dir(dir) else { return entries };

    let mut dirs: Vec<DirEntry> = Vec::new();
    for entry in read.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') || name == "node_modules" || name == "target" || name == "vendor" {
            continue;
        }
        let is_git = path.join(".git").exists();
        dirs.push(DirEntry { name, is_git_repo: is_git });
    }

    dirs.sort_by(|a, b| {
        b.is_git_repo
            .cmp(&a.is_git_repo)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    entries.extend(dirs);
    entries
}

fn render_picker(
    frame: &mut Frame,
    current_dir: &Path,
    entries: &[DirEntry],
    state: &mut ListState,
    scroll_offset: usize,
) {
    let bg = Color::Rgb(18, 18, 18);
    let surface = Color::Rgb(30, 30, 30);
    let accent = Color::Rgb(0, 220, 200);
    let text_white = Color::White;
    let text_dim = Color::DarkGray;
    let git_green = Color::Rgb(0, 220, 100);

    frame.render_widget(
        Block::default().style(Style::default().bg(bg)),
        frame.area(),
    );

    let [header_area, list_area, help_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(5),
        Constraint::Length(1),
    ]).areas(frame.area());

    // header
    let header = Block::default()
        .title(" Select a Project Folder ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent));
    frame.render_widget(&header, header_area);

    let inner = header.inner(header_area);
    let path_str = current_dir.display().to_string();
    let max_w = inner.width.saturating_sub(2) as usize;
    let display = if path_str.len() > max_w {
        format!("...{}", &path_str[path_str.len().saturating_sub(max_w.saturating_sub(3))..])
    } else {
        path_str
    };
    frame.render_widget(
        Paragraph::new(Span::styled(display, Style::default().fg(text_white))),
        Rect::new(inner.x + 1, inner.y, inner.width.saturating_sub(2), 1),
    );

    // list
    let list_horiz = Layout::horizontal([
        Constraint::Length(2),
        Constraint::Min(10),
        Constraint::Length(2),
    ]).areas::<3>(list_area);
    let list_content_area = list_horiz[1];

    let visible_start = scroll_offset;
    let visible_end = (scroll_offset + list_content_area.height as usize).min(entries.len());
    let visible_entries = &entries[visible_start..visible_end];

    let items: Vec<ListItem> = visible_entries
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let abs_idx = visible_start + i;
            let suffix = if e.name == ".." {
                String::new()
            } else if e.is_git_repo {
                "  [git]".to_string()
            } else {
                "  /".to_string()
            };
            let line = format!("  {}{}", e.name, suffix);
            let style = if e.is_git_repo {
                Style::default().fg(git_green)
            } else {
                Style::default().fg(text_white)
            };
            let is_selected = state.selected() == Some(abs_idx);

            ListItem::new(Line::styled(line, style))
                .style(if is_selected {
                    Style::default().fg(Color::Black).bg(accent)
                } else {
                    Style::default()
                })
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .highlight_symbol(" ▶");

    frame.render_widget(list, list_content_area);

    // Scroll indicators
    if scroll_offset > 0 {
        frame.render_widget(
            Paragraph::new(Span::styled(" ▲", Style::default().fg(text_dim))),
            Rect::new(list_content_area.x + list_content_area.width - 2, list_content_area.y, 2, 1),
        );
    }
    if visible_start + (list_content_area.height as usize) < entries.len() {
        frame.render_widget(
            Paragraph::new(Span::styled(" ▼", Style::default().fg(text_dim))),
            Rect::new(
                list_content_area.x + list_content_area.width - 2,
                list_content_area.y + list_content_area.height - 1,
                2,
                1,
            ),
        );
    }

    // help bar
    let help = Span::styled(
        " Enter:Select  Backspace:Up  Home:~  jk/↑↓/PgUp/PgDn:Navigate  q:Quit",
        Style::default().fg(text_dim).bg(surface),
    );
    frame.render_widget(
        Paragraph::new(help).style(Style::default().bg(surface)),
        help_area,
    );
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}
