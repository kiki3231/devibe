use std::path::{Path, PathBuf};

/// Recursively discover git repositories under `root`, max depth 3.
pub fn discover_repos(root: &Path) -> Vec<PathBuf> {
    let mut repos = Vec::new();
    discover_recursive(root, 0, &mut repos);
    repos
}

/// Discover repos from multiple root paths (for refresh).
pub fn discover_all(roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut all = Vec::new();
    for root in roots {
        all.extend(discover_repos(root));
    }
    all.sort();
    all.dedup();
    all
}

fn discover_recursive(dir: &Path, depth: u32, repos: &mut Vec<PathBuf>) {
    if depth > 3 || !dir.is_dir() {
        return;
    }
    if dir.join(".git").exists() {
        repos.push(dir.to_path_buf());
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && !skip_dir(&path) {
            discover_recursive(&path, depth + 1, repos);
        }
    }
}

fn skip_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.starts_with('.') || n == "node_modules" || n == "target" || n == "vendor")
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_hidden_dirs() {
        assert!(skip_dir(Path::new("/tmp/.git")));
        assert!(skip_dir(Path::new("/tmp/.config")));
    }

    #[test]
    fn test_skip_known_dirs() {
        assert!(skip_dir(Path::new("/tmp/node_modules")));
        assert!(skip_dir(Path::new("/tmp/target")));
        assert!(skip_dir(Path::new("/tmp/vendor")));
    }

    #[test]
    fn test_skip_invalid_path() {
        assert!(skip_dir(Path::new("")));
    }

    #[test]
    fn test_not_skip_normal_dirs() {
        assert!(!skip_dir(Path::new("/tmp/myproject")));
        assert!(!skip_dir(Path::new("/tmp/src")));
    }

    #[test]
    fn test_discover_repos_empty_dir() {
        let tmp = std::env::temp_dir().join("devibe_test_empty");
        let _ = std::fs::create_dir_all(&tmp);
        let repos = discover_repos(&tmp);
        assert!(repos.is_empty());
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_discover_repos_finds_git() {
        let tmp = std::env::temp_dir().join("devibe_test_repo");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".git")).unwrap();
        let repos = discover_repos(&tmp);
        assert_eq!(repos.len(), 1);
        assert!(repos[0].ends_with("devibe_test_repo"));
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
