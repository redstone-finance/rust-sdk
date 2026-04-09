use alloc::vec::Vec;
use core::mem;

use crate::{
    core::{config::Config, matrix::Matrix, validator::Validator},
    network::error::Error,
    protocol::data_package::DataPackage,
    types::Value,
    utils::median::Median,
    FeedId,
};

type FeedErrors = Vec<Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedResult {
    pub feed: FeedId,
    pub result: Result<Value, FeedErrors>,
}

pub fn process_values(
    config: &Config,
    data_packages: Vec<DataPackage>,
) -> Result<Vec<FeedResult>, Error> {
    let feed_count = config.feed_ids().len();
    let signer_count = config.signers().len();

    let mut feed_errors: Vec<FeedErrors> = vec![vec![]; feed_count];
    let mut matrix = Matrix::<Option<Value>>::new(feed_count, signer_count);

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
            let feed_idx = match config.feed_index(data_point.feed_id) {
                Some(idx) => idx,
                _ => continue,
            };

            if data_point.value.is_zero() {
                feed_errors[feed_idx].push(Error::ZeroDataPoint(data_point.feed_id));

                continue;
            }

            let value = matrix.mut_unchecked_at(feed_idx, signer_idx);

            if value.is_some() {
                return Err(Error::ReoccurringFeedId(data_point.feed_id));
            }

            *value = Some(data_point.value);
        }
    }

    aggregate_values(matrix, config, feed_errors)
}

