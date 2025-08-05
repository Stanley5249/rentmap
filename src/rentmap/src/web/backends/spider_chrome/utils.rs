use std::fmt::Debug;
use std::time::Duration;

use futures::StreamExt;
use spider_chrome::Page as ChromiumPage;
use spider_chrome::cdp::IntoEventKind;
use spider_chrome::cdp::browser_protocol::network::EventLoadingFinished;
use spider_chrome::error::CdpError;
use tracing::trace;

#[allow(dead_code)]
pub async fn wait_for_event<T>(
    page: &ChromiumPage,
    timeout_duration: Duration,
) -> Result<(), CdpError>
where
    T: Debug + Unpin + IntoEventKind,
{
    let mut events = page.event_listener::<T>().await?;

    let future = async {
        match events.next().await {
            Some(event) => trace!(?event, "received event"),
            None => trace!("event stream closed before receiving event"),
        };
    };

    if (tokio::time::timeout(timeout_duration, future).await).is_err() {
        trace!("timed out waiting for event");
    }

    Ok(())
}

/// Wait for network to be idle (no network events for `network_idle_duration`) or timeout after `timeout_duration`.
pub async fn wait_for_network_idle(
    page: &ChromiumPage,
    idle_duration: Duration,
    timeout_duration: Duration,
) -> Result<(), CdpError> {
    let mut events = page.event_listener::<EventLoadingFinished>().await?;

    let future = async {
        loop {
            tokio::select! {
                _ = tokio::time::sleep(idle_duration) => {
                    trace!(duration = ?idle_duration, "network idle");
                    break;
                },
                Some(event) = events.next() => {
                    trace!(?event, "received network event");
                },
                else => {
                    trace!("event stream closed before network idle");
                    break;
                },
            }
        }
    };

    if (tokio::time::timeout(timeout_duration, future).await).is_err() {
        trace!("timed out waiting for network idle");
    }

    Ok(())
}
