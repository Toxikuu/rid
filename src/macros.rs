// src/macros.rs
//
// defines macros for use elsewhere

// write pr in a sec for messages suppressed by -q

#[macro_export]
macro_rules! pr {
    ($($arg:tt)*) => {{
        use $crate::flags::QUIET;
        if !*QUIET.lock().unwrap() {
            println!("\x1b[30;3m{}\x1b[0m", format!($($arg)*))
        }
    }};
}

#[macro_export]
macro_rules! vpr {
    ($($arg:tt)*) => {{
        use $crate::flags::VERBOSE;
        if *VERBOSE.lock().unwrap() {
            let f = std::path::Path::new(file!())
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");
            println!("\x1b[34;1m[{}] {}\x1b[0m", f, format!($($arg)*))
        }
    }};
}

#[macro_export]
macro_rules! erm {
    ($($arg:tt)*) => {
        eprintln!("\x1b[31;1m  {}\x1b[0m", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {
        println!("\x1b[36;1m  {}\x1b[0m", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => {
        panic!("\x1b[31;1m  {}\x1b[0m", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! yn {
    ($question:expr, $default:expr) => {{
        use std::io::{self, Write};
        let mut answer = $default;
        loop {
            let default_text = match $default {
                true => "Y/n",
                false => "y/N",
            };

            print!("\x1b[35;1m  {} ({}): \x1b[0m", $question, default_text);
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
