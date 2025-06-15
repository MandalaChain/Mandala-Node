use std::net::SocketAddr;

use codec::Encode;
use cumulus_primitives_core::ParaId;
use fc_db::kv::frontier_database_dir;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
#[cfg(feature = "mandala-native")]
use mandala_runtime::Block;
#[cfg(feature = "niskala-native")]
use niskala_runtime::Block;

use log::info;
use sc_cli::{
    ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
    NetworkParams, Result, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::{
    config::{BasePath, PrometheusConfig},
    DatabaseSource,
};
use sp_runtime::{
    traits::{AccountIdConversion, Block as BlockT, HashingFor, Zero},
    StateVersion,
};

#[cfg(feature = "mandala-native")]
use crate::chain_spec::mandala::{Dev, NodeChainSpec};
#[cfg(feature = "niskala-native")]
use crate::chain_spec::niskala::{Dev, Live, NodeChainSpec};
#[cfg(feature = "try-runtime")]
use crate::service::ParachainNativeExecutor;

use crate::{
    chain_spec::{self},
    cli::{Cli, RelayChainCli, Subcommand},
    eth::db_config_dir,
    service::new_partial,
};

fn load_spec(id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
    Ok(match id {
        #[cfg(feature = "niskala-native")]
        "dev" => Box::new(
            <NodeChainSpec<Dev> as chain_spec::niskala::CustomChainSpecProperties>::build(),
        ),
        #[cfg(feature = "niskala-native")]
        "paseo" => Box::new(
            <NodeChainSpec<Live> as chain_spec::niskala::CustomChainSpecProperties>::build(),
        ),

        #[cfg(feature = "mandala-native")]
        "dev" => Box::new(
            <NodeChainSpec<Dev> as chain_spec::mandala::CustomChainSpecProperties>::build(),
        ),
        path => {
            #[cfg(feature = "mandala-native")]
            {
                Box::new(chain_spec::mandala::ChainSpec::from_json_file(
                    std::path::PathBuf::from(path),
                )?)
            }

            #[cfg(feature = "niskala-native")]
            {
                Box::new(chain_spec::niskala::ChainSpec::from_json_file(
                    std::path::PathBuf::from(path),
                )?)
            }
        }
    })
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        #[cfg(feature = "niskala-native")]
        {
            "Niskala Collator".into()
        }
        #[cfg(feature = "mandala-native")]
        {
            "Mandala Collator".into()
        }
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Mandala Collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		{} <parachain-args> -- <relay-chain-args>",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/MandalaChain/Mandala-Node/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2024
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id)
    }
}

impl SubstrateCli for RelayChainCli {
    fn impl_name() -> String {
        #[cfg(feature = "niskala-native")]
        {
            "Niskala Collator".into()
        }

        #[cfg(feature = "mandala-native")]
        {
            "Mandala Collator".into()
        }
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Mandala Collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		{} <parachain-args> -- <relay-chain-args>",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/MandalaChain/Mandala-Node/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2024
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
    }
}

