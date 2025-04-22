#[derive(Debug, Clone, Copy)]
pub enum Signers {
    Avax,
    Primary,
}

impl Signers {
    pub fn get_signers(&self) -> Vec<&str> {
        match self {
            Signers::Avax => AVAX_SIGNERS.to_vec(),
            Signers::Primary => PRIMARY_SIGNERS.to_vec(),
        }
    }
}

pub const AVAX_SIGNERS: [&str; 5] = [
    "109B4a318A4F5ddcbCA6349B45f881B4137deaFB",
    "12470f7aba85c8b81d63137dd5925d6ee114952b",
    "1ea62d73edf8ac05dfcea1a34b9796e937a29eff",
    "83cba8c619fb629b81a65c2e67fe15cf3e3c9747",
    "2c59617248994D12816EE1Fa77CE0a64eEB456BF",
];

pub const PRIMARY_SIGNERS: [&str; 5] = [
    "8bb8f32df04c8b654987daaed53d6b6091e3b774",
    "deb22f54738d54976c4c0fe5ce6d408e40d88499",
    "51ce04be4b3e32572c4ec9135221d0691ba7d202",
    "dd682daec5a90dd295d14da4b0bec9281017b5be",
    "9c5ae89c4af6aa32ce58588dbaf90d18a855b6de",
];
