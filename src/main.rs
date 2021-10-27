use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Deserialization error: {0}")]
    JSonDeserialization(#[from] serde_json::Error),
    #[error("No filename given")]
    NoArg(),
}

#[derive(Copy, Clone, Debug, Default)]
struct LogStats {
    count: usize,
    bytes: usize,
}

#[derive(Debug)]
struct LogsStats {
    log_type_stats: HashMap<String, LogStats>,
}

impl LogsStats {
    pub fn new() -> LogsStats {
        LogsStats {
            log_type_stats: HashMap::new(),
        }
    }
    ///Inserts or updates already existing log stat.
    pub fn update_log_stats(&mut self, log_type: &str, bytes_count: usize) {
        let stats_entry = self
            .log_type_stats
            .entry(log_type.to_owned())
            .or_insert_with(LogStats::default);
        stats_entry.count += 1;
        stats_entry.bytes += bytes_count;
    }
}

#[derive(Deserialize, Debug)]
struct Log<'a> {
    #[serde(borrow)]
    #[serde(rename = "type")]
    log_type: &'a str,
}

fn pretty_print_stats<W: std::io::Write>(
    writer: W,
    log_stats: &LogsStats,
) -> Result<(), std::io::Error> {
    let mut tw = tabwriter::TabWriter::new(writer);
    writeln!(&mut tw, "Log Type\tCount\tTotal Bytes")?;
    for (key, value) in log_stats.log_type_stats.iter() {
        writeln!(&mut tw, "{}\t{}\t{}", key, value.count, value.bytes)?;
    }
    tw.flush()?;
    Ok(())
}

fn main() -> Result<(), AppError> {
    let filename = std::env::args().nth(1).ok_or_else(AppError::NoArg)?;
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    let mut log_stats = LogsStats::new();
    for line in lines {
        let message = line?;
        let message_len_bytes = message.len();
        match serde_json::from_str::<Log>(&message) {
            Ok(log) => {
                log_stats.update_log_stats(log.log_type, message_len_bytes);
            }
            Err(e) => {
                eprintln!("Error while processing log: {} Reason: {}", message, e);
            }
        };
    }
    pretty_print_stats(std::io::stdout(), &log_stats)?;
    Ok(())
}
