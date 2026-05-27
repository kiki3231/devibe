use chrono::{NaiveDate, NaiveDateTime, DateTime, Datelike, Timelike, Duration, Utc};
use git2::Repository;
use rayon::prelude::*;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

const MAX_COMMITS_PER_REPO: usize = 5000;

#[derive(Serialize)]
pub struct DashboardData {
    pub summary: Summary,
    pub daily_commits: Vec<(NaiveDate, u32)>,
    pub heatmap: HeatmapData,
    pub languages: Vec<(String, u32)>,
    pub top_repos: Vec<(String, u32)>,
    #[serde(skip)]
    pub authors: Vec<(String, u32)>,
    #[serde(skip)]
    pub warnings: Vec<String>,
}

#[derive(Serialize)]
pub struct Summary {
    pub repo_count: usize,
    pub total_commits: u32,
    pub lines_added: u64,
    pub lines_deleted: u64,
    pub active_days: u32,
    pub since_days: i64,
    pub total_authors: u32,
    pub warning_count: u32,
}

#[derive(Serialize)]
pub struct HeatmapData {
    pub grid: [[u32; 24]; 7],
    pub max_count: u32,
}

struct RepoResult {
    repo_name: String,
    commit_count: u32,
    file_extensions: HashMap<String, u32>,
    authors: HashMap<String, u32>,
    commits: Vec<CommitEntry>,
    warnings: Vec<String>,
}

struct CommitEntry {
    datetime: NaiveDateTime,
    insertions: u64,
    deletions: u64,
}

pub fn compute(repos: &[PathBuf], days: u32) -> DashboardData {
    let today = Utc::now().date_naive();
    let daily_window = days as i64;
    let total_repos = repos.len();

    eprintln!("Scanning {} {}...", total_repos, if total_repos == 1 { "repository" } else { "repositories" });

    let results: Vec<RepoResult> = repos
        .par_iter()
        .enumerate()
        .filter_map(|(i, repo_path)| {
            let name = repo_name(repo_path);
            eprintln!("  [{}/{}] {}", i + 1, total_repos, name);
            compute_repo_stats(repo_path)
        })
        .collect();

    let scanned = results.len();
    if scanned < total_repos {
        eprintln!("Note: {}/{} repos scanned successfully ({} had errors).", scanned, total_repos, total_repos - scanned);
    }

    let mut all_warnings: Vec<String> = Vec::new();
    let mut all_commits: Vec<CommitEntry> = Vec::new();
    let mut all_extensions: HashMap<String, u32> = HashMap::new();
    let mut repo_commit_counts: Vec<(String, u32)> = Vec::new();
    let mut author_counts: HashMap<String, u32> = HashMap::new();

    for r in results {
        repo_commit_counts.push((r.repo_name, r.commit_count));
        for (ext, count) in r.file_extensions {
            *all_extensions.entry(ext).or_default() += count;
        }
        for (author, count) in r.authors {
            *author_counts.entry(author).or_default() += count;
        }
        all_commits.extend(r.commits);
        all_warnings.extend(r.warnings);
    }

    // --- summary ---
    let total_commits = all_commits.len() as u32;
    let lines_added: u64 = all_commits.iter().map(|c| c.insertions).sum();
    let lines_deleted: u64 = all_commits.iter().map(|c| c.deletions).sum();

    let unique_days: HashSet<NaiveDate> = all_commits
        .iter()
        .map(|c| c.datetime.date())
        .collect();
    let active_days = unique_days.len() as u32;

    let earliest = all_commits
        .iter()
        .map(|c| c.datetime.date())
        .min()
        .unwrap_or(today);
    let since_days = today.signed_duration_since(earliest).num_days().max(1);

    // --- daily commits ---
    let mut daily_map: HashMap<NaiveDate, u32> = HashMap::new();
    for i in 0..daily_window {
        daily_map.insert(today - Duration::days(i), 0);
    }
    for c in &all_commits {
        let date = c.datetime.date();
        if let Some(cnt) = daily_map.get_mut(&date) {
            *cnt += 1;
        }
    }
    let mut daily_commits: Vec<(NaiveDate, u32)> = daily_map.into_iter().collect();
    daily_commits.sort_by_key(|(d, _)| *d);

    // --- heatmap ---
    let mut grid = [[0u32; 24]; 7];
    for c in &all_commits {
        let day = c.datetime.weekday().num_days_from_monday() as usize;
        let hour = c.datetime.hour() as usize;
        grid[day][hour] += 1;
    }
    let max_count = grid.iter().flat_map(|r| r.iter()).max().copied().unwrap_or(1);

    // --- languages ---
    let mut languages: Vec<(String, u32)> = all_extensions
        .into_iter()
        .map(|(ext, count)| (extension_to_language(&ext).to_string(), count))
        .fold(HashMap::new(), |mut acc, (lang, count)| {
            *acc.entry(lang).or_default() += count;
            acc
        })
        .into_iter()
        .collect();
    languages.sort_by_key(|b| std::cmp::Reverse(b.1));
    languages.truncate(8);

    // --- top repos ---
    repo_commit_counts.sort_by_key(|b| std::cmp::Reverse(b.1));
    repo_commit_counts.truncate(10);

    // --- authors ---
    let total_authors = author_counts.len() as u32;
    let mut authors: Vec<(String, u32)> = author_counts.into_iter().collect();
    authors.sort_by_key(|b| std::cmp::Reverse(b.1));
    authors.truncate(10);

    let warning_count = all_warnings.len() as u32;

    DashboardData {
        summary: Summary {
            repo_count: repos.len(),
            total_commits,
            lines_added,
            lines_deleted,
            active_days,
            since_days,
            total_authors,
            warning_count,
        },
        daily_commits,
        heatmap: HeatmapData { grid, max_count },
        languages,
        top_repos: repo_commit_counts,
        authors,
        warnings: all_warnings,
    }
}

