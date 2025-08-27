//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

// FIXME : change to use the actual block type when configuring mandala
#[cfg(all(feature = "niskala-native", not(feature = "mandala-native")))]
use niskala_runtime::{opaque::Block, AccountId, Balance, Nonce};

#[cfg(feature = "mandala-native")]
use mandala_runtime::{opaque::Block, AccountId, Balance, Nonce};

use sc_client_api::{
    backend::{AuxStore, Backend, StorageProvider},
    client::BlockchainEvents,
    UsageProvider,
};
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool::ChainApi;
use sp_api::{CallApiAt, ProvideRuntimeApi};
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::traits::Block as BlockT;
use substrate_frame_rpc_system::SystemApiServer;

mod eth;
pub use self::eth::{create_eth, EthDeps};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Full client dependencies
/// Full client dependencies
pub struct FullDeps<C, P, A: ChainApi, CT, CIDP> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// Ethereum-compatibility specific dependencies.
    pub eth: EthDeps<Block, C, P, A, CT, CIDP>,
}
pub struct DefaultEthConfig<C, BE>(std::marker::PhantomData<(C, BE)>);

impl<C, BE> fc_rpc::EthConfig<Block, C> for DefaultEthConfig<C, BE>
where
    C: StorageProvider<Block, BE> + Sync + Send + 'static,
    BE: Backend<Block> + 'static,
{
    type EstimateGasAdapter = ();
    type RuntimeStorageOverride =
        fc_rpc::frontier_backend_client::SystemAccountId32StorageOverride<Block, C, BE>;
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, A: ChainApi, BE, CT, CIDP>(
    deps: FullDeps<C, P, A, CT, CIDP>,
    subscription_task_executor: SubscriptionTaskExecutor,
    pubsub_notification_sinks: Arc<
        fc_mapping_sync::EthereumBlockNotificationSinks<
            fc_mapping_sync::EthereumBlockNotification<Block>,
        >,
    >,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + AuxStore
        + HeaderMetadata<Block, Error = BlockChainError>
        + Send
        + Sync
        + CallApiAt<Block>
        + UsageProvider<Block>
        + StorageProvider<Block, BE>
        + BlockchainEvents<Block>
        + 'static,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: sp_consensus_aura::AuraApi<Block, AuraId>,
    C::Api: sp_block_builder::BlockBuilder<Block>,
    C::Api: fp_rpc::ConvertTransactionRuntimeApi<Block>,
    C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
    P: sc_transaction_pool_api::TransactionPool<Block = Block> + Sync + Send + 'static,
    A: ChainApi<Block = Block> + 'static,
    CIDP: sp_inherents::CreateInherentDataProviders<Block, ()> + Send + 'static,
    CT: fp_rpc::ConvertTransaction<<Block as BlockT>::Extrinsic> + Send + Sync + 'static,
    BE: Backend<Block> + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::System;

    let mut io = RpcExtension::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe: _,
        eth,
    } = deps;

    io.merge(System::new(client.clone(), pool.clone()).into_rpc())?;
    io.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    // Ethereum compatibility RPCs
    let io = create_eth::<Block, C, BE, P, A, CT, CIDP, DefaultEthConfig<C, BE>>(
        io,
        eth,
        subscription_task_executor,
        pubsub_notification_sinks,
    )?;
    Ok(io)
}
