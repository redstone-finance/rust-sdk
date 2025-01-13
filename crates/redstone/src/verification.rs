use crate::{network::error::Error, TimestampMillis};

/// Verify if the update performed adheres to the logic:
/// If `price_write_timestamp` is None variant, update is always fine - it represent first write and it is always ok.
/// Otherwise, the `time_now` must be strictly greater from `write_time`.
/// If `updater` is not in the `trusted_updaters` set then the update also need to happen after at least `min_time_between_updates`.
///
/// # Arguments
///
/// * `updater` - Address which want to update the data.
/// * `trusted_updaters` - Set of trusted addressess.
/// * `price_write_timestamp` - Last write time to the data.
/// * `time_now` - Time of the current attempted update.
/// * `min_time_between_updates` - Minimal amount of time that must pass between non-trusted updates.
///
/// # Returns
///
/// Returns a `Result<(), Error>`, Ok in case it is fine to perform the update.
pub fn verify_update<T: PartialEq>(
    updater: &T,
    trusted_updaters: &[T],
    price_write_timestamp: Option<TimestampMillis>,
    time_now: TimestampMillis,
    min_time_between_updates: TimestampMillis,
) -> Result<(), Error> {
    let write_time = match price_write_timestamp {
        Some(write_time) if write_time.is_after(time_now) => {
            return Err(Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp)
        }
        Some(write_time) => write_time,
        _ => return Ok(()), // first write,
    };

    if trusted_updaters.contains(updater) {
        return Ok(()); // no further checks required
    }

    let allowed_untrusted_updated_time = write_time.add(min_time_between_updates);

    if allowed_untrusted_updated_time.is_after(time_now) {
        return Err(Error::TimestampMustBeGreaterThanBefore);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        network::error::Error, verification::verify_update, SignerAddress, TimestampMillis,
    };

    fn trusted_updaters() -> Vec<SignerAddress> {
        vec![vec![1].into(), vec![2].into(), vec![3].into()]
    }

    fn non_trusted() -> SignerAddress {
        vec![0].into()
    }

    fn trusted() -> SignerAddress {
        vec![1].into()
    }

    struct TestCaseCommon {
        pub time_now: u64,
        pub min_time_between_updates: u64,
        pub trusted_updaters: Vec<SignerAddress>,
    }

    impl TestCaseCommon {
        fn common() -> TestCaseCommon {
            TestCaseCommon {
                time_now: 1_000,
                min_time_between_updates: 10,
                trusted_updaters: trusted_updaters(),
            }
        }

        fn check_with(
            &self,
            updater: &SignerAddress,
            price_write_timestamp: Option<TimestampMillis>,
        ) -> Result<(), Error> {
            verify_update(
                updater,
                &self.trusted_updaters,
                price_write_timestamp,
                self.time_now.into(),
                self.min_time_between_updates.into(),
            )
        }
    }

    #[test]
    fn first_write_is_ok() -> Result<(), Error> {
        TestCaseCommon::common().check_with(&trusted(), None)?;
        TestCaseCommon::common().check_with(&non_trusted(), None)
    }

    #[test]
    fn non_trusted_write_after_wait_time_is_ok() -> Result<(), Error> {
        let common = TestCaseCommon::common();

        common.check_with(
            &non_trusted(),
            Some((common.time_now - common.min_time_between_updates - 1).into()),
        )
    }

    #[test]
    fn non_trusted_write_before_wait_time_is_err() {
        let common = TestCaseCommon::common();

        let res = common.check_with(
            &non_trusted(),
            Some((common.time_now - common.min_time_between_updates).into()),
        );

        assert_eq!(res, Err(Error::TimestampMustBeGreaterThanBefore));
    }

    #[test]
    fn trusted_write_before_wait_time_is_ok() -> Result<(), Error> {
        let common = TestCaseCommon::common();

        common.check_with(
            &trusted(),
            Some((common.time_now - common.min_time_between_updates).into()),
        )
    }

    #[test]
    fn trusted_write_on_current_time_is_err() {
        let common = TestCaseCommon::common();

        let res = common.check_with(&trusted(), Some(common.time_now.into()));

        assert_eq!(
            res,
            Err(Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp)
        );
    }

    #[test]
    fn non_trusted_write_on_current_time_is_err() {
        let common = TestCaseCommon::common();

        let res = common.check_with(&non_trusted(), Some(common.time_now.into()));

        assert_eq!(
            res,
            Err(Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp)
        );
    }
}
