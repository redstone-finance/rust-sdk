use std::collections::HashMap;

use redstone::Value;

use crate::package_signers::Signers;

#[macro_export]
macro_rules! hashmap {
    ($( $key:expr => $val:expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key.to_string(), Value::from($val)); )*
        map
    }};
}
pub const SAMPLE_SYSTEM_TIMESTAMP_OLD: u64 = 1707738300;
pub const SAMPLE_SYSTEM_TIMESTAMP: u64 = 1725975900;
pub const SAMPLE_SYSTEM_TIMESTAMP_2: u64 = 1725976000;

pub const DEFAULT_SIGNERS_THRESHOLD: u8 = 3;

#[derive(Debug, Clone)]
pub struct Sample {
    pub content: &'static str,
    pub values: HashMap<String, Value>,
    pub timestamp: u64,
    pub system_timestamp: u64,
    pub signers: Signers,
}

impl Sample {
    pub fn feeds(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }

    pub fn any_valid() -> Self {
        sample_eth_3sig()
    }
}

pub fn sample_eth_btc_avax_5sig_old() -> Sample {
    Sample {
        content: include_str!("ETH_BTC_AVAX_5sig.hex"),
        values: hashmap![
            "ETH" => 248111446713u128,
            "BTC" => 4783856731782u128,
            "AVAX" => 3859000000u128
        ],
        timestamp: 1707738270000,
        system_timestamp: SAMPLE_SYSTEM_TIMESTAMP_OLD,
        signers: Signers::Avax,
    }
}

pub fn sample_eth_btc_avax_5sig() -> Sample {
    Sample {
        content: include_str!("ETH_BTC_AVAX_5sig.hex"),
        values: hashmap![
            "ETH" => 233933981770u128,
            "BTC" => 5678054152708u128,
            "AVAX" => 2376928690u128
        ],
        timestamp: 1725975800000,
        system_timestamp: SAMPLE_SYSTEM_TIMESTAMP,
        signers: Signers::Avax,
    }
}

pub fn sample_eth_btc_avax_5sig_2() -> Sample {
    Sample {
        content: include_str!("ETH_BTC_AVAX_5sig_2.hex"),
        values: hashmap![
            "ETH" => 234067119798u128,
            "BTC" => 5682347620349u128,
            "AVAX" => 2378176208u128
        ],
        timestamp: 1725975870000,
        system_timestamp: SAMPLE_SYSTEM_TIMESTAMP_2,
        signers: Signers::Avax,
    }
}

pub fn sample_eth_3sig() -> Sample {
    Sample {
        content: include_str!("ETH_PRIMARY_3sig.hex"),
        values: hashmap![
            "ETH" => 159504422175_u128
        ],
        timestamp: 1744563500000,
        system_timestamp: 1744563500000,
        signers: Signers::Primary,
    }
}

pub fn sample_eth_2sig() -> Sample {
    Sample {
        content: include_str!("ETH_PRIMARY_2sig.hex"),
        values: hashmap![
            "ETH" => 12345_u128
        ],
        timestamp: 1744563500000,
        system_timestamp: 1744563500000,
        signers: Signers::Avax,
    }
}

pub fn sample_eth_3sig_newer() -> Sample {
    Sample {
        content: include_str!("ETH_PRIMARY_3sig_newer.hex"),
        values: hashmap![
            "ETH" => 159526674144_u128
        ],
        timestamp: 1744563600000,
        system_timestamp: 1744563600000,
        signers: Signers::Primary,
    }
}

pub fn sample_btc_eth_3sig() -> Sample {
    Sample {
        content: include_str!("ETH_BTC_PRIMARY_3sig.hex"),
        values: hashmap![
            "ETH" =>  156537608660_u128,
            "BTC" => 8396083019375_u128
        ],
        timestamp: 1744829560000,
        system_timestamp: 1744829560000,
        signers: Signers::Primary,
    }
}

pub fn sample_btc_eth_3sig_newer() -> Sample {
    Sample {
        content: include_str!("ETH_BTC_PRIMARY_3sig_newer.hex"),
        values: hashmap![
            "ETH" =>  156537608660_u128,
            "BTC" => 8396083019375_u128
        ],
        timestamp: 1744829650000,
        system_timestamp: 1744829650000,
        signers: Signers::Primary,
    }
}

pub fn sample_btc_5sig() -> Sample {
    Sample {
        content: include_str!("BTC_PRIMARY_5sig.hex"),
        values: hashmap![
            "BTC" => 8396206788771_u128
        ],
        timestamp: 1744829680000,
        system_timestamp: 1744829680000,
        signers: Signers::Primary,
    }
}

pub fn sample_btc_5sig_newer() -> Sample {
    Sample {
        content: include_str!("BTC_PRIMARY_5sig_newer.hex"),
        values: hashmap![
            "BTC" => 8407244389442_u128
        ],
        timestamp: 1744829750000,
        system_timestamp: 1744829750000,
        signers: Signers::Primary,
    }
}
