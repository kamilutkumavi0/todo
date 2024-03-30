//! Creates date stucture for todo
mod days;
use days::days_in_month;
use core::cmp::Ordering;
use std::fmt;

fn split(input: String) -> (i32, i32, i32){
    let mut day = String::new();
    let mut month = String::new();
    let mut year = String::new();
    let mut count = 0;
    for i in input.chars(){
        if i == '/'{
            count += 1;
        } 
        else if count == 0{
            day.push(i);
        }
        else if count == 1{
            month.push(i);
        }
        else if count == 2{
            year.push(i)
        }
    }
    (day.parse::<i32>().unwrap(), month.parse::<i32>().unwrap(), year.parse::<i32>().unwrap())
}

/// TodoDate Error enum for checking user input
#[derive(PartialEq, Debug, Clone)]
pub enum TodoDateError{
    InCorrectDay,
    InCorrectMonth,
    InCorrectDate,
}

/// Creates a todo date for todo structure
#[derive(PartialEq, Debug, Eq, Ord, Clone)]
pub struct TodoDate{
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

impl PartialOrd for TodoDate{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        if self.year > other.year{
            return Some(self.cmp(self));
        } else if self.year < other.year {
            return Some(self.cmp(self));
        } else {
            if self.month > other.month{
                return Some(self.cmp(self));
            } else if self.month < other.month{
                return Some(self.cmp(self));            
            } else {
                if self.day > other.day{
                    return Some(self.cmp(self));
                } else {
                    return Some(self.cmp(other));
                }
            }
        }
    }
    fn lt(&self, other: &Self) -> bool{
        if self.year < other.year || 
            self.year == other.year && self.month < other.month ||
            self.year == other.year && self.month == other.month && self.day < other.day
        {
            return true;
        }
        false
    }
    fn gt(&self, other: &Self) -> bool{
        if self.year > other.year || 
            self.year == other.year && self.month > other.month ||
            self.year == other.year && self.month == other.month && self.day > other.day
        {
            return true;
        }
        false
    }
    fn le(&self, other: &Self) -> bool{
        if self.year > other.year || 
            self.year == other.year && self.month > other.month ||
            self.year == other.year && self.month == other.month && self.day > other.day
        {
            return false;
        }
        true
    }
    fn ge(&self, other: &Self) -> bool{
        if self.year < other.year || 
            self.year == other.year && self.month < other.month ||
            self.year == other.year && self.month == other.month && self.day < other.day
        {
            return false;
        }
        true
    }
}
impl fmt::Display for TodoDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.day, self.month, self.year)
    }
}
impl TodoDate{
    /// Creates a new todo date structure
    pub fn new(day: i32, month: i32, year: i32) -> Result<Self, TodoDateError> {
        if month <= 0 || month > 12{
            return Err(TodoDateError::InCorrectMonth);
        }
        if day <= 0 || day>days_in_month(month,year){
            return Err(TodoDateError::InCorrectDay);
        } 
        
        Ok(Self {day, month, year})
    }
    pub fn from_string(date: String) -> Result<Self, TodoDateError>{
        let (day, month, year) = split(date);
        if month <= 0 || month > 12{
            return Err(TodoDateError::InCorrectMonth);
        }
        if day <= 0 || day>days_in_month(month,year){
            return Err(TodoDateError::InCorrectDay);
        } 
        
        Ok(Self {day, month, year})
    }
}
