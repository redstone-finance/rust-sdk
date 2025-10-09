//! Module containing primitives used for verification of the contract data and logic.
//!
//! See
//! * [verify_untrusted_update] - for untrusted updaters
//! * [verify_trusted_update] - for trusted updaters
//! * [verify_signers_config] - verify integrity of the config
//! * [verify_data_staleness] - verify if data on chain is stale
//! * [UpdateTimestampVerifier] - for verifying timestamps with static dispatch between Trusted/Untrusted source

use crate::{
    network::error::Error, utils::slice::check_no_duplicates, SignerAddress, TimestampMillis,
};

/// Timestamp verifier, with variants for trusted/nontrusted updaters.
pub enum UpdateTimestampVerifier {
    Trusted,
    Untrusted,
}

impl UpdateTimestampVerifier {
    /// Checks if the `updater` is in the trusted set.
    ///  If yes, returns `Trusted` variant, `Untrusted` otherwise.
    pub fn verifier<T: PartialEq>(updater: &T, trusted: &[T]) -> Self {
        match trusted.contains(updater) {
            true => UpdateTimestampVerifier::Trusted,
            false => UpdateTimestampVerifier::Untrusted,
        }
    }

    /// For trusted variant see [verify_trusted_update].
    /// For untrusted variant see [verify_untrusted_update].
    pub fn verify_timestamp(
        &self,
        time_now: TimestampMillis,
        last_write_time: Option<TimestampMillis>,
        min_time_between_updates: TimestampMillis,
        last_package_time: Option<TimestampMillis>,
        new_package_time: TimestampMillis,
    ) -> Result<(), Error> {
        match self {
            UpdateTimestampVerifier::Trusted => verify_trusted_update(
                time_now,
                last_write_time,
                last_package_time,
                new_package_time,
            ),
            UpdateTimestampVerifier::Untrusted => verify_untrusted_update(
                time_now,
                last_write_time,
                min_time_between_updates,
                last_package_time,
                new_package_time,
            ),
        }
    }
}

/// MIN_TIME_BETWEEN_UPDATES_FOR_TRUSTED is set to 0,
/// since trusted can update as long as write timestamp is increasing.
const MIN_TIME_BETWEEN_UPDATES_FOR_TRUSTED: TimestampMillis = TimestampMillis::from_millis(0);
/// MAX_SIGNER_COUNT describes maximum number of signers in Config.
const MAX_SIGNER_COUNT: usize = u8::MAX as usize;

/// Verifies if:
/// * if `last_write_time` is not None if between `last_write_time` and `time_now`
///     passed strictly more than `min_time_between_updates`.
pub fn verify_write_timestamp(
    time_now: TimestampMillis,
    last_write_time: Option<TimestampMillis>,
    min_time_between_updates: TimestampMillis,
) -> Result<(), Error> {
    match last_write_time {
        Some(write_time)
            if write_time
                .add(min_time_between_updates)
                .is_same_or_after(time_now) =>
        {
            Err(Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp(time_now, write_time))
        }
        _ => Ok(()),
    }
}

/// Verifies if:
/// * The package timestamp is strictly increasing.
pub fn verify_package_timestamp(
    last_package_time: Option<TimestampMillis>,
    new_package_time: TimestampMillis,
) -> Result<(), Error> {
    match last_package_time {
        Some(last_package_time) if new_package_time.is_same_or_before(last_package_time) => Err(
            Error::DataTimestampMustBeGreaterThanBefore(new_package_time, last_package_time),
        ),
        _ => Ok(()),
    }
}

/// Verifies if:
/// * Package timestamps are strictly increasing
/// * This is the first write or the time between writes is strictly increasing
pub fn verify_trusted_update(
    time_now: TimestampMillis,
    last_write_time: Option<TimestampMillis>,
    last_package_time: Option<TimestampMillis>,
    new_package_time: TimestampMillis,
) -> Result<(), Error> {
    verify_package_timestamp(last_package_time, new_package_time)?;

    verify_write_timestamp(
        time_now,
        last_write_time,
        MIN_TIME_BETWEEN_UPDATES_FOR_TRUSTED,
    )
}

