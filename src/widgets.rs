use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::stats::HeatmapData;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Daily,
    Heatmap,
    Languages,
    Repos,
}

fn border_style(focus: Panel, me: Panel) -> Style {
    if focus == me {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
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

pub fn render_daily_chart(frame: &mut Frame, area: Rect, data: &[(chrono::NaiveDate, u32)], focus: Panel) {
    let block = Block::default()
        .title(" Commits per Day (last 14 days) ")
        .borders(Borders::ALL)
        .border_style(border_style(focus, Panel::Daily));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if data.is_empty() || inner.width < 10 || inner.height < 5 {
        return;
    }

    let max_val = data.iter().map(|(_, c)| *c).max().unwrap_or(1).max(1) as f64;
    let bar_w = 2u16;
    let gap = 1u16;
    let total_w = (bar_w + gap) * data.len() as u16;

    let skip = if total_w > inner.width {
        ((total_w as f64 - inner.width as f64) / (bar_w + gap) as f64).ceil() as usize
    } else {
        0
    };

    let chart_h = inner.height.saturating_sub(2);
    let base_y = inner.y + chart_h.saturating_sub(1);

    for (i, (date, count)) in data.iter().enumerate() {
        if i % (skip.max(1)) != 0 && skip > 0 {
            continue;
        }
        let x = inner.x + (i as u16 * (bar_w + gap)).min(inner.width.saturating_sub(bar_w));
        let h = if max_val > 0.0 {
            (*count as f64 / max_val * chart_h as f64).ceil() as u16
        } else {
            0
        };

        for row in 0..h {
            let y = base_y.saturating_sub(row);
            if let Some(r) = clip(Rect::new(x, y, 2, 1), inner) {
                let style = bar_style(*count as f64 / max_val);
                frame.render_widget(Paragraph::new(Span::styled("██", style)), r);
            }
        }

        if chart_h >= 4 && i % 2 == 0 && base_y + 1 < inner.bottom() {
            let label = date.format("%m/%d").to_string();
            if let Some(r) = clip(Rect::new(x, base_y + 1, 5, 1), inner) {
                frame.render_widget(
                    Paragraph::new(Span::styled(label, Style::default().fg(Color::DarkGray))),
                    r,
                );
            }
        }
    }

    let max_label = format!("{}", max_val as u32);
    let label_len = (max_label.len() as u16 + 1).min(inner.width);
    if let Some(r) = clip(Rect::new(inner.x, inner.y, label_len, 1), inner) {
        frame.render_widget(
            Paragraph::new(Span::styled(max_label, Style::default().fg(Color::DarkGray))),
            r,
        );
    }
}

fn bar_style(ratio: f64) -> Style {
    let color = match ratio {
        x if x >= 0.8 => Color::Rgb(0, 220, 100),
        x if x >= 0.5 => Color::Rgb(0, 180, 80),
        x if x >= 0.2 => Color::Rgb(80, 160, 60),
        _ => Color::Rgb(60, 80, 60),
    };
    Style::default().fg(color)
}

// --- Heatmap ---

pub fn render_heatmap(frame: &mut Frame, area: Rect, data: &HeatmapData, focus: Panel) {
    let block = Block::default()
        .title(" Activity Heatmap (hour x weekday) ")
        .borders(Borders::ALL)
        .border_style(border_style(focus, Panel::Heatmap));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 52 || inner.height < 10 {
        frame.render_widget(
            Paragraph::new("Terminal too small for heatmap").centered().fg(Color::DarkGray),
            inner,
        );
        return;
    }

    let day_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let cell_w = 2u16;
    let start_x = inner.x + 4;
    let start_y = inner.y + 1;

    // hour headers
    for h in (0..24).step_by(3) {
        let x = start_x + h as u16 * cell_w;
        let label = format!("{:02}", h);
        if let Some(r) = clip(Rect::new(x, inner.y, 2, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(label, Style::default().fg(Color::DarkGray))),
                r,
            );
        }
    }

    // cells
    for day in 0..7 {
        let y = start_y + day as u16;
        if let Some(r) = clip(Rect::new(inner.x, y, 3, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(day_names[day], Style::default().fg(Color::White))),
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
            let bg = heat_color(intensity);
            if let Some(r) = clip(Rect::new(x, y, cell_w, 1), inner) {
                let cell = Block::default().style(Style::default().bg(bg));
                frame.render_widget(cell, r);
            }
        }
    }

    // legend
    if inner.height >= 10 {
        let legend_y = start_y + 8;
        let legend_x = start_x + 25;
        let steps = [0.0, 0.25, 0.5, 0.75, 1.0];
        let labels = ["0", "1/4", "1/2", "3/4", "1"];
        for (i, (&intensity, label)) in steps.iter().zip(labels.iter()).enumerate() {
            let x = legend_x + i as u16 * 3;
            if let Some(r) = clip(Rect::new(x, legend_y, 2, 1), inner) {
                let cell = Block::default().style(Style::default().bg(heat_color(intensity)));
                frame.render_widget(cell, r);
            }
            if let Some(r) = clip(Rect::new(x, legend_y + 1, 3, 1), inner) {
                frame.render_widget(
                    Paragraph::new(Span::styled(*label, Style::default().fg(Color::DarkGray))),
                    r,
                );
            }
        }
    }
}

fn heat_color(intensity: f64) -> Color {
    if intensity <= 0.0 {
        Color::Rgb(30, 30, 30)
    } else if intensity < 0.25 {
        Color::Rgb(20, 80, 40)
    } else if intensity < 0.5 {
        Color::Rgb(0, 140, 60)
    } else if intensity < 0.75 {
        Color::Rgb(0, 190, 80)
    } else {
        Color::Rgb(0, 240, 100)
    }
}

// --- Language bars ---

pub fn render_languages(frame: &mut Frame, area: Rect, languages: &[(String, u32)], focus: Panel) {
    let block = Block::default()
        .title(" Languages (by files) ")
        .borders(Borders::ALL)
        .border_style(border_style(focus, Panel::Languages));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if languages.is_empty() {
        return;
    }

    let total = languages.iter().map(|(_, c)| *c).sum::<u32>() as f64;
    // Reserve 12 for name + 1 gap + 6 for " XX%" label = 19
    let bar_zone = inner.width.saturating_sub(19);

    for (i, (name, count)) in languages.iter().enumerate() {
        let y = inner.y + i as u16;
        if y >= inner.bottom() {
            break;
        }

        let pct = if total > 0.0 { *count as f64 / total } else { 0.0 };
        let bar_w = (bar_zone as f64 * pct) as u16;

        // name
        if let Some(r) = clip(Rect::new(inner.x, y, 12, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(format!("{:>12}", name), Style::default().fg(Color::White))),
                r,
            );
        }

        // bar
        if bar_w > 0 {
            if let Some(r) = clip(Rect::new(inner.x + 13, y, bar_w, 1), inner) {
                let bar = "█".repeat(r.width as usize);
                frame.render_widget(
                    Paragraph::new(Span::styled(bar, Style::default().fg(lang_color(name)))),
                    r,
                );
            }
        }

        // percentage
        let pct_label = format!(" {:.0}%", pct * 100.0);
        let pct_w = pct_label.len() as u16;
        let pct_x = inner.x + 13 + bar_w;
        if let Some(r) = clip(Rect::new(pct_x, y, pct_w, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(pct_label, Style::default().fg(Color::DarkGray))),
                r,
            );
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
        _ => Color::Rgb(100, 180, 100),
    }
}

