use chrono::prelude::*;
use csv::Writer;
use json::{object, JsonValue};
use rand::{prelude::ThreadRng, Rng};
use std::io::stdin;

fn generate_fake_hours_start_end(number_of_hours_worked: u8) -> JsonValue {
    const START_WORKING_HOURS: u8 = 8;
    const END_WORKING_HOURS: u8 = 22;

    let start_hour = get_random_number(
        START_WORKING_HOURS,
        END_WORKING_HOURS - number_of_hours_worked,
    );

    let end_hour = get_random_number(number_of_hours_worked + start_hour, END_WORKING_HOURS);

    let start_date_time: DateTime<Local> = Local
        .datetime_from_str(&format!("2014-11-28 {}:00", start_hour), "%Y-%m-%d %k:%M")
        .expect("Failed to parse string in the correct format");

    let end_date_time: DateTime<Local> = Local
        .datetime_from_str(&format!("2014-11-28 {}:00", end_hour), "%Y-%m-%d %k:%M")
        .expect("Failed to parse string in the correct format");

    return object! {
        start: start_hour,
        end: end_hour,
        start_time: start_date_time.time().format("%l:%M %P").to_string(),
        end_time: end_date_time.time().format("%l:%M %P").to_string()
    };
}

fn main() -> () {
    let mut timesheet_hours: String = String::new();
    let mut timesheet_days: String = String::new();
    let mut timesheet_rate: String = String::new();

    request_input(
        "Please input total number of hours you want to create timesheet for:",
        &mut timesheet_hours,
    );

    request_input("Enter number of days you \"worked\":", &mut timesheet_days);
    request_input("Enter your hourly rate:", &mut timesheet_rate);

    let timesheet_hours: u16 = timesheet_hours.trim_end().parse().unwrap();
    let timesheet_days: u16 = timesheet_days.trim_end().parse().unwrap();
    let timesheet_rate: u16 = timesheet_rate.trim_end().parse().unwrap();
    println!("> Generating timesheet for {timesheet_hours} hours worked over {timesheet_days} days(${timesheet_rate}/hr)");

    let mut random_hours: Vec<JsonValue> = Vec::new();
    let mut sum_hours_worked: u16 = 0;

    let mut wtr = Writer::from_path("timesheet.csv").unwrap();
    wtr.write_record(&["Day", "Hours Worked", "Start Time", "End Time"])
        .unwrap();
    // Generate random range
    for day_index in 1..timesheet_days + 1 {
        let mut random_hour_worked: u16;
        'inner: loop {
            random_hour_worked = get_random_number(1, 8) as u16;
            if sum_hours_worked + random_hour_worked <= timesheet_hours {
                break 'inner;
            }
        }

        let start_end_time: JsonValue = generate_fake_hours_start_end(random_hour_worked as u8);

        let time_slot_obj = object! {
            "start_time" =>  format!("{}",start_end_time["start_time"]),
            "end_time" => format!("{}",start_end_time["end_time"]),
            "hour" => random_hour_worked,
            "day" => day_index
        };

        wtr.write_record(&[
            format!("{}", time_slot_obj["day"]),
            format!("{}", time_slot_obj["hour"]),
            format!("{}", time_slot_obj["start_time"]),
            format!("{}", time_slot_obj["end_time"]),
        ])
        .unwrap();

        sum_hours_worked += random_hour_worked;
        random_hours.push(time_slot_obj);
    }

    wtr.write_record(&["", &sum_hours_worked.to_string(), "", ""])
        .unwrap();
    println!("Total hours worked: {}", sum_hours_worked);

    wtr.flush().unwrap();
}

fn get_random_number(min: u8, max: u8) -> u8 {
    let mut rng: ThreadRng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(min..=max);
    return random_number;
}

fn request_input(message: &str, mut variable: &mut String) -> () {
    println!("{message}");
    stdin()
        .read_line(&mut variable)
        .expect("Failed to read line");
}
