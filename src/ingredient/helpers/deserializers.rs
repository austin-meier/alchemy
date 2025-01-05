use serde::de::{Deserializer, Deserialize, Error};
use super::layer::Layer;

pub fn parse_layer<'de, D>(d: D) -> Result<Layer, D::Error>
where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|layer: Option<_>| {
            let layer_name = layer.unwrap_or("none");
            match layer_name {
                "background" => Layer::Background,
                "bleed" => Layer::Bleed,
                "print" => Layer::Print,
                "mask" => Layer::Mask,
                _ => Layer::None
            }
        })
}

pub fn parse_bootleg_boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s.to_ascii_lowercase().as_str() {
        "true" => Ok(true),
        "y" => Ok(true),
        "false" => Ok(false),
        "n" => Ok(false),
        _ => Err(Error::unknown_variant(s, &["true", "false", "Y", "N"])),
    }
}