use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input: PathBuf,

    #[arg(long)]
    output: PathBuf,

    #[arg(long)]
    include: Vec<PathBuf>,

    #[arg(long)]
    project_root: PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut visited = HashSet::new();
    let content = expand_file(&args.input, &args.include, &mut visited, &args.project_root)
        .unwrap_or_else(|e| panic!("Expand failed: {}", e));
    let mut f = fs::File::create(&args.output).expect("Failed to write output");
    f.write_all(content.as_bytes()).unwrap();
}

fn expand_file(
    path: &Path,
    include_dirs: &[PathBuf],
    visited: &mut HashSet<PathBuf>,
    project_root: &Path,
) -> Result<String, String> {
    let abs_path = fs::canonicalize(path).map_err(|e| format!("Canonicalize failed: {}", e))?;
    if visited.contains(&abs_path) {
        return Ok(format!(
            "// Skipped (already included): {}\n",
            path.display()
        ));
    }
    visited.insert(abs_path.clone());

    let content = fs::read_to_string(&abs_path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    let parent_dir = abs_path.parent().unwrap();
    let mut result = String::new();
    let is_library = {
        let relative_path = abs_path.strip_prefix(project_root).unwrap_or(&abs_path);
        relative_path.starts_with("external/") || relative_path.starts_with("library/")
    };
    for line in content.lines() {
        let trimmed = line.trim();
        if is_library {
            if trimmed.starts_with("//") {
                continue;
            }
            if trimmed.starts_with("#include <") && trimmed.ends_with('>') {
                continue;
            }
        }

        if let Some(include_target) = parse_include(trimmed) {
            if is_system_include(&include_target) {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            let mut included_path = parent_dir.join(&include_target);
            if !included_path.exists() {
                included_path = include_dirs
                    .iter()
                    .map(|dir| dir.join(&include_target))
                    .find(|p| p.exists())
                    .ok_or_else(|| format!("Include not found: {}", include_target))?;
            }

            result.push_str(
                &expand_file(&included_path, include_dirs, visited, project_root)
                    .map_err(|e| format!("In {}: {}", path.display(), e))?,
            );
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    Ok(result)
}

fn parse_include(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("#include \"") && trimmed.ends_with('"') {
        Some(trimmed[10..trimmed.len() - 1].to_string())
    } else {
        None
    }
}

fn is_system_include(s: &str) -> bool {
    s.starts_with('<') && s.ends_with('>')
}
