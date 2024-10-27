// src/macros.rs
//
// defines macros for use elsewhere

// write pr in a sec for messages suppressed by -q

#[macro_export]
macro_rules! pr {
    ($($arg:tt)*) => {{
        use $crate::flags::QUIET;
        if *QUIET.lock().unwrap() {
            println!("\x1b[30;3m{}\x1b[0m", format!($($arg)*))
        }
    }};
}

#[macro_export]
macro_rules! vpr {
    ($($arg:tt)*) => {{
        use $crate::flags::VERBOSE;
        if *VERBOSE.lock().unwrap() {
            println!("\x1b[34;1m{}\x1b[0m", format!($($arg)*))
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
