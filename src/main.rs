use std::fs;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom, Result};
use chrono::{Utc, Local, TimeZone, DateTime};

fn milliseconds_to_local_time(millis: u64) -> DateTime<Local> {
    let seconds = (millis / 1000) as i64;
    let nanos = ((millis % 1000) * 1_000_000) as u32;
    return Local.timestamp(seconds, nanos);
}

fn write_eval_file(value: u64, file: &mut File) -> Result<()> {
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&(!value).to_be_bytes())?;
    file.set_len(8)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Argument Invalid. Config file should specified");
        return;
    }
    let config_path = &args[1];
    let contents = fs::read_to_string(config_path).unwrap_or_else(|error| {
        panic!("File Invalid {}: {}", config_path, error)
    });

    let mut success = 0;
    for line in contents.lines()  {
        if line.is_empty() {
            continue;
        }
        let eval_file_path = Path::new(line);
        if !eval_file_path.is_file() {
            println!("File not exists: {} ", line);
            continue;
        }
        let mut eval_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(eval_file_path).unwrap();
        let mut buf = [0; 8];
        eval_file.read_exact(&mut buf).unwrap();
        let origin = ! u64::from_be_bytes(buf);
        let expired = milliseconds_to_local_time(origin + 3600*24*30*1000u64);
        let new_value = Utc::now().timestamp_millis() as u64;
        let new_expired = milliseconds_to_local_time(new_value + 3600*24*30*1000u64);
        write_eval_file(new_value, &mut eval_file).expect("Write file Failed");

        println!("File [{}]: expired_at={} => {}", line, expired.to_string(), new_expired.to_string());
        success += 1;
    };
    println!("Finish: {}", success);
}
