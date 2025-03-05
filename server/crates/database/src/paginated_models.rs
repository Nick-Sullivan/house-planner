use anyhow::{anyhow, Error, Result};
use aws_sdk_dynamodb::types::AttributeValue;
use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;
use std::collections::HashMap;

pub type DbKey = HashMap<String, AttributeValue>;

pub fn serialise_db_key(db_key: DbKey) -> Result<String> {
    let json_map: HashMap<_, _> = db_key
        .into_iter()
        .map(|(k, v)| Ok((k, attribute_to_json_value(&v)?)))
        .collect::<Result<_>>()?;
    let json_string = serde_json::to_string(&json_map)?;
    Ok(general_purpose::STANDARD.encode(json_string))
}

pub fn deserialise_db_key(encoded: &str) -> Result<DbKey, Error> {
    let json_string = String::from_utf8(general_purpose::STANDARD.decode(encoded)?)?;
    let json_map: HashMap<String, Value> = serde_json::from_str(&json_string)?;
    let db_key = json_map
        .into_iter()
        .map(|(k, v)| Ok((k, json_value_to_attribute(&v)?)))
        .collect::<Result<_>>()?;
    Ok(db_key)
}
fn attribute_to_json_value(attr: &AttributeValue) -> Result<Value> {
    match attr {
        AttributeValue::S(s) => Ok(Value::String(format!("S:{}", s))),
        AttributeValue::N(n) => Ok(Value::String(format!("N:{}", n))),
        AttributeValue::Bool(b) => Ok(Value::String(format!("B:{}", b))),
        _ => Err(anyhow!("Unsupported AttributeValue type")),
    }
}

fn json_value_to_attribute(value: &Value) -> Result<AttributeValue> {
    match value {
        Value::String(s) if s.starts_with("S:") => Ok(AttributeValue::S(s[2..].to_string())),
        Value::String(s) if s.starts_with("N:") => Ok(AttributeValue::N(s[2..].to_string())),
        Value::String(s) if s.starts_with("B:") => Ok(AttributeValue::Bool(s[2..].parse()?)),
        _ => Err(anyhow!("Unsupported JSON value type")),
    }
}

pub struct PaginatedDbRequest<T> {
    pub value: T,
    pub limit: Option<i32>,
    pub last_evaluated_key: Option<DbKey>,
}

pub struct PaginatedDbResponse<T> {
    pub items: Vec<T>,
    pub last_evaluated_key: Option<DbKey>,
}
