// Copyright (c) 2018-2022 The MobileCoin Foundation
#![deny(missing_docs)]

//! MobileCoin Fog View Router target
use grpcio::ChannelBuilder;
use mc_attest_net::{Client, RaClient};
use mc_common::{logger::log, time::SystemTimeProvider};
use mc_fog_api::view_grpc::FogViewStoreApiClient;
use mc_fog_view_enclave::{SgxViewEnclave, ENCLAVE_FILE};
use mc_fog_view_server::{
    config::FogViewRouterConfig, fog_view_router_server::FogViewRouterServer,
};
use mc_util_cli::ParserWithBuildInfo;
use mc_util_grpc::ConnectionUriGrpcioChannel;
use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};

fn main() {
    mc_common::setup_panic_handler();
    let (logger, _global_logger_guard) =
        mc_common::logger::create_app_logger(mc_common::logger::o!());
    let config = FogViewRouterConfig::parse();

    let enclave_path = env::current_exe()
        .expect("Could not get the path of our executable")
        .with_file_name(ENCLAVE_FILE);
    log::info!(
        logger,
        "enclave path {}, responder ID {}",
        enclave_path.to_str().unwrap(),
        &config.client_responder_id
    );
    let sgx_enclave = SgxViewEnclave::new(
        enclave_path,
        config.client_responder_id.clone(),
        config.omap_capacity,
        logger.clone(),
    );

    // TODO: Remove and get from a config.
    let mut fog_view_store_grpc_clients = HashMap::new();
    let grpc_env = Arc::new(
        grpcio::EnvBuilder::new()
            .name_prefix("Main-RPC".to_string())
            .build(),
    );
    for shard_uri in config.shard_uris.clone() {
        let fog_view_store_grpc_client = FogViewStoreApiClient::new(
            ChannelBuilder::default_channel_builder(grpc_env.clone())
                .connect_to_uri(&shard_uri, &logger),
        );
        fog_view_store_grpc_clients.insert(shard_uri, Arc::new(fog_view_store_grpc_client));
    }
    let fog_view_store_grpc_clients = Arc::new(RwLock::new(fog_view_store_grpc_clients));

    let ias_client = Client::new(&config.ias_api_key).expect("Could not create IAS client");
    let mut router_server = FogViewRouterServer::new(
        config,
        sgx_enclave,
        ias_client,
        fog_view_store_grpc_clients,
        SystemTimeProvider::default(),
        logger,
    );
    router_server.start();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
