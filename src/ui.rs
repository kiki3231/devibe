use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;
use crate::widgets::{self, Panel};
use crate::theme::Theme;

pub fn render(frame: &mut Frame, app: &App) {
    let theme = app.theme;
    let bg = Block::default().style(Style::default().bg(theme.bg()));
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

    render_summary(frame, summary_area, &app.data.summary, theme);
    widgets::render_daily_chart(
        frame, daily_area, &app.data.daily_commits,
        app.focused_panel, theme, app.scroll_offset,
    );
    widgets::render_heatmap(
        frame, heatmap_area, &app.data.heatmap,
        app.focused_panel, theme,
    );
    widgets::render_languages(
        frame, lang_area, &app.data.languages,
        app.focused_panel, theme, app.scroll_offset,
    );

    // Bottom-right panel: show repos (4) or authors (5) based on focus
    if app.focused_panel == Panel::Authors {
        widgets::render_authors(
            frame, repos_area, &app.data.authors,
            app.focused_panel, theme, app.scroll_offset,
        );
    } else {
        widgets::render_top_repos(
            frame, repos_area, &app.data.top_repos,
            app.focused_panel, theme, app.scroll_offset,
        );
    }

    render_help(frame, help_area, app.focused_panel, theme);
}

fn render_summary(frame: &mut Frame, area: Rect, s: &crate::stats::Summary, theme: Theme) {
    let text = format!(
        " Repos: {}  |  Commits: {}  |  Lines: +{} / -{}  |  Active days: {} ({}d)  |  Authors: {}  |  {} theme  |  devibe v0.2.0",
        s.repo_count,
        s.total_commits,
        fmt_count(s.lines_added),
        fmt_count(s.lines_deleted),
        s.active_days,
        s.since_days,
        s.total_authors,
        theme.name(),
    );
    let p = Paragraph::new(Span::styled(
        text,
        Style::default().fg(theme.text_dim()).bg(theme.surface()),
    ))
    .style(Style::default().bg(theme.surface()));
    frame.render_widget(p, area);
}

fn render_help(frame: &mut Frame, area: Rect, focus: Panel, theme: Theme) {
    let items = [
        ("1", Panel::Daily),
        ("2", Panel::Heatmap),
        ("3", Panel::Languages),
        ("4", Panel::Repos),
        ("5", Panel::Authors),
    ];

    let mut spans: Vec<Span> = vec![Span::styled(" ", Style::default().bg(theme.surface()))];
    for (key, panel) in &items {
        let style = if focus == *panel {
            Style::default().fg(Color::Black).bg(theme.accent())
        } else {
            Style::default().fg(theme.text_dim()).bg(theme.surface())
        };
        spans.push(Span::styled(format!(" {}({})", key, panel.label()), style));
        spans.push(Span::styled(" ", Style::default().bg(theme.surface())));
    }
    spans.push(Span::styled(
        "  r:Refresh  t:Theme  ↑↓/jk:Scroll  q:Quit",
        Style::default().fg(theme.border()).bg(theme.surface()),
    ));

    let p = Paragraph::new(Line::from(spans)).style(Style::default().bg(theme.surface()));
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
