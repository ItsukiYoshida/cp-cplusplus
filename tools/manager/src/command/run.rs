use std::{io::{self, BufRead, BufReader, Write}, process::{Command, Stdio}, sync::mpsc, thread};
use anyhow::Result;

use crate::model::Output;

pub fn run() -> Result<()> {
    let mut child = Command::new("./main")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
    let child_stdout = child.stdout.take().expect("Failed to open stdout");
    let child_stderr = child.stderr.take().expect("Failed to open stderr");

    let (tx, rx) = mpsc::channel();

    let tx_stdout = tx.clone();
    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(child_stdout);
        for line in reader.lines() {
            if tx_stdout.send(Output::Stdout(line.unwrap())).is_err() {
                break;
            }
        }
    });

    let tx_stderr = tx.clone();
    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(child_stderr);
        for line in reader.lines() {
            if tx_stderr.send(Output::Stderr(line.unwrap())).is_err() {
                break;
            }
        }
    });

    let stdin_thread = thread::spawn(move || {
        println!("[MANAGER] Enter text to send to child's stdin (Ctrl+D to finish):");
        let mut buffer = String::new();
        while io::stdin().read_line(&mut buffer).unwrap_or(0) > 0 {
            if child_stdin.write_all(buffer.as_bytes()).is_err() {
                break;
            }
            buffer.clear();
        }
        println!("[MANAGER] stdin closed.");
    });

    drop(tx);

    for received in rx {
        match received {
            Output::Stdout(line) => println!("[STDOUT] {}", line),
            Output::Stderr(line) => eprintln!("[STDERR] {}", line),
        }
    }

    stdin_thread.join().expect("Stdin thread panicked");
    stdout_thread.join().expect("Stdout thread panicked");
    stderr_thread.join().expect("Stderr thread panicked");
    child.wait()?;

    println!("[MANAGER] Child process finished.");
    Ok(())
}
