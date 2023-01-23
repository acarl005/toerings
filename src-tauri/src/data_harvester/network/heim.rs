//! Gets network data via heim.

use std::time::Instant;

use super::NetworkHarvest;

// TODO: Eventually make it so that this thing also takes individual usage into account, so we can show per-interface!
pub async fn get_network_data(
    prev_net_access_time: Instant,
    prev_net_rx: &mut u64,
    prev_net_tx: &mut u64,
    curr_time: Instant,
) -> crate::utils::error::Result<Option<NetworkHarvest>> {
    use futures::StreamExt;

    let io_data = heim::net::io_counters().await?;
    futures::pin_mut!(io_data);
    let mut total_rx: u64 = 0;
    let mut total_tx: u64 = 0;

    while let Some(io) = io_data.next().await {
        if let Ok(io) = io {
            // TODO: Use bytes as the default instead, perhaps?
            // Since you might have to do a double conversion (bytes -> bits -> bytes) in some cases;
            // but if you stick to bytes, then in the bytes, case, you do no conversion, and in the bits case,
            // you only do one conversion...
            total_rx += io.bytes_recv().get::<heim::units::information::bit>();
            total_tx += io.bytes_sent().get::<heim::units::information::bit>();
        }
    }

    let elapsed_time = curr_time.duration_since(prev_net_access_time).as_secs_f64();

    let (rx, tx) = if elapsed_time == 0.0 {
        (0, 0)
    } else {
        (
            ((total_rx.saturating_sub(*prev_net_rx)) as f64 / elapsed_time) as u64,
            ((total_tx.saturating_sub(*prev_net_tx)) as f64 / elapsed_time) as u64,
        )
    };

    *prev_net_rx = total_rx;
    *prev_net_tx = total_tx;
    Ok(Some(NetworkHarvest {
        rx,
        tx,
        total_rx,
        total_tx,
    }))
}
