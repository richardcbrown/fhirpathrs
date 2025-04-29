pub mod arithmetic_type;
pub mod date;
pub mod date_time;
pub mod offset;
pub mod quantity;
pub mod time;
pub mod type_info;
pub mod utils;

#[cfg(test)]
mod test {
    use serde_json::Value;

    use crate::evaluate::data_types::{
        date::Date,
        date_time::{DateTime, DateTimePrecision},
        time::{Time, TimePrecision},
    };

    #[test]
    fn test_parse_basic_time() {
        let time = Time::try_from(&Value::String("@T10:11:12".to_string())).unwrap();

        assert_eq!(
            time,
            Time {
                hours: Some(10),
                minutes: Some(11),
                seconds: Some(12),
                millis: None,
                precision: TimePrecision::Seconds
            }
        )
    }

    #[test]
    fn test_parse_basic_datetime() {
        let datetime =
            DateTime::try_from(&Value::String("@2025-01-01T10:11:12.123+02:00".to_string()))
                .unwrap();

        assert_eq!(
            datetime,
            DateTime {
                date: Date {
                    years: Some(2025),
                    months: Some(01),
                    days: Some(01)
                },
                time: Some(Time {
                    hours: Some(8),
                    minutes: Some(11),
                    seconds: Some(12),
                    millis: Some(123),
                    precision: TimePrecision::Seconds
                }),
                precision: DateTimePrecision::Seconds
            }
        )
    }

    #[test]
    fn test_datetime_cmp_none() {
        let result =
            DateTime::try_from(&Value::String("@2025-01-01T10:11:12.123+02:00".to_string()))
                .unwrap()
                .partial_cmp(
                    &DateTime::try_from(&Value::String("@2025-01-01T10:11".to_string())).unwrap(),
                );

        assert_eq!(result, None)
    }

    #[test]
    fn test_datetime_cmp_greater() {
        let result =
            DateTime::try_from(&Value::String("@2025-01-01T10:11:12.123+01:00".to_string()))
                .unwrap()
                .partial_cmp(
                    &DateTime::try_from(&Value::String(
                        "@2025-01-01T10:11:12.123+02:00".to_string(),
                    ))
                    .unwrap(),
                );

        assert_eq!(result, Some(std::cmp::Ordering::Greater))
    }

    #[test]
    fn test_datetime_cmp_lesser() {
        let result =
            DateTime::try_from(&Value::String("@2025-01-01T10:11:12.123+02:00".to_string()))
                .unwrap()
                .partial_cmp(
                    &DateTime::try_from(&Value::String(
                        "@2025-01-01T10:11:12.123+01:00".to_string(),
                    ))
                    .unwrap(),
                );

        assert_eq!(result, Some(std::cmp::Ordering::Less))
    }

    #[test]
    fn test_datetime_cmp_equal() {
        let result = DateTime::try_from(&Value::String("@2025-01-01".to_string()))
            .unwrap()
            .partial_cmp(&DateTime::try_from(&Value::String("@2025-01-01".to_string())).unwrap());

        assert_eq!(result, Some(std::cmp::Ordering::Equal))
    }
}