macro_rules! construct_async_run {
    (
        | $components:ident,
        $cli:ident,
        $cmd:ident,
        $config:ident,
        $eth_config:ident | $($code:tt)*
    ) => {
        {
		let runner = $cli.create_runner($cmd)?;
		runner.async_run(|mut $config| {
			let $components = new_partial(&mut $config, &$eth_config)?;
			let task_manager = $components.task_manager;
			{ $( $code )* }.map(|v| (v, task_manager))
		})
        }
    };
}
impl Cli {
    #[allow(dead_code)]
    fn runtime_version(spec: &dyn sc_service::ChainSpec) -> &'static RuntimeVersion {
        match spec {
            #[cfg(feature = "niskala-native")]
            _ => &niskala_runtime::VERSION,
            #[cfg(feature = "mandala-native")]
            _ => &mandala_runtime::VERSION,
        }
    }
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();
    let eth_cfg = cli.eth.clone();

    match &cli.subcommand {
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),

        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            construct_async_run!(|components, cli, cmd, config, eth_cfg| {
                Ok(cmd.run(components.client, components.import_queue))
            })
        }

        Some(Subcommand::ExportState(cmd)) => {
            construct_async_run!(|components, cli, cmd, config, eth_cfg| {
                Ok(cmd.run(components.client, config.chain_spec))
            })
        }

        Some(Subcommand::ExportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config, eth_cfg| {
                Ok(cmd.run(components.client, config.database))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config, eth_cfg| {
                Ok(cmd.run(components.client, components.import_queue))
            })
        }
        Some(Subcommand::Revert(cmd)) => {
            construct_async_run!(|components, cli, cmd, config, eth_cfg| {
                Ok(cmd.run(components.client, components.backend, None))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| {
                // Remove Frontier offchain db
                let db_config_dir = db_config_dir(&config);
                match cli.eth.frontier_backend_type {
                    crate::eth::BackendType::KeyValue => {
                        let frontier_database_config = match config.database {
                            DatabaseSource::RocksDb { .. } => DatabaseSource::RocksDb {
                                path: frontier_database_dir(&db_config_dir, "db"),
                                cache_size: 0,
                            },
                            DatabaseSource::ParityDb { .. } => DatabaseSource::ParityDb {
                                path: frontier_database_dir(&db_config_dir, "paritydb"),
                            },
                            _ => {
                                return Err(format!(
                                    "Cannot purge `{:?}` database",
                                    config.database
                                )
                                .into());
                            }
                        };
                        cmd.base.run(frontier_database_config)?;
                    }
                    crate::eth::BackendType::Sql => {
                        let db_path = db_config_dir.join("sql");
                        match std::fs::remove_dir_all(&db_path) {
                            Ok(_) => {
                                println!("{:?} removed.", &db_path);
                            }
                            Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => {
                                eprintln!("{:?} did not exist.", &db_path);
                            }
                            Err(err) => {
                                return Err(format!(
                                    "Cannot purge `{:?}` database: {:?}",
                                    db_path, err
                                )
                                .into());
                            }
                        };
                    }
                }

                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relay_chain_args.iter()),
                );

                let polkadot_config = SubstrateCli::create_configuration(
                    &polkadot_cli,
                    &polkadot_cli,
                    config.tokio_handle.clone(),
                )
                .map_err(|err| format!("Relay chain argument error: {}", err))?;

                cmd.run(config, polkadot_config)
            })
        }
        Some(Subcommand::ExportGenesisHead(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|mut config| {
                let partials = new_partial(&mut config, &eth_cfg)?;
                let _spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                let _ = cmd.run(partials.client);

                Ok(())
            })
        }
        Some(Subcommand::ExportGenesisWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                cmd.run(&*spec)
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(&**cmd)?;
            // Switch on the concrete benchmark sub-command-
            match &**cmd {
                BenchmarkCmd::Pallet(cmd) => {
                    if cfg!(feature = "runtime-benchmarks") {
                        runner.sync_run(|config| {
                            #[allow(deprecated)]
                            cmd.run::<HashingFor<Block>, crate::service::HostFunctions>(config)
                        })
                    } else {
                        Err("Benchmarking wasn't enabled when building the node. \
					You can enable it with `--features runtime-benchmarks`."
                            .into())
                    }
                }
                BenchmarkCmd::Block(cmd) => runner.sync_run(|mut config| {
                    let partials = new_partial(&mut config, &eth_cfg)?;
                    cmd.run(partials.client)
                }),
                #[cfg(not(feature = "runtime-benchmarks"))]
                BenchmarkCmd::Storage(_) => Err(sc_cli::Error::Input(
                    "Compile with --features=runtime-benchmarks \
						to enable storage benchmarks."
                        .into(),
                )),
                #[cfg(feature = "runtime-benchmarks")]
                BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
                    let partials = new_partial(&mut config, &eth_cfg)?;
                    let db = partials.backend.expose_db();
                    let storage = partials.backend.expose_storage();
                    cmd.run(config, partials.client.clone(), db, storage)
                }),
                BenchmarkCmd::Machine(cmd) => {
                    runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
                }
                // NOTE: this allows the Client to leniently implement
                // new benchmark commands without requiring a companion MR.
                #[allow(unreachable_patterns)]
                _ => Err("Benchmarking sub-command unsupported".into()),
            }
        }
        #[cfg(feature = "try-runtime")]
        Some(Subcommand::TryRuntime(cmd)) => {
            use frontier_parachain_runtime::MILLISECS_PER_BLOCK;
            use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
            use try_runtime_cli::block_building_info::timestamp_with_aura_info;

            let runner = cli.create_runner(cmd)?;

            type HostFunctionsOf<E> = ExtendedHostFunctions<
                sp_io::SubstrateHostFunctions,
                <E as NativeExecutionDispatch>::ExtendHostFunctions,
            >;

            // grab the task manager.
            let registry = &runner
                .config()
                .prometheus_config
                .as_ref()
                .map(|cfg| &cfg.registry);
            let task_manager =
                sc_service::TaskManager::new(runner.config().tokio_handle.clone(), *registry)
                    .map_err(|e| format!("Error: {:?}", e))?;
            let info_provider = timestamp_with_aura_info(MILLISECS_PER_BLOCK);

            runner.async_run(|_| {
                Ok((
                    cmd.run::<Block, HostFunctionsOf<ParachainNativeExecutor>, _>(Some(
                        info_provider,
                    )),
                    task_manager,
                ))
            })
        }
        #[cfg(not(feature = "try-runtime"))]
        Some(Subcommand::TryRuntime) => Err("Try-runtime was not enabled when building the node. \
			You can enable it with `--features try-runtime`."
            .into()),

        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            let collator_options = cli.run.collator_options();

            runner.run_node_until_exit(|config| async move {
                let hwbench = (!cli.no_hardware_benchmarks)
                    .then_some(config.database.path().map(|database_path| {
                        let _ = std::fs::create_dir_all(database_path);
                        sc_sysinfo::gather_hwbench(Some(database_path))
                    }))
                    .flatten();

                let para_id = {
                    #[cfg(feature = "niskala-native")]
                    {
                        chain_spec::niskala::Extensions::try_get(&*config.chain_spec)
                            .map(|e| e.para_id)
                            .ok_or("Could not find parachain ID in chain-spec.")?
                    }

                    #[cfg(feature = "mandala-native")]
                    {
                        chain_spec::mandala::Extensions::try_get(&*config.chain_spec)
                            .map(|e| e.para_id)
                            .ok_or("Could not find parachain ID in chain-spec.")?
                    }

                    #[cfg(not(any(feature = "niskala-native", feature = "mandala-native")))]
                    {
                        return Err("No runtime feature enabled".to_string());
                    }
                };

                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relay_chain_args.iter()),
                );

                let id = ParaId::from(para_id);

                let parachain_account =
                    AccountIdConversion::<polkadot_primitives::AccountId>::into_account_truncating(
                        &id,
                    );

                let tokio_handle = config.tokio_handle.clone();
                let polkadot_config =
                    SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
                        .map_err(|err| format!("Relay chain argument error: {}", err))?;

                info!("Parachain id: {:?}", id);
                info!("Parachain Account: {}", parachain_account);
                info!(
                    "Is collating: {}",
                    if config.role.is_authority() {
                        "yes"
                    } else {
                        "no"
                    }
                );

                crate::service::start_parachain_node(
                    config,
                    polkadot_config,
                    eth_cfg,
                    collator_options,
                    id,
                    hwbench,
                )
                .await
                .map(|r| r.0)
                .map_err(Into::into)
            })
        }
    }
}

