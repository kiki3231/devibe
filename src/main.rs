mod scanner;
mod stats;
mod app;
mod ui;
mod widgets;

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
}

fn main() {
    let cli = Cli::parse();

    let paths = if let Some(repo) = cli.repo {
        vec![repo]
    } else {
        let root = cli.scan.unwrap_or_else(|| PathBuf::from("."));
        scanner::discover_repos(&root)
    };

    if paths.is_empty() {
        eprintln!("devibe: no git repositories found in the given path.");
        eprintln!("Hint: use --scan <dir> to search a directory, or --repo <dir> for a single repo.");
        std::process::exit(1);
    }

    let data = stats::compute(&paths);

    if cli.summary {
        print_summary(&data);
        return;
    }

    app::run(data);
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
