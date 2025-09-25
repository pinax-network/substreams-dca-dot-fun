use common::{
    bytes_to_hex,
    clickhouse::{common_key, set_clock, set_log},
};
use substreams::pb::substreams::Clock;

use proto::pb::dca::dot::fun::v1 as dca_dot_fun;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    events: dca_dot_fun::Events,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    let mut index = 0u64; // relative index

    /* ────────────────────── Order-Level Events ───────────────────── */
    // CancelOrder
    for e in events.cancel_order {
        let row = tables
            .create_row("cancel_order", common_key(&clock, index))
            .set("order_id", &e.order_id)
            .set("vault", bytes_to_hex(&e.vault));
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    // CreateOrder
    for e in events.create_order {
        let row = tables
            .create_row("create_order", common_key(&clock, index))
            .set("order_id", &e.order_id)
            .set("creator", bytes_to_hex(&e.creator))
            .set("recipient", bytes_to_hex(&e.recipient))
            .set("token_in", bytes_to_hex(&e.token_in))
            .set("token_out", bytes_to_hex(&e.token_out))
            .set("spend_amount", &e.spend_amount)
            .set("repeats", &e.repeats)
            .set("slippage", &e.slippage)
            .set("freq_interval", &e.freq_interval)
            .set("scaling_interval", &e.scaling_interval)
            .set("last_run", &e.last_run)
            .set("protocol_fee", e.protocol_fee)
            .set("vault", bytes_to_hex(&e.vault))
            .set("stake_asset_in", e.stake_asset_in)
            .set("stake_asset_out", e.stake_asset_out);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    // FillOrder
    for e in events.fill_order {
        let row = tables
            .create_row("fill_order", common_key(&clock, index))
            .set("order_id", &e.order_id)
            .set("event_caller", bytes_to_hex(&e.event_caller))
            .set("recipient", bytes_to_hex(&e.recipient))
            .set("fill_amount", &e.fill_amount)
            .set("amount_of_token_out", &e.amount_of_token_out)
            .set("protocol_fee", e.protocol_fee)
            .set("token_in_price", &e.token_in_price)
            .set("token_out_price", &e.token_out_price)
            .set("scaling_factor", &e.scaling_factor);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    /* ─────────── Pause / Circuit-Breaker Events ─────────── */

    for e in events.pause_create {
        let row = tables
            .create_row("pause_create", common_key(&clock, index))
            .set("is_paused", e.is_paused);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.pause_fill {
        let row = tables
            .create_row("pause_fill", common_key(&clock, index))
            .set("is_paused", e.is_paused);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    /* ───── Strategy & Protocol Configuration Events ───── */

    for e in events.set_fee_collector {
        let row = tables
            .create_row("set_fee_collector", common_key(&clock, index))
            .set("fee_collector", bytes_to_hex(&e.fee_collector));
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_max_feed_age {
        let row = tables
            .create_row("set_max_feed_age", common_key(&clock, index))
            .set("max_feed_age_create_order", &e.max_feed_age_create_order)
            .set("max_feed_age_fill_order", &e.max_feed_age_fill_order);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_max_scaling_interval {
        let row = tables
            .create_row("set_max_scaling_interval", common_key(&clock, index))
            .set("max_scaling_interval", &e.max_scaling_interval);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_min_execution_value {
        let row = tables
            .create_row("set_min_execution_value", common_key(&clock, index))
            .set("min_execution_value", &e.min_execution_value)
            .set("execution_variance", &e.execution_variance);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_min_order_frequency_interval {
        let row = tables
            .create_row("set_min_order_frequency_interval", common_key(&clock, index))
            .set("min_order_frequency_interval", &e.min_order_frequency_interval);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_min_max_slippage {
        let row = tables
            .create_row("set_min_max_slippage", common_key(&clock, index))
            .set("slippage_min", &e.slippage_min)
            .set("slippage_max", &e.slippage_max);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_yield_split {
        let row = tables
            .create_row("set_yield_split", common_key(&clock, index))
            .set("yield_split", &e.yield_split);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_protocol_fee {
        let row = tables
            .create_row("set_protocol_fee", common_key(&clock, index))
            .set("protocol_fee", e.protocol_fee);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_timestamp_tolerance {
        let row = tables
            .create_row("set_timestamp_tolerance", common_key(&clock, index))
            .set("timestamp_tolerance", &e.timestamp_tolerance);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_vault_factory {
        let row = tables
            .create_row("set_vault_factory", common_key(&clock, index))
            .set("vault_factory", bytes_to_hex(&e.vault_factory));
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_verifier_dot_fun {
        let row = tables
            .create_row("set_verifier_dot_fun", common_key(&clock, index))
            .set("verifier_dot_fun", bytes_to_hex(&e.verifier_dot_fun));
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_native_token {
        let row = tables
            .create_row("set_native_token", common_key(&clock, index))
            .set("wrapped_native", bytes_to_hex(&e.wrapped_native))
            .set("native_token", bytes_to_hex(&e.native_token))
            .set("native_token_feed", bytes_to_hex(&e.native_token_feed))
            .set("native_token_decimals", e.native_token_decimals)
            .set("native_token_is_stakable", e.native_token_is_stakable);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    /* ─────────────── Token-Specific Events ─────────────── */

    for e in events.set_token_props {
        let row = tables
            .create_row("set_token_props", common_key(&clock, index))
            .set("token", bytes_to_hex(&e.token))
            .set("feed", bytes_to_hex(&e.feed))
            .set("token_decimals", e.token_decimals)
            .set("token_symbol", &e.token_symbol)
            .set("token_name", &e.token_name)
            .set("is_active", e.is_active)
            .set("is_stakable", e.is_stakable);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.set_token_state {
        let row = tables
            .create_row("set_token_state", common_key(&clock, index))
            .set("token", bytes_to_hex(&e.token))
            .set("is_active", e.is_active);
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    /* ───── Role-Based Access-Control Events ───── */

    for e in events.role_admin_changed {
        let row = tables
            .create_row("role_admin_changed", common_key(&clock, index))
            .set("role", bytes_to_hex(&e.role))
            .set("previous_admin_role", bytes_to_hex(&e.previous_admin_role))
            .set("new_admin_role", bytes_to_hex(&e.new_admin_role));
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.role_granted {
        let row = tables
            .create_row("role_granted", common_key(&clock, index))
            .set("role", bytes_to_hex(&e.role))
            .set("account", bytes_to_hex(&e.account))
            .set("sender", bytes_to_hex(&e.sender));
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    for e in events.role_revoked {
        let row = tables
            .create_row("role_revoked", common_key(&clock, index))
            .set("role", bytes_to_hex(&e.role))
            .set("account", bytes_to_hex(&e.account))
            .set("sender", bytes_to_hex(&e.sender));
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }

    /* ───── External Integration Events ───── */

    for e in events.set_aave_pool {
        let row = tables
            .create_row("set_aave_pool", common_key(&clock, index))
            .set("aave_pool", bytes_to_hex(&e.aave_pool));
        set_log(&clock, index, e.tx_hash, e.contract, e.ordinal, e.caller, row);
        index += 1;
    }
    /* ────────────────────── Blocks ───────────────────── */
    // ONLY include blocks if events are present
    if tables.tables.len() > 0 {
        set_clock(
            &clock,
            tables.create_row("blocks", [("block_num", clock.number.to_string())]),
        );
    }

    Ok(tables.to_database_changes())
}
