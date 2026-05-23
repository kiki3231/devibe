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
    let mut selected: usize = 0;

    loop {
        let entries = list_dirs(&current_dir);

        // clamp selection
        if entries.is_empty() {
            selected = 0;
        } else if selected >= entries.len() {
            selected = entries.len().saturating_sub(1);
        }

        terminal.draw(|f| render_picker(f, &current_dir, &entries, selected)).unwrap();

        let Ok(event) = event::read() else { continue };
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return None,
                KeyCode::Up | KeyCode::Char('k') => {
                    selected = selected.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !entries.is_empty() {
                        selected = (selected + 1).min(entries.len() - 1);
                    }
                }
                KeyCode::Enter => {
                    if entries.is_empty() {
                        continue;
                    }
                    let chosen = &entries[selected];
                    if chosen.name == ".." {
                        if let Some(parent) = current_dir.parent() {
                            current_dir = parent.to_path_buf();
                            selected = 0;
                        }
                    } else {
                        let new_dir = current_dir.join(&chosen.name);
                        // If it's a git repo, select it and return
                        if new_dir.join(".git").exists() {
                            return Some(new_dir);
                        }
                        // Otherwise enter the directory
                        if new_dir.is_dir() {
                            current_dir = new_dir;
                            selected = 0;
                        }
                    }
                }
                KeyCode::Backspace => {
                    if let Some(parent) = current_dir.parent() {
                        current_dir = parent.to_path_buf();
                        selected = 0;
                    }
                }
                KeyCode::Home => {
                    if let Some(home) = dirs_home() {
                        current_dir = home;
                        selected = 0;
                    }
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

    // parent dir
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
            // Show dotfiles but mark them differently — still useful for picking project dirs
            // Actually, many projects are in hidden dirs (e.g. ~/.config/nvim). Show them.
        }
        let is_git = path.join(".git").exists();
        dirs.push(DirEntry { name, is_git_repo: is_git });
    }

    // sort: git repos first, then alphabetically
    dirs.sort_by(|a, b| {
        b.is_git_repo.cmp(&a.is_git_repo)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    entries.extend(dirs);
    entries
}

fn render_picker(frame: &mut Frame, current_dir: &Path, entries: &[DirEntry], selected: usize) {
    let bg = Block::default().style(Style::default().bg(Color::Rgb(18, 18, 18)));
    frame.render_widget(bg, frame.area());

    let [header_area, list_area, help_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(5),
        Constraint::Length(1),
    ]).areas(frame.area());

    // header
    let header = Block::default()
        .title(" Select a Project Folder ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = header.inner(header_area);
    frame.render_widget(header, header_area);

    let path_str = current_dir.display().to_string();
    let path_display = if path_str.len() > inner.width as usize {
        format!("...{}", &path_str[path_str.len().saturating_sub(inner.width as usize - 3)..])
    } else {
        path_str
    };
    frame.render_widget(
        Paragraph::new(Span::styled(path_display, Style::default().fg(Color::White))),
        Rect::new(inner.x + 1, inner.y, inner.width.saturating_sub(2), 1),
    );

    // list
    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let list_inner = list_block.inner(list_area);
    frame.render_widget(list_block, list_area);

    let visible = list_inner.height as usize;
    let start = if selected >= visible {
        selected - visible + 1
    } else {
        0
    };

    for (i, entry) in entries.iter().enumerate().skip(start).take(visible) {
        let y = list_inner.y + (i - start) as u16;
        if y >= list_inner.bottom() {
            break;
        }
        let is_cursor = i == selected;
        let prefix = if is_cursor { " ▶ " } else { "   " };
        let suffix = if entry.name == ".." {
            ""
        } else if entry.is_git_repo {
            "  [git]"
        } else {
            "  /"
        };

        let line = format!("{}{}{}", prefix, entry.name, suffix);
        let max_w = list_inner.width.saturating_sub(1) as usize;
        let truncated: String = line.chars().take(max_w).collect();

        let style = if is_cursor {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else if entry.is_git_repo {
            Style::default().fg(Color::Rgb(0, 220, 100))
        } else {
            Style::default().fg(Color::White)
        };
        frame.render_widget(
            Paragraph::new(Span::styled(truncated, style)),
            Rect::new(list_inner.x, y, list_inner.width, 1),
        );
    }

    // help bar
    let help = Span::styled(
        " Enter:Select   Backspace:Up   Home:~   j/k/↑/↓:Navigate   q:Quit",
        Style::default().fg(Color::DarkGray).bg(Color::Rgb(30, 30, 30)),
    );
    frame.render_widget(
        Paragraph::new(help).style(Style::default().bg(Color::Rgb(30, 30, 30))),
        help_area,
    );
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}
