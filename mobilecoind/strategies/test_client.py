#!/usr/bin/env python3

# Copyright (c) 2018-2022 The MobileCoin Foundation

"""
The purpose of this strategy is to test client behavior with a mobilecoind wallet.

Example setup and usage:
```
    python3 test_client.py --key-dir ../../../target/sample_data/master/keys/
```
"""
import argparse
import glob
import logging
import mobilecoind_api_pb2
import os
import sys
import time
from accounts import connect, load_key_and_register, poll, wait_for_accounts_sync, TransferStatus
from google.protobuf.empty_pb2 import Empty

logging.basicConfig(stream = sys.stdout, level = logging.INFO, format="%(levelname)s:%(module)s:%(lineno)s: %(message)s")


def parse_args() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mobilecoind-host",
                        default="localhost",
                        type=str,
                        help="Mobilecoind host")
    parser.add_argument("--mobilecoind-port",
                        default="4444",
                        type=str,
                        help="Mobilecoind port")
    parser.add_argument("--key-dir",
                        type=str,
                        help="Path to directory of account_keys")
    parser.add_argument("--max-seconds",
                        type=int,
                        default=40,
                        help="Number of seconds to wait for a tx to clear")
    return parser.parse_args()


def run_test(stub, amount, monitor_id, dest, max_seconds):
    tx_stats = {}
    sync_start = time.time()
    wait_for_accounts_sync(stub, [monitor_id, dest.monitor_id], 3)
    logging.info("Time to sync: %s", time.time() - sync_start)

    resp = stub.GetBalance(
        mobilecoind_api_pb2.GetBalanceRequest(monitor_id=monitor_id))
    starting_balance = resp.balance
    logging.info("Starting balance prior to transfer: %s", starting_balance)

    tx_resp = stub.SendPayment(
        mobilecoind_api_pb2.SendPaymentRequest(
            sender_monitor_id=monitor_id,
            sender_subaddress=0,
            outlay_list=[
                mobilecoind_api_pb2.Outlay(
                    value=amount,
                    receiver=dest.public_address,
                )
            ],
            fee=0,
            tombstone=0,
        ))

    tx_stats[0] = {
        'start': time.time(),
        'time_delta': None,
        'tombstone': tx_resp.sender_tx_receipt.tombstone,
        'block_delta': None,
        'status': TransferStatus.pending,
        'receipt': tx_resp,
    }
    stats = poll(monitor_id, tx_stats, stub)
    # FIXME: Move max seconds check inside polling
    assert tx_stats[0]['time_delta'] < max_seconds, "Did not clear in time"
    assert tx_stats[0]['status'] == TransferStatus.success, "Transfer did not succeed"
    return stats


if __name__ == '__main__':
    args = parse_args()
    logging.debug(args)

    stub = connect(args.mobilecoind_host, args.mobilecoind_port)
    accounts = [
        load_key_and_register(k, stub)
        for k in sorted(glob.glob(os.path.join(args.key_dir, '*.json')))
    ]

    monitor_ids = [a.monitor_id for a in accounts]
    # Go through each account and have all their friends transact to them
    for i, account_data in enumerate(accounts):
        wait_for_accounts_sync(stub, monitor_ids, 3)
        # Get starting balance
        resp = stub.GetBalance(
            mobilecoind_api_pb2.GetBalanceRequest(monitor_id=account_data.monitor_id))
        balance = resp.balance
        logging.info("Starting balance for account %s: %s", i, resp.balance)

        # Note: due to the transaction fee, we can't assume we have enough funds
        # to divide equally among all our friends, so add an extra factor.
        amount = 10  # int(balance / (len(accounts)*10))

        # Create a pool of transfers to all other accounts
        logging.info("Transferring %s each to %s accounts", amount, len(accounts))

        # FIXME: no reason it can't also send to itself
        src_accounts = {a.monitor_id for a in accounts}
        src_accounts.remove(account_data.monitor_id)

        for i, src in enumerate(src_accounts):
            stats = run_test(stub, amount, src, account_data, args.max_seconds)
            logging.info("Test %s succeeded", i)
            logging.debug(stats)

    logging.info("All transfers successful")
