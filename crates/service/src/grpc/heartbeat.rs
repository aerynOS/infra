//! Heartbeats for grpc streams
use std::{pin::Pin, time::Duration};

use rand::Rng as _;
use tokio::time::{self, Instant, Sleep};

/// Average heartbeat interval
///
/// Proxies between a client and server can abort a stream whose request
/// body stalls (Caddy does after 1 minute by default), so this must stay
/// well under any such timeout.
pub const INTERVAL: Duration = Duration::from_secs(30);

/// Ticks every [`INTERVAL`], jittered by +/- 10%
pub struct Heartbeat {
    sleep: Pin<Box<Sleep>>,
}

impl Heartbeat {
    /// Returns a new [`Heartbeat`]
    pub fn new() -> Self {
        Self {
            sleep: Box::pin(time::sleep_until(Instant::now())),
        }
    }

    /// Resolves at the next tick
    pub async fn tick(&mut self) {
        (&mut self.sleep).await;

        let jitter = rand::thread_rng().gen_range(0.9..=1.1);

        self.sleep.as_mut().reset(Instant::now() + INTERVAL.mul_f64(jitter));
    }
}

impl Default for Heartbeat {
    fn default() -> Self {
        Self::new()
    }
}