/// Verifies if:
/// * Package timestamps are strictly increasing
/// * This is the first write or the time between writes is strictly greater than `min_time_between_updates`
pub fn verify_untrusted_update(
    time_now: TimestampMillis,
    last_write_time: Option<TimestampMillis>,
    min_time_between_updates: TimestampMillis,
    last_package_time: Option<TimestampMillis>,
    new_package_time: TimestampMillis,
) -> Result<(), Error> {
    verify_package_timestamp(last_package_time, new_package_time)?;

    verify_write_timestamp(time_now, last_write_time, min_time_between_updates)
}

/// Verifies if:
/// * Data is still within its time-to-live period
/// * Current time has not exceeded the staleness threshold (write_time + data_ttl)
pub fn verify_data_staleness(
    write_time: TimestampMillis,
    time_now: TimestampMillis,
    data_ttl: TimestampMillis,
) -> Result<(), Error> {
    let staleness_threshold = write_time.add(data_ttl);
    if staleness_threshold.is_same_or_before(time_now) {
        return Err(Error::DataStaleness {
            time_now,
            write_time,
            staleness_threshold,
        });
    }

    Ok(())
}

/// Verifies if:
/// * signer list is non empty and contains at least `threshold` of elements.
fn verify_signer_count_in_threshold(signers: &[SignerAddress], threshold: u8) -> Result<(), Error> {
    if signers.len() < threshold as usize || signers.is_empty() {
        return Err(Error::ConfigInsufficientSignerCount(
            signers.len() as u8,
            threshold,
        ));
    }

    Ok(())
}

/// Verifies if:
/// * signer list is not larger than max u8 value.
fn verify_signer_count_not_exceeded(signers: &[SignerAddress]) -> Result<(), Error> {
    if signers.len() > MAX_SIGNER_COUNT {
        return Err(Error::ConfigExceededSignerCount(
            signers.len(),
            MAX_SIGNER_COUNT,
        ));
    }

    Ok(())
}

/// Verifies if:
/// * signer list does not contain invalid address.
fn verify_signers_validity(signers: &[SignerAddress]) -> Result<(), Error> {
    for signer in signers {
        if signer.is_zero() {
            return Err(Error::ConfigInvalidSignerAddress(*signer));
        }
    }

    Ok(())
}

/// Verifies if:
/// * signer list contains no duplicates
/// * signer list is non empty and contains at least `threshold` of elements.
/// * signer list is not larger than max u8 value.
/// * signer list does not contain invalid address.
pub fn verify_signers_config(signers: &[SignerAddress], threshold: u8) -> Result<(), Error> {
    verify_signer_count_in_threshold(signers, threshold)?;
    verify_signer_count_not_exceeded(signers)?;
    verify_signers_validity(signers)?;

    check_no_duplicates(signers).map_err(Error::ConfigReoccurringSigner)
}

#[cfg(test)]
mod tests {
    use crate::{
        contract::verification::{
            verify_data_staleness, verify_trusted_update, verify_untrusted_update,
        },
        network::error::Error,
    };

    #[test]
    fn first_write_is_ok() -> Result<(), Error> {
        verify_trusted_update(1000.into(), None, None, 1.into())?;

        verify_untrusted_update(1000.into(), None, 1.into(), None, 1.into())
    }

    #[test]
    fn non_trusted_write_after_wait_time_is_ok() -> Result<(), Error> {
        verify_untrusted_update(1000.into(), Some(900.into()), 99.into(), None, 1.into())
    }

    #[test]
    fn non_trusted_write_before_wait_time_is_err() {
        let res = verify_untrusted_update(999.into(), Some(900.into()), 99.into(), None, 1.into());

        assert_eq!(
            res,
            Err(
                Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp(
                    999.into(),
                    900.into()
                )
            )
        );
    }

    #[test]
    fn trusted_write_before_wait_time_is_ok() -> Result<(), Error> {
        verify_trusted_update(901.into(), Some(900.into()), None, 1.into())
    }

    #[test]
    fn trusted_write_on_current_time_is_err() {
        let res = verify_trusted_update(900.into(), Some(900.into()), None, 1.into());

        assert_eq!(
            res,
            Err(
                Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp(
                    900.into(),
                    900.into()
                )
            )
        );
    }

