pub mod env;
pub mod sample;

pub use paste;
pub use redstone;

#[macro_export]
macro_rules! test_price_adapter_impl {
    ($(
        $price_adapter_impl:ty, $id:ident
    ),*) => {
        ::redstone_testing::test_price_adapter_instantiate_impl!($($price_adapter_impl, $id),*);
        ::redstone_testing::test_price_adapter_read_impl!($($price_adapter_impl, $id),*);
        ::redstone_testing::test_price_adapter_write_impl!($($price_adapter_impl, $id),*);
    };
}

#[macro_export]
macro_rules! test_price_adapter_multi_feed_specific_impl {
    ($(
        $price_adapter_impl:ty, $id:ident
    ),*) => {
        ::redstone_testing::test_price_adapter_impl!($($price_adapter_impl, $id),*);
        ::redstone_testing::test_price_adapter_multi_feed_specific_testcases_impls!($($price_adapter_impl, $id),*);
    };
}

#[macro_export]
macro_rules! test_price_adapter_feed_specific_impl {
    ($(
        $price_adapter_impl:ty, $id:ident
    ),*) => {
        ::redstone_testing::test_price_adapter_impl!($($price_adapter_impl, $id),*);
        ::redstone_testing::test_price_adapter_feed_specific_testcases_impls!($($price_adapter_impl, $id),*);
    };
}

#[macro_export]
macro_rules! test_price_adapter_multi_feed_specific_testcases_impls {
    ($(
        $price_adapter_impl:ty, $id:ident
    ),*) => {
        #[cfg(test)]
        mod feed_test {
            use redstone_testing::env::run_env::PriceAdapterRunEnv;
            use redstone_testing::sample::{
                sample_eth_btc_avax_5sig
            };
            ::redstone_testing::paste::paste! {
                $(
            #[test]
            fn [<test_write_prices_twice_different_ $id>]() {
                let sample = &sample_eth_btc_avax_5sig();
                let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

                sample.test_write_prices(&mut price_adapter, vec!["ETH"].into());
                sample.test_write_prices(&mut price_adapter, vec!["BTC"].into());
            }
        )*}
        }
    };
}

#[macro_export]
macro_rules! test_price_adapter_feed_specific_testcases_impls {
    ($(
        $price_adapter_impl:ty, $id:ident
    ),*) => {
        #[cfg(test)]
        mod multifeed_test {
            use redstone_testing::env::run_env::PriceAdapterRunEnv;
            use redstone_testing::sample::{
                sample_eth_btc_avax_5sig
            };
            ::redstone_testing::paste::paste! {
                $(
            #[test]
            #[should_panic(expected = "Timestamp must be greater than before")]
            fn [<test_write_prices_twice_different_ $id>]() {
                let sample = &sample_eth_btc_avax_5sig();
                let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

                sample.test_write_prices(&mut price_adapter, vec!["ETH"].into());
                sample.test_write_prices(&mut price_adapter, vec!["BTC"].into());
            }
        )*}
        }
    };
}

#[macro_export]
macro_rules! test_price_adapter_instantiate_impl {
    ($(
        $price_adapter_impl:ty, $id:ident
    ),*) => {
        #[cfg(test)]
        mod instantiate_tests {
            use redstone_testing::env::run_env::PriceAdapterRunEnv;

            const SIGNERS: [[u8; 2]; 3] = [[17u8, 17u8], [34u8, 34u8], [51u8, 17u8]];

            fn signers() -> Vec<Vec<u8>> {
                SIGNERS.map(|x| x.to_vec()).to_vec()
            }

        ::redstone_testing::paste::paste! {
        $(
            #[test]
            fn [<test_instantiate_ $id>]() {
                let mut price_adapter =
                    <$price_adapter_impl>::instantiate(3, signers(), None);
            }

            #[test]
            fn [<test_instantiate_signer_count_thresholds_ $id>]() {
                for i in 0u8..4 {
                    let _ = <$price_adapter_impl>::instantiate(
                        i,
                        signers(),
                        None,
                    );
                }
            }

            #[should_panic(expected = "Wrong configuration signer count, got 3 signers, expected at minimum 4")]
            #[test]
            fn [<test_instantiate_wrong_signer_count_threshold_ $id>]() {
                <$price_adapter_impl>::instantiate(4, signers(), None);
            }

            #[should_panic(expected = "Wrong configuration signer count, got 0 signers, expected at minimum 4")]
            #[test]
            fn [<test_instantiate_empty_signers_ $id>]() {
                <$price_adapter_impl>::instantiate(4, vec![], None);
            }
        )*
        }
        }
    };
}

