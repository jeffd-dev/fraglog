use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;

fn main() {
    // Check parameters
    let args: Vec<String> = env::args().collect();
    if &args[1] == "help"{
        display_help();
        return;
    }

    if env::args().count() < 3 {
        println!("ERROR - Missing parameters");
        display_help();
        return
    }

    let config_log_file_path = &args[1];
    let config_start_date: &str = &args[2];
    if config_start_date.chars().count() < 8 {
        panic!("<period_start> should contains at least 8 characters");
    }
    let config_end_date: &str = &args[3];
    if config_end_date.chars().count() < 8 {
        panic!("<period_end> should contains at least 8 characters");
    }

    let mut verbose_mode = false;
    if env::args().count() > 4 && &args[1] == "verbose" {
        verbose_mode = true;
    }

    if verbose_mode {
        println!("Fraglog version 0.1b");
        println!("Reading {config_log_file_path}...");
    }

    // Determines whether it is a datetime or a time (only)
    let search_start = detect_between_date_datetime_or_time(&config_start_date).unwrap();
    let search_end = detect_between_date_datetime_or_time(&config_end_date).unwrap();
    let search_mode : ExpectedFormat;

    if search_start.empty_date && search_end.empty_date {
        search_mode = ExpectedFormat::TimeOnly;
        if verbose_mode {
            println!("Parameters contains only time period");
        }
    } else if search_start.empty_date == false && search_end.empty_date == false {
        search_mode = ExpectedFormat::DateAndTime;
        if verbose_mode {
            println!("Parameters contains datetime period");
        }
    } else {
        panic!("Invalid parameters, <period_start> and <period_end> should have the same format");
    }

    // Parse file and filter lines
    match search_mode {
        ExpectedFormat::DateAndTime => parse_datetime_logfile(&config_log_file_path, &search_start, &search_end, verbose_mode),
        ExpectedFormat::TimeOnly => parse_time_logfile(&config_log_file_path, &search_start.get_time(), &search_end.get_time(), verbose_mode),
    }

    if verbose_mode {
        println!("End");
    }
}

fn parse_datetime_logfile(config_log_file_path: &str, search_start: &Datetime, search_end: &Datetime, verbose_mode: bool) {
    let mut is_first_line: bool = true;
    let mut search_period_found = false;

    if let Ok(lines) = read_lines(&config_log_file_path) {
        for line in lines.map_while(Result::ok) {
            let line_datetime = parse_datetime(&line).unwrap();
            
            if is_first_line {
                if line_datetime.compare_with(&search_end) > 0 {
                    panic!("ERROR, the file start with {},  after the search period", line_datetime);
                }
                if line_datetime.compare_with(&search_start) > 0 && verbose_mode {
                    println!("Warning, the file start during the search period, missing some lines at the start");
                }
                is_first_line = false;
            }

            if search_period_found {
                if line_datetime.compare_with(&search_end) > 0{
                    break;
                } else {
                    println!("{}", line);
                }
            } else {
                if line_datetime.compare_with(&search_start) > 0{
                    search_period_found = true;
                    println!("{}", line);
                }
            }
        }
    }
}

fn parse_time_logfile(config_log_file_path: &str, search_start: &Time, search_end: &Time, verbose_mode: bool) {
    let mut is_first_line: bool = true;
    let mut search_period_found = false;

    if let Ok(lines) = read_lines(&config_log_file_path) {
        for line in lines.map_while(Result::ok) {
            let line_time = parse_time(&line).unwrap();
            
            if is_first_line {
                if line_time.compare_with(&search_end) > 0 {
                    panic!("ERROR, the file start with {},  after the search period", line_time);
                }
                if line_time.compare_with(&search_start) > 0 && verbose_mode {
                    println!("Warning, the file start during the search period, missing some lines at the start");
                }
                is_first_line = false;
            }

            if search_period_found {
                if line_time.compare_with(&search_end) > 0{
                    break;
                } else {
                    println!("{}", line);
                }
            } else {
                if line_time.compare_with(&search_start) > 0{
                    search_period_found = true;
                    println!("{}", line);
                }
            }
        }
    }
}