impl DefaultConfigurationValues for RelayChainCli {
    fn p2p_listen_port() -> u16 {
        30334
    }

    fn rpc_listen_port() -> u16 {
        9945
    }

    fn prometheus_listen_port() -> u16 {
        9616
    }
}

impl CliConfiguration<Self> for RelayChainCli {
    fn shared_params(&self) -> &SharedParams {
        self.base.base.shared_params()
    }

    fn import_params(&self) -> Option<&ImportParams> {
        self.base.base.import_params()
    }

    fn network_params(&self) -> Option<&NetworkParams> {
        self.base.base.network_params()
    }

    fn keystore_params(&self) -> Option<&KeystoreParams> {
        self.base.base.keystore_params()
    }

    fn base_path(&self) -> Result<Option<BasePath>> {
        Ok(self
            .shared_params()
            .base_path()?
            .or_else(|| self.base_path.clone().map(Into::into)))
    }

    fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_addr(default_listen_port)
    }

    fn prometheus_config(
        &self,
        default_listen_port: u16,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<PrometheusConfig>> {
        self.base
            .base
            .prometheus_config(default_listen_port, chain_spec)
    }

    fn init<F>(
        &self,
        _support_url: &String,
        _impl_version: &String,
        _logger_hook: F,
        _config: &sc_service::Configuration,
    ) -> Result<()>
    where
        F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
    {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, is_dev: bool) -> Result<String> {
        let chain_id = self.base.base.chain_id(is_dev)?;

        Ok(if chain_id.is_empty() {
            self.chain_id.clone().unwrap_or_default()
        } else {
            chain_id
        })
    }

    fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
        self.base.base.role(is_dev)
    }

    fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
        self.base.base.transaction_pool(is_dev)
    }

    fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
        self.base.base.trie_cache_maximum_size()
    }

    fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
        self.base.base.rpc_methods()
    }

    fn rpc_max_connections(&self) -> Result<u32> {
        self.base.base.rpc_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
        self.base.base.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> Result<Option<u64>> {
        self.base.base.default_heap_pages()
    }

    fn force_authoring(&self) -> Result<bool> {
        self.base.base.force_authoring()
    }

    fn disable_grandpa(&self) -> Result<bool> {
        self.base.base.disable_grandpa()
    }

    fn max_runtime_instances(&self) -> Result<Option<usize>> {
        self.base.base.max_runtime_instances()
    }

    fn announce_block(&self) -> Result<bool> {
        self.base.base.announce_block()
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.base.base.telemetry_endpoints(chain_spec)
    }

    fn node_name(&self) -> Result<String> {
        self.base.base.node_name()
    }
}

