mod scanner;
mod stats;
mod app;
mod ui;
mod widgets;
mod picker;
mod theme;
mod config;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "devibe", version, about = "Terminal developer dashboard — visualize your git activity")]
struct Cli {
    /// Scan all git repos under this directory (max depth 3)
    #[arg(short, long)]
    scan: Option<PathBuf>,

    /// Analyze a single git repo
    #[arg(short, long)]
    repo: Option<PathBuf>,

    /// Print text summary and exit (no TUI, suitable for CI)
    #[arg(long)]
    summary: bool,

    /// Number of days for the daily chart (default: 14)
    #[arg(short = 'd', long, default_value = "14")]
    days: u32,

    /// Export data as JSON and exit
    #[arg(long, value_name = "FILE")]
    export_json: Option<PathBuf>,

    /// Export data as CSV and exit
    #[arg(long, value_name = "FILE")]
    export_csv: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let cfg = config::Config::load();

    let days = if cli.days != 14 { cli.days } else { cfg.days };

    let paths = if let Some(repo) = cli.repo {
        vec![repo]
    } else if let Some(scan_dir) = cli.scan {
        scanner::discover_repos(&scan_dir)
    } else if cli.summary {
        scanner::discover_repos(&PathBuf::from("."))
    } else {
        let current = PathBuf::from(".");
        let repos = scanner::discover_repos(&current);
        if !repos.is_empty() {
            repos
        } else {
            let start = std::env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."));
            let picked = picker::run_picker(&start);
            match picked {
                Some(dir) => scanner::discover_repos(&dir),
                None => {
                    eprintln!("devibe: no directory selected.");
                    std::process::exit(0);
                }
            }
        }
    };

    if paths.is_empty() {
        eprintln!("devibe: no git repositories found in the given path.");
        eprintln!("Hint: use --scan <dir> to search a directory, or --repo <dir> for a single repo.");
        std::process::exit(1);
    }

    let data = stats::compute(&paths, days);

    if let Some(json_path) = cli.export_json {
        match serde_json::to_string_pretty(&data) {
            Ok(json) => {
                std::fs::write(&json_path, json).unwrap_or_else(|e| {
                    eprintln!("devibe: failed to write JSON: {}", e);
                });
                println!("JSON exported to {}", json_path.display());
            }
            Err(e) => eprintln!("devibe: failed to serialize JSON: {}", e),
        }
        return;
    }

    if let Some(csv_path) = cli.export_csv {
        let csv = stats::export_csv(&data);
        std::fs::write(&csv_path, csv).unwrap_or_else(|e| {
            eprintln!("devibe: failed to write CSV: {}", e);
        });
        println!("CSV exported to {}", csv_path.display());
        return;
    }

    if cli.summary {
        print_summary(&data);
        return;
    }

    let theme = cfg.theme
        .as_deref()
        .and_then(|t| match t.to_lowercase().as_str() {
            "dark" => Some(theme::Theme::Dark),
            "light" => Some(theme::Theme::Light),
            "gruvbox" => Some(theme::Theme::Gruvbox),
            "nord" => Some(theme::Theme::Nord),
            "catppuccin" => Some(theme::Theme::Catppuccin),
            "monokai" => Some(theme::Theme::Monokai),
            "onedark" => Some(theme::Theme::OneDark),
            _ => None,
        })
        .unwrap_or(theme::Theme::Dark);

    app::run(data, paths, days, theme);
}

fn print_summary(data: &stats::DashboardData) {
    let s = &data.summary;
    println!("╔══════════════════════════════════════╗");
    println!("║        devibe — Dashboard           ║");
    println!("╠══════════════════════════════════════╣");
    println!("║ Repos scanned:     {:>4}             ║", s.repo_count);
    println!("║ Total commits:     {:>4}             ║", s.total_commits);
    println!("║ Lines added:       {:>4}             ║", fmt_count(s.lines_added));
    println!("║ Lines deleted:     {:>4}             ║", fmt_count(s.lines_deleted));
    println!("║ Active days:       {:>4}             ║", s.active_days);
    println!("╠══════════════════════════════════════╣");

    if !data.daily_commits.is_empty() {
        println!("║ Recent daily commits:               ║");
        for (date, count) in &data.daily_commits {
            let bar = "█".repeat((*count as usize).min(40));
            println!("║  {}  {:>3}  {}║", date.format("%m/%d"), count, bar);
        }
    }

    println!("╠══════════════════════════════════════╣");
    if !data.languages.is_empty() {
        println!("║ Languages (by files):               ║");
        for (name, count) in &data.languages {
            println!("║  {:>14}  {:>5} files           ║", name, count);
        }
    }

    println!("╠══════════════════════════════════════╣");
    if !data.top_repos.is_empty() {
        println!("║ Top repos (by commits):             ║");
        for (i, (name, count)) in data.top_repos.iter().enumerate() {
            println!("║  {:>2}. {:>16}  {:>5} commits   ║", i + 1, name, count);
        }
    }
    println!("╚══════════════════════════════════════╝");
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
