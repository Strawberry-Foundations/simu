#[macro_export]
macro_rules! error {
    ($string:expr) => {
        eprintln!("{}", format!("\x1b[1m\x1b[41m  ERROR  \x1b[49m\x1b[31m    {}\x1b[0m", $string))
    };
}

#[macro_export]
macro_rules! exception {
    ($string:expr) => {
        eprintln!("{}", format!("\x1b[1m\x1b[41m  ERROR  \x1b[49m\x1b[31m    {}\x1b[0m", $string));
        std::process::exit(1)
    };
}

#[macro_export]
macro_rules! warning {
    ($string:expr) => {
        println!("{}", format!("\x1b[1m\x1b[43m  WARNING  \x1b[49m\x1b[33m    {}\x1b[0m", $string))
    };
}

#[macro_export]
macro_rules! success {
    ($string:expr) => {
        println!("{}", format!("\x1b[1m\x1b[42m  SUCCESS  \x1b[49m\x1b[32m  {}\x1b[0m", $string))
    };
}

#[macro_export]
macro_rules! info {
    ($string:expr) => {
        println!("{}", format!("\x1b[1m\x1b[46m  INFO  \x1b[49m\x1b[36m     {}\x1b[0m", $string))
    };
}

#[macro_export]
macro_rules! command {
    ($string:expr) => {
        println!("{}", format!("\x1b[1m\x1b[46m  COMMAND  \x1b[49m\x1b[36m  {}\x1b[0m", $string))
    };
}