fn display_help(){
    println!("Usage: fraglog <log_filepath> <period_start> <period_end> ['verbose']");
    println!("Supported period format: YYYY-MM-DD HH:MM:SS");
    println!("Example: fraglog myfile.log '2025-04-10 10:00:00' '2025-04-11 10:00:00' 'verbose'");
}

struct Time{
    hour: u8,
    minut: u8,
    second: u8,
}

impl Time {
    fn compare_with(&self, another: &Time) -> i8 {
        if another.hour > self.hour {
            return -1;
        } else if another.hour < self.hour {
            return 1;
        } else {
            if another.minut > self.minut {
                return -1;
            } else if another.minut < self.minut {
                return 1;
            } else {
                if another.second > self.second {
                    return -1;
                } else if another.second < self.second {
                    return 1;
                } else {
                    return 0;
                }
            }
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}:{}:{}", &self.hour, &self.minut, &self.second)
    }
}

struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    fn compare_with(&self, another: &Date) -> i8 {
        if another.year > self.year {
            return -1;
        } else if another.year < self.year {
            return 1;
        } else {
            if another.month > self.month {
                return -1;
            } else if another.month < self.month {
                return 1;
            } else {
                if another.day > self.day {
                    return -1;
                } else if another.day < self.day {
                    return 1;
                } else {
                    return 0;
                }
            }
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}-{}-{}", &self.year, &self.month, &self.day)
    }
}


struct Datetime {
    date: Date,
    time: Time,
    empty_date: bool,
    empty_time: bool,
}

impl Datetime {
    fn get_time(&self) -> &Time {
        return &self.time;
    }

    fn compare_with(&self, another: &Datetime) -> i8 {
        let date_comparison = self.date.compare_with(&another.date);
        match date_comparison {
            0 => return self.time.compare_with(&another.time),
            _ => return date_comparison,
        }
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}T{}", &self.date, &self.time)
    }
}

enum ExpectedFormat {
    TimeOnly,
    DateAndTime,
}

fn detect_between_date_datetime_or_time(input_value: &str) -> Option<Datetime> {
    let input_size = input_value.chars().count();
    let time_value = Time {hour: 0, minut: 0, second: 0};
    let date_value = Date {year:0, month:0, day: 0};
    let mut result = Datetime {date: date_value, time: time_value, empty_date: true, empty_time: true};

    if input_size == 8 {
        result.time = parse_time(&input_value).unwrap();
        result.empty_time = false;
    } else if input_size == 10 {
        result.date = parse_date(&input_value).unwrap();
        result.empty_date = false;
    } else if input_size == 19 {
        result = parse_datetime(&input_value).unwrap();
        result.empty_date = false;
        result.empty_time = false;
    } else {
        panic!("Invalid size for date or time");
    }

    return Some(result);
}

fn parse_time(raw_line: &str) -> Result<Time, &str> {
    if raw_line.chars().count() < 8{
        return Err(&"Invalid time format, length should contain at least 8 chars");
    }
    let hour = &raw_line[0..2];
    let minut = &raw_line[3..5];
    let second = &raw_line[6..8];
    let time_found = Time {
        hour:hour.parse().expect("Invalid int value for hour"),
        minut:minut.parse().expect("Invalid int value for minut"),
        second:second.parse().expect("Invalid int value for second"),
    };
    return Ok(time_found);
}

fn parse_date(raw_line: &str) -> Result<Date, &str> {
    let year = &raw_line[0..4];
    let month = &raw_line[5..7];
    let day = &raw_line[8..10];
    let date_found = Date {
        year:year.parse().expect("Invalid int value for year"),
        month:month.parse().expect("Invalid int value for month"),
        day:day.parse().expect("Invalid int value for day"),
    };
    return Ok(date_found);
}