#[macro_export]
macro_rules! test_price_adapter_read_impl {
    ($(
        $price_adapter_impl:ty, $id:ident
    ),*) => {
    #[cfg(test)]
    mod read_tests {
        use redstone_testing::env::run_env::PriceAdapterRunEnv;
        use redstone_testing::sample::{
            sample_eth_btc_avax_5sig, sample_eth_btc_avax_5sig_2, sample_eth_btc_avax_5sig_old,
        };


        ::redstone_testing::paste::paste! {$(
        #[test]
        fn [<test_get_prices_ $id>]() {
            let sample = &sample_eth_btc_avax_5sig();
            let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

            sample.test_get_prices(&mut price_adapter, None);
        }

        #[should_panic(expected = "Missing data feed value for #0 (BTC)")]
        #[test]
        fn [<test_get_prices_not_written_ $id>]() {
            let sample = &sample_eth_btc_avax_5sig();
            let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

            sample.test_get_prices(&mut price_adapter, None);
            sample.verify_written_values(&mut price_adapter, vec!["BTC"].into());
        }

        #[test]
        fn [<test_write_and_get_prices_same_ $id>]() {
            let sample = &sample_eth_btc_avax_5sig();
            let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

            sample.test_write_prices(&mut price_adapter, None);
            sample.test_get_prices(&mut price_adapter, None);
            sample.test_get_prices(&mut price_adapter, vec!["BTC", "ETH"].into());
        }

        #[test]
        fn [<test_write_and_get_prices_different_ $id>]() {
            let sample = &sample_eth_btc_avax_5sig();
            let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

            sample.test_write_prices(&mut price_adapter, None);
            sample_eth_btc_avax_5sig_2().test_get_prices(&mut price_adapter, None);
            sample.verify_written_values(&mut price_adapter, None);
        }

        #[test]
        fn [<test_write_and_get_prices_override_older_ $id>]() {
            let sample = &sample_eth_btc_avax_5sig_2();
            let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

            sample.test_write_prices(&mut price_adapter, None);
            sample_eth_btc_avax_5sig().test_get_prices(&mut price_adapter, None);
            sample.verify_written_values(&mut price_adapter, None);
        }

        #[should_panic(expected = "Timestamp 1725975870000 is too future for #0")]
        #[test]
        fn [<test_get_prices_timestamp_error_ $id>]() {
            let sample = &sample_eth_btc_avax_5sig_old();
            let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

            sample_eth_btc_avax_5sig_2().test_get_prices(&mut price_adapter, None);
        }
        )*}
        }
    };
}

#[macro_export]
macro_rules! test_price_adapter_write_impl {
    ($(
        $price_adapter_impl:ty, $id:ident
    ),*) => {
        #[cfg(test)]
        mod write_tests {
            use redstone_testing::env::run_env::PriceAdapterRunEnv;
            use redstone_testing::{
                hashmap,
                sample::{
                    sample_eth_btc_avax_5sig, sample_eth_btc_avax_5sig_2, sample_eth_btc_avax_5sig_old,
                },
            };
            use redstone_testing::redstone::Value;

            ::redstone_testing::paste::paste! {$(
            #[test]
            fn [<test_write_prices_ $id>]() {
                let sample = &sample_eth_btc_avax_5sig();
                let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

                sample.test_write_prices(&mut price_adapter, None);
            }

            #[should_panic(expected = "Missing data feed value for #0 (AVAX)")]
            #[test]
            fn [<test_write_prices_not_all_ $id>]() {
                let sample = &sample_eth_btc_avax_5sig();
                let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

                sample.test_write_prices(&mut price_adapter, vec!["ETH", "BTC"].into());
                sample.verify_written_values(&mut price_adapter, vec!["BTC"].into());
                sample.verify_written_values(&mut price_adapter, vec!["AVAX"].into());
            }

            #[should_panic(expected = "Timestamp must be greater than before")]
            #[test]
            fn [<test_write_prices_twice_same_ $id>]() {
                let sample = &sample_eth_btc_avax_5sig();
                let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

                sample.test_write_prices(&mut price_adapter, None);
                sample.test_write_prices(&mut price_adapter, None);
            }

            #[test]
            fn [<test_write_prices_override_ $id>]() {
                let sample = &sample_eth_btc_avax_5sig();
                let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

                sample.test_write_prices(&mut price_adapter, None);
                sample_eth_btc_avax_5sig_2().test_write_prices(&mut price_adapter, None);
            }

            #[test]
            fn [<test_write_prices_override_different_ $id>]() {
                let sample = &sample_eth_btc_avax_5sig_2();
                let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

                sample_eth_btc_avax_5sig().test_write_prices(&mut price_adapter, None);
                sample.test_write_prices(&mut price_adapter, vec!["ETH"].into());

                let mut new_sample = sample.clone();

                new_sample.values = hashmap!("ETH" => sample.values["ETH"]);
                new_sample.verify_written_values(&mut price_adapter, None);
            }

            #[should_panic(expected = "Timestamp must be greater than before")]
            #[test]
            fn [<test_write_prices_override_by_older_ $id>]() {
                let sample = &sample_eth_btc_avax_5sig_2();
                let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

                sample.test_write_prices(&mut price_adapter, None);
                sample_eth_btc_avax_5sig().test_write_prices(&mut price_adapter, None);
            }

            #[should_panic(expected = "Timestamp 1725975870000 is too future for #0")]
            #[test]
            fn [<test_write_prices_timestamp_error_ $id>]() {
                let sample = &sample_eth_btc_avax_5sig_old();
                let mut price_adapter: $price_adapter_impl = sample.instantiate_price_adapter();

                sample_eth_btc_avax_5sig_2().test_write_prices(&mut price_adapter, None);
            }
            )*}
        }
    };
}
