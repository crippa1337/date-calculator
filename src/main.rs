use chrono::Datelike;

fn main() {
    let current_date = chrono::Utc::now();
    let curr_year = current_date.year();
    let curr_month = current_date.month();
    let curr_day = current_date.day();

    let buf = std::io::stdin();
    let mut input = String::new();
    buf.read_line(&mut input).unwrap();

    println!("{}", curr_year);
}

fn is_leap_year(year: u32) -> bool {
    if year % 400 == 0 {
        return true;
    } else if year % 400 != 0 && year % 100 == 0 {
        return false;
    }

    year % 400 != 0 && year % 100 != 0 && year % 4 == 0
}

fn days_in_month(year: u32, month: u32) -> u32 {
    let feb = if is_leap_year(year) { 29 } else { 28 };

    match month {
        1 => 31,
        2 => feb,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_years() {
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(2001));
        assert!(!is_leap_year(2002));
        assert!(!is_leap_year(2003));
        assert!(is_leap_year(2004));
        assert!(!is_leap_year(2005));
        assert!(!is_leap_year(2006));
        assert!(is_leap_year(2020));
        assert!(is_leap_year(2044));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2021));
    }
}
