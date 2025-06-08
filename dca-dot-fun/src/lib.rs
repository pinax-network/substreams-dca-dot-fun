use common::logs_with_caller;
use proto::pb::dca::dot::fun::v1 as dca_dot_fun;
use substreams_abis::evm::dca_dot_fun::events;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<dca_dot_fun::Events, substreams::errors::Error> {
    let mut events = dca_dot_fun::Events::default();

    for trx in block.transactions() {
        for (log, caller) in logs_with_caller(&block, trx) {
            // -- FillOrder --
            if let Some(event) = events::FillOrder::match_and_decode(log) {
                events.fill_orders.push(dca_dot_fun::FillOrder {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    order_id: event.order_id.to_string(),
                    recipient: event.recipient.to_vec(),
                    fill_amount: event.fill_amount.to_string(),
                    amount_of_token_out: event.amount_of_token_out.to_string(),
                    protocol_fee: event.protocol_fee.to_u64(),
                    token_in_price: event.token_in_price.to_string(),
                    token_out_price: event.token_out_price.to_string(),
                    scaling_factor: event.scaling_factor.to_string(),
                });
            }

            // -- CreateOrder --
            if let Some(event) = events::CreateOrder::match_and_decode(log) {
                events.create_orders.push(dca_dot_fun::CreateOrder {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    order_id: event.order_id.to_string(),
                    creator: event.creator.to_vec(),
                    recipient: event.recipient.to_vec(),
                    token_in: event.token_in.to_vec(),
                    token_out: event.token_out.to_vec(),
                    spend_amount: event.spend_amount.to_string(),
                    repeats: event.repeats.to_string(),
                    slippage: event.slippage.to_string(),
                    freq_interval: event.freq_interval.to_string(),
                    scaling_interval: event.scaling_interval.to_string(),
                    last_run: event.last_run.to_string(),
                    protocol_fee: event.protocol_fee.to_u64(),
                    vault: event.vault.to_vec(),
                    stake_asset_in: event.stake_asset_in,
                    stake_asset_out: event.stake_asset_out,
                });
            }

            // -- CancelOrder --
            if let Some(event) = events::CancelOrder::match_and_decode(log) {
                events.cancel_orders.push(dca_dot_fun::CancelOrder {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    order_id: event.order_id.to_string(),
                    vault: event.vault.to_vec(),
                });
            }

            // -- PauseCreate --
            if let Some(event) = events::PauseCreate::match_and_decode(log) {
                events.cancel_orders.push(dca_dot_fun::PauseCreate {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    is_paused: event.is_paused,
                });
            }

            // -- PauseFill --
            if let Some(event) = events::PauseFill::match_and_decode(log) {
                events.cancel_orders.push(dca_dot_fun::PauseFill {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    is_paused: event.is_paused,
                });
            }
        }
    }

    Ok(events)
}
