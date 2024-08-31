use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl TryFrom<i32> for DayOfWeek {
    type Error = DayOfWeekParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DayOfWeek::Monday),
            2 => Ok(DayOfWeek::Tuesday),
            3 => Ok(DayOfWeek::Wednesday),
            4 => Ok(DayOfWeek::Thursday),
            5 => Ok(DayOfWeek::Friday),
            6 => Ok(DayOfWeek::Saturday),
            7 => Ok(DayOfWeek::Sunday),
            _ => Err(DayOfWeekParseError {
                invalid_value: value,
            }),
        }
    }
}

#[derive(Debug)]
pub struct DayOfWeekParseError {
    invalid_value: i32,
}
impl fmt::Display for DayOfWeekParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid value {} for DayOfWeek. Only numbers from range 1..=7 are allowed.",
            self.invalid_value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_parsing() {
        // test if each digit is properly parsed to DayOfWeek
        // let january: Month = 1.try_into().unwrap();
        // assert_eq!(january, Month::January);
        // let february: Month = 2.try_into().unwrap();
        // assert_eq!(february, Month::February);
        // let march: Month = 3.try_into().unwrap();
        // assert_eq!(march, Month::March);
        // let april: Month = 4.try_into().unwrap();
        // assert_eq!(april, Month::April);
        // let may: Month = 5.try_into().unwrap();
        // assert_eq!(may, Month::May);
        // let june: Month = 6.try_into().unwrap();
        // assert_eq!(june, Month::June);
        // let july: Month = 7.try_into().unwrap();
        // assert_eq!(july, Month::July);
        // let august: Month = 8.try_into().unwrap();
        // assert_eq!(august, Month::August);
        // let september: Month = 9.try_into().unwrap();
        // assert_eq!(september, Month::September);
        // let october: Month = 10.try_into().unwrap();
        // assert_eq!(october, Month::October);
        // let november: Month = 11.try_into().unwrap();
        // assert_eq!(november, Month::November);
        // let december: Month = 12.try_into().unwrap();
        // assert_eq!(december, Month::December);
        let monday: DayOfWeek = 1.try_into().unwrap();
        assert_eq!(monday, DayOfWeek::Monday);
        let tuesday: DayOfWeek = 2.try_into().unwrap();
        assert_eq!(tuesday, DayOfWeek::Tuesday);
        let wednesday: DayOfWeek = 3.try_into().unwrap();
        assert_eq!(wednesday, DayOfWeek::Wednesday);
        let thursday: DayOfWeek = 4.try_into().unwrap();
        assert_eq!(thursday, DayOfWeek::Thursday);
        let friday: DayOfWeek = 5.try_into().unwrap();
        assert_eq!(friday, DayOfWeek::Friday);
        let saturday: DayOfWeek = 6.try_into().unwrap();
        assert_eq!(saturday, DayOfWeek::Saturday);
        let sunday: DayOfWeek = 7.try_into().unwrap();
        assert_eq!(sunday, DayOfWeek::Sunday);
    }

    #[test]
    fn invalid_parsing() {
        // test if parsing an invalid digit returns an error
        let invalid_day: Result<DayOfWeek, DayOfWeekParseError> = (-1).try_into();
        assert!(invalid_day.is_err());
        let invalid_day: Result<DayOfWeek, DayOfWeekParseError> = 0.try_into();
        assert!(invalid_day.is_err());
        let invalid_day: Result<DayOfWeek, DayOfWeekParseError> = 8.try_into();
        assert!(invalid_day.is_err());
        let invalid_day: Result<DayOfWeek, DayOfWeekParseError> = 9.try_into();
        assert!(invalid_day.is_err());
    }
}