    #[test]
    fn verify_package_timestamp_increase_is_ok() -> Result<(), Error> {
        verify_trusted_update(902.into(), Some(900.into()), Some(0.into()), 1.into())?;
        verify_untrusted_update(
            902.into(),
            Some(900.into()),
            1.into(),
            Some(0.into()),
            1.into(),
        )
    }

    #[test]
    fn verify_package_timestamp_non_increase_is_err() {
        let res = verify_trusted_update(901.into(), Some(900.into()), Some(1.into()), 1.into());
        assert_eq!(
            res,
            Err(Error::DataTimestampMustBeGreaterThanBefore(
                1.into(),
                1.into()
            ))
        );

        let res = verify_untrusted_update(
            901.into(),
            Some(900.into()),
            1.into(),
            Some(1.into()),
            1.into(),
        );
        assert_eq!(
            res,
            Err(Error::DataTimestampMustBeGreaterThanBefore(
                1.into(),
                1.into()
            ))
        );
    }
    #[test]
    fn data_is_fresh_when_within_ttl() {
        let wrote_at = 1704096000000.into();
        let ttl_30min = 1800000.into();
        let time_now = 1704097200000.into();

        let result = verify_data_staleness(wrote_at, time_now, ttl_30min);

        assert!(result.is_ok());
    }

    #[test]
    fn data_is_stale_when_exactly_at_threshold() {
        let wrote_at = 1704096000000.into();
        let ttl_30min = 1800000.into();
        let time_now = 1704097800000.into();

        let result = verify_data_staleness(wrote_at, time_now, ttl_30min);

        assert!(result.is_err());
    }

    #[test]
    fn data_is_stale_when_past_threshold() {
        let wrote_at = 1704096000000.into();
        let ttl_30min = 1800000.into();
        let time_now = 1704098100000.into();

        let result = verify_data_staleness(wrote_at, time_now, ttl_30min);

        assert!(result.is_err());
    }

    #[test]
    fn error_contains_correct_timestamps() {
        let wrote_at = 1704096000000.into();
        let ttl_30min = 1800000.into();
        let time_now = 1704097800000.into();
        let staleness_threshold = 1704097800000.into();

        let result = verify_data_staleness(wrote_at, time_now, ttl_30min);
        let expected_error = Error::DataStaleness {
            time_now,
            write_time: wrote_at,
            staleness_threshold,
        };

        assert_eq!(result.unwrap_err(), expected_error);
    }

    #[test]
    fn zero_ttl_makes_data_immediately_stale() {
        let wrote_at = 1704096000000.into();
        let ttl_zero = 0.into();
        let time_now = 1704096000000.into();

        let result = verify_data_staleness(wrote_at, time_now, ttl_zero);

        assert!(result.is_err());
    }

    #[test]
    fn large_ttl_keeps_data_fresh() {
        let wrote_at = 1704096000000.into();
        let large_ttl = (u64::MAX / 2).into();
        let time_now = 1704182400000.into();

        let result = verify_data_staleness(wrote_at, time_now, large_ttl);

        assert!(result.is_ok());
    }

    #[test]
    fn large_ttl_does_not_overflow() {
        let wrote_at = 1704096000000.into();
        let ttl_max = u64::MAX.into();
        let time_now = 1704182400000.into();

        let result = verify_data_staleness(wrote_at, time_now, ttl_max);

        assert!(result.is_ok());
    }

    #[test]
    fn data_fresh_immediately_after_write() {
        let wrote_at = 1704096000000.into();
        let ttl_30min = 1800000.into();
        let time_now = 1704096000000.into();

        let result = verify_data_staleness(wrote_at, time_now, ttl_30min);

        assert!(result.is_ok());
    }

    #[test]
    fn boundary_condition_one_millisecond_before_expiry() {
        let wrote_at = 1704096000000.into();
        let ttl_30min = 1800000.into();
        let time_now = 1704097799999.into();

        let result = verify_data_staleness(wrote_at, time_now, ttl_30min);

        assert!(result.is_ok());
    }

    #[test]
    fn boundary_condition_one_millisecond_after_expiry() {
        let wrote_at = 1704096000000.into();
        let ttl_30min = 1800000.into();
        let time_now = 1704097800001.into();

        let result = verify_data_staleness(wrote_at, time_now, ttl_30min);

        assert!(result.is_err());
    }
}
