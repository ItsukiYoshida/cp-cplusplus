pub mod model;
use anyhow::Result;
use model::Config;
use walkdir::WalkDir;
use std::{env, fs, path::PathBuf, time::UNIX_EPOCH};

const TEMPLATE: &str = r#"
#include <gtest/gtest.h>
#include <cstdio>
#include <fstream>
#include <string>
#include <cctype>

void run_solution(const std::string& exe, const std::string& input, std::string& output) {{
    std::string cmd = exe + " < " + input;
    FILE* pipe = popen(cmd.c_str(), "r");
    if (!pipe) throw std::runtime_error("popen failed");
    char buffer[4096];
    output.clear();
    while (fgets(buffer, sizeof(buffer), pipe)) {{
        output += buffer;
    }}
    pclose(pipe);
    output.erase(
        std::find_if(
            output.rbegin(), 
            output.rend(), 
            [](unsigned char ch) {{return !std::isspace(ch);}}).base(), 
            output.end()
        );
}}

{tests}
"#;

fn escape_cpp_string(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            _ => escaped.push(c),
        }
    }
    escaped
}

fn is_dir_updated(path: &PathBuf) -> Result<bool> {
    let dir = path.as_path();
    let timestamp = dir.join(".timestamp");
    if !timestamp.exists() {
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&timestamp)?;
        return Ok(true);
    }
    let last_check_time = fs::metadata(&timestamp)
        .and_then(|meta| meta.modified())
        .unwrap_or(UNIX_EPOCH);

    let latest_time = WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path() != timestamp)
        .filter_map(|entry| entry.metadata().ok())
        .filter_map(|meta| meta.modified().ok())
        .max();
    
    let update = match latest_time {
        Some(latest) => latest > last_check_time,
        None => false
    };

    if update {
        fs::remove_file(&timestamp)?;
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&timestamp)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn run(config: Config) -> Result<()> {
    let problems = vec!["A", "B", "C", "D", "E", "F", "G", "H"]
        .into_iter()
        .map(String::from)
        .collect();

    let problems = config.problems.unwrap_or(problems);

    let root_dir = match config.project_root {
        Some(path) => path.canonicalize()?,
        None => env::current_dir()?,
    };

    println!("Root directory: {:?}", root_dir);

    for problem in problems.iter() {
        let tdir = root_dir.join("test").join(problem.to_uppercase());
        if !tdir.exists() {
            println!("Test directory not found, skipping: {:?}", tdir);
            continue;
        }

        if let Ok(updated) = is_dir_updated(&tdir) {
            if !updated {
                println!("No Update at Problem {}", problem);
                continue;
            }
        }

        let mut answers: Vec<PathBuf> = fs::read_dir(&tdir)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    filename.starts_with(&problem.to_uppercase()) && filename.ends_with(".ans")
                } else {
                    false
                }
            })
            .collect();
        answers.sort();

        if answers.is_empty() {
            println!("No answer files found for problem {}", problem);
            continue;
        }

        let mut test_cases = Vec::new();
        for (idx, ans_path) in answers.iter().enumerate() {
            let input = ans_path.with_extension("");

            if !input.exists() {
                println!("Warning: Input file not found for {:?}, skipping", ans_path);
                continue;
            }

            let expected = escape_cpp_string(fs::read_to_string(ans_path)?.trim_end());

            let exe_path = root_dir
                .join("build")
                .join(problem.to_lowercase())
                .join(problem.to_lowercase())
                .to_string_lossy()
                .replace("\\", "/");

            let input_path = input.to_string_lossy().replace("\\", "/");

            let test_case = format!(
                r#"
TEST({}, Case{}) {{
    std::string output;
    run_solution("{}", "{}", output);
    EXPECT_EQ(output, "{}");
}}
"#,
                problem.to_uppercase(),
                idx,
                exe_path,
                input_path,
                expected
            );
            test_cases.push(test_case);
        }

        if test_cases.is_empty() {
            continue;
        }

        let tests_dir = root_dir.join("tests");
        fs::create_dir_all(&tests_dir)?;

        let out_file_path =
            tests_dir.join(format!("generated_test_{}.cpp", problem.to_lowercase()));
        let final_content = TEMPLATE.replace("{tests}", &test_cases.join(""));

        fs::write(&out_file_path, final_content)?;
        println!("Successfully generated test file {:?}", out_file_path);
    }

    Ok(())
}
