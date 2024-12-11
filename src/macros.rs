// src/macros.rs
//
// defines macros for use elsewhere

#[macro_export]
macro_rules! pr {
    ($($arg:tt)*) => {{
        use $crate::flags::QUIET;
        if !*QUIET.lock().unwrap() {
            use $crate::config::CONFIG;
            println!("\x1b[{}{}\x1b[0m", CONFIG.colors.default, format!($($arg)*))
        }
    }};
}

#[macro_export]
macro_rules! vpr {
    ($($arg:tt)*) => {{
        use $crate::flags::VERBOSE;
        if *VERBOSE.lock().unwrap() {
            use $crate::config::CONFIG;
            let f = std::path::Path::new(file!())
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");
            println!("\x1b[{}[{}] {}\x1b[0m", CONFIG.colors.verbose, f, format!($($arg)*))
        }
    }};
}

#[macro_export]
macro_rules! erm {
    ($($arg:tt)*) => {{
        use $crate::config::CONFIG;
        eprintln!("\x1b[{}{}\x1b[0m", CONFIG.colors.danger, format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {{
        use $crate::config::CONFIG;
        println!("\x1b[{}{}\x1b[0m", CONFIG.colors.message, format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => {{
        use $crate::config::CONFIG;
        panic!("\x1b{}{}\x1b[0m", CONFIG.colors.danger, format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! yn {
    ($question:expr, $default:expr) => {{
        use std::io::{self, Write};
        use $crate::config::CONFIG;
        let mut answer = $default;
        loop {
            let default_text = match $default {
                true => "Y/n",
                false => "y/N",
            };

            print!("\x1b[{}{} ({}): \x1b[0m", CONFIG.colors.prompt, $question, default_text);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    answer = true;
                    break;
                }
                "n" | "no" => {
                    answer = false;
                    break;
                }
                "" => break,
                _ => {
                    erm!("Invalid input");
                    continue;
                }
            }
        }

        answer
    }};
}
