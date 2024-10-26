// src/macros.rs
//
// defines macros for use elsewhere

// pr! usage:
// pr!("hello") prints hello unless -q
// pr!("hello", 'v') prints hello if -v
// pr!("hello", 'q') prints hello despite -q
//
// note to self: avoid return in macros because it can cause unexpected behavior

#[macro_export]
macro_rules! pr {
    ($fmt:expr) => {{
        use $crate::flags::QUIET;

        if !*QUIET.lock().unwrap() {
            println!("{}", $fmt);
        }
    }};
    ($fmt:expr, $flag:expr) => {{
        use $crate::flags::{QUIET, VERBOSE};

        match $flag {
            'v' => {
                if *VERBOSE.lock().unwrap() {
                    println!("{}", $fmt);
                }
            }
            'q' => {
                println!("{}", $fmt);
            }
            _ => {
                if !*QUIET.lock().unwrap() {
                    println!("{}", $fmt);
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! erm {
    ($($arg:tt)*) => {
        eprintln!("\x1b[31;1m{}\x1b[0m", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {
        println!("\x1b[36;1m  {}\x1b[0m", format!($($arg)*))
    };
}
