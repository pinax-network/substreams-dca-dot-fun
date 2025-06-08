pub mod clickhouse;
use substreams::{hex, log, pb::substreams::Clock, scalar::BigInt, Hex};
use substreams_ethereum::pb::eth::v2::{block::DetailLevel, Block, Log, TransactionTrace};

pub type Address = Vec<u8>;
pub type Hash = Vec<u8>;
pub const NULL_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");
pub const NULL_HASH: [u8; 32] = hex!("0000000000000000000000000000000000000000000000000000000000000000");
pub const NATIVE_ADDRESS: [u8; 20] = hex!("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");

// Used to enforce ERC-20 decimals to be between 0 and 255
pub fn bigint_to_uint8(bigint: &substreams::scalar::BigInt) -> Option<i32> {
    if bigint.lt(&BigInt::zero()) {
        log::info!("bigint_to_uint8: value is negative");
        return None;
    }
    if bigint.gt(&BigInt::from(255)) {
        log::info!("bigint_to_uint8: value is greater than 255");
        return None;
    }
    Some(bigint.to_i32())
}

pub fn bigint_to_uint64(bigint: &substreams::scalar::BigInt) -> Option<u64> {
    if bigint.lt(&BigInt::zero()) {
        log::info!("bigint_to_uint64: value is negative");
        return None;
    }
    if bigint.gt(&BigInt::from(u64::MAX)) {
        log::info!("bigint_to_uint64: value is greater than u64::MAX");
        return None;
    }
    Some(bigint.to_u64())
}

pub fn bigint_to_i32(bigint: &substreams::scalar::BigInt) -> Option<i32> {
    if bigint.lt(&BigInt::zero()) {
        log::info!("bigint_to_i32: value is negative");
        return None;
    }
    if bigint.gt(&BigInt::from(i32::MAX)) {
        log::info!("bigint_to_i32: value is greater than i32::MAX");
        return None;
    }
    Some(bigint.to_i32())
}

pub fn is_zero_address<T: AsRef<[u8]>>(addr: T) -> bool {
    addr.as_ref() == NULL_ADDRESS
}

pub fn logs_with_caller<'a>(block: &'a Block, trx: &'a TransactionTrace) -> Vec<(&'a Log, Option<Address>)> {
    let mut results = vec![];

    if block.detail_level() == DetailLevel::DetaillevelExtended {
        for (log, call_view) in trx.logs_with_calls() {
            results.push((log, Some(call_view.call.caller.to_vec())));
        }
    } else {
        for log in trx.receipt().logs() {
            results.push((log.log, None));
        }
    }

    results
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    format! {"0x{}", Hex::encode(bytes)}.to_string()
}

// In ClickHouse, an aggregate function like argMax can only take one expression as the "ordering" argument.
// So we typically combine (block_num, index) into a single monotonic integer.
// For example, if each of block_num and index fits in 32 bits, we can do:
// max(toUInt64(block_num) * 2^32 + index) AS version
pub fn to_global_sequence(clock: &Clock, index: u64) -> u64 {
    (clock.number << 32) + index
}
