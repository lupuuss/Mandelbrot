use std::io;
use std::io::prelude::*;

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn format_time(miliseconds: u128) -> String {

    let in_miliseconds = miliseconds;

    let miliseconds = miliseconds % 1000;
    let in_seconds = in_miliseconds / 1000;

    let seconds = in_seconds % 60;
    let minutes = in_seconds / 60;

    let mut time_str = String::new();

    if minutes > 0 {
        time_str += &minutes.to_string();
        time_str += " min ";
    }

    if seconds > 0 {
        time_str += &seconds.to_string();
        time_str += " s ";
    }

    if miliseconds > 0 {
        time_str += &miliseconds.to_string();
        time_str += " ms"
    }

    return time_str;
}

pub fn bytes_string(bytes: u64) -> String {

    let mib = ((bytes as f64 / 1_048_576.0) * 1000.0).round() / 1000.0;
    let mb = ((bytes as f64 / 1_000_000.0) * 1000.0).round() / 1000.0; 
    return mb.to_string() + " MB (" + &mib.to_string() + " MiB)";
}

pub fn calc_ram_req<StoredType>(total_elements: u64) -> u64 {
    return 2 * calc_array_total_size::<StoredType>(total_elements);
}

pub fn calc_array_total_size<StoredType>(total_elements: u64) -> u64 {
    return std::mem::size_of::<StoredType>() as u64 * total_elements;
}