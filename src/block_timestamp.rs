use anyhow::Context;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime};
use substreams_ethereum::pb::eth::v2 as eth;

#[derive(Debug, PartialEq)]
pub struct BlockTimestamp(chrono::NaiveDateTime);

impl BlockTimestamp {
    pub fn from_key(key: &str) -> Self {
        Self::try_from_key(key).unwrap()
    }

    pub fn try_from_key(key: &str) -> Result<Self, anyhow::Error> {
        let millis = u64::from_str_radix(key, 10)
            .with_context(|| format!("invalid timestamp key {}", key))?;
        let seconds = millis / 1000;
        let nanos = (millis % 1000) * 1000000;

        Ok(BlockTimestamp(NaiveDateTime::from_timestamp(
            seconds as i64,
            nanos as u32,
        )))
    }

    pub fn from_block(blk: &eth::Block) -> Self {
        let header = blk.header.as_ref().unwrap();
        let timestamp = header.timestamp.as_ref().unwrap();

        BlockTimestamp(NaiveDateTime::from_timestamp(
            timestamp.seconds,
            timestamp.nanos as u32,
        ))
    }

    pub fn start_of_day(&self) -> NaiveDateTime {
        self.0.date().and_time(NaiveTime::default())
    }

    pub fn start_of_day_key(&self) -> String {
        to_key(self.start_of_day())
    }

    pub fn start_of_month(&self) -> NaiveDateTime {
        self.0
            .with_day(1)
            .unwrap()
            .date()
            .and_time(NaiveTime::default())
    }

    pub fn start_of_month_key(&self) -> String {
        to_key(self.start_of_month())
    }

    pub fn end_of_day(&self) -> NaiveDateTime {
        self.0.date().and_time(last_time())
    }

    pub fn end_of_day_key(&self) -> String {
        to_key(self.end_of_day())
    }

    pub fn end_of_month(&self) -> NaiveDateTime {
        // Next month calculation
        let (y, m) = match (self.0.year(), self.0.month()) {
            (y, m) if m == 12 => (y + 1, 1),
            (y, m) => (y, m + 1),
        };
        let start_of_next_month = NaiveDate::from_ymd(y, m, 1).and_time(NaiveTime::default());
        let end_of_month = start_of_next_month - Duration::nanoseconds(1);

        end_of_month
    }

    pub fn end_of_month_key(&self) -> String {
        to_key(self.end_of_month())
    }
}

impl ToString for BlockTimestamp {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Into<String> for BlockTimestamp {
    fn into(self) -> String {
        self.0.to_string()
    }
}

fn last_time() -> NaiveTime {
    NaiveTime::from_hms_nano(23, 59, 59, 999999999)
}

fn to_key(input: NaiveDateTime) -> String {
    input.timestamp_millis().to_string()
}

#[cfg(test)]
mod tests {
    use super::BlockTimestamp;
    use chrono::NaiveDate;

    fn timestamp(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        millis: u32,
    ) -> BlockTimestamp {
        BlockTimestamp(NaiveDate::from_ymd(year, month, day).and_hms_milli(hour, min, sec, millis))
    }

    #[test]
    fn it_block_timestamp_try_from_key() {
        assert_eq!(
            BlockTimestamp::from_key("1435708800000"),
            timestamp(2015, 07, 01, 00, 00, 00, 000)
        );

        assert_eq!(
            BlockTimestamp::from_key("1669852799999"),
            timestamp(2022, 11, 30, 23, 59, 59, 999)
        );
    }

    #[test]
    fn it_block_timestamp_start_of_day() {
        let input = timestamp(2021, 7, 5, 10, 21, 54, 354);
        assert_eq!(input.start_of_day().to_string(), "2021-07-05 00:00:00");
    }

    #[test]
    fn it_block_timestamp_start_of_month() {
        let input = timestamp(2021, 7, 5, 10, 21, 54, 354);
        assert_eq!(input.start_of_month().to_string(), "2021-07-01 00:00:00");
    }

    #[test]
    fn it_block_timestamp_end_of_day() {
        let input = timestamp(2021, 7, 5, 10, 21, 54, 354);
        assert_eq!(
            input.end_of_day().to_string(),
            "2021-07-05 23:59:59.999999999"
        );
    }

    #[test]
    fn it_block_timestamp_end_of_month() {
        let input = timestamp(2021, 7, 5, 10, 21, 54, 354);
        assert_eq!(
            input.end_of_month().to_string(),
            "2021-07-31 23:59:59.999999999"
        );
    }
}
