CREATE TABLE IF NOT EXISTS cursors
(
    id        String,
    cursor    String,
    block_num Int64,
    block_id  String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

-- Assigns --
CREATE TABLE IF NOT EXISTS assigns (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    ordinal              UInt64, -- log.ordinal
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42) COMMENT 'caller address', -- call.caller

    -- log --
    contract             FixedString(42) COMMENT 'contract address',

    -- event --
    `to`                 FixedString(42),
    punk_index           UInt64,

    -- indexes (transaction) --
    INDEX idx_tx_hash            (tx_hash)                  TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller             (caller)                   TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_contract           (contract)                 TYPE set(16) GRANULARITY 4,
    INDEX idx_to                 (`to`)                     TYPE bloom_filter GRANULARITY 4,
    INDEX idx_punk_index         (punk_index)               TYPE set(128) GRANULARITY 4,

) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);
