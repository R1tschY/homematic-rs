use serde::{Deserialize, Deserializer};

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: i32 = Deserialize::deserialize(deserializer)?;
    Ok(value != 0)
}

pub fn deserialize_bool_option<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i32> = Deserialize::deserialize(deserializer)?;
    Ok(value.map(|x| x != 0))
}

pub fn role_list<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Deserialize::deserialize(deserializer)?;
    Ok(value.map(|s| {
        if s.is_empty() {
            vec![]
        } else {
            s.split(' ').map(|s| s.to_string()).collect()
        }
    }))
}
