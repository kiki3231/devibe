use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::stats::HeatmapData;
use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Daily,
    Heatmap,
    Languages,
    Repos,
    Authors,
}

impl Panel {
    pub fn label(&self) -> &'static str {
        match self {
            Panel::Daily => "Daily",
            Panel::Heatmap => "Heatmap",
            Panel::Languages => "Languages",
            Panel::Repos => "Repos",
            Panel::Authors => "Authors",
        }
    }
}

fn border_style(theme: Theme, focus: Panel, me: Panel) -> Style {
    if focus == me {
        Style::default().fg(theme.accent())
    } else {
        Style::default().fg(theme.border())
    }
}

/// Clip a rect to stay within boundary, returning None if it has zero area.
fn clip(rect: Rect, bounds: Rect) -> Option<Rect> {
    let x = rect.x.max(bounds.x);
    let y = rect.y.max(bounds.y);
    let right = rect.right().min(bounds.right());
    let bottom = rect.bottom().min(bounds.bottom());
    if right <= x || bottom <= y {
        return None;
    }
    Some(Rect::new(x, y, right - x, bottom - y))
}

// --- Daily commit bar chart ---

pub fn render_daily_chart(
    frame: &mut Frame,
    area: Rect,
    data: &[(chrono::NaiveDate, u32)],
    focus: Panel,
    theme: Theme,
    _scroll: usize,
) {
    let block = Block::default()
        .title(" Commits per Day ")
        .borders(Borders::ALL)
        .border_style(border_style(theme, focus, Panel::Daily));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if data.is_empty() || inner.width < 10 || inner.height < 5 {
        if data.is_empty() {
            frame.render_widget(
                Paragraph::new("No commit data")
                    .centered()
                    .fg(theme.text_dim()),
                inner,
            );
        }
        return;
    }

    let max_val = data.iter().map(|(_, c)| *c).max().unwrap_or(1).max(1) as f64;
    let bar_w = 2u16;
    let gap = 1u16;
    let total_w = (bar_w + gap) * data.len() as u16;

    let skip = if total_w > inner.width {
        ((total_w as f64 - inner.width as f64) / (bar_w + gap) as f64).ceil() as usize + 1
    } else {
        1
    };

    let chart_h = inner.height.saturating_sub(2);
    let base_y = inner.y + chart_h.saturating_sub(1);
    let bars = theme.bar_colors();

    for (i, (date, count)) in data.iter().enumerate() {
        if i % skip != 0 && skip > 1 {
            continue;
        }
        let visual_i = i / skip;
        let x = inner.x + (visual_i as u16 * (bar_w + gap)).min(inner.width.saturating_sub(bar_w));
        let h = if max_val > 0.0 {
            (*count as f64 / max_val * chart_h as f64).ceil() as u16
        } else {
            0
        };

        let ratio = *count as f64 / max_val;
        let color = if ratio >= 0.8 {
            bars[3]
        } else if ratio >= 0.5 {
            bars[2]
        } else if ratio >= 0.2 {
            bars[1]
        } else {
            bars[0]
        };

        for row in 0..h {
            let y = base_y.saturating_sub(row);
            if let Some(r) = clip(Rect::new(x, y, 2, 1), inner) {
                frame.render_widget(Paragraph::new(Span::styled("▐▌", Style::default().fg(color))), r);
            }
        }

        if chart_h >= 4 && visual_i as i32 % 2 == 0 && base_y + 1 < inner.bottom() {
            let label = date.format("%m/%d").to_string();
            if let Some(r) = clip(Rect::new(x, base_y + 1, 5, 1), inner) {
                frame.render_widget(
                    Paragraph::new(Span::styled(label, Style::default().fg(theme.text_dim()))),
                    r,
                );
            }
        }
    }

    let max_label = format!("{}", max_val as u32);
    if let Some(r) = clip(Rect::new(inner.x + 1, inner.y, max_label.len() as u16, 1), inner) {
        frame.render_widget(
            Paragraph::new(Span::styled(max_label, Style::default().fg(theme.text_dim()))),
            r,
        );
    }
}

// --- Heatmap ---

