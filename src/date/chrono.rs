use chrono::{DateTime, Local, Datelike, Timelike, Weekday};

pub fn current() -> String {
    let now: DateTime<Local> = Local::now();

    let day_of_week = match now.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };

    let day = now.day();
    let month = match now.month() {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    };

    let year = now.year();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();

    format!(
        "{}, {} {} {} {:02}:{:02}:{:02}",
        day_of_week, day, month, year, hour, minute, second
    )
}