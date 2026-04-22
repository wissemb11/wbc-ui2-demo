#!/usr/bin/env rust-script
// samples.rs — Rich variety of Rust println! outputs for demos & testing
// Run with: rustc samples.rs && ./samples   (or cargo script, rust-script, etc.)

use std::fmt::{Display, Debug};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    banner("RUST OUTPUT SAMPLES");
    println!("Perfect for testing highlighters, logs, docs, or converters\n");

    section(1, "Basic values");
    println!("Hello, Rustaceans!");
    println!("The answer is {}", 42);
    println!("π ≈ {:.10}", std::f64::consts::PI);
    println!("true = {}, false = {}, null = Option::None", true, false);

    section(2, "Formatted strings");
    let name = "Grace Hopper";
    let year = 1906;
    println!("{name} was born in {year}.");
    println!("Binary: {:b}, Hex: {:x}, OCT: {:o}", 42, 42, 42);
    println!("Padded: |{:>8}|{:<8}|", "right", "left");

    section(3, "Colors (ANSI escape codes)");
    print!("{}Success!{} ", "\x1b[92;1m", "\x1b[0m");
    print!("{}Warn!{} ", "\x1b[93;1m", "\x1b[0m");
    println!("{}Err!{}\n", "\x1b[91;1m", "\x1b[0m");

    section(4, "Vectors & pretty printing");
    let planets = vec!["Mercury", "Venus", "Earth", "Mars", "Jupiter"];
    println!("Planets: {planets:?}");
    println!("First three: {:#?}", &planets[..3]);

    section(5, "Structured data");
    #[derive(Debug)]
    struct Scientist {
        name: &'static str,
        born: u16,
        known_for: &'static str,
    }
    let scientists = vec![
        Scientist { name: "Ada Lovelace",    born: 1815, known_for: "First programmer" },
        Scientist { name: "Alan Turing",     born: 1912, known_for: "Turing machine" },
        Scientist { name: "Claude Shannon",  born: 1916, known_for: "Information theory" },
    ];
    for s in &scientists {
        println!("• {} ({}) – {}", s.name, s.born, s.known_for);
    }
    println!("\nDebug view of the vector:\n{:#?}", scientists);

    section(6, "JSON output");
    let json_data = serde_json::json!({
        "project": "Rust samples",
        "features": ["colored output", "tables", "progress bar"],
        "generated_at": chrono::Utc::now().to_rfc3339()
    });
    println!("Pretty JSON:\n{}", serde_json::to_string_pretty(&json_data).unwrap());

    section(7, "ASCII table");
    println!("+{:-<50}+", "");
    println!("| {:<15} | {:>10} | {:>15} |", "Language", "Year", "Paradigm");
    println!("|{:-<15}-|{:-<10}-|{:-<15}|", "", "", "");
    let rows = [
        ("Rust",      2015, "Multi-paradigm"),
        ("Python",    1991, "Multi-paradigm"),
        ("Haskell",   1990, "Pure functional"),
        ("Go",        2009, "Imperative"),
    ];
    for (lang, year, paradigm) in rows {
        println!("| {:<15} | {:>10} | {:>15} |", lang, year, paradigm);
    }
    println!("+{:-<50}+", "");

    section(8, "Progress bar simulation");
    print!("Compiling project ");
    for i in 0..=20 {
        let bar = "█".repeat(i as usize) + &"░".repeat(20 - i as usize);
        print!("\rCompiling... [{bar}] {}%", i * 5);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        sleep(Duration::from_millis(80));
    }
    println!("\nCompile successful!");

    section(9, "Timing example");
    let now = Instant::now();
    let fib = fibonacci(40);
    println!("fibonacci(40) = {fib} (took {:.2?})", now.elapsed());

    section(10, "Result & error handling simulation");
    let division = divide(10.0, 0.0);
    match division {
        Ok(result) => println!("10 / 0 = {result}"),
        Err(e) => println!("Division failed: {e}"),
    }

    println!("\nAll done in {:.2?}!", start.elapsed());
    println!("This file showcases 10 different output styles in Rust.");
}

// Helper functions
fn banner(text: &str) {
    println!("{}", "═".repeat(60));
    println!("{:^60}", text);
    println!("{}", "═".repeat(60));
}

fn section(num: usize, title: &str) {
    println!("\n{:>2}. {title}", num);
    println!("{}", "─".repeat(50));
}

fn fibonacci(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero!"))
    } else {
        Ok(a / b)
    }
}
