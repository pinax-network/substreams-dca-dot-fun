use common::{
    bytes_to_hex,
    clickhouse::{common_key, set_log},
};
use proto::pb::evm::cryptopunks;
use substreams::pb::substreams::Clock;

use proto::pb::evm;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

#[substreams::handlers::map]
pub fn db_out(clock: Clock, events: evm::dca::v1::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    let mut index = 0; // relative index for events

    for event in events.assigns {
        let key = common_key(&clock, index);
        let row = tables.create_row("assigns", key).set("to", bytes_to_hex(&event.to)).set("index", &event.index);

        set_log(&clock, index, event.tx_hash, event.contract, event.ordinal, event.caller, row);
        index += 1;
    }

    Ok(tables.to_database_changes())
}
