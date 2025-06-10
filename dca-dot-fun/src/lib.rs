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
            // ────────────────────── Order-Level Events ──────────────────────
            // -- FillOrder --
            if let Some(event) = events::FillOrder::match_and_decode(log) {
                events.fill_order.push(dca_dot_fun::FillOrder {
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
                    protocol_fee: event.protocol_fee.to_string(),
                    token_in_price: event.token_in_price.to_string(),
                    token_out_price: event.token_out_price.to_string(),
                    scaling_factor: event.scaling_factor.to_string(),
                });
            }

            // -- CreateOrder --
            if let Some(event) = events::CreateOrder::match_and_decode(log) {
                events.create_order.push(dca_dot_fun::CreateOrder {
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
                    protocol_fee: event.protocol_fee.to_string(),
                    vault: event.vault.to_vec(),
                    stake_asset_in: event.stake_asset_in,
                    stake_asset_out: event.stake_asset_out,
                });
            }

            // -- CancelOrder --
            if let Some(event) = events::CancelOrder::match_and_decode(log) {
                events.cancel_order.push(dca_dot_fun::CancelOrder {
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

            // ───────────────── Pause / Circuit-Breaker ─────────────────
            // -- PauseCreate --
            if let Some(event) = events::PauseCreate::match_and_decode(log) {
                events.pause_create.push(dca_dot_fun::PauseCreate {
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
                events.pause_fill.push(dca_dot_fun::PauseFill {
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

            // ───── Strategy & Protocol Configuration Events ─────
            // -- SetExecutionVarience --
            if let Some(event) = events::SetExecutionVarience::match_and_decode(log) {
                events
                    .set_execution_varience
                    .push(dca_dot_fun::SetExecutionVarience {
                        // -- transaction --
                        tx_hash: trx.hash.to_vec(),
                        // -- call --
                        caller: caller.clone(),
                        // -- log --
                        ordinal: log.ordinal,
                        contract: log.address.to_vec(),
                        // -- event --
                        execution_varience: event.execution_varience.to_string(),
                    });
            }

            // -- SetFeeCollector --
            if let Some(event) = events::SetFeeCollector::match_and_decode(log) {
                events.set_fee_collector.push(dca_dot_fun::SetFeeCollector {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    fee_collector: event.fee_collector.to_vec(),
                });
            }

            // -- SetMaxFeedAgeFillOrder --
            if let Some(event) = events::SetMaxFeedAgeFillOrder::match_and_decode(log) {
                events
                    .set_max_feed_age_fill_order
                    .push(dca_dot_fun::SetMaxFeedAgeFillOrder {
                        // -- transaction --
                        tx_hash: trx.hash.to_vec(),
                        // -- call --
                        caller: caller.clone(),
                        // -- log --
                        ordinal: log.ordinal,
                        contract: log.address.to_vec(),
                        // -- event --
                        max_feed_age: event.max_feed_age.to_string(),
                    });
            }

            // -- SetMaxFeedAgeCreateOrder --
            if let Some(event) = events::SetMaxFeedAgeCreateOrder::match_and_decode(log) {
                events
                    .set_max_feed_age_create_order
                    .push(dca_dot_fun::SetMaxFeedAgeCreateOrder {
                        // -- transaction --
                        tx_hash: trx.hash.to_vec(),
                        // -- call --
                        caller: caller.clone(),
                        // -- log --
                        ordinal: log.ordinal,
                        contract: log.address.to_vec(),
                        // -- event --
                        max_feed_age: event.max_feed_age.to_string(),
                    });
            }

            // -- SetMaxScalingFactor --
            if let Some(event) = events::SetMaxScalingFactor::match_and_decode(log) {
                events
                    .set_max_scaling_factor
                    .push(dca_dot_fun::SetMaxScalingFactor {
                        // -- transaction --
                        tx_hash: trx.hash.to_vec(),
                        // -- call --
                        caller: caller.clone(),
                        // -- log --
                        ordinal: log.ordinal,
                        contract: log.address.to_vec(),
                        // -- event --
                        max_scaling_factor: event.max_scaling_factor.to_string(),
                    });
            }

            // -- SetMaxSlippage --
            if let Some(event) = events::SetMaxSlippage::match_and_decode(log) {
                events.set_max_slippage.push(dca_dot_fun::SetMaxSlippage {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    slippage_max: event.slippage_max.to_string(),
                });
            }

            // -- SetMinExecutionValue --
            if let Some(event) = events::SetMinExecutionValue::match_and_decode(log) {
                events
                    .set_min_execution_value
                    .push(dca_dot_fun::SetMinExecutionValue {
                        // -- transaction --
                        tx_hash: trx.hash.to_vec(),
                        // -- call --
                        caller: caller.clone(),
                        // -- log --
                        ordinal: log.ordinal,
                        contract: log.address.to_vec(),
                        // -- event --
                        min_execution_value: event.min_execution_value.to_string(),
                    });
            }

            // -- SetMinOrderFrequencyInterval --
            if let Some(event) = events::SetMinOrderFrequencyInterval::match_and_decode(log) {
                events
                    .set_min_order_frequency_interval
                    .push(dca_dot_fun::SetMinOrderFrequencyInterval {
                        // -- transaction --
                        tx_hash: trx.hash.to_vec(),
                        // -- call --
                        caller: caller.clone(),
                        // -- log --
                        ordinal: log.ordinal,
                        contract: log.address.to_vec(),
                        // -- event --
                        min_order_frequency_interval: event.min_order_frequency_interval.to_string(),
                    });
            }

            // -- SetMinSlippage --
            if let Some(event) = events::SetMinSlippage::match_and_decode(log) {
                events.set_min_slippage.push(dca_dot_fun::SetMinSlippage {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    slippage_min: event.slippage_min.to_string(),
                });
            }

            // -- SetYieldSplit --
            if let Some(event) = events::SetYieldSplit::match_and_decode(log) {
                events.set_yield_split.push(dca_dot_fun::SetYieldSplit {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    yield_split: event.yield_split.to_string(),
                });
            }

            // -- SetProtocolFee --
            if let Some(event) = events::SetProtocolFee::match_and_decode(log) {
                events.set_protocol_fee.push(dca_dot_fun::SetProtocolFee {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    protocol_fee: event.protocol_fee.to_string(),
                });
            }

            // ─────────────────────── Token-Specific Events ───────────────────
            // -- SetTokenProps --
            if let Some(event) = events::SetTokenProps::match_and_decode(log) {
                events.set_token_props.push(dca_dot_fun::SetTokenProps {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    token: event.token.to_vec(),
                    feed: event.feed.to_vec(),
                    token_decimals: event.token_decimals.to_u64(),
                    token_symbol: event.token_symbol.to_string(),
                    token_name: event.token_name.to_string(),
                    is_active: event.is_active,
                    is_stakable: event.is_stakable,
                });
            }

            // -- SetTokenState --
            if let Some(event) = events::SetTokenState::match_and_decode(log) {
                events.set_token_state.push(dca_dot_fun::SetTokenState {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    token: event.token.to_vec(),
                    is_active: event.is_active,
                });
            }

            // ───── Role-Based Access-Control (RBAC) Events ─────
            // -- RoleAdminChanged --
            if let Some(event) = events::RoleAdminChanged::match_and_decode(log) {
                events.role_admin_changed.push(dca_dot_fun::RoleAdminChanged {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    role: event.role.to_vec(),
                    previous_admin_role: event.previous_admin_role.to_vec(),
                    new_admin_role: event.new_admin_role.to_vec(),
                });
            }

            // -- RoleGranted --
            if let Some(event) = events::RoleGranted::match_and_decode(log) {
                events.role_granted.push(dca_dot_fun::RoleGranted {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    role: event.role.to_vec(),
                    account: event.account.to_vec(),
                    sender: event.sender.to_vec(),
                });
            }

            // -- RoleRevoked --
            if let Some(event) = events::RoleRevoked::match_and_decode(log) {
                events.role_revoked.push(dca_dot_fun::RoleRevoked {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    role: event.role.to_vec(),
                    account: event.account.to_vec(),
                    sender: event.sender.to_vec(),
                });
            }

            // ───────── External Integrations & Adapters ─────────
            // -- SetAavePool --
            if let Some(event) = events::SetAavePool::match_and_decode(log) {
                events.set_aave_pool.push(dca_dot_fun::SetAavePool {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),
                    // -- call --
                    caller: caller.clone(),
                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),
                    // -- event --
                    aave_pool: event.aave_pool.to_vec(),
                });
            }
        }
    }

    Ok(events)
}
