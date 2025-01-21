macro_rules! writeln_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr) => {
        match $value {
            value if !value.to_string().is_empty() => {
                writeln!($handle, "\x1B[0;31m{}\x1B[0m  {}", $entry, value).unwrap();
            }
            _ => {}
        }
    };

        // ($handle:expr, $entry:expr, $value1:expr, $value2:expr) => {
        //     match ($value1, $value2) {
        //         (value1, value2) if !value1.to_string().is_empty() && !value2.to_string().is_empty() => {
        //             writeln!($handle, "\x1B[0;31m{}\x1B[0m  {} {}", $entry, value1, value2).unwrap();
        //         }
        //         (value1, _) if !value1.to_string().is_empty() => {
        //             writeln!($handle, "\x1B[0;31m{}\x1B[0m  {}", $entry, value1).unwrap();
        //         }
        //         (_, value2) if !value2.to_string().is_empty() => {
        //             writeln!($handle, "\x1B[0;31m{}\x1B[0m  {}", $entry, value2).unwrap();
        //         }
        //         _ => {}
        //     }
        // };

    ($handle:expr, $entry:expr, $value1:expr, $value2:expr, $value3:expr) => { // This code is specifically used for username@hostname.
        match ($value1, $value2, $value3) {
            (value1, value2, value3) if !value1.to_string().is_empty() && !value2.to_string().is_empty() && !value3.to_string().is_empty() => {
                writeln!($handle, "\x1B[0;31m{}\x1B[0m  {}{}{}", $entry, value1, value2, value3).unwrap();
            }
            (value1, _, _) if !value1.to_string().is_empty() => {
                writeln!($handle, "\x1B[0;31m{}\x1B[0m  {}", $entry, value1).unwrap();
            }
            (_, value2, _) if !value2.to_string().is_empty() => {
                writeln!($handle, "\x1B[0;31m{}\x1B[0m  {}", $entry, value2).unwrap();
            }
            (_, _, value3) if !value3.to_string().is_empty() => {
                writeln!($handle, "\x1B[0;31m{}\x1B[0m  {}", $entry, value3).unwrap();
            }
            _ => {}
        }
    };
}

macro_rules! get_env_var {
    ($var:expr) => {
        std::env::var($var).unwrap_or_else(|_| String::new())
    };
}