fn compute_repo_stats(path: &Path) -> Option<RepoResult> {
    let repo = Repository::open(path)
        .inspect_err(|e| eprintln!("  Warning: failed to open {}: {}", path.display(), e))
        .ok()?;
    let head = repo.head()
        .inspect_err(|e| eprintln!("  Warning: no HEAD in {}: {}", path.display(), e))
        .ok()?;
    let head_commit = head.peel_to_commit()
        .inspect_err(|e| eprintln!("  Warning: failed to resolve HEAD in {}: {}", path.display(), e))
        .ok()?;

    let mut warnings: Vec<String> = Vec::new();

    // --- collect file extensions from HEAD tree ---
    let mut file_extensions: HashMap<String, u32> = HashMap::new();
    if let Ok(tree) = head_commit.tree() {
        tree.walk(git2::TreeWalkMode::PreOrder, |_root, entry| {
            if entry.kind() == Some(git2::ObjectType::Blob) {
                if let Some(name) = entry.name() {
                    if let Some(ext) = Path::new(name).extension() {
                        let e = ext.to_string_lossy().to_lowercase();
                        if !e.is_empty() && e.len() <= 10 {
                            *file_extensions.entry(e).or_default() += 1;
                        }
                    }
                }
            }
            git2::TreeWalkResult::Ok
        }).ok();
    }

    // --- revwalk ---
    let mut revwalk = match repo.revwalk() {
        Ok(r) => r,
        Err(e) => {
            warnings.push(format!("{}: revwalk failed: {}", repo_name(path), e));
            return Some(RepoResult {
                repo_name: repo_name(path),
                commit_count: 0,
                file_extensions,
                authors: HashMap::new(),
                commits: Vec::new(),
                warnings,
            });
        }
    };
    revwalk.push(head_commit.id()).ok()?;
    revwalk.set_sorting(git2::Sort::TIME).ok()?;

    let mut commits = Vec::new();
    let mut commit_count = 0u32;
    let mut authors: HashMap<String, u32> = HashMap::new();

    for oid in revwalk {
        if commits.len() >= MAX_COMMITS_PER_REPO {
            break;
        }
        let oid = match oid {
            Ok(o) => o,
            Err(_) => continue,
        };
        let commit = match repo.find_commit(oid) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let time = commit.time();
        let dt = match DateTime::from_timestamp(time.seconds(), 0) {
            Some(d) => d.naive_utc(),
            None => continue,
        };

        let author = commit.author();
        let author_name = author
            .name()
            .unwrap_or("unknown")
            .to_string();
        *authors.entry(author_name).or_default() += 1;

        let tree = match commit.tree() {
            Ok(t) => t,
            Err(_) => continue,
        };
        let parent_tree = commit.parent(0).ok().and_then(|p| p.tree().ok());

        let diff = match repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None) {
            Ok(d) => d,
            Err(_) => continue,
        };
        let stats = match diff.stats() {
            Ok(s) => s,
            Err(_) => continue,
        };

        commits.push(CommitEntry {
            datetime: dt,
            insertions: stats.insertions() as u64,
            deletions: stats.deletions() as u64,
        });
        commit_count += 1;
    }

    Some(RepoResult {
        repo_name: repo_name(path),
        commit_count,
        file_extensions,
        authors,
        commits,
        warnings,
    })
}

pub fn repo_name(path: &Path) -> String {
    let resolved = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    resolved
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string()
}

