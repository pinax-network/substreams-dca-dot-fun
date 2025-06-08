/* ╔════════════════════ ORDER-LEVEL EVENTS ═══════════════════╗ */

/* CancelOrder */
CREATE TABLE IF NOT EXISTS cancel_order (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    order_id  UInt256,
    vault     FixedString(42),

    /* indexes (base) ---------------------------------------- */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) --------------------------------------- */
    INDEX idx_order_id  (order_id)  TYPE bloom_filter GRANULARITY 4,
    INDEX idx_vault     (vault)     TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* CreateOrder */
CREATE TABLE IF NOT EXISTS create_order (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    order_id          UInt256,
    creator           FixedString(42),
    recipient         FixedString(42),
    token_in          FixedString(42),
    token_out         FixedString(42),
    spend_amount      UInt256,
    repeats           UInt256,
    slippage          UInt256,
    freq_interval     UInt256,
    scaling_interval  UInt256,
    last_run          UInt256,
    protocol_fee      UInt64,
    vault             FixedString(42),
    stake_asset_in    UInt8,
    stake_asset_out   UInt8,

    /* indexes (base) ---------------------------------------- */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) --------------------------------------- */
    INDEX idx_order_id          (order_id)           TYPE bloom_filter GRANULARITY 4,
    INDEX idx_creator           (creator)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_recipient         (recipient)          TYPE bloom_filter GRANULARITY 4,
    INDEX idx_token_in          (token_in)           TYPE bloom_filter GRANULARITY 4,
    INDEX idx_token_out         (token_out)          TYPE bloom_filter GRANULARITY 4,
    INDEX idx_spend_amount      (spend_amount)       TYPE bloom_filter GRANULARITY 4,
    INDEX idx_repeats           (repeats)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_slippage          (slippage)           TYPE bloom_filter GRANULARITY 4,
    INDEX idx_freq_interval     (freq_interval)      TYPE bloom_filter GRANULARITY 4,
    INDEX idx_scaling_interval  (scaling_interval)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_last_run          (last_run)           TYPE bloom_filter GRANULARITY 4,
    INDEX idx_protocol_fee      (protocol_fee)       TYPE set(128)     GRANULARITY 4,
    INDEX idx_vault             (vault)              TYPE bloom_filter GRANULARITY 4,
    INDEX idx_stake_asset_in    (stake_asset_in)     TYPE set(2)       GRANULARITY 4,
    INDEX idx_stake_asset_out   (stake_asset_out)    TYPE set(2)       GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* FillOrder */
CREATE TABLE IF NOT EXISTS fill_order (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    order_id            UInt256,
    recipient           FixedString(42),
    fill_amount         UInt256,
    amount_of_token_out UInt256,
    protocol_fee        UInt64,
    token_in_price      UInt256,
    token_out_price     UInt256,
    scaling_factor      UInt256,

    /* indexes (base) ---------------------------------------- */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) --------------------------------------- */
    INDEX idx_order_id         (order_id)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_recipient        (recipient)           TYPE bloom_filter GRANULARITY 4,
    INDEX idx_fill_amount      (fill_amount)         TYPE bloom_filter GRANULARITY 4,
    INDEX idx_amount_out       (amount_of_token_out) TYPE bloom_filter GRANULARITY 4,
    INDEX idx_protocol_fee     (protocol_fee)        TYPE set(128)     GRANULARITY 4,
    INDEX idx_token_in_price   (token_in_price)      TYPE bloom_filter GRANULARITY 4,
    INDEX idx_token_out_price  (token_out_price)     TYPE bloom_filter GRANULARITY 4,
    INDEX idx_scaling_factor   (scaling_factor)      TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* ╔════════════ PAUSE / CIRCUIT-BREAKER EVENTS ════════════╗ */

/* PauseCreate */
CREATE TABLE IF NOT EXISTS pause_create (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    is_paused UInt8,

    /* indexes (base) ---------------------------------------- */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) --------------------------------------- */
    INDEX idx_is_paused (is_paused) TYPE set(2)       GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* PauseFill */
CREATE TABLE IF NOT EXISTS pause_fill (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    is_paused UInt8,

    /* indexes (base) ---------------------------------------- */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) --------------------------------------- */
    INDEX idx_is_paused (is_paused) TYPE set(2)       GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* ╔══ STRATEGY & PROTOCOL CONFIGURATION EVENTS (UInt256) ══╗ */

/* SetExecutionVarience */
CREATE TABLE IF NOT EXISTS set_execution_varience (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    execution_varience UInt256,

    /* indexes (base) ---------------------------------------- */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) --------------------------------------- */
    INDEX idx_execution_varience (execution_varience) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetFeeCollector */
CREATE TABLE IF NOT EXISTS set_fee_collector (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    fee_collector FixedString(42),

    /* indexes (base) ---------------------------------------- */
    INDEX idx_tx_hash   (tx_hash)       TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)        TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)      TYPE set(16)      GRANULARITY 4,

    /* indexes (event) --------------------------------------- */
    INDEX idx_fee_collector (fee_collector) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetMaxFeedAgeFillOrder */
