use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl TryFrom<i32> for Month {
    type Error = MonthParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(MonthParseError {
                invalid_month: value,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MonthParseError {
    invalid_month: i32,
}
impl fmt::Display for MonthParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid value {} for Month. Only numbers from range 1..=12 are allowed.",
            self.invalid_month
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_parsing() {
        // test if each digit is properly parsed to Month
        let january: Month = 1.try_into().unwrap();
        assert_eq!(january, Month::January);
        let february: Month = 2.try_into().unwrap();
        assert_eq!(february, Month::February);
        let march: Month = 3.try_into().unwrap();
        assert_eq!(march, Month::March);
        let april: Month = 4.try_into().unwrap();
        assert_eq!(april, Month::April);
        let may: Month = 5.try_into().unwrap();
        assert_eq!(may, Month::May);
        let june: Month = 6.try_into().unwrap();
        assert_eq!(june, Month::June);
        let july: Month = 7.try_into().unwrap();
        assert_eq!(july, Month::July);
        let august: Month = 8.try_into().unwrap();
        assert_eq!(august, Month::August);
        let september: Month = 9.try_into().unwrap();
        assert_eq!(september, Month::September);
        let october: Month = 10.try_into().unwrap();
        assert_eq!(october, Month::October);
        let november: Month = 11.try_into().unwrap();
        assert_eq!(november, Month::November);
        let december: Month = 12.try_into().unwrap();
        assert_eq!(december, Month::December);
    }

    #[test]
    fn invalid_parsing() {
        // test if parsing an invalid digit returns an error
        let invalid_month: Result<Month, MonthParseError> = (-1).try_into();
        assert!(invalid_month.is_err());
        let invalid_month: Result<Month, MonthParseError> = 0.try_into();
        assert!(invalid_month.is_err());
        let invalid_month: Result<Month, MonthParseError> = 13.try_into();
        assert!(invalid_month.is_err());
        let invalid_month: Result<Month, MonthParseError> = 14.try_into();
        assert!(invalid_month.is_err());
    }
}
