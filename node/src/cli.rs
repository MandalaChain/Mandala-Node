use crate::eth::EthConfiguration;
use std::path::PathBuf;

/// Sub-commands supported by the collator.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// Remove the whole chain.
    PurgeChain(cumulus_client_cli::PurgeChainCmd),

    /// keystore related utilities
    #[command(subcommand)]
    Key(sc_cli::KeySubcommand),

    /// Export the genesis head data of the parachain.
    ///
    /// Head data is the encoded block header.
    #[command(alias = "export-genesis-state")]
    ExportGenesisHead(cumulus_client_cli::ExportGenesisHeadCommand),

    /// Export the genesis wasm of the parachain.
    ExportGenesisWasm(cumulus_client_cli::ExportGenesisWasmCommand),

    /// Sub-commands concerned with benchmarking.
    /// The pallet benchmarking moved to the `pallet` sub-command.
    #[command(subcommand)]
    Benchmark(Box<frame_benchmarking_cli::BenchmarkCmd>),

    /// Try-runtime has migrated to a standalone
    /// [CLI](<https://github.com/paritytech/try-runtime-cli>). The subcommand exists as a stub and
    /// deprecation notice. It will be removed entirely some time after Janurary 2024.
    TryRuntime,
}

const AFTER_HELP_EXAMPLE: &str = color_print::cstr!(
    r#"<bold><underline>Examples:</></>
   <bold>Mandala-node build-spec --disable-default-bootnode > plain-parachain-chainspec.json</>
           Export a chainspec for a local testnet in json format.
   <bold>Mandala-node --chain plain-parachain-chainspec.json --tmp -- --chain rococo-local</>
           Launch a full node with chain specification loaded from plain-parachain-chainspec.json.
   <bold>Mandala-node</>
           Launch a full node with default parachain <italic>local-testnet</> and relay chain <italic>rococo-local</>.
   <bold>Mandala-node --collator</>
           Launch a collator with default parachain <italic>local-testnet</> and relay chain <italic>rococo-local</>.
 "#
);

#[derive(Debug, clap::Parser)]
#[command(
    propagate_version = true,
    args_conflicts_with_subcommands = true,
    subcommand_negates_reqs = true
)]
#[command(after_help = AFTER_HELP_EXAMPLE)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[command(flatten)]
    pub run: cumulus_client_cli::RunCmd,

    /// Disable automatic hardware benchmarks.
    ///
    /// By default these benchmarks are automatically ran at startup and measure
    /// the CPU speed, the memory bandwidth and the disk speed.
    ///
    /// The results are then printed out in the logs, and also sent as part of
    /// telemetry, if telemetry is enabled.
    #[arg(long)]
    pub no_hardware_benchmarks: bool,

    /// Relay chain arguments
    #[arg(raw = true)]
    pub relay_chain_args: Vec<String>,

    // Frontier arguments
    #[command(flatten)]
    pub eth: EthConfiguration,
}

#[derive(Debug)]
pub struct RelayChainCli {
    /// The actual relay chain cli object.
    pub base: polkadot_cli::RunCmd,

    /// Optional chain id that should be passed to the relay chain.
    pub chain_id: Option<String>,

    /// The base path that should be used by the relay chain.
    pub base_path: Option<PathBuf>,
}

impl RelayChainCli {
    /// Parse the relay chain CLI parameters using the para chain `Configuration`.
    pub fn new<'a>(
        para_config: &sc_service::Configuration,
        relay_chain_args: impl Iterator<Item = &'a String>,
    ) -> Self {
        let extension = {
            #[cfg(feature = "niskala-native")]
            {
                crate::chain_spec::niskala::Extensions::try_get(&*para_config.chain_spec)
            }

            #[cfg(feature = "mandala-native")]
            {
                crate::chain_spec::mandala::Extensions::try_get(&*para_config.chain_spec)
            }
        };
        let chain_id = extension.map(|e| e.relay_chain.clone());
        let base_path = Some(para_config.base_path.path().join("polkadot"));
        Self {
            base_path,
            chain_id,
            base: clap::Parser::parse_from(relay_chain_args),
        }
    }
}
