/// Create a enum for a year whitch is leak or not can understandable 
#[derive(PartialEq, Debug)]
enum Leak{
    Leak,
    NotLeak,
}
/// Takes a year argumant and returns Leak enum
fn is_leak(year:i32) -> Leak{
    if year % 400 == 0{
        return Leak::Leak;
    } else if year % 100 == 0{
        return Leak::NotLeak;
    } else if year % 4 == 0{
        return Leak::Leak;
    } else {
        return Leak::NotLeak;
    }
}
/// Takes two argumant month and year and return total day
pub fn days_in_month(month: i32, year: i32) -> i32{
    match month{
        1 => 31,
        2 => {
            if is_leak(year) == Leak::Leak{
                29
            } else {
                28
            }
        },
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
        i32::MIN..=0_i32 | 13_i32..=i32::MAX => 0, 
    }
}
