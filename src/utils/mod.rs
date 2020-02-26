use clap::ArgMatches;

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

pub fn parse_julia_c(matches: &ArgMatches) -> Option<(f64, f64)> {

    let julia_real = match matches.value_of("real") {
        Some(real) => Some(real.parse::<f64>().unwrap()),
        None => None
    };
    let julia_imag = match matches.value_of("imag") {
        Some(imag) => Some(imag.parse::<f64>().unwrap()),
        None => None
    };

    if julia_imag.is_some() || julia_real.is_some() {
        Some((julia_real.unwrap_or_default(), julia_imag.unwrap_or_default()))
    } else {
        None
    }
}

pub fn numeric_validator(arg: String) -> Result<(), String> {

    if arg.parse::<f64>().is_ok() {
        Ok(())
    } else {
        Err(String::from("Must be a number!"))
    }
}