pub fn render_heatmap(
    frame: &mut Frame,
    area: Rect,
    data: &HeatmapData,
    focus: Panel,
    theme: Theme,
) {
    let block = Block::default()
        .title(" Activity Heatmap (hour x weekday) ")
        .borders(Borders::ALL)
        .border_style(border_style(theme, focus, Panel::Heatmap));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 52 || inner.height < 10 {
        frame.render_widget(
            Paragraph::new("Terminal too small for heatmap")
                .centered()
                .fg(theme.text_dim()),
            inner,
        );
        return;
    }

    let day_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let cell_w = 2u16;
    let start_x = inner.x + 4;
    let start_y = inner.y + 1;
    let hc = theme.heatmap_colors();
    let empty = theme.heatmap_empty();

    for h in (0..24).step_by(3) {
        let x = start_x + h as u16 * cell_w;
        let label = format!("{:02}", h);
        if let Some(r) = clip(Rect::new(x, inner.y, 2, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(label, Style::default().fg(theme.text_dim()))),
                r,
            );
        }
    }

    for day in 0..7 {
        let y = start_y + day as u16;
        if let Some(r) = clip(Rect::new(inner.x, y, 3, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(day_names[day], Style::default().fg(theme.text()))),
                r,
            );
        }

        for hour in 0..24 {
            let x = start_x + hour as u16 * cell_w;
            let count = data.grid[day][hour];
            let intensity = if data.max_count > 0 {
                count as f64 / data.max_count as f64
            } else {
                0.0
            };
            let bg = if intensity <= 0.0 {
                empty
            } else if intensity < 0.25 {
                hc[0]
            } else if intensity < 0.5 {
                hc[1]
            } else if intensity < 0.75 {
                hc[2]
            } else {
                hc[3]
            };
            if let Some(r) = clip(Rect::new(x, y, cell_w, 1), inner) {
                let cell = Block::default().style(Style::default().bg(bg));
                frame.render_widget(cell, r);
            }
        }
    }

    if inner.height >= 10 {
        let legend_y = start_y + 8;
        let legend_x = start_x + 20;
        let steps = [0.0, 0.25, 0.5, 0.75, 1.0];

        if let Some(r) = clip(Rect::new(legend_x.saturating_sub(5), legend_y, 4, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled("Less", Style::default().fg(theme.text_dim()))),
                r,
            );
        }

        for (i, &intensity) in steps.iter().enumerate() {
            let x = legend_x + i as u16 * 2;
            let bg = if intensity <= 0.0 {
                empty
            } else if intensity < 0.25 {
                hc[0]
            } else if intensity < 0.5 {
                hc[1]
            } else if intensity < 0.75 {
                hc[2]
            } else {
                hc[3]
            };
            if let Some(r) = clip(Rect::new(x, legend_y, 2, 1), inner) {
                let cell = Block::default().style(Style::default().bg(bg));
                frame.render_widget(cell, r);
            }
        }

        if let Some(r) = clip(Rect::new(legend_x + 11, legend_y, 4, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled("More", Style::default().fg(theme.text_dim()))),
                r,
            );
        }
    }
}

// --- Language bars ---

pub fn render_languages(
    frame: &mut Frame,
    area: Rect,
    languages: &[(String, u32)],
    focus: Panel,
    theme: Theme,
    scroll: usize,
) {
    let block = Block::default()
        .title(" Languages (by files) ")
        .borders(Borders::ALL)
        .border_style(border_style(theme, focus, Panel::Languages));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if languages.is_empty() {
        return;
    }

    let total = languages.iter().map(|(_, c)| *c).sum::<u32>() as f64;
    let bar_zone = inner.width.saturating_sub(19);

    let visible_rows = inner.height as usize;
    let max_scroll = languages.len().saturating_sub(visible_rows);
    let scroll = scroll.min(max_scroll);

    for i in 0..languages.len() {
        if i < scroll {
            continue;
        }
        let display_row = i - scroll;
        let y = inner.y + display_row as u16;
        if y >= inner.bottom() {
            break;
        }

        let (name, count) = &languages[i];
        let pct = if total > 0.0 { *count as f64 / total } else { 0.0 };
        let bar_w = (bar_zone as f64 * pct) as u16;

        if let Some(r) = clip(Rect::new(inner.x, y, 12, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(
                    format!("{:>12}", name),
                    Style::default().fg(theme.text()),
                )),
                r,
            );
        }

        if bar_w > 0 {
            if let Some(r) = clip(Rect::new(inner.x + 13, y, bar_w, 1), inner) {
                let bar = "█".repeat(r.width as usize);
                frame.render_widget(
                    Paragraph::new(Span::styled(bar, Style::default().fg(lang_color(name)))),
                    r,
                );
            }
        }

        let pct_label = format!(" {:.0}%", pct * 100.0);
        let pct_w = pct_label.len() as u16;
        let pct_x = inner.x + 13 + bar_w;
        if let Some(r) = clip(Rect::new(pct_x, y, pct_w, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(pct_label, Style::default().fg(theme.text_dim()))),
                r,
            );
        }
    }

    if languages.len() > visible_rows {
        let scrolled = scroll.min(max_scroll);
        if scrolled > 0 {
            if let Some(r) = clip(Rect::new(inner.x, inner.y, inner.width, 1), inner) {
                frame.render_widget(
                    Paragraph::new(Span::styled(" ▲ more", Style::default().fg(theme.text_dim()))),
                    r,
                );
            }
        }
        if scrolled < max_scroll {
            if let Some(r) = clip(Rect::new(inner.x, inner.bottom() - 1, inner.width, 1), inner) {
                frame.render_widget(
                    Paragraph::new(Span::styled(" ▼ more", Style::default().fg(theme.text_dim()))),
                    r,
                );
            }
        }
    }
}

