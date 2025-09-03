use alloc::vec::Vec;

use crate::{
    core::{config::Config, validator::Validator},
    network::error::Error,
    protocol::data_package::DataPackage,
    types::Value,
    utils::median::Median,
    FeedId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedValue {
    pub feed: FeedId,
    pub value: Value,
}

pub fn process_values(
    config: &Config,
    data_packages: Vec<DataPackage>,
) -> Result<Vec<FeedValue>, Error> {
    let feed_count = config.feed_ids().len();
    let signer_count = config.signers().len();
    let size = feed_count * signer_count;

    let mut values = vec![None; size];

    for data_package in data_packages {
        let signer_address = match data_package.signer_address.as_ref() {
            Some(address) => address,
            _ => continue,
        };

        let signer_idx = match config.signer_index(signer_address) {
            Some(idx) => idx,
            _ => continue,
        };

        for data_point in data_package.data_points {
            if data_point.value.is_zero() {
                continue;
            }

            let feed_idx = match config.feed_index(data_point.feed_id) {
                Some(idx) => idx,
                _ => continue,
            };

            if values[feed_idx * signer_count + signer_idx].is_some() {
                return Err(Error::ReoccurringFeedId(data_point.feed_id));
            }

            values[feed_idx * signer_count + signer_idx] = Some(data_point.value);
        }
    }

    aggregate_values(values, config)
}

fn aggregate_values(values: Vec<Option<Value>>, config: &Config) -> Result<Vec<FeedValue>, Error> {
    let feed_count = config.feed_ids().len();
    let signer_count = config.signers().len();

    let mut result = vec![];

    for feed in 0..feed_count {
        let feed_values: Vec<_> = values
            .iter()
            .skip(feed * signer_count)
            .take(signer_count)
            .flatten()
            .map(|v| v.to_u256())
            .collect();

        if feed_values.len() < config.signer_count_threshold() as usize {
            continue;
        }

        let median = match feed_values.median() {
            Some(median) => median,
            _ => continue,
        };

        result.push(FeedValue {
            feed: config.feed_ids()[feed],
            value: Value::from_u256(median),
        });
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use redstone_utils::hex::make_hex_value_from_string;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::{
        core::{
            aggregator::process_values,
            config::Config,
            test_helpers::{
                AVAX, BTC, ETH, TEST_SIGNER_ADDRESS_1, TEST_SIGNER_ADDRESS_2,
                TEST_SIGNER_ADDRESS_3, TEST_SIGNER_ADDRESS_4,
            },
        },
        network::error::Error,
        protocol::data_package::DataPackage,
    };

    #[test]
    fn test_aggregate_values_basic_single_feed() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 2100, TEST_SIGNER_ADDRESS_2, None),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, (2050u128).into());
    }

    #[test]
    fn test_aggregate_values_multiple_feeds() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![
            DataPackage::test_multi_data_point(
                vec![(ETH, 2000), (BTC, 50000)],
                TEST_SIGNER_ADDRESS_1,
                None,
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 2100), (BTC, 51000)],
                TEST_SIGNER_ADDRESS_2,
                None,
            ),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 2);

        let eth_value = result[0];
        let btc_value = result[1];

        assert_eq!(eth_value.feed, make_hex_value_from_string(ETH));
        assert_eq!(btc_value.feed, make_hex_value_from_string(BTC));
        assert_eq!(eth_value.value, (2050u128).into());
        assert_eq!(btc_value.value, (50500u128).into());
    }

    #[test]
    fn test_aggregate_values_insufficient_signers() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(2));
        let data_packages = vec![DataPackage::test_single_data_point(
            ETH,
            2000,
            TEST_SIGNER_ADDRESS_1,
            None,
        )];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_aggregate_values_exactly_threshold_signers() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(2));
        let data_packages = vec![
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 2100, TEST_SIGNER_ADDRESS_2, None),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, (2050u128).into());
    }

    #[test]
    fn test_aggregate_values_reoccurring_feed_id_error() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 2100, TEST_SIGNER_ADDRESS_1, None),
        ];

        let result = process_values(&config, data_packages);

        assert!(matches!(result, Err(Error::ReoccurringFeedId(_))));
    }

    #[test]
    fn test_aggregate_values_invalid_signer_address() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let mut data_package =
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_1, None);
        data_package.signer_address = None;
        let data_packages = vec![data_package];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_aggregate_values_unknown_signer() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![DataPackage::test_single_data_point(
            ETH, 2000, "aaabbb", None,
        )];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_aggregate_values_unknown_feed_id() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![DataPackage::test_single_data_point(
            "UNKNOWN",
            2000,
            TEST_SIGNER_ADDRESS_1,
            None,
        )];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_aggregate_values_zero_median_filtered_out() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![
            DataPackage::test_single_data_point(ETH, 0, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 0, TEST_SIGNER_ADDRESS_2, None),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_aggregate_values_median_calculation_odd_count() {
        let config = Config::test(
            Some(1),
            vec![
                TEST_SIGNER_ADDRESS_1,
                TEST_SIGNER_ADDRESS_2,
                TEST_SIGNER_ADDRESS_3,
            ],
            vec![ETH],
            None,
            None,
            None,
        );
        let data_packages = vec![
            DataPackage::test_single_data_point(ETH, 1000, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_2, None),
            DataPackage::test_single_data_point(ETH, 3000, TEST_SIGNER_ADDRESS_3, None),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, (2000u128).into());
    }

    #[test]
    fn test_aggregate_values_median_calculation_even_count() {
        let config = Config::test(
            Some(1),
            vec![
                TEST_SIGNER_ADDRESS_1,
                TEST_SIGNER_ADDRESS_2,
                TEST_SIGNER_ADDRESS_3,
                TEST_SIGNER_ADDRESS_4,
            ],
            vec![ETH],
            None,
            None,
            None,
        );
        let data_packages = vec![
            DataPackage::test_single_data_point(ETH, 1000, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_2, None),
            DataPackage::test_single_data_point(ETH, 3000, TEST_SIGNER_ADDRESS_3, None),
            DataPackage::test_single_data_point(ETH, 4000, TEST_SIGNER_ADDRESS_4, None),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, (2500u128).into());
    }

    #[test]
    fn test_aggregate_values_partial_signer_coverage() {
        let config = Config::test(
            Some(2),
            vec![
                TEST_SIGNER_ADDRESS_1,
                TEST_SIGNER_ADDRESS_2,
                TEST_SIGNER_ADDRESS_3,
            ],
            vec![ETH, BTC],
            None,
            None,
            None,
        );
        let data_packages = vec![
            DataPackage::test_multi_data_point(
                vec![(ETH, 2000), (BTC, 50000)],
                TEST_SIGNER_ADDRESS_1,
                None,
            ),
            DataPackage::test_single_data_point(ETH, 2100, TEST_SIGNER_ADDRESS_2, None),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 1);

        let eth_value = result[0];
        assert_eq!(eth_value.feed, make_hex_value_from_string(ETH));
        assert_eq!(eth_value.value, (2050u128).into());
    }

    #[test]
    fn test_aggregate_values_mixed_valid_invalid_data() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let mut invalid_package =
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_1, None);
        invalid_package.signer_address = None;

        let data_packages = vec![
            invalid_package,
            DataPackage::test_single_data_point("UNKNOWN", 1500, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 2100, TEST_SIGNER_ADDRESS_2, None),
            DataPackage::test_single_data_point(BTC, 51000, TEST_SIGNER_ADDRESS_1, None),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_aggregate_values_empty_data_packages() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_aggregate_values_single_signer_multiple_feeds() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![DataPackage::test_multi_data_point(
            vec![(ETH, 2000), (BTC, 50000)],
            TEST_SIGNER_ADDRESS_1,
            None,
        )];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_aggregate_values_all_feed_types() {
        let config = Config::test(
            Some(1),
            vec![TEST_SIGNER_ADDRESS_1, TEST_SIGNER_ADDRESS_2],
            vec![ETH, BTC, AVAX],
            None,
            None,
            None,
        );
        let data_packages = vec![
            DataPackage::test_multi_data_point(
                vec![(ETH, 2000), (BTC, 50000), (AVAX, 40)],
                TEST_SIGNER_ADDRESS_1,
                None,
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 2100), (BTC, 51000), (AVAX, 42)],
                TEST_SIGNER_ADDRESS_2,
                None,
            ),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 3);

        let eth_result = result[0];
        let btc_result = result[1];
        let avax_result = result[2];

        assert_eq!(eth_result.feed, make_hex_value_from_string(ETH));
        assert_eq!(btc_result.feed, make_hex_value_from_string(BTC));
        assert_eq!(avax_result.feed, make_hex_value_from_string(AVAX));
        assert_eq!(eth_result.value, (2050u128).into());
        assert_eq!(btc_result.value, (50500u128).into());
        assert_eq!(avax_result.value, (41u128).into());
    }

    #[test]
    fn test_aggregate_values_complex_scenario() {
        let config = Config::test(
            Some(3),
            vec![
                TEST_SIGNER_ADDRESS_1,
                TEST_SIGNER_ADDRESS_2,
                TEST_SIGNER_ADDRESS_3,
                TEST_SIGNER_ADDRESS_4,
            ],
            vec![ETH, BTC, AVAX],
            None,
            None,
            None,
        );

        let data_packages = vec![
            DataPackage::test_multi_data_point(
                vec![(ETH, 2000), (BTC, 50000), (AVAX, 20)],
                TEST_SIGNER_ADDRESS_1,
                None,
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 2100), (BTC, 51000)],
                TEST_SIGNER_ADDRESS_2,
                None,
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 1950), (AVAX, 22)],
                TEST_SIGNER_ADDRESS_3,
                None,
            ),
            DataPackage::test_single_data_point(BTC, 50500, TEST_SIGNER_ADDRESS_4, None),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 2);

        let eth_value = result[0];
        let btc_value = result[1];

        assert_eq!(eth_value.feed, make_hex_value_from_string(ETH));
        assert_eq!(btc_value.feed, make_hex_value_from_string(BTC));

        assert_eq!(eth_value.value, (2000u128).into());
        assert_eq!(btc_value.value, (50500u128).into());
    }

    #[test]
    fn test_aggregate_values_extreme_threshold() {
        let config = Config::test(
            Some(4),
            vec![
                TEST_SIGNER_ADDRESS_1,
                TEST_SIGNER_ADDRESS_2,
                TEST_SIGNER_ADDRESS_3,
                TEST_SIGNER_ADDRESS_4,
            ],
            vec![ETH],
            None,
            None,
            None,
        );
        let data_packages = vec![
            DataPackage::test_single_data_point(ETH, 1900, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_2, None),
            DataPackage::test_single_data_point(ETH, 2100, TEST_SIGNER_ADDRESS_3, None),
            DataPackage::test_single_data_point(ETH, 2200, TEST_SIGNER_ADDRESS_4, None),
        ];

        let result = process_values(&config, data_packages).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, (2050u128).into());
    }
}
