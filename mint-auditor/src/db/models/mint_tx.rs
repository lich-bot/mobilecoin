// Copyright (c) 2018-2022 The MobileCoin Foundation

//! Model file for the mint_txs table.

use crate::{
    db::{
        last_insert_rowid,
        schema::{audited_mints, mint_txs},
        Conn,
    },
    Error,
};
use diesel::{
    dsl::{exists, not},
    prelude::*,
};
use hex::ToHex;
use mc_account_keys::PublicAddress;
use mc_api::printable::PrintableWrapper;
use mc_blockchain_types::BlockIndex;
use mc_transaction_core::{mint::MintTx as CoreMintTx, TokenId};
use mc_util_serial::{decode, encode};
use serde::{Deserialize, Serialize};

/// Diesel model for the `mint_txs` table.
/// This stores audit data for a specific block index.
#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Insertable, PartialEq, Queryable, Serialize,
)]
pub struct MintTx {
    /// Auto incrementing primary key.
    id: Option<i32>,

    /// The block index at which this mint tx appreared.
    block_index: i64,

    /// The token id this mint tx is for.
    token_id: i64,

    /// The amount being minted.
    amount: i64,

    /// The nonce, as hex-encoded bytes.
    nonce_hex: String,

    /// The recipient of the mint.
    recipient_b58_address: String,

    /// Tombstone block.
    tombstone_block: i64,

    /// The protobuf-serialized MintTx.
    protobuf: Vec<u8>,

    /// The mint config id, when we are able to match it with one.
    mint_config_id: Option<i32>,
}

impl MintTx {
    /// Get id.
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    /// Get block index.
    pub fn block_index(&self) -> u64 {
        self.block_index as u64
    }

    /// Get token id.
    pub fn token_id(&self) -> TokenId {
        TokenId::from(self.token_id as u64)
    }

    /// Get amount.
    pub fn amount(&self) -> u64 {
        self.amount as u64
    }

    /// Get nonce.
    pub fn nonce_hex(&self) -> &str {
        &self.nonce_hex
    }

    /// Get recipient b58 address.
    pub fn recipient_b58_address(&self) -> &str {
        &self.recipient_b58_address
    }

    /// Get tombstone block.
    pub fn tombstone_block(&self) -> u64 {
        self.tombstone_block as u64
    }

    /// Get mint config id, when we are able to match it with one.
    pub fn mint_config_id(&self) -> Option<i32> {
        self.mint_config_id
    }

    /// Get the original MintTx
    pub fn decode(&self) -> Result<CoreMintTx, Error> {
        Ok(decode(&self.protobuf)?)
    }

    /// Insert a new MintTx into the database.
    // TODO split into from_core_mint_tx and insert(&mut self)
    pub fn insert(
        block_index: BlockIndex,
        mint_config_id: Option<i32>,
        tx: &CoreMintTx,
        conn: &Conn,
    ) -> Result<Self, Error> {
        let recipient = PublicAddress::new(&tx.prefix.spend_public_key, &tx.prefix.view_public_key);
        let mut wrapper = PrintableWrapper::new();
        wrapper.set_public_address((&recipient).into());
        let recipient_b58_address = wrapper.b58_encode()?;

        let mut obj = Self {
            id: None,
            block_index: block_index as i64,
            token_id: tx.prefix.token_id as i64,
            amount: tx.prefix.amount as i64,
            nonce_hex: tx.prefix.nonce.encode_hex(),
            recipient_b58_address,
            tombstone_block: tx.prefix.tombstone_block as i64,
            protobuf: encode(tx),
            mint_config_id,
        };

        diesel::insert_into(mint_txs::table)
            .values(&obj)
            .execute(conn)?;

        obj.id = Some(diesel::select(last_insert_rowid).get_result::<i32>(conn)?);

        Ok(obj)
    }

    /// Attempt to find all MintTxs that do not have a matching entry in the
    /// `audited_mints` table.
    pub fn find_unaudited_mint_txs(conn: &Conn) -> Result<Vec<MintTx>, Error> {
        Ok(mint_txs::table
            .filter(not(exists(
                audited_mints::table
                    .select(audited_mints::mint_tx_id)
                    .filter(audited_mints::mint_tx_id.nullable().eq(mint_txs::id)),
            )))
            .load(conn)?)
    }