fn aggregate_values(
    values: Matrix<Option<Value>>,
    config: &Config,
    mut feed_errors: Vec<FeedErrors>,
) -> Result<Vec<FeedResult>, Error> {
    let mut results = vec![];

    for (feed, row) in values.rows_iter().enumerate() {
        let feed_id = config.feed_ids()[feed];
        let feed_values: Vec<_> = row.flatten().copied().collect();

        if feed_values.len() < config.signer_count_threshold() as usize {
            let mut errors = mem::take(&mut feed_errors[feed]);
            errors.push(Error::InsufficientSignerCount(
                feed_values.len(),
                config.signer_count_threshold() as usize,
                feed_id,
            ));

            results.push(FeedResult {
                feed: feed_id,
                result: Err(errors),
            });

            continue;
        }

        let median = match feed_values.median() {
            Some(median) => median,
            _ => {
                let mut errors = mem::take(&mut feed_errors[feed]);
                errors.push(Error::ArrayIsEmpty);

                results.push(FeedResult {
                    feed: feed_id,
                    result: Err(errors),
                });

                continue;
            }
        };

        results.push(FeedResult {
            feed: feed_id,
            result: Ok(median),
        });
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use redstone_utils::hex::make_hex_value_from_string;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::{
        core::{
            aggregator::{process_values, FeedErrors, FeedResult},
            config::Config,
            test_helpers::{
                AVAX, BTC, ETH, TEST_SIGNER_ADDRESS_1, TEST_SIGNER_ADDRESS_2,
                TEST_SIGNER_ADDRESS_3, TEST_SIGNER_ADDRESS_4,
            },
        },
        network::error::Error,
        protocol::data_package::DataPackage,
        types::Value,
        FeedId,
    };

    fn ok_results(results: &[FeedResult]) -> Vec<(FeedId, Value)> {
        results
            .iter()
            .filter_map(|r| r.result.as_ref().ok().map(|v| (r.feed, *v)))
            .collect()
    }

    fn err_feeds(results: &[FeedResult]) -> Vec<FeedId> {
        results
            .iter()
            .filter_map(|r| r.result.as_ref().err().map(|_| r.feed))
            .collect()
    }

    fn all_errors(results: &[FeedResult]) -> Vec<(FeedId, &FeedErrors)> {
        results
            .iter()
            .filter_map(|r| r.result.as_ref().err().map(|e| (r.feed, e)))
            .collect()
    }

    #[test]
    fn test_aggregate_values_basic_single_feed() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 2100, TEST_SIGNER_ADDRESS_2, None),
        ];

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert!(ok.iter().any(|&(_, v)| v == 2050u128.into()));
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

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert!(ok.contains(&(make_hex_value_from_string(ETH), 2050u128.into())));
        assert!(ok.contains(&(make_hex_value_from_string(BTC), 50500u128.into())));
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

        let results = process_values(&config, data_packages).unwrap();

        assert!(ok_results(&results).is_empty());
        assert!(err_feeds(&results).contains(&make_hex_value_from_string(ETH)));
    }

    #[test]
    fn test_aggregate_values_exactly_threshold_signers() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(2));
        let data_packages = vec![
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 2100, TEST_SIGNER_ADDRESS_2, None),
        ];

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert!(ok.contains(&(make_hex_value_from_string(ETH), 2050u128.into())));
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
    fn test_aggregate_values_no_signer_address() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let mut data_package =
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_1, None);
        data_package.signer_address = None;

        let results = process_values(&config, vec![data_package]).unwrap();

        assert!(ok_results(&results).is_empty());
    }

    #[test]
    fn test_aggregate_values_unknown_signer() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![DataPackage::test_single_data_point(
            ETH, 2000, "aaabbb", None,
        )];

        let results = process_values(&config, data_packages).unwrap();

        assert!(ok_results(&results).is_empty());
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

        let results = process_values(&config, data_packages).unwrap();

        assert!(ok_results(&results).is_empty());
    }

    #[test]
    fn test_aggregate_values_zero_values_produce_errors() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![
            DataPackage::test_single_data_point(ETH, 0, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 0, TEST_SIGNER_ADDRESS_2, None),
        ];

        let results = process_values(&config, data_packages).unwrap();
        let errs = all_errors(&results);
        let eth_feed = make_hex_value_from_string(ETH);

        assert!(ok_results(&results).is_empty());

        let (_, eth_errors) = errs.iter().find(|(f, _)| *f == eth_feed).unwrap();
        let zero_count = eth_errors
            .iter()
            .filter(|e| matches!(e, Error::ZeroDataPoint(_)))
            .count();
        assert_eq!(zero_count, 2);
        assert!(eth_errors
            .iter()
            .any(|e| matches!(e, Error::InsufficientSignerCount(0, _, _))));
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

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0].1, 2000u128.into());
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

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0].1, 2500u128.into());
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

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0], (make_hex_value_from_string(ETH), 2050u128.into()));
        assert!(err_feeds(&results).contains(&make_hex_value_from_string(BTC)));
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

        let results = process_values(&config, data_packages).unwrap();

        assert_eq!(ok_results(&results).len(), 2);
    }

    #[test]
    fn test_aggregate_values_empty_data_packages() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));

        let results = process_values(&config, vec![]).unwrap();

        assert!(ok_results(&results).is_empty());
    }

    #[test]
    fn test_aggregate_values_single_signer_multiple_feeds() {
        let config = Config::test_with_signer_count_threshold_or_default(Some(1));
        let data_packages = vec![DataPackage::test_multi_data_point(
            vec![(ETH, 2000), (BTC, 50000)],
            TEST_SIGNER_ADDRESS_1,
            None,
        )];

        let results = process_values(&config, data_packages).unwrap();

        assert_eq!(ok_results(&results).len(), 2);
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

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert_eq!(ok.len(), 3);
        assert_eq!(ok[0], (make_hex_value_from_string(ETH), 2050u128.into()));
        assert_eq!(ok[1], (make_hex_value_from_string(BTC), 50500u128.into()));
        assert_eq!(ok[2], (make_hex_value_from_string(AVAX), 41u128.into()));
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

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert_eq!(ok.len(), 2);
        assert_eq!(ok[0], (make_hex_value_from_string(ETH), 2000u128.into()));
        assert_eq!(ok[1], (make_hex_value_from_string(BTC), 50500u128.into()));
        assert!(err_feeds(&results).contains(&make_hex_value_from_string(AVAX)));
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

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0].1, 2050u128.into());
    }

    // --- new tests ---

    #[test]
    fn test_result_ordering_matches_config_feed_order() {
        let config = Config::test(
            Some(1),
            vec![TEST_SIGNER_ADDRESS_1],
            vec![BTC, AVAX, ETH],
            None,
            None,
            None,
        );
        let data_packages = vec![DataPackage::test_multi_data_point(
            vec![(ETH, 2000), (AVAX, 40), (BTC, 50000)],
            TEST_SIGNER_ADDRESS_1,
            None,
        )];

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert_eq!(ok[0].0, make_hex_value_from_string(BTC));
        assert_eq!(ok[1].0, make_hex_value_from_string(AVAX));
        assert_eq!(ok[2].0, make_hex_value_from_string(ETH));
    }

    #[test]
    fn test_all_feeds_below_threshold() {
        let config = Config::test(
            Some(3),
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

        let results = process_values(&config, data_packages).unwrap();

        assert_eq!(results.len(), 2);
        assert!(ok_results(&results).is_empty());
        assert_eq!(err_feeds(&results).len(), 2);
    }

    #[test]
    fn test_zero_mixed_with_valid_drops_errors() {
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
            DataPackage::test_single_data_point(ETH, 0, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_2, None),
            DataPackage::test_single_data_point(ETH, 3000, TEST_SIGNER_ADDRESS_3, None),
        ];

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0].1, 2500u128.into());
        assert!(err_feeds(&results).is_empty());
    }

    #[test]
    fn test_zero_values_accumulate_with_insufficient_signer_errors() {
        let config = Config::test(
            Some(2),
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
            DataPackage::test_single_data_point(ETH, 0, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 0, TEST_SIGNER_ADDRESS_2, None),
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_3, None),
        ];

        let results = process_values(&config, data_packages).unwrap();
        let errs = all_errors(&results);

        assert_eq!(errs.len(), 1);

        let (_, eth_errors) = &errs[0];
        assert_eq!(
            eth_errors
                .iter()
                .filter(|e| matches!(e, Error::ZeroDataPoint(_)))
                .count(),
            2
        );
        assert!(eth_errors
            .iter()
            .any(|e| matches!(e, Error::InsufficientSignerCount(1, 2, _))));
    }

    #[test]
    fn test_insufficient_signers_error_details() {
        let config = Config::test(
            Some(3),
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
        let data_packages = vec![DataPackage::test_single_data_point(
            ETH,
            2000,
            TEST_SIGNER_ADDRESS_1,
            None,
        )];

        let results = process_values(&config, data_packages).unwrap();
        let [ref feed_result] = results.as_slice() else {
            panic!("expected exactly one result");
        };

        assert_eq!(feed_result.feed, make_hex_value_from_string(ETH));

        let errs = feed_result.result.as_ref().unwrap_err();
        assert!(errs
            .iter()
            .any(|e| matches!(e, Error::InsufficientSignerCount(1, 3, _))));
    }

    #[test]
    fn test_each_feed_gets_a_result_entry() {
        let config = Config::test(
            Some(1),
            vec![TEST_SIGNER_ADDRESS_1],
            vec![ETH, BTC, AVAX],
            None,
            None,
            None,
        );
        let data_packages = vec![DataPackage::test_single_data_point(
            ETH,
            2000,
            TEST_SIGNER_ADDRESS_1,
            None,
        )];

        let results = process_values(&config, data_packages).unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(ok_results(&results).len(), 1);
        assert_eq!(err_feeds(&results).len(), 2);
    }

    #[test]
    fn test_median_unsorted_input() {
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
            DataPackage::test_single_data_point(ETH, 4000, TEST_SIGNER_ADDRESS_1, None),
            DataPackage::test_single_data_point(ETH, 1000, TEST_SIGNER_ADDRESS_2, None),
            DataPackage::test_single_data_point(ETH, 3000, TEST_SIGNER_ADDRESS_3, None),
            DataPackage::test_single_data_point(ETH, 2000, TEST_SIGNER_ADDRESS_4, None),
        ];

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert_eq!(ok[0].1, 2500u128.into());
    }

    #[test]
    fn test_single_signer_single_value() {
        let config = Config::test(
            Some(1),
            vec![TEST_SIGNER_ADDRESS_1],
            vec![ETH],
            None,
            None,
            None,
        );
        let data_packages = vec![DataPackage::test_single_data_point(
            ETH,
            42,
            TEST_SIGNER_ADDRESS_1,
            None,
        )];

        let results = process_values(&config, data_packages).unwrap();
        let ok = ok_results(&results);

        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0].1, 42u128.into());
    }
}
