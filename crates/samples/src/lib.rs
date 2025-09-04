mod package_signers;

use crate::package_signers::Signers;

pub const SAMPLE_SYSTEM_TIMESTAMP_OLD: u64 = 1707738300;
pub const SAMPLE_SYSTEM_TIMESTAMP: u64 = 1725975900;
pub const SAMPLE_SYSTEM_TIMESTAMP_2: u64 = 1725976000;

pub const DEFAULT_SIGNERS_THRESHOLD: u8 = 3;

#[derive(Debug, Clone)]
pub struct Sample {
    pub content: &'static str,
    pub feeds: Vec<String>,
    pub timestamp: u64,
    pub system_timestamp: u64,
    pub signers: Signers,
}

impl Sample {
    pub fn feeds(&self) -> Vec<String> {
        self.feeds.clone()
    }

    pub fn any_valid() -> Self {
        sample_eth_3sig()
    }
}

pub fn sample_eth_btc_avax_5sig_old() -> Sample {
    Sample {
        content: include_str!("../samples/ETH_BTC_AVAX_5sig.hex"),
        feeds: vec!["ETH".to_owned(), "BTC".to_owned(), "AVAX".to_owned()],
        timestamp: 1707738270000,
        system_timestamp: SAMPLE_SYSTEM_TIMESTAMP_OLD,
        signers: Signers::Avax,
    }
}

pub fn sample_eth_btc_avax_5sig() -> Sample {
    Sample {
        content: include_str!("../samples/ETH_BTC_AVAX_5sig.hex"),
        feeds: vec!["ETH".to_owned(), "BTC".to_owned(), "AVAX".to_owned()],
        timestamp: 1725975800000,
        system_timestamp: SAMPLE_SYSTEM_TIMESTAMP,
        signers: Signers::Avax,
    }
}

pub fn sample_eth_btc_avax_5sig_2() -> Sample {
    Sample {
        content: include_str!("../samples/ETH_BTC_AVAX_5sig_2.hex"),
        feeds: vec!["ETH".to_owned(), "BTC".to_owned(), "AVAX".to_owned()],
        timestamp: 1725975870000,
        system_timestamp: SAMPLE_SYSTEM_TIMESTAMP_2,
        signers: Signers::Avax,
    }
}

pub fn sample_eth_3sig() -> Sample {
    Sample {
        content: include_str!("../samples/ETH_PRIMARY_3sig.hex"),
        feeds: vec!["ETH".to_owned()],
        timestamp: 1744563500000,
        system_timestamp: 1744563500000,
        signers: Signers::Primary,
    }
}

pub fn sample_eth_2sig() -> Sample {
    Sample {
        content: include_str!("../samples/ETH_PRIMARY_2sig.hex"),
        feeds: vec!["ETH".to_owned()],
        timestamp: 1744563500000,
        system_timestamp: 1744563500000,
        signers: Signers::Avax,
    }
}

pub fn sample_eth_3sig_newer() -> Sample {
    Sample {
        content: include_str!("../samples/ETH_PRIMARY_3sig_newer.hex"),
        feeds: vec!["ETH".to_owned()],
        timestamp: 1744563600000,
        system_timestamp: 1744563600000,
        signers: Signers::Primary,
    }
}

pub fn sample_btc_eth_3sig() -> Sample {
    Sample {
        content: include_str!("../samples/ETH_BTC_PRIMARY_3sig.hex"),
        feeds: vec!["ETH".to_owned(), "BTC".to_owned()],
        timestamp: 1744829560000,
        system_timestamp: 1744829560000,
        signers: Signers::Primary,
    }
}

pub fn sample_btc_eth_3sig_newer() -> Sample {
    Sample {
        content: include_str!("../samples/ETH_BTC_PRIMARY_3sig_newer.hex"),
        feeds: vec!["ETH".to_owned(), "BTC".to_owned()],
        timestamp: 1744829650000,
        system_timestamp: 1744829650000,
        signers: Signers::Primary,
    }
}

pub fn sample_btc_5sig() -> Sample {
    Sample {
        content: include_str!("../samples/BTC_PRIMARY_5sig.hex"),
        feeds: vec!["BTC".to_owned()],
        timestamp: 1744829680000,
        system_timestamp: 1744829680000,
        signers: Signers::Primary,
    }
}

pub fn sample_btc_5sig_newer() -> Sample {
    Sample {
        content: include_str!("../samples/BTC_PRIMARY_5sig_newer.hex"),
        feeds: vec!["BTC".to_owned()],
        timestamp: 1744829750000,
        system_timestamp: 1744829750000,
        signers: Signers::Primary,
    }
}