    /// Attempt to find a MintTx that has a given nonce and no matching entry in
    /// the `audited_mints` table.
    pub fn find_unaudited_mint_tx_by_nonce(
        nonce_hex: &str,
        conn: &Conn,
    ) -> Result<Option<MintTx>, Error> {
        Ok(mint_txs::table
            .filter(mint_txs::nonce_hex.eq(nonce_hex))
            .filter(not(exists(
                audited_mints::table
                    .select(audited_mints::mint_tx_id)
                    .filter(audited_mints::mint_tx_id.nullable().eq(mint_txs::id)),
            )))
            .first(conn)
            .optional()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{
        models::AuditedMint,
        test_utils::{create_gnosis_safe_deposit, insert_gnosis_deposit, TestDbContext},
    };
    use mc_common::logger::{test_with_logger, Logger};
    use mc_transaction_core::TokenId;
    use mc_transaction_core_test_utils::{create_mint_config_tx_and_signers, create_mint_tx};

    #[test_with_logger]
    fn insert_enforces_uniqueness(logger: Logger) {
        let mut rng = mc_util_test_helper::get_seeded_rng();
        let test_db_context = TestDbContext::default();
        let mint_auditor_db = test_db_context.get_db_instance(logger.clone());
        let token_id1 = TokenId::from(1);

        let conn = mint_auditor_db.get_conn().unwrap();

        let (_mint_config_tx1, signers1) = create_mint_config_tx_and_signers(token_id1, &mut rng);
        let mint_tx1 = create_mint_tx(token_id1, &signers1, 100, &mut rng);

        // Store a MintTx for the first time.
        MintTx::insert(5, None, &mint_tx1, &conn).unwrap();

        // Trying again should fail.
        assert!(MintTx::insert(5, None, &mint_tx1, &conn).is_err());
    }

    #[test_with_logger]
    fn test_find_unaudited_mint_tx_by_nonce(logger: Logger) {
        let mut rng = mc_util_test_helper::get_seeded_rng();
        let test_db_context = TestDbContext::default();
        let mint_auditor_db = test_db_context.get_db_instance(logger.clone());
        let token_id1 = TokenId::from(1);
        let conn = mint_auditor_db.get_conn().unwrap();

        // Create gnosis deposits.
        let mut deposit1 = create_gnosis_safe_deposit(100, &mut rng);
        let mut deposit2 = create_gnosis_safe_deposit(200, &mut rng);

        // Create two MintTxs.
        let (_mint_config_tx1, signers1) = create_mint_config_tx_and_signers(token_id1, &mut rng);
        let mut mint_tx1 = create_mint_tx(token_id1, &signers1, 100, &mut rng);
        let mut mint_tx2 = create_mint_tx(token_id1, &signers1, 100, &mut rng);

        mint_tx1.prefix.nonce = hex::decode(&deposit1.expected_mc_mint_tx_nonce_hex()).unwrap();
        mint_tx2.prefix.nonce = hex::decode(&deposit2.expected_mc_mint_tx_nonce_hex()).unwrap();

        // Since they haven't been inserted yet, they should not be found.
        assert!(MintTx::find_unaudited_mint_tx_by_nonce(
            &hex::encode(&mint_tx1.prefix.nonce),
            &conn
        )
        .unwrap()
        .is_none());

        assert!(MintTx::find_unaudited_mint_tx_by_nonce(
            &hex::encode(&mint_tx2.prefix.nonce),
            &conn
        )
        .unwrap()
        .is_none());

        // Insert the first MintTx, it should now be found.
        let sql_mint_tx1 = MintTx::insert(5, None, &mint_tx1, &conn).unwrap();

        assert_eq!(
            MintTx::find_unaudited_mint_tx_by_nonce(&hex::encode(&mint_tx1.prefix.nonce), &conn)
                .unwrap()
                .unwrap(),
            sql_mint_tx1
        );

        assert!(MintTx::find_unaudited_mint_tx_by_nonce(
            &hex::encode(&mint_tx2.prefix.nonce),
            &conn
        )
        .unwrap()
        .is_none());

        // Insert the second MintTx, they should both be found.
        let sql_mint_tx2 = MintTx::insert(5, None, &mint_tx2, &conn).unwrap();

        assert_eq!(
            MintTx::find_unaudited_mint_tx_by_nonce(&hex::encode(&mint_tx1.prefix.nonce), &conn)
                .unwrap()
                .unwrap(),
            sql_mint_tx1
        );

        assert_eq!(
            MintTx::find_unaudited_mint_tx_by_nonce(&hex::encode(&mint_tx2.prefix.nonce), &conn)
                .unwrap()
                .unwrap(),
            sql_mint_tx2
        );

        // Insert a row to the `audited_mints` table marking the first MintTx as
        // audited. We should no longer be able to find it.
        insert_gnosis_deposit(&mut deposit1, &conn);
        let audited_mint = AuditedMint {
            id: None,
            mint_tx_id: sql_mint_tx1.id().unwrap(),
            gnosis_safe_deposit_id: deposit1.id().unwrap(),
        };
        diesel::insert_into(audited_mints::table)
            .values(audited_mint)
            .execute(&conn)
            .unwrap();

        assert!(MintTx::find_unaudited_mint_tx_by_nonce(
            &hex::encode(&mint_tx1.prefix.nonce),
            &conn
        )
        .unwrap()
        .is_none());

        assert_eq!(
            MintTx::find_unaudited_mint_tx_by_nonce(&hex::encode(&mint_tx2.prefix.nonce), &conn)
                .unwrap()
                .unwrap(),
            sql_mint_tx2
        );

        // Mark the second mint as audited. We should no longer be able to find it.
        insert_gnosis_deposit(&mut deposit2, &conn);
        let audited_mint = AuditedMint {
            id: None,
            mint_tx_id: sql_mint_tx2.id().unwrap(),
            gnosis_safe_deposit_id: deposit2.id().unwrap(),
        };
        diesel::insert_into(audited_mints::table)
            .values(audited_mint)
            .execute(&conn)
            .unwrap();

        assert!(MintTx::find_unaudited_mint_tx_by_nonce(
            &hex::encode(&mint_tx1.prefix.nonce),
            &conn
        )
        .unwrap()
        .is_none());

        assert!(MintTx::find_unaudited_mint_tx_by_nonce(
            &hex::encode(&mint_tx2.prefix.nonce),
            &conn
        )
        .unwrap()
        .is_none());
    }
}