/// Generate the genesis block from a given ChainSpec.
#[allow(dead_code)]
pub fn generate_genesis_block<Block: BlockT>(
    chain_spec: &dyn ChainSpec,
    genesis_state_version: StateVersion,
) -> std::result::Result<Block, String> {
    let storage = chain_spec.build_storage()?;

    let child_roots = storage.children_default.iter().map(|(sk, child_content)| {
        let state_root =
            <<<Block as BlockT>::Header as sp_runtime::traits::Header>::Hashing as sp_runtime::traits::Hash>::trie_root(
                child_content.data.clone().into_iter().collect(),
                genesis_state_version
            );
        (sk.clone(), state_root.encode())
    });
    let state_root =
        <<<Block as BlockT>::Header as sp_runtime::traits::Header>::Hashing as sp_runtime::traits::Hash>::trie_root(
            storage.top.clone().into_iter().chain(child_roots).collect(),
            genesis_state_version
        );

    let extrinsics_root =
        <<<Block as BlockT>::Header as sp_runtime::traits::Header>::Hashing as sp_runtime::traits::Hash>::trie_root(
            Vec::new(),
            genesis_state_version
        );

    Ok(Block::new(
        <<Block as BlockT>::Header as sp_runtime::traits::Header>::new(
            Zero::zero(),
            extrinsics_root,
            state_root,
            Default::default(),
            Default::default(),
        ),
        Default::default(),
    ))
}
