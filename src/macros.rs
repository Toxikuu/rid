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