fn lang_color(name: &str) -> Color {
    match name {
        "Rust" => Color::Rgb(222, 165, 132),
        "Python" => Color::Rgb(53, 114, 165),
        "JavaScript" | "JSX" => Color::Rgb(240, 224, 48),
        "TypeScript" | "TSX" => Color::Rgb(49, 120, 198),
        "Go" => Color::Rgb(0, 173, 216),
        "Java" => Color::Rgb(176, 114, 25),
        "Kotlin" => Color::Rgb(127, 82, 255),
        "C" | "C Header" => Color::Rgb(85, 85, 85),
        "C++" | "C++ Header" => Color::Rgb(243, 75, 125),
        "C#" => Color::Rgb(149, 48, 204),
        "Ruby" => Color::Rgb(204, 52, 45),
        "PHP" => Color::Rgb(79, 93, 149),
        "Haskell" => Color::Rgb(94, 80, 134),
        "Elixir" => Color::Rgb(107, 75, 147),
        "Shell" => Color::Rgb(137, 193, 67),
        "Docker" => Color::Rgb(56, 139, 209),
        "Lua" => Color::Rgb(0, 0, 128),
        "Vue" => Color::Rgb(65, 184, 131),
        "Svelte" => Color::Rgb(255, 62, 0),
        "Astro" => Color::Rgb(255, 93, 1),
        _ => Color::Rgb(100, 180, 100),
    }
}

// --- Top repos ---

pub fn render_top_repos(
    frame: &mut Frame,
    area: Rect,
    repos: &[(String, u32)],
    focus: Panel,
    theme: Theme,
    scroll: usize,
) {
    let block = Block::default()
        .title(" Top Repositories (by commits) ")
        .borders(Borders::ALL)
        .border_style(border_style(theme, focus, Panel::Repos));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if repos.is_empty() {
        return;
    }

    let max_count = repos.first().map(|(_, c)| *c).unwrap_or(1).max(1);
    let bar_zone = inner.width.saturating_sub(29);
    let bars = theme.bar_colors();

    let visible_rows = inner.height as usize;
    let max_scroll = repos.len().saturating_sub(visible_rows);
    let scroll = scroll.min(max_scroll);

    for i in 0..repos.len() {
        if i < scroll {
            continue;
        }
        let display_row = i - scroll;
        let y = inner.y + display_row as u16;
        if y >= inner.bottom() {
            break;
        }

        let (name, count) = &repos[i];

        if let Some(r) = clip(Rect::new(inner.x, y, 4, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(
                    format!(" {:>2}.", i + 1),
                    Style::default().fg(theme.text_dim()),
                )),
                r,
            );
        }

        let display_name = if name.len() > 16 { &name[..15] } else { name };
        if let Some(r) = clip(Rect::new(inner.x + 4, y, 16, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(
                    format!("{:<16}", display_name),
                    Style::default().fg(theme.text()),
                )),
                r,
            );
        }

        let bar_w = (*count as f64 / max_count as f64 * bar_zone as f64) as u16;
        if bar_w > 0 {
            if let Some(r) = clip(Rect::new(inner.x + 21, y, bar_w, 1), inner) {
                let intensity = *count as f64 / max_count as f64;
                let color = if intensity >= 0.8 {
                    bars[3]
                } else if intensity >= 0.5 {
                    bars[2]
                } else if intensity >= 0.2 {
                    bars[1]
                } else {
                    bars[0]
                };
                let bar = "█".repeat(r.width as usize);
                frame.render_widget(
                    Paragraph::new(Span::styled(bar, Style::default().fg(color))),
                    r,
                );
            }
        }

        let count_label = format!(" {}", count);
        let cw = count_label.len() as u16;
        if let Some(r) = clip(Rect::new(inner.x + 22 + bar_w, y, cw, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(count_label, Style::default().fg(theme.text_dim()))),
                r,
            );
        }
    }

    if repos.len() > visible_rows {
        if scroll > 0 {
            if let Some(r) = clip(Rect::new(inner.x, inner.y, inner.width, 1), inner) {
                frame.render_widget(
                    Paragraph::new(Span::styled(" ▲ more", Style::default().fg(theme.text_dim()))),
                    r,
                );
            }
        }
        if scroll < max_scroll {
            if let Some(r) = clip(Rect::new(inner.x, inner.bottom() - 1, inner.width, 1), inner) {
                frame.render_widget(
                    Paragraph::new(Span::styled(" ▼ more", Style::default().fg(theme.text_dim()))),
                    r,
                );
            }
        }
    }
}