// --- Top repos ---

pub fn render_top_repos(frame: &mut Frame, area: Rect, repos: &[(String, u32)], focus: Panel) {
    let block = Block::default()
        .title(" Top Repositories (by commits) ")
        .borders(Borders::ALL)
        .border_style(border_style(focus, Panel::Repos));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if repos.is_empty() {
        return;
    }

    let max_count = repos.first().map(|(_, c)| *c).unwrap_or(1).max(1);
    // 4 (rank) + 16 (name) + 1 (gap) + 8 (count label) = 29
    let bar_zone = inner.width.saturating_sub(29);

    for (i, (name, count)) in repos.iter().enumerate() {
        let y = inner.y + i as u16;
        if y >= inner.bottom() {
            break;
        }

        // rank
        if let Some(r) = clip(Rect::new(inner.x, y, 4, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(format!(" {:>2}.", i + 1), Style::default().fg(Color::DarkGray))),
                r,
            );
        }

        // name
        let display_name = if name.len() > 16 { &name[..15] } else { name };
        if let Some(r) = clip(Rect::new(inner.x + 4, y, 16, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(format!("{:<16}", display_name), Style::default().fg(Color::White))),
                r,
            );
        }

        // bar
        let bar_w = (*count as f64 / max_count as f64 * bar_zone as f64) as u16;
        if bar_w > 0 {
            if let Some(r) = clip(Rect::new(inner.x + 21, y, bar_w, 1), inner) {
                let intensity = *count as f64 / max_count as f64;
                let bg = match intensity {
                    x if x >= 0.8 => Color::Rgb(0, 220, 100),
                    x if x >= 0.5 => Color::Rgb(0, 180, 80),
                    x if x >= 0.2 => Color::Rgb(80, 160, 60),
                    _ => Color::Rgb(60, 80, 60),
                };
                let bar = "█".repeat(r.width as usize);
                frame.render_widget(
                    Paragraph::new(Span::styled(bar, Style::default().fg(bg))),
                    r,
                );
            }
        }

        // count
        let count_label = format!(" {}", count);
        let cw = count_label.len() as u16;
        if let Some(r) = clip(Rect::new(inner.x + 22 + bar_w, y, cw, 1), inner) {
            frame.render_widget(
                Paragraph::new(Span::styled(count_label, Style::default().fg(Color::DarkGray))),
                r,
            );
        }
    }
}
