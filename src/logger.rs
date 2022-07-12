macro_rules! log_fatal {
    ($val:expr) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[30;41m[FATAL]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val),
        );
    };
    ($val:expr, $($args:tt),*) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[30;41m[FATAL]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val,$($args),*),
        );
    };
}

macro_rules! log_error {
    ($val:expr) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[1;91m[ERROR]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val),
        );
    };
    ($val:expr, $($args:tt),*) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[1;91m[ERROR]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val,$($args),*),
        );
    };
}

macro_rules! log_warn {
    ($val:expr) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[1;93m[WARNING]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val),
        );
    };
    ($val:expr, $($args:tt),*) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[1;93m[WARNING]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val,$($args),*),
        );
    };
}

macro_rules! log_info {
    ($val:expr) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[1;94m[INFO]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val),
        );
    };
    ($val:expr, $($args:tt),*) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[1;94m[INFO]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val,$($args),*),
        );
    };
}

macro_rules! log_debug {
    ($val:expr) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[1;92m[DEBUG]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val),
        );
    };
    ($val:expr, $($args:tt),*) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[1;92m[DEBUG]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val,$($args),*),
        );
    };
}

macro_rules! log_trace {
    ($val:expr) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[1;37m[TRACE]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val),
        );
    };
    ($val:expr, $($args:tt),*) => {
        eprintln!("\x1b[1;92m{}\x1b[0m \x1b[1;37m[TRACE]: {} \x1b[0m",
            format!("[{}:{}]", file!(), line!()),
            format!($val,$($args),*),
        );
    };
}
