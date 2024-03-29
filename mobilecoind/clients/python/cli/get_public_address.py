#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# Copyright (c) 2018-2022 The MobileCoin Foundation

""" displays the b58 public address and URI that correspond to a master key """

import argparse

import os,sys
sys.path.insert(1, os.path.realpath(os.path.join(os.path.pardir, "lib")))
import mobilecoin

if __name__ == '__main__':
    # Connect to mobilecoind
    mobilecoind = mobilecoin.Client("localhost:4444", ssl=False)

    # Parse the arguments
    parser = argparse.ArgumentParser(
        description='Displays public address information for a provided master key, or for a random master key if no key is provided.')
    parser.add_argument('-k', '--key', help='account master key', type=str)
    parser.add_argument('-m', '--mnemonic', help='account key as mnemonic string', type=str)
    parser.add_argument('-s', '--subaddress', help='(optional) subaddress', nargs='?', const=mobilecoin.DEFAULT_SUBADDRESS_INDEX, type=int, default=mobilecoin.DEFAULT_SUBADDRESS_INDEX)
    args = parser.parse_args()

    # create a monitor and use it to calculate the public address
    if args.key:
        entropy_bytes = bytes.fromhex(args.key) if args.key else mobilecoind.generate_entropy()
        account_key = mobilecoind.get_account_key(entropy_bytes).account_key
        entropy_display = entropy_bytes.hex()
    else:
        mnemonic = args.mnemonic if args.mnemonic else mobilecoind.generate_mnemonic()
        account_key = mobilecoind.get_account_key_from_mnemonic(mnemonic)
        entropy_display = mnemonic
    monitor_id = mobilecoind.add_monitor(account_key, first_subaddress=args.subaddress).monitor_id
    public_address = mobilecoind.get_public_address(monitor_id, subaddress_index=args.subaddress).public_address

    # print the public address information
    print("\n")
    print("    {:<18}{}".format("Master Key:", entropy_display))
    print("    {:<18}{}".format("Subaddress Index:", args.subaddress))
    print("    {:<18}{}".format("Address Code:", public_address.b58_code))
    print("    {:<18}{}".format("Address URL:", "mob58://"+ public_address.b58_code))
    print("\n")
