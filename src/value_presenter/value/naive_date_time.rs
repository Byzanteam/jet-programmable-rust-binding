use core::convert::TryFrom;

use serde_json::Value;
use time::{
    format_description::well_known::Rfc3339, macros::format_description, Date, Month,
    PrimitiveDateTime, Time,
};

use super::json_codec::JsonCodec;

#[derive(Debug)]
pub struct ParseNaiveDateTimeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaiveDateTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanosecond: u32,
}

impl NaiveDateTime {
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
        }
    }
}

impl JsonCodec for NaiveDateTime {
    type Err = ParseNaiveDateTimeError;

    fn from_json(value: &Value) -> Result<Self, Self::Err> {
        match value.as_str() {
            Some(str) => {
                let str = normalize(str);

                match PrimitiveDateTime::parse(&str, &Rfc3339) {
                    Ok(pdt) => Ok(NaiveDateTime {
                        year: pdt.year(),
                        month: pdt.month() as u8,
                        day: pdt.day(),
                        hour: pdt.hour(),
                        minute: pdt.minute(),
                        second: pdt.second(),
                        nanosecond: pdt.nanosecond(),
                    }),
                    Err(_err) => Err(ParseNaiveDateTimeError),
                }
            }
            None => Err(ParseNaiveDateTimeError),
        }
    }

    fn to_json(&self) -> Value {
        let date =
            Date::from_calendar_date(self.year, Month::try_from(self.month).unwrap(), self.day)
                .unwrap();
        let time =
            Time::from_hms_nano(self.hour, self.minute, self.second, self.nanosecond).unwrap();

        let pdt = PrimitiveDateTime::new(date, time);

        let format = if pdt.nanosecond() == 0 {
            format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]")
        } else {
            format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:1+]"
            )
        };

        Value::String(pdt.format(&format).unwrap())
    }
}

fn normalize(str: &str) -> String {
    if str.ends_with('Z') {
        str.to_string()
    } else {
        str.to_string() + "Z"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_from_json() {
        let expected = NaiveDateTime {
            year: 2022,
            month: 4,
            day: 29,
            hour: 7,
            minute: 34,
            second: 10,
            nanosecond: 420159000,
        };

        {
            let json = json!("2022-04-29T07:34:10.420159");
            let pdt = NaiveDateTime::from_json(&json).unwrap();

            assert_eq!(pdt, expected);
        }

        // with timezone
        {
            let json = json!("2022-04-29T07:34:10.420159Z");
            let pdt = NaiveDateTime::from_json(&json).unwrap();

            assert_eq!(pdt, expected);
        }

        // without ms
        {
            let json = json!("2022-04-29T07:34:10Z");
            let pdt = NaiveDateTime::from_json(&json).unwrap();
            let expected = NaiveDateTime {
                year: 2022,
                month: 4,
                day: 29,
                hour: 7,
                minute: 34,
                second: 10,
                nanosecond: 0,
            };

            assert_eq!(pdt, expected);
        }

        // invalid json
        {
            let json = json!("2022-04-29 07:34");
            let pdt = NaiveDateTime::from_json(&json);

            assert!(matches!(pdt, Err(_)));
        }
    }

    #[test]
    fn test_to_json() {
        {
            let json = json!("2022-04-29T07:34:10");
            let pdt = NaiveDateTime::from_json(&json);

            assert_eq!(pdt.unwrap().to_json(), json);
        }

        // with nanosecond
        {
            let json = json!("2022-04-29T07:34:10.420159");
            let pdt = NaiveDateTime::from_json(&json);

            assert_eq!(pdt.unwrap().to_json(), json);
        }
    }
}
