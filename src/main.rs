use chrono::{DateTime, NaiveDateTime, Utc};

fn help() {
    println!(
        "usage:
sfutils <Discord snowflake>
    Get information about a Discord snowflake."
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => {
            let sf = match args[1].parse::<i64>() {
                Ok(i) => i,
                Err(_) => {
                    println!("Invalid snowflake.");
                    std::process::exit(1);
                },
              };
            let msg_date: DateTime<Utc> = DateTime::from_utc(
                NaiveDateTime::from_timestamp(((sf >> 22) + 1420070400000) / 1000, 0),
                Utc,
            );
            println!(
                "\x1b[0;34mSnowflake:\x1b[0m           \x1b[0;32m{}\x1b[0m",
                sf
            );
            println!(
                "\x1b[0;34mDate:\x1b[0m                \x1b[0;32m{}\x1b[0m",
                msg_date.format("%Y-%m-%d %H:%M:%S %Z")
            );
            println!(
                "\x1b[0;34mEpoch:\x1b[0m               \x1b[0;32m{}\x1b[0m",
                ((sf >> 22) + 1420070400000) / 1000
            );
            println!(
                "\x1b[0;34mEpoch (ms):\x1b[0m          \x1b[0;32m{}\x1b[0m",
                (sf >> 22) + 1420070400000
            );
            println!(
                "\x1b[0;34mInternal Worker ID:\x1b[0m  \x1b[0;32m{}\x1b[0m",
                (sf & 0x3E0000) >> 17
            );
            println!(
                "\x1b[0;34mInternal Process ID:\x1b[0m \x1b[0;32m{}\x1b[0m",
                (sf & 0x1F000) >> 12
            );
            println!(
                "\x1b[0;34mIncrement:\x1b[0m           \x1b[0;32m{}\x1b[0m",
                sf & 0xFFF
            );
        }
        _ => {
            help();
        }
    }
}