CREATE TABLE IF NOT EXISTS set_max_feed_age_fill_order (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    max_feed_age UInt256,

    /* indexes (base) ---------------------------------------- */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) --------------------------------------- */
    INDEX idx_max_feed_age (max_feed_age) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetMaxFeedAgeCreateOrder */
CREATE TABLE IF NOT EXISTS set_max_feed_age_create_order (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    max_feed_age UInt256,

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_max_feed_age (max_feed_age) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetMaxScalingFactor */
CREATE TABLE IF NOT EXISTS set_max_scaling_factor (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    max_scaling_factor UInt256,

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_max_scaling_factor (max_scaling_factor) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetMaxSlippage */
CREATE TABLE IF NOT EXISTS set_max_slippage (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    max_slippage UInt256,

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_max_slippage (max_slippage) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetMinExecutionValue */
CREATE TABLE IF NOT EXISTS set_min_execution_value (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    min_execution_value UInt256,

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_min_execution_value (min_execution_value) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetMinOrderFrequencyInterval */
CREATE TABLE IF NOT EXISTS set_min_order_frequency_interval (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    min_order_frequency_interval UInt256,

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_min_order_frequency_interval (min_order_frequency_interval) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetMinSlippage */
CREATE TABLE IF NOT EXISTS set_min_slippage (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    min_slippage UInt256,

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_min_slippage (min_slippage) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetYieldSplit */
CREATE TABLE IF NOT EXISTS set_yield_split (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    yield_split UInt256,

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_yield_split (yield_split) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetProtocolFee */
CREATE TABLE IF NOT EXISTS set_protocol_fee (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    protocol_fee UInt64,

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_protocol_fee (protocol_fee) TYPE set(128) GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* ╔════════════ TOKEN-SPECIFIC EVENTS ════════════╗ */

/* SetTokenProps */
CREATE TABLE IF NOT EXISTS set_token_props (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    token           FixedString(42),
    feed            FixedString(66),
    token_decimals  UInt32,
    token_symbol    String,
    token_name      String,
    is_active       UInt8,
    is_stakable     UInt8,

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_token           (token)           TYPE bloom_filter GRANULARITY 4,
    INDEX idx_feed            (feed)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_token_symbol    (token_symbol)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_token_name      (token_name)      TYPE bloom_filter GRANULARITY 4,
    INDEX idx_token_decimals  (token_decimals)  TYPE set(256)    GRANULARITY 4,
    INDEX idx_is_active       (is_active)       TYPE set(2)      GRANULARITY 4,
    INDEX idx_is_stakable     (is_stakable)     TYPE set(2)      GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* SetTokenState */
CREATE TABLE IF NOT EXISTS set_token_state (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    token     FixedString(42),
    is_active UInt8,

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_token     (token)     TYPE bloom_filter GRANULARITY 4,
    INDEX idx_is_active (is_active) TYPE set(2)       GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* ╔════════════ RBAC EVENTS ════════════╗ */

/* RoleAdminChanged */
CREATE TABLE IF NOT EXISTS role_admin_changed (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    role                FixedString(66),
    previous_admin_role FixedString(66),
    new_admin_role      FixedString(66),

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_role          (role)                 TYPE bloom_filter GRANULARITY 4,
    INDEX idx_previous_role (previous_admin_role)  TYPE bloom_filter GRANULARITY 4,
    INDEX idx_new_role      (new_admin_role)       TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* RoleGranted */
CREATE TABLE IF NOT EXISTS role_granted (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    role    FixedString(66),
    account FixedString(42),
    sender  FixedString(42),

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_role    (role)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_account (account) TYPE bloom_filter GRANULARITY 4,
    INDEX idx_sender  (sender)  TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* RoleRevoked */
CREATE TABLE IF NOT EXISTS role_revoked (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    role    FixedString(66),
    account FixedString(42),
    sender  FixedString(42),

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_role    (role)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_account (account) TYPE bloom_filter GRANULARITY 4,
    INDEX idx_sender  (sender)  TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

/* ╔════════ EXTERNAL INTEGRATION EVENTS ═════════╗ */

/* SetAavePool */
CREATE TABLE IF NOT EXISTS set_aave_pool (
    -- block --
    block_num   UInt32,
    block_hash  FixedString(66),
    timestamp   DateTime(0, 'UTC'),

    -- ordering --
    ordinal          UInt64,
    `index`          UInt64,
    global_sequence  UInt64,

    -- transaction --
    tx_hash   FixedString(66),

    -- call --
    caller    FixedString(42),

    -- log --
    contract  FixedString(42),

    -- event --
    aave_pool FixedString(42),

    /* indexes (base) */
    INDEX idx_tx_hash   (tx_hash)   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller    (caller)    TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract  (contract)  TYPE set(16)      GRANULARITY 4,

    /* indexes (event) */
    INDEX idx_aave_pool (aave_pool) TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);
