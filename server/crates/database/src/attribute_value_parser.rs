use anyhow::Error;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.6f";

pub trait AttributeValueParser: Sized {
    fn parse(value: Option<&AttributeValue>) -> Result<Self, Error>;
}

pub fn parse_attribute_value<T: AttributeValueParser>(
    value: Option<&AttributeValue>,
) -> Result<T, Error> {
    T::parse(value)
}

impl AttributeValueParser for String {
    fn parse(value: Option<&AttributeValue>) -> Result<Self, Error> {
        let value = value.ok_or(anyhow::anyhow!("Key not found"))?;
        let result = value
            .as_s()
            .map_err(|_| anyhow::anyhow!("Expected string"))?
            .clone();
        Ok(result)
    }
}

impl AttributeValueParser for Option<String> {
    fn parse(value: Option<&AttributeValue>) -> Result<Self, Error> {
        match value {
            None => Ok(None),
            Some(attr_value) => {
                let result = attr_value
                    .as_s()
                    .map_err(|_| anyhow::anyhow!("Expected string"))?
                    .clone();
                Ok(Some(result))
            }
        }
    }
}

impl AttributeValueParser for i32 {
    fn parse(value: Option<&AttributeValue>) -> Result<Self, Error> {
        let value = value.ok_or(anyhow::anyhow!("Key not found"))?;
        let result = value
            .as_n()
            .map_err(|_| anyhow::anyhow!("Expected number"))?
            .parse::<i32>()
            .map_err(|_| anyhow::anyhow!("Could not parse number"))?;
        Ok(result)
    }
}

impl AttributeValueParser for f64 {
    fn parse(value: Option<&AttributeValue>) -> Result<Self, Error> {
        let value = value.ok_or(anyhow::anyhow!("Key not found"))?;
        let result = value
            .as_s()
            .map_err(|_| anyhow::anyhow!("Expected number"))?
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Could not parse number"))?;
        Ok(result)
    }
}
impl AttributeValueParser for bool {
    fn parse(value: Option<&AttributeValue>) -> Result<Self, Error> {
        let value = value.ok_or(anyhow::anyhow!("Key not found"))?;
        let result = value
            .as_bool()
            .map_err(|_| anyhow::anyhow!("Expected bool"))?
            .clone();
        Ok(result)
    }
}

impl AttributeValueParser for DateTime<Utc> {
    fn parse(value: Option<&AttributeValue>) -> Result<Self, Error> {
        let value = value.ok_or(anyhow::anyhow!("Key not found"))?;
        let result = value
            .as_s()
            .map_err(|_| anyhow::anyhow!("Expected string"))?;
        let naive_datetime = NaiveDateTime::parse_from_str(result, DATETIME_FORMAT)
            .map_err(|_| anyhow::anyhow!("Could not parse datetime"))?;
        let datetime = naive_datetime.and_utc();
        Ok(datetime)
    }
}

impl AttributeValueParser for Uuid {
    fn parse(value: Option<&AttributeValue>) -> Result<Self, Error> {
        let value = value.ok_or(anyhow::anyhow!("Key not found"))?;
        let result = value
            .as_s()
            .map_err(|_| anyhow::anyhow!("Expected string"))?
            .clone();
        let uuid = Uuid::parse_str(&result).map_err(|_| anyhow::anyhow!("Could not parse UUID"))?;
        Ok(uuid)
    }
}

pub fn single<T>(vec: Vec<T>) -> Result<T, Error> {
    if vec.len() == 1 {
        Ok(vec.into_iter().next().unwrap())
    } else {
        Err(anyhow::anyhow!("Expected exactly one element"))
    }
}
