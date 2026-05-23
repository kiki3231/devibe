use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;
use crate::widgets::{self, Panel};

pub fn render(frame: &mut Frame, app: &App) {
    let bg = Block::default().style(Style::default().bg(Color::Rgb(18, 18, 18)));
    frame.render_widget(bg, frame.area());

    let [summary_area, main_area, bottom_area, help_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(10),
        Constraint::Length(10),
        Constraint::Length(1),
    ]).areas(frame.area());

    let [daily_area, heatmap_area] = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ]).areas(main_area);

    let [lang_area, repos_area] = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(60),
    ]).areas(bottom_area);

    render_summary(frame, summary_area, &app.data.summary);
    widgets::render_daily_chart(frame, daily_area, &app.data.daily_commits, app.focused_panel);
    widgets::render_heatmap(frame, heatmap_area, &app.data.heatmap, app.focused_panel);
    widgets::render_languages(frame, lang_area, &app.data.languages, app.focused_panel);
    widgets::render_top_repos(frame, repos_area, &app.data.top_repos, app.focused_panel);
    render_help(frame, help_area, app.focused_panel);
}

fn render_summary(frame: &mut Frame, area: Rect, s: &crate::stats::Summary) {
    let text = format!(
        " Repos: {}  |  Commits: {}  |  Lines: +{} / -{}  |  Active days: {} (over {}d)  |  devibe v0.1.0",
        s.repo_count, s.total_commits, fmt_count(s.lines_added), fmt_count(s.lines_deleted), s.active_days, s.since_days
    );
    let span = Span::styled(
        text,
        Style::default().fg(Color::Rgb(180, 180, 180)).bg(Color::Rgb(30, 30, 30)),
    );
    let p = Paragraph::new(span).style(Style::default().bg(Color::Rgb(30, 30, 30)));
    frame.render_widget(p, area);
}

fn render_help(frame: &mut Frame, area: Rect, focus: Panel) {
    let items = [
        ("1", "Daily", focus == Panel::Daily),
        ("2", "Heatmap", focus == Panel::Heatmap),
        ("3", "Languages", focus == Panel::Languages),
        ("4", "Repos", focus == Panel::Repos),
    ];

    let mut spans: Vec<Span> = vec![Span::styled(" ", Style::default())];
    for (key, label, active) in &items {
        let style = if *active {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default().fg(Color::Gray).bg(Color::Rgb(30, 30, 30))
        };
        spans.push(Span::styled(format!("{}({})", key, label), style));
        spans.push(Span::styled("  ", Style::default().bg(Color::Rgb(30, 30, 30))));
    }
    spans.push(Span::styled(
        "   q:Quit",
        Style::default().fg(Color::DarkGray).bg(Color::Rgb(30, 30, 30)),
    ));

    let p = Paragraph::new(Line::from(spans)).style(Style::default().bg(Color::Rgb(30, 30, 30)));
    frame.render_widget(p, area);
}

fn fmt_count(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{}k", n / 1_000)
    } else {
        n.to_string()
    }
}
