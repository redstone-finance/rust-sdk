//! Module containing primitives used for verification of the contract data and logic.
//!
//! See
//! * [verify_untrusted_update] - for untrusted updaters
//! * [verify_trusted_update] - for trusted updaters
//! * [verify_signers_config] - verify integrity of the config
//! * [UpdateTimestampVerifier] - for verifying timestamps with static dispatch between Trusted/Untrusted source.

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
        last_package_time: TimestampMillis,
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
/// passed strictly more than `min_time_between_updates`.
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
            Err(Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp)
        }
        _ => Ok(()),
    }
}

/// Verifies if:
/// * The package timestamp is strictly increasing.
pub fn verify_package_timestamp(
    last_package_time: TimestampMillis,
    new_package_time: TimestampMillis,
) -> Result<(), Error> {
    if new_package_time.is_same_or_before(last_package_time) {
        return Err(Error::TimestampMustBeGreaterThanBefore);
    }

    Ok(())
}

/// Verifies if:
/// * Package timestamps are strictly increasing
/// * This is the first write or the time between writes is strictly increasing
pub fn verify_trusted_update(
    time_now: TimestampMillis,
    last_write_time: Option<TimestampMillis>,
    last_package_time: TimestampMillis,
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
    last_package_time: TimestampMillis,
    new_package_time: TimestampMillis,
) -> Result<(), Error> {
    verify_package_timestamp(last_package_time, new_package_time)?;

    verify_write_timestamp(time_now, last_write_time, min_time_between_updates)
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
/// * signer list contains no duplicates
/// * signer list is non empty and contains at least `threshold` of elements.
/// * signer list is not larger than max u8 value.
pub fn verify_signers_config(signers: &[SignerAddress], threshold: u8) -> Result<(), Error> {
    verify_signer_count_in_threshold(signers, threshold)?;
    verify_signer_count_not_exceeded(signers)?;

    check_no_duplicates(signers).map_err(Error::ConfigReocuringSigner)
}

#[cfg(test)]
mod tests {
    use crate::{
        contract::verification::{verify_trusted_update, verify_untrusted_update},
        network::error::Error,
    };

    #[test]
    fn first_write_is_ok() -> Result<(), Error> {
        verify_trusted_update(1000.into(), None, 0.into(), 1.into())?;

        verify_untrusted_update(1000.into(), None, 1.into(), 0.into(), 1.into())
    }

    #[test]
    fn non_trusted_write_after_wait_time_is_ok() -> Result<(), Error> {
        verify_untrusted_update(1000.into(), Some(900.into()), 99.into(), 0.into(), 1.into())
    }

    #[test]
    fn non_trusted_write_before_wait_time_is_err() {
        let res =
            verify_untrusted_update(999.into(), Some(900.into()), 99.into(), 0.into(), 1.into());

        assert_eq!(
            res,
            Err(Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp)
        );
    }

    #[test]
    fn trusted_write_before_wait_time_is_ok() -> Result<(), Error> {
        verify_trusted_update(901.into(), Some(900.into()), 0.into(), 1.into())
    }

    #[test]
    fn trusted_write_on_current_time_is_err() {
        let res = verify_trusted_update(900.into(), Some(900.into()), 0.into(), 1.into());

        assert_eq!(
            res,
            Err(Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp)
        );
    }

    #[test]
    fn verify_package_timestamp_increase_is_ok() -> Result<(), Error> {
        verify_trusted_update(902.into(), Some(900.into()), 0.into(), 1.into())?;
        verify_untrusted_update(902.into(), Some(900.into()), 1.into(), 0.into(), 1.into())
    }

    #[test]
    fn verify_package_timestamp_non_increase_is_err() {
        let res = verify_trusted_update(901.into(), Some(900.into()), 1.into(), 1.into());
        assert_eq!(res, Err(Error::TimestampMustBeGreaterThanBefore));

        let res =
            verify_untrusted_update(901.into(), Some(900.into()), 1.into(), 1.into(), 1.into());
        assert_eq!(res, Err(Error::TimestampMustBeGreaterThanBefore));
    }
}