fn parse_datetime(raw_line: &str) -> Result<Datetime, &str> {
    if raw_line.chars().count() < 19{
        return Err(&"Invalid datetime format, length should contain at least 19 chars");
    }
    let date_found = parse_date(&raw_line[0..10]).expect("Invalid Date for this line");
    let time_found = parse_time(&raw_line[11..19]).expect("Invalid Time for this line");
    return Ok(Datetime{
        date: date_found,
        time: time_found,
        empty_date: false,
        empty_time: false,
    });
    
}

/* From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    // Returns an Iterator to the Reader of the lines of the file.
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_datetime(year: u16, month: u8, day: u8, hour: u8, minut: u8, second: u8) -> Datetime {
        let date = Date {year: year, month: month, day: day};
        let time = Time {hour: hour, minut: minut, second: second};
        return Datetime{date: date, time:time, empty_date: false, empty_time: false};
    }

    #[test]
    fn test_parse_datetime_valid_input() {
        let valid_date = "2025-03-15Z06:12:30";
        let result = parse_datetime(&valid_date).unwrap();
        assert_eq!(result.date.year, 2025);
        assert_eq!(result.date.month, 3);
        assert_eq!(result.date.day, 15);
        assert_eq!(result.time.hour, 6);
        assert_eq!(result.time.minut, 12);
        assert_eq!(result.time.second, 30);
    }

    #[test]
    fn test_date_compare_with(){
        let reference_date = Date { year:2025, month:5, day:10 };
        assert_eq!(reference_date.compare_with(&reference_date), 0);

        let previous_year = Date { year:2024, month:7, day:10 };
        assert_eq!(reference_date.compare_with(&previous_year), 1);

        let previous_month = Date { year:2025, month:4, day:15 };
        assert_eq!(reference_date.compare_with(&previous_month), 1);

        let previous_day = Date { year:2025, month:5, day:9 };
        assert_eq!(reference_date.compare_with(&previous_day), 1);

        let next_year = Date { year:2026, month:4, day:10 };
        assert_eq!(reference_date.compare_with(&next_year), -1);

        let next_month = Date { year:2025, month:7, day:8 };
        assert_eq!(reference_date.compare_with(&next_month), -1);

        let next_day = Date { year:2025, month:5, day:11 };
        assert_eq!(reference_date.compare_with(&next_day), -1);
    }

    #[test]
    fn test_time_compare_with(){
        let reference_time = Time { hour:12, minut:15, second:30 };
        assert_eq!(reference_time.compare_with(&reference_time), 0);

        let previous_hour = Time { hour:11, minut:16, second:30 };
        assert_eq!(reference_time.compare_with(&previous_hour), 1);

        let previous_minut = Time { hour:12, minut:14, second:42 };
        assert_eq!(reference_time.compare_with(&previous_minut), 1);

        let previous_second = Time { hour:12, minut:15, second:29 };
        assert_eq!(reference_time.compare_with(&previous_second), 1);

        let next_hour = Time { hour:13, minut:05, second:30 };
        assert_eq!(reference_time.compare_with(&next_hour), -1);

        let next_minut = Time { hour:12, minut:16, second:15 };
        assert_eq!(reference_time.compare_with(&next_minut), -1);

        let next_second = Time { hour:12, minut:15, second:42 };
        assert_eq!(reference_time.compare_with(&next_second), -1);
    }

    #[test]
    fn test_datetime_compare_with(){
        let reference_date = build_datetime(2025, 5, 10, 5, 10, 15);
        assert_eq!(reference_date.compare_with(&reference_date), 0);

        let previous_date = build_datetime(2025, 5, 9, 5, 10, 15);
        assert_eq!(reference_date.compare_with(&previous_date), 1);

        let previous_time = build_datetime(2025, 5, 10, 5, 10, 3);
        assert_eq!(reference_date.compare_with(&previous_time), 1);

        let after_date = build_datetime(2025, 5, 21, 5, 10, 15);
        assert_eq!(reference_date.compare_with(&after_date), -1);

        let after_time = build_datetime(2025, 5, 10, 5, 10, 30);
        assert_eq!(reference_date.compare_with(&after_time), -1);
    }


}
