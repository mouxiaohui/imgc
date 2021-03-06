use serde::Serialize;

#[derive(Serialize)]
pub struct Colors {
    pub rgb: (u8, u8, u8),
    pub hex: String,
}

impl Colors {
    pub fn new(rgb: (u8, u8, u8)) -> Self {
        Self {
            rgb,
            hex: String::new(),
        }
    }

    pub fn generate_hex(&mut self) {
        let mut hex = "#".to_owned();
        hex.push_str(&format!("{:02X}", self.rgb.0));
        hex.push_str(&format!("{:02X}", self.rgb.1));
        hex.push_str(&format!("{:02X}", self.rgb.2));

        self.hex.push_str(&hex);
    }
}
