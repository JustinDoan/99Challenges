#[derive(Debug)]
struct CustomError(String);
use chrono::TimeZone;
use std::io;
use std::io::Write;
extern crate chrono;

fn main() -> Result<(), CustomError> {
    let mut year = String::new();
    let mut month = String::new();
    let mut day = String::new();
    print!("Enter your year of birth: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut year)
        .map_err(|err| CustomError(format!("Unable to read line: {}", err)))?;

    print!("Enter your month of birth: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut month)
        .map_err(|err| CustomError(format!("Unable to read line: {}", err)))?;

    print!("Enter your day of birth: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut day)
        .map_err(|err| CustomError(format!("Unable to read line: {}", err)))?;

    let parsed_year: i32 = year.trim().parse::<i32>().map_err(|err| {
        CustomError(format!(
            "Error reading your input for year -> `{}`: {}",
            year, err
        ))
    })?;
    let parsed_month: u32 = month.trim().parse().map_err(|err| {
        CustomError(format!(
            "Error reading your input for month -> `{}`: {}",
            month, err
        ))
    })?;
    let parsed_day: u32 = day.trim().parse().map_err(|err| {
        CustomError(format!(
            "Error reading your input for day -> `{}`: {}",
            day, err
        ))
    })?;

    let dt = chrono::Local
        .ymd(parsed_year, parsed_month, parsed_day)
        .and_hms(0, 0, 0);

    let time_passed = chrono::Utc::now().signed_duration_since(dt);
    println!("Time passed in seconds: {}", time_passed.num_seconds());
    Ok(())
}
