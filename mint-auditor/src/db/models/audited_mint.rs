// Copyright (c) 2018-2022 The MobileCoin Foundation

use crate::{
    db::{schema::audited_mints, transaction, Conn, GnosisSafeDeposit, MintTx},
    gnosis::AuditedSafeConfig,
    Error,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Diesel model for the `audited_mints` table.
/// This stores audit data linking MintTxs with matching GnosisSafeDeposits.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Insertable, PartialEq, Queryable, Serialize)]
pub struct AuditedMint {
    /// Id (required to keep Diesel happy).
    pub id: Option<i32>,

    /// Id pointing to the MintTx table.
    pub mint_tx_id: i32,

    /// Id pointing to the GnosisSafeDeposit table.
    pub gnosis_safe_deposit_id: i32,
}

impl AuditedMint {
    /// Attempt to find a matching MintTx for a given GnosisSafeDeposit, and if
    /// successful return the MintTx and record the match in the database.
    /// Note that each MintTx can be matched to at most one GnosisSafeDeposit,
    /// so calling this repeatedly over the same deposit will fail.
    pub fn attempt_match_deposit_with_mint(
        deposit: &GnosisSafeDeposit,
        config: &AuditedSafeConfig,
        conn: &Conn,
    ) -> Result<MintTx, Error> {
        // We only operate on objects that were saved to the database.
        let deposit_id = deposit.id().ok_or(Error::ObjectNotSaved)?;

        // The deposit safe needs to match the audited safe configuration.
        if deposit.safe_address() != &config.safe_addr {
            // TODO: a specific variant? depends on whether the code that calls this is
            // likely to fail
            return Err(Error::Other(format!(
                "Gnosis safe deposit address {} does not match audited safe address {}",
                deposit.safe_address(),
                config.safe_addr
            )));
        }

        // Get the audited token information
        let audited_token = config
            .get_token_by_eth_contract_addr(deposit.token_address())
            .ok_or_else(|| {
                Error::Other(format!(
                    "Gnosis safe deposit token address {} is not audited",
                    deposit.token_address()
                ))
            })?;

        transaction(conn, |conn| {
            // Currently we only support 1:1 mapping between deposits and mints, so ensure
            // that there isn't already a match for this deposit.
            let existing_match = audited_mints::table
                .filter(audited_mints::gnosis_safe_deposit_id.eq(deposit_id))
                .first::<AuditedMint>(conn)
                .optional()?;
            if let Some(existing_match) = existing_match {
                return Err(Error::AlreadyExists(format!(
                    "GnosisSafeDeposit id={} already matched with mint_tx_id={}",
                    existing_match.gnosis_safe_deposit_id, existing_match.mint_tx_id
                )));
            }

            // See if we can find a MintTx that matches the expected nonce and has not been
            // associated with a deposit.
            let mint_tx = MintTx::find_unaudited_mint_tx_by_nonce(
                deposit.expected_mc_mint_tx_nonce_hex(),
                conn,
            )?
            .ok_or(Error::NotFound)?;

            // Sanity - find_audited_mint_tx_by_nonce is broken if it returns a MintTx with
            // a mismatching nonce.
            assert_eq!(mint_tx.nonce_hex(), deposit.expected_mc_mint_tx_nonce_hex());

            // Found a mint, check to see if the amount matches the deposit.
            if mint_tx.amount() != deposit.amount() {
                return Err(Error::DepositAndMintMismatch(format!(
                    "MintTx amount={} does not match GnosisSafeDeposit amount={} (nonce={})",
                    mint_tx.amount(),
                    deposit.amount(),
                    deposit.expected_mc_mint_tx_nonce_hex(),
                )));
            }

            // Check and see if the tokens match.
            if audited_token.token_id != mint_tx.token_id() {
                return Err(Error::DepositAndMintMismatch(format!(
                    "MintTx token_id={} does not match audited token_=id{} (nonce={})",
                    mint_tx.token_id(),
                    audited_token.token_id,
                    deposit.expected_mc_mint_tx_nonce_hex(),
                )));
            }

            // Associate the deposit with the mint.
            let audited_mint = Self {
                id: None,
                mint_tx_id: mint_tx
                    .id()
                    .expect("got a MintTx without id but database auto-populates that field"),
                gnosis_safe_deposit_id: deposit_id,
            };
            diesel::insert_into(audited_mints::table)
                .values(&audited_mint)
                .execute(conn)?;

            Ok(mint_tx)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db::{
            models::AuditedMint,
            test_utils::{
                create_gnosis_safe_deposit, insert_gnosis_deposit, insert_mint_tx_from_deposit,
                test_gnosis_config, TestDbContext,
            },
        },
        gnosis::EthAddr,
    };
    use mc_common::logger::{test_with_logger, Logger};
    use mc_transaction_core::TokenId;
    use mc_transaction_core_test_utils::{create_mint_config_tx_and_signers, create_mint_tx};
    use std::str::FromStr;

    fn assert_audited_mints_table_is_empty(conn: &Conn) {
        let num_rows: i64 = audited_mints::table
            .select(diesel::dsl::count(audited_mints::id))
            .first(conn)
            .unwrap();
        assert_eq!(num_rows, 0);
    }

    #[test_with_logger]
    fn test_attempt_match_deposit_with_mint_happy_flow(logger: Logger) {
        let config = &test_gnosis_config().safes[0];
        let mut rng = mc_util_test_helper::get_seeded_rng();
        let test_db_context = TestDbContext::default();
        let mint_auditor_db = test_db_context.get_db_instance(logger.clone());
        let conn = mint_auditor_db.get_conn().unwrap();

        // Create gnosis deposits.
        let mut deposit1 = create_gnosis_safe_deposit(100, &mut rng);
        let mut deposit2 = create_gnosis_safe_deposit(200, &mut rng);

        insert_gnosis_deposit(&mut deposit1, &conn);
        insert_gnosis_deposit(&mut deposit2, &conn);

        // Initially the database is empty.
        assert!(matches!(
            AuditedMint::attempt_match_deposit_with_mint(&deposit1, config, &conn),
            Err(Error::NotFound)
        ));
        assert!(matches!(
            AuditedMint::attempt_match_deposit_with_mint(&deposit1, config, &conn),
            Err(Error::NotFound)
        ));
        assert_audited_mints_table_is_empty(&conn);

        // Insert the first MintTx to the database, we should get a match now.
        let sql_mint_tx1 = insert_mint_tx_from_deposit(&deposit1, &conn, &mut rng);
        assert_eq!(
            sql_mint_tx1,
            AuditedMint::attempt_match_deposit_with_mint(&deposit1, config, &conn).unwrap()
        );
        assert!(matches!(
            AuditedMint::attempt_match_deposit_with_mint(&deposit2, config, &conn),
            Err(Error::NotFound)
        ));

        // Insert the second MintTx to the database, we should get a match on both.
        let sql_mint_tx2 = insert_mint_tx_from_deposit(&deposit2, &conn, &mut rng);
        assert!(matches!(
            AuditedMint::attempt_match_deposit_with_mint(&deposit1, config, &conn),
            Err(Error::AlreadyExists(_))
        ));
        assert_eq!(
            sql_mint_tx2,
            AuditedMint::attempt_match_deposit_with_mint(&deposit2, config, &conn).unwrap()
        );

        // Trying again should return AlreadyExists
        assert!(matches!(
            AuditedMint::attempt_match_deposit_with_mint(&deposit2, config, &conn),
            Err(Error::AlreadyExists(_))
        ));
    }

    #[test_with_logger]
    fn test_attempt_match_deposit_with_mint_amount_mismatch(logger: Logger) {
        let config = &test_gnosis_config().safes[0];
        let mut rng = mc_util_test_helper::get_seeded_rng();
        let test_db_context = TestDbContext::default();
        let mint_auditor_db = test_db_context.get_db_instance(logger.clone());
        let token_id1 = config.tokens[0].token_id;
        let conn = mint_auditor_db.get_conn().unwrap();

        // Create gnosis deposit.
        let mut deposit1 = create_gnosis_safe_deposit(100, &mut rng);
        insert_gnosis_deposit(&mut deposit1, &conn);

        // Create  MintTxs with a mismatching amount.
        let (_mint_config_tx1, signers1) = create_mint_config_tx_and_signers(token_id1, &mut rng);
        let mut mint_tx1 = create_mint_tx(token_id1, &signers1, deposit1.amount() + 1, &mut rng);

        mint_tx1.prefix.nonce = hex::decode(&deposit1.expected_mc_mint_tx_nonce_hex()).unwrap();

        // Insert the first MintTx to the database, and check that the mismatch is
        // detected.
        MintTx::insert(0, None, &mint_tx1, &conn).unwrap();
        assert!(matches!(
            AuditedMint::attempt_match_deposit_with_mint(&deposit1, config, &conn),
            Err(Error::DepositAndMintMismatch(_))
        ));

        // Check that nothing was written to the `audited_mints` table
        assert_audited_mints_table_is_empty(&conn);
    }

    #[test_with_logger]
    fn test_attempt_match_deposit_with_mint_unsaved_object(logger: Logger) {
        let config = &test_gnosis_config().safes[0];
        let mut rng = mc_util_test_helper::get_seeded_rng();
        let test_db_context = TestDbContext::default();
        let mint_auditor_db = test_db_context.get_db_instance(logger.clone());
        let conn = mint_auditor_db.get_conn().unwrap();

        let deposit1 = create_gnosis_safe_deposit(100, &mut rng);

        assert!(matches!(
            AuditedMint::attempt_match_deposit_with_mint(&deposit1, config, &conn),
            Err(Error::ObjectNotSaved)
        ));

        // Check that nothing was written to the `audited_mints` table
        assert_audited_mints_table_is_empty(&conn);
    }

    #[test_with_logger]
    fn test_attempt_match_deposit_with_mint_mismatched_safe_address(logger: Logger) {
        let mut config = test_gnosis_config().safes[0].clone();
        let mut rng = mc_util_test_helper::get_seeded_rng();
        let test_db_context = TestDbContext::default();
        let mint_auditor_db = test_db_context.get_db_instance(logger.clone());
        let conn = mint_auditor_db.get_conn().unwrap();

        let mut deposit1 = create_gnosis_safe_deposit(100, &mut rng);
        insert_gnosis_deposit(&mut deposit1, &conn);
        insert_mint_tx_from_deposit(&deposit1, &conn, &mut rng);

        config.safe_addr = EthAddr::from_str("0x0000000000000000000000000000000000000000").unwrap();
        assert!(matches!(
            AuditedMint::attempt_match_deposit_with_mint(&deposit1, &config, &conn),
            Err(Error::Other(_))
        ));

        // Check that nothing was written to the `audited_mints` table
        assert_audited_mints_table_is_empty(&conn);
    }

    #[test_with_logger]
    fn test_attempt_match_deposit_with_mint_mismatched_token_id(logger: Logger) {
        let mut config = test_gnosis_config().safes[0].clone();
        let mut rng = mc_util_test_helper::get_seeded_rng();
        let test_db_context = TestDbContext::default();
        let mint_auditor_db = test_db_context.get_db_instance(logger.clone());
        let conn = mint_auditor_db.get_conn().unwrap();

        let mut deposit1 = create_gnosis_safe_deposit(100, &mut rng);
        insert_gnosis_deposit(&mut deposit1, &conn);
        insert_mint_tx_from_deposit(&deposit1, &conn, &mut rng);

        config.tokens[0].token_id = TokenId::from(123);

        assert!(matches!(
            AuditedMint::attempt_match_deposit_with_mint(&deposit1, &config, &conn),
            Err(Error::DepositAndMintMismatch(_))
        ));

        // Check that nothing was written to the `audited_mints` table
        assert_audited_mints_table_is_empty(&conn);
    }

    #[test_with_logger]
    fn test_attempt_match_deposit_with_mint_mismatched_token_address(logger: Logger) {
        let mut config = test_gnosis_config().safes[0].clone();
        let mut rng = mc_util_test_helper::get_seeded_rng();
        let test_db_context = TestDbContext::default();
        let mint_auditor_db = test_db_context.get_db_instance(logger.clone());
        let conn = mint_auditor_db.get_conn().unwrap();

        let mut deposit1 = create_gnosis_safe_deposit(100, &mut rng);
        insert_gnosis_deposit(&mut deposit1, &conn);
        insert_mint_tx_from_deposit(&deposit1, &conn, &mut rng);

        config.tokens[0].eth_token_contract_addr =
            EthAddr::from_str("0x0000000000000000000000000000000000000000").unwrap();

        assert!(matches!(
            AuditedMint::attempt_match_deposit_with_mint(&deposit1, &config, &conn),
            Err(Error::Other(_))
        ));

        // Check that nothing was written to the `audited_mints` table
        assert_audited_mints_table_is_empty(&conn);
    }
}