pub fn export_csv(data: &DashboardData) -> String {
    let mut out = String::from("type,label,value\n");

    for (date, count) in &data.daily_commits {
        out.push_str(&format!("daily,{},{}\n", date.format("%Y-%m-%d"), count));
    }

    for day in 0..7 {
        for hour in 0..24 {
            out.push_str(&format!(
                "heatmap,day{}_hour{:02},{}\n",
                day, hour, data.heatmap.grid[day][hour]
            ));
        }
    }

    for (lang, count) in &data.languages {
        out.push_str(&format!("language,{},{}\n", lang, count));
    }

    for (repo, count) in &data.top_repos {
        out.push_str(&format!("repo,{},{}\n", repo, count));
    }

    out
}

pub fn extension_to_language(ext: &str) -> &str {
    match ext {
        "rs" => "Rust",
        "py" | "pyw" => "Python",
        "js" | "mjs" | "cjs" => "JavaScript",
        "ts" => "TypeScript",
        "jsx" => "JSX",
        "tsx" => "TSX",
        "go" => "Go",
        "java" => "Java",
        "kt" | "kts" => "Kotlin",
        "c" => "C",
        "h" => "C Header",
        "cpp" | "cc" | "cxx" | "c++" => "C++",
        "hpp" | "hh" | "hxx" | "h++" => "C++ Header",
        "cs" => "C#",
        "rb" => "Ruby",
        "php" => "PHP",
        "swift" => "Swift",
        "scala" | "sc" => "Scala",
        "sh" | "bash" | "zsh" => "Shell",
        "fish" => "Fish",
        "json" => "JSON",
        "yaml" | "yml" => "YAML",
        "xml" | "xsl" | "xsd" => "XML",
        "html" | "htm" => "HTML",
        "css" => "CSS",
        "scss" | "sass" => "SCSS",
        "less" => "Less",
        "sql" => "SQL",
        "md" | "mdx" | "markdown" => "Markdown",
        "dockerfile" | "dockerignore" => "Docker",
        "nix" => "Nix",
        "lua" => "Lua",
        "r" => "R",
        "zig" => "Zig",
        "hs" => "Haskell",
        "elm" => "Elm",
        "clj" | "cljs" | "cljc" | "edn" => "Clojure",
        "ex" | "exs" | "heex" => "Elixir",
        "erl" | "hrl" => "Erlang",
        "dart" => "Dart",
        "vim" => "Vim",
        "tf" | "tfvars" => "Terraform",
        "proto" => "Protobuf",
        "cmake" | "cmake.in" => "CMake",
        "gradle" | "gradle.kts" => "Gradle",
        "lock" => "Lockfile",
        "toml" => "TOML",
        "cfg" | "conf" | "config" | "ini" => "Config",
        "svg" => "SVG",
        "png" | "jpg" | "jpeg" | "gif" | "ico" | "webp" => "Image",
        "vue" => "Vue",
        "svelte" => "Svelte",
        "astro" => "Astro",
        "ql" => "GraphQL",
        "prisma" => "Prisma",
        _ => "Other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_to_language() {
        assert_eq!(extension_to_language("rs"), "Rust");
        assert_eq!(extension_to_language("py"), "Python");
        assert_eq!(extension_to_language("js"), "JavaScript");
        assert_eq!(extension_to_language("ts"), "TypeScript");
        assert_eq!(extension_to_language("go"), "Go");
        assert_eq!(extension_to_language("java"), "Java");
        assert_eq!(extension_to_language("cpp"), "C++");
        assert_eq!(extension_to_language("rb"), "Ruby");
        assert_eq!(extension_to_language("md"), "Markdown");
        assert_eq!(extension_to_language("toml"), "TOML");
        assert_eq!(extension_to_language("unknown_ext"), "Other");
    }

    #[test]
    fn test_language_aliases() {
        assert_eq!(extension_to_language("pyw"), "Python");
        assert_eq!(extension_to_language("mjs"), "JavaScript");
        assert_eq!(extension_to_language("cc"), "C++");
        assert_eq!(extension_to_language("hpp"), "C++ Header");
        assert_eq!(extension_to_language("yml"), "YAML");
        assert_eq!(extension_to_language("mdx"), "Markdown");
    }

    #[test]
    fn test_export_csv_has_header() {
        let data = DashboardData {
            summary: Summary {
                repo_count: 0,
                total_commits: 0,
                lines_added: 0,
                lines_deleted: 0,
                active_days: 0,
                since_days: 0,
                total_authors: 0,
                warning_count: 0,
            },
            daily_commits: vec![],
            heatmap: HeatmapData { grid: [[0; 24]; 7], max_count: 0 },
            languages: vec![],
            top_repos: vec![],
            authors: vec![],
            warnings: vec![],
        };
        let csv = export_csv(&data);
        assert!(csv.starts_with("type,label,value\n"));
    }
}
