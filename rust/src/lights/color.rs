use serde::{Serialize, Deserialize, Deserializer};

/// LED light color
#[derive(Copy, Clone, Debug)]
pub struct LedColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// quick maps from #,#,# to LedColor
impl From<(u8, u8, u8)> for LedColor {
    fn from(value: (u8, u8, u8)) -> Self {
        LedColor{ r: value.0, g: value.1, b: value.2 }
    }
}

impl From<[u8; 3]> for LedColor {
    fn from(value: [u8; 3]) -> Self {
        LedColor{ r: value[0], g: value[1], b: value[2] }
    }
}

// map from 
impl Into<[u8; 4]> for LedColor {
    fn into(self) -> [u8; 4] {
        [self.b, self.r, self.g, 0]
    }
}

impl Serialize for LedColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let hexcode = format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b);
        serializer.serialize_str(&hexcode)
    }
}

impl<'de> Deserialize<'de> for LedColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        let buf = String::deserialize(deserializer)?;
        if buf.len() != 7 || &buf[0..1] != "#" || !buf[1..7].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(serde::de::Error::custom(format!("Hex code not of form #XXXXXX : {buf}")))
        };
        let r = u8::from_str_radix(&buf[1..3], 16).unwrap();
        let g = u8::from_str_radix(&buf[3..5], 16).unwrap();
        let b = u8::from_str_radix(&buf[5..7], 16).unwrap();
        Ok(LedColor { r, g, b })
    }
}
