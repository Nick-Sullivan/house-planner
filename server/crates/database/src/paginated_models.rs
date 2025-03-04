use anyhow::{anyhow, Error, Result};
use aws_sdk_dynamodb::types::AttributeValue;
use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};
use std::collections::HashMap;

pub type DbKey = HashMap<String, AttributeValue>;

pub fn serialise_db_key(db_key: DbKey) -> Result<String> {
    let mut json_map = serde_json::Map::new();
    for (key, value) in db_key {
        json_map.insert(key.clone(), attribute_to_json_value(&value)?);
    }
    let json = Value::Object(json_map);
    let json_string = serde_json::to_string(&json)?;
    Ok(general_purpose::STANDARD.encode(json_string))
}

pub fn deserialise_db_key(encoded: &str) -> Result<DbKey, Error> {
    let json_string = String::from_utf8(general_purpose::STANDARD.decode(encoded)?)?;
    let value: Value = serde_json::from_str(&json_string)?;
    if let Value::Object(map) = value {
        let mut result = DbKey::new();
        for (key, value) in map {
            result.insert(key, json_value_to_attribute(&value)?);
        }
        Ok(result)
    } else {
        Err(anyhow!("Expected JSON object"))
    }
}

fn attribute_to_json_value(attr: &AttributeValue) -> Result<Value> {
    match attr {
        AttributeValue::S(s) => Ok(Value::String(s.clone())),
        AttributeValue::N(n) => Ok(json!(n)),
        AttributeValue::Bool(b) => Ok(Value::Bool(*b)),
        _ => Err(anyhow!("Unsupported AttributeValue type")),
    }
}

fn json_value_to_attribute(value: &Value) -> Result<AttributeValue> {
    match value {
        Value::String(s) => Ok(AttributeValue::S(s.clone())),
        Value::Number(n) => Ok(AttributeValue::N(n.to_string())),
        Value::Bool(b) => Ok(AttributeValue::Bool(*b)),
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