// --- Authors ---

pub fn render_authors(
    frame: &mut Frame,
    area: Rect,
    authors: &[(String, u32)],
    focus: Panel,
    theme: Theme,
    scroll: usize,
) {
    let block = Block::default()
        .title(" Top Authors (by commits) ")
        .borders(Borders::ALL)
        .border_style(border_style(theme, focus, Panel::Authors));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if authors.is_empty() {
        frame.render_widget(
            Paragraph::new("No author data available")
                .centered()
                .fg(theme.text_dim()),
            inner,
        );
        return;
    }

    let max_count = authors.first().map(|(_, c)| *c).unwrap_or(1).max(1);
    let bar_zone = inner.width.saturating_sub(29);
    let bars = theme.bar_colors();

    let visible_rows = inner.height as usize;
    let max_scroll = authors.len().saturating_sub(visible_rows);
    let scroll = scroll.min(max_scroll);

    for i in 0..authors.len() {
        if i < scroll {
            continue;
        }
        let display_row = i - scroll;
        let y = inner.y + display_row as u16;
        if y >= inner.bottom() {
            break;
        }

        let (name, count) = &authors[i];

        if let Some(r) = clip(Rect::new(inner.x, y, 4, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(
                    format!(" {:>2}.", i + 1),
                    Style::default().fg(theme.text_dim()),
                )),
                r,
            );
        }

        let display_name = if name.len() > 20 { &name[..19] } else { name };
        if let Some(r) = clip(Rect::new(inner.x + 4, y, 20, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(
                    format!("{:<20}", display_name),
                    Style::default().fg(theme.text()),
                )),
                r,
            );
        }

        let bar_w = (*count as f64 / max_count as f64 * bar_zone as f64) as u16;
        if bar_w > 0 {
            if let Some(r) = clip(Rect::new(inner.x + 25, y, bar_w, 1), inner) {
                let intensity = *count as f64 / max_count as f64;
                let color = if intensity >= 0.8 {
                    bars[3]
                } else if intensity >= 0.5 {
                    bars[2]
                } else if intensity >= 0.2 {
                    bars[1]
                } else {
                    bars[0]
                };
                let bar = "█".repeat(r.width as usize);
                frame.render_widget(
                    Paragraph::new(Span::styled(bar, Style::default().fg(color))),
                    r,
                );
            }
        }

        let count_label = format!(" {}", count);
        let cw = count_label.len() as u16;
        if let Some(r) = clip(Rect::new(inner.x + 26 + bar_w, y, cw, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(count_label, Style::default().fg(theme.text_dim()))),
                r,
            );
        }
    }

    if authors.len() > visible_rows {
        if scroll > 0 {
            if let Some(r) = clip(Rect::new(inner.x, inner.y, inner.width, 1), inner) {
                frame.render_widget(
                    Paragraph::new(Span::styled(" ▲ more", Style::default().fg(theme.text_dim()))),
                    r,
                );
            }
        }
        if scroll < max_scroll {
            if let Some(r) = clip(Rect::new(inner.x, inner.bottom() - 1, inner.width, 1), inner) {
                frame.render_widget(
                    Paragraph::new(Span::styled(" ▼ more", Style::default().fg(theme.text_dim()))),
                    r,
                );
            }
        }
    }
}
