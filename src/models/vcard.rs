//! vCard/jCard model

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// vCard in jCard format (RFC 7095)
#[derive(Debug, Clone)]
pub struct VCard {
    properties: Vec<VCardProperty>,
}

impl VCard {
    /// Parse from jCard array format
    pub fn from_array(arr: &[Value]) -> Option<Self> {
        if arr.len() != 2 {
            return None;
        }

        if arr[0].as_str() != Some("vcard") {
            return None;
        }

        let props = arr[1].as_array()?;
        let mut properties = Vec::new();

        for prop in props {
            if let Some(p) = VCardProperty::from_value(prop) {
                properties.push(p);
            }
        }

        Some(Self { properties })
    }

    /// Get formatted name
    pub fn name(&self) -> Option<&str> {
        self.get_property_value("fn")
    }

    /// Get email
    pub fn email(&self) -> Option<&str> {
        self.get_property_value("email")
    }

    /// Get telephone
    pub fn tel(&self) -> Option<&str> {
        self.get_property_value("tel")
    }

    /// Get organization
    pub fn org(&self) -> Option<&str> {
        self.get_property_value("org")
    }

    /// Get address components
    pub fn address(&self) -> Option<VCardAddress> {
        let prop = self.properties.iter().find(|p| p.name == "adr")?;

        // Check for label parameter (pre-formatted address)
        let label = prop
            .parameters
            .get("label")
            .and_then(|v| v.as_str())
            .map(std::string::ToString::to_string);

        if let VCardValue::Structured(parts) = &prop.value
            && parts.len() >= 7
        {
            return Some(VCardAddress {
                label,
                po_box: parts[0].clone(),
                extended: parts[1].clone(),
                street: parts[2].clone(),
                locality: parts[3].clone(),
                region: parts[4].clone(),
                postal_code: parts[5].clone(),
                country: parts[6].clone(),
            });
        }
        None
    }

    fn get_property_value(&self, name: &str) -> Option<&str> {
        self.properties
            .iter()
            .find(|p| p.name == name)
            .and_then(|p| p.value.as_str())
    }

    pub fn properties(&self) -> &[VCardProperty] {
        &self.properties
    }
}

/// vCard property
#[derive(Debug, Clone)]
pub struct VCardProperty {
    pub name: String,
    pub parameters: serde_json::Map<String, Value>,
    pub value_type: String,
    pub value: VCardValue,
}

impl VCardProperty {
    fn from_value(val: &Value) -> Option<Self> {
        let arr = val.as_array()?;
        if arr.len() < 4 {
            return None;
        }

        let name = arr[0].as_str()?.to_string();
        let parameters = arr[1].as_object()?.clone();
        let value_type = arr[2].as_str()?.to_string();
        let value = VCardValue::from_json(&arr[3]);

        Some(Self {
            name,
            parameters,
            value_type,
            value,
        })
    }
}

/// vCard value types
#[derive(Debug, Clone)]
pub enum VCardValue {
    Text(String),
    Structured(Vec<String>),
    Array(Vec<String>),
}

impl VCardValue {
    fn from_json(val: &Value) -> Self {
        match val {
            Value::String(s) => Self::Text(s.clone()),
            Value::Array(arr) => {
                let items: Vec<String> = arr
                    .iter()
                    .filter_map(|v| v.as_str().map(std::string::ToString::to_string))
                    .collect();
                if items.len() == arr.len() {
                    Self::Structured(items)
                } else {
                    Self::Array(vec![])
                }
            }
            _ => Self::Text(val.to_string()),
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s),
            _ => None,
        }
    }
}

/// vCard address
#[derive(Debug, Clone)]
pub struct VCardAddress {
    pub label: Option<String>,
    pub po_box: String,
    pub extended: String,
    pub street: String,
    pub locality: String,
    pub region: String,
    pub postal_code: String,
    pub country: String,
}

// Custom deserialization for VCard
impl<'de> Deserialize<'de> for VCard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr = Vec::<Value>::deserialize(deserializer)?;
        Self::from_array(&arr).ok_or_else(|| serde::de::Error::custom("Invalid vCard format"))
    }
}

impl Serialize for VCard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element("vcard")?;

        let mut props = Vec::new();
        for prop in &self.properties {
            let p = serde_json::json!([
                prop.name,
                prop.parameters,
                prop.value_type,
                match &prop.value {
                    VCardValue::Text(s) => Value::String(s.clone()),
                    VCardValue::Structured(v) | VCardValue::Array(v) => {
                        Value::Array(v.iter().map(|s| Value::String(s.clone())).collect())
                    }
                }
            ]);
            props.push(p);
        }
        seq.serialize_element(&props)?;
        seq.end()
    }
}
