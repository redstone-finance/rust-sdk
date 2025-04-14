pub mod sample_test_helpers;

use std::collections::HashMap;

use redstone::Value;

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
pub const SIGNERS: [&str; 5] = [
    "0x109B4a318A4F5ddcbCA6349B45f881B4137deaFB",
    "0x12470f7aba85c8b81d63137dd5925d6ee114952b",
    "0x1ea62d73edf8ac05dfcea1a34b9796e937a29eff",
    "0x83cba8c619fb629b81a65c2e67fe15cf3e3c9747",
    "0x2c59617248994D12816EE1Fa77CE0a64eEB456BF",
];

#[derive(Debug, Clone)]
pub struct Sample {
    pub content: &'static str,
    pub values: HashMap<String, Value>,
    pub timestamp: u64,
    pub system_timestamp: u64,
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
    }
}

pub fn sample_eth_3sig() -> Sample {
    Sample {
        content: include_str!("ETH_PRIMARY_3sig.hex"),
        values: hashmap![
            "ETH" => 159504422175
        ],
        timestamp: 1744563500000,
        system_timestamp: 0,
    }
}

pub fn sample_eth_3sig_newer() -> Sample {
    Sample {
        content: include_str!("ETH_PRIMARY_3sig_newer.hex"),
        values: hashmap![
            "ETH" => 159504422175
        ],
        timestamp: 159526674144,
        system_timestamp: 1744563600000,
    }
}


