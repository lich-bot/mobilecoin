// NOTE: This file is auto-generated by Diesel.
// Run `diesel migration run` to update (inside the reserve-auditor directory)
#![allow(missing_docs)]

table! {
    audited_burns (id) {
        id -> Nullable<Integer>,
        burn_tx_out_id -> Integer,
        gnosis_safe_withdrawal_id -> Integer,
    }
}

table! {
    audited_mints (id) {
        id -> Nullable<Integer>,
        mint_tx_id -> Integer,
        gnosis_safe_deposit_id -> Integer,
    }
}

table! {
    block_audit_data (id) {
        id -> Nullable<Integer>,
        block_index -> BigInt,
    }
}

table! {
    block_balance (id) {
        id -> Nullable<Integer>,
        block_index -> BigInt,
        token_id -> BigInt,
        balance -> BigInt,
    }
}

table! {
    burn_tx_outs (id) {
        id -> Nullable<Integer>,
        block_index -> BigInt,
        token_id -> BigInt,
        amount -> BigInt,
        public_key_hex -> Text,
        protobuf -> Binary,
    }
}

table! {
    counters (id) {
        id -> Integer,
        num_blocks_synced -> BigInt,
        num_burns_exceeding_balance -> BigInt,
        num_mint_txs_without_matching_mint_config -> BigInt,
        num_mismatching_mints_and_deposits -> BigInt,
        num_mismatching_burns_and_withdrawals -> BigInt,
        num_unknown_ethereum_token_deposits -> BigInt,
        num_unknown_ethereum_token_withdrawals -> BigInt,
        num_mints_to_unknown_safe -> BigInt,
        num_burns_from_unknown_safe -> BigInt,
        num_unexpected_errors_matching_deposits_to_mints -> BigInt,
        num_unexpected_errors_matching_mints_to_deposits -> BigInt,
        num_unexpected_errors_matching_withdrawals_to_burns -> BigInt,
        num_unexpected_errors_matching_burns_to_withdrawals -> BigInt,
    }
}

table! {
    gnosis_safe_deposits (id) {
        id -> Nullable<Integer>,
        eth_tx_hash -> Text,
        eth_block_number -> BigInt,
        safe_addr -> Text,
        token_addr -> Text,
        amount -> BigInt,
        expected_mc_mint_tx_nonce_hex -> Text,
    }
}

table! {
    gnosis_safe_txs (eth_tx_hash) {
        eth_tx_hash -> Text,
        raw_tx_json -> Text,
    }
}

table! {
    gnosis_safe_withdrawals (id) {
        id -> Nullable<Integer>,
        eth_tx_hash -> Text,
        eth_block_number -> BigInt,
        safe_addr -> Text,
        token_addr -> Text,
        amount -> BigInt,
        mc_tx_out_public_key_hex -> Text,
    }
}

table! {
    mint_config_txs (id) {
        id -> Nullable<Integer>,
        block_index -> BigInt,
        token_id -> BigInt,
        nonce_hex -> Text,
        total_mint_limit -> BigInt,
        tombstone_block -> BigInt,
        protobuf -> Binary,
    }
}

table! {
    mint_configs (id) {
        id -> Nullable<Integer>,
        mint_config_tx_id -> Integer,
        mint_limit -> BigInt,
        protobuf -> Binary,
    }
}

table! {
    mint_txs (id) {
        id -> Nullable<Integer>,
        block_index -> BigInt,
        token_id -> BigInt,
        amount -> BigInt,
        nonce_hex -> Text,
        recipient_b58_addr -> Text,
        tombstone_block -> BigInt,
        protobuf -> Binary,
        mint_config_id -> Nullable<Integer>,
    }
}

joinable!(audited_burns -> burn_tx_outs (burn_tx_out_id));
joinable!(audited_burns -> gnosis_safe_withdrawals (gnosis_safe_withdrawal_id));
joinable!(audited_mints -> gnosis_safe_deposits (gnosis_safe_deposit_id));
joinable!(audited_mints -> mint_txs (mint_tx_id));
joinable!(gnosis_safe_deposits -> gnosis_safe_txs (eth_tx_hash));
joinable!(gnosis_safe_withdrawals -> gnosis_safe_txs (eth_tx_hash));
joinable!(mint_configs -> mint_config_txs (mint_config_tx_id));
joinable!(mint_txs -> mint_configs (mint_config_id));

allow_tables_to_appear_in_same_query!(
    audited_burns,
    audited_mints,
    block_audit_data,
    block_balance,
    burn_tx_outs,
    counters,
    gnosis_safe_deposits,
    gnosis_safe_txs,
    gnosis_safe_withdrawals,
    mint_config_txs,
    mint_configs,
    mint_txs,
);
