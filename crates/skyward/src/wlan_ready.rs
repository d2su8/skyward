//! Wait for an interface whose firmware is still finishing initialization.

use anyhow::Result;
use std::io::{Error, ErrorKind};
use std::thread;
use std::time::{Duration, Instant};

/// Retry only the driver's busy response. Other failures retain their
/// original context, and a driver that never becomes ready has a deadline.
pub fn enable(mut set_up: impl FnMut() -> Result<()>, timeout: Duration) -> Result<()> {
    let until = Instant::now() + timeout;
    loop {
        match set_up() {
            Err(e) if e.downcast_ref::<Error>().is_some_and(|e| e.kind() == ErrorKind::ResourceBusy)
                && Instant::now() < until => {
                thread::sleep(Duration::from_millis(100).min(until.saturating_duration_since(Instant::now())));
            }
            result => return result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization_busy_is_retried_until_the_interface_is_ready() {
        let mut attempts = 0;
        enable(|| {
            attempts += 1;
            if attempts == 1 {
                return Err(anyhow::Error::from(Error::from_raw_os_error(libc::EBUSY)).context("bring wlan0 up"));
            }
            Ok(())
        }, Duration::from_secs(1)).unwrap();
        assert_eq!(attempts, 2);
    }

    #[test]
    fn a_permanent_failure_is_returned_without_retrying() {
        let mut attempts = 0;
        let error = enable(|| {
            attempts += 1;
            Err(anyhow::Error::from(Error::from(ErrorKind::PermissionDenied)).context("bring wlan0 up"))
        }, Duration::from_secs(1)).unwrap_err();
        assert_eq!(attempts, 1);
        assert_eq!(error.downcast_ref::<Error>().unwrap().kind(), ErrorKind::PermissionDenied);
        assert!(error.to_string().contains("wlan0"));
    }

    #[test]
    fn an_expired_deadline_returns_busy_without_waiting() {
        let mut attempts = 0;
        let error = enable(|| {
            attempts += 1;
            Err(Error::from(ErrorKind::ResourceBusy).into())
        }, Duration::ZERO).unwrap_err();
        assert_eq!(attempts, 1);
        assert_eq!(error.downcast_ref::<Error>().unwrap().kind(), ErrorKind::ResourceBusy);
    }

    #[test]
    fn a_driver_that_stays_busy_eventually_stops_retrying() {
        let mut attempts = 0;
        let error = enable(|| {
            attempts += 1;
            Err(Error::from(ErrorKind::ResourceBusy).into())
        }, Duration::from_millis(10)).unwrap_err();
        assert!(attempts >= 1);
        assert_eq!(error.downcast_ref::<Error>().unwrap().kind(), ErrorKind::ResourceBusy);
    }
}
