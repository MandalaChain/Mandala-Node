#[cfg(feature = "mandala-native")]
pub mod mandala;
#[cfg(feature = "niskala-native")]
pub mod niskala;

#[cfg(feature = "niskala-native")]
pub use niskala_runtime::{
    AccountId, AuraConfig, AuraExtConfig, BalancesConfig, BaseFeeConfig, CollatorSelectionConfig,
    EVMChainIdConfig, EVMConfig, EthereumConfig, ParachainInfoConfig, PolkadotXcmConfig,
    RuntimeGenesisConfig, SS58Prefix, SessionConfig, Signature, SudoConfig, SystemConfig,
    EXISTENTIAL_DEPOSIT, UNIT, WASM_BINARY,
};

#[cfg(feature = "mandala-native")]
pub use mandala_runtime::{
    AccountId, AuraConfig, AuraExtConfig, BalancesConfig, BaseFeeConfig, CollatorSelectionConfig,
    EVMChainIdConfig, EVMConfig, EthereumConfig, ParachainInfoConfig, PolkadotXcmConfig,
    RuntimeGenesisConfig, SS58Prefix, SessionConfig, Signature, SudoConfig, SystemConfig,
    EXISTENTIAL_DEPOSIT, UNIT, WASM_BINARY,
};

pub use cumulus_primitives_core::ParaId;
pub use fp_evm::GenesisAccount;
pub use sc_chain_spec::{ChainSpecBuilder, ChainSpecExtension, ChainSpecGroup};
pub use sc_network::config::MultiaddrWithPeerId;
pub use sc_service::{ChainType, GenericChainSpec, Properties};
pub use sc_telemetry::TelemetryEndpoints;
pub use serde::{Deserialize, Serialize};
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_core::{crypto::Ss58Codec, ecdsa, sr25519, Pair, Public, TypedGet, H160, U256};
pub use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiAddress,
};
pub use std::{collections::BTreeMap, marker::PhantomData};

#[macro_export(local_inner_macros)]
macro_rules! account_id {
    ($id:literal) => {
        AccountId::from(hex_literal::hex!($id))
    };
    () => {};
}
pub use crate::account_id;

// The URL for the telemetry server.
pub const DEFAULT_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

pub const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

#[cfg(feature = "niskala-native")]
pub fn template_session_keys(keys: AuraId) -> niskala_runtime::SessionKeys {
    niskala_runtime::SessionKeys { aura: keys }
}

#[cfg(feature = "mandala-native")]
pub fn template_session_keys(keys: AuraId) -> mandala_runtime::SessionKeys {
    mandala_runtime::SessionKeys { aura: keys }
}

pub trait CustomChainSpecProperties {
    fn wasm_binary() -> &'static [u8] {
        WASM_BINARY.expect("Development wasm not available")
    }

    fn token_symbol() -> &'static str;
    fn token_decimals() -> u8;
    fn evm_chain_id() -> u64;
    fn initial_balance() -> u128;

    fn extension() -> Extensions;

    fn chain_name() -> &'static str;

    // substrate environment chain identifier, e.g "dev", "local", "testnet", "mainnet"
    fn chain_identifier() -> &'static str;

    fn chain_type() -> ChainType;

    fn parachain_id() -> u32;

    /// extension for custom properties, override this if you have some custom chain spec properties
    fn chain_spec_properties_ext(mut chainspec_prop: Properties) -> Properties {
        chainspec_prop
    }

    fn default_chain_spec_properties() -> Properties {
        let mut properties = serde_json::map::Map::new();
        properties.insert("tokenSymbol".into(), Self::token_symbol().into());
        properties.insert("tokenDecimals".into(), (18).into());
        properties.insert("ss58Format".into(), (6629).into());
        properties.insert("isEthereum".into(), true.into());

        Self::chain_spec_properties_ext(properties)
    }

    fn chain_spec_prop() -> Properties {
        let mut default = Self::default_chain_spec_properties();
        Self::chain_spec_properties_ext(default)
    }

    // default prefunded accounts, override this if you have some custom prefunded accounts
    fn endowed_accounts() -> Vec<AccountId> {
        vec![
            // Balthar
            AccountId::from(hex_literal::hex!(
                "3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0"
            )),
            // Charleth
            AccountId::from(hex_literal::hex!(
                "798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc"
            )),
            // Dorothy
            AccountId::from(hex_literal::hex!(
                "773539d4Ac0e786233D90A233654ccEE26a613D9"
            )),
            // Ethan
            AccountId::from(hex_literal::hex!(
                "Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB"
            )),
            // Faith
            AccountId::from(hex_literal::hex!(
                "C0F0f4ab324C46e55D02D0033343B4Be8A55532d"
            )),
        ]
    }

    fn initial_authorities() -> Vec<AuraId> {
        // assuming for local testing the relay chain is alice and bob
        vec![
            authority_keys_from_seed("Charlie"),
            authority_keys_from_seed("Ferdie"),
        ]
    }

    fn enable_println() -> bool {
        true
    }

    fn root_key() -> AccountId {
        // Balthar
        AccountId::from(hex_literal::hex!(
            "3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0"
        ))
    }

    fn runtime_genesis_config() -> serde_json::Value {
        Self::testnet_genesis()
    }

    /// Configure initial storage state for FRAME modules.
    fn testnet_genesis() -> serde_json::Value {
        serde_json::json!({
        "parachainInfo":  {
            "parachainId": Into::<ParaId>::into( Self::parachain_id()),
            },
            "auraExt": AuraExtConfig {
                ..Default::default()
            },
            "collatorSelection": {
                "candidacyBond": EXISTENTIAL_DEPOSIT * 16,
                "invulnerables": Self::initial_authorities()
                    .iter()
                    .map(|x| Into::<AccountId>::into(x.clone().into_inner().0))
                    .collect::<Vec<_>>(),
                // TODO : idk bout this
                "desiredCandidates": 20,
            },
            "polkadotXcm":  {
                "safeXcmVersion": Some(SAFE_XCM_VERSION),
            },
            "session": SessionConfig {
                keys: Self::initial_authorities()
                    .iter()
                    .map(|x| {
                        (
                            x.clone().into_inner().0.into(),
                            x.clone().into_inner().0.into(),
                            template_session_keys(x.clone()),
                        )
                    })
                    .collect(),
            },
            "system": SystemConfig {
                ..Default::default()
            },
            "balances": BalancesConfig {
                balances: Self::endowed_accounts()
                    .iter()
                    .cloned()
                    .map(|k| (k, Self::initial_balance()))
                    .collect(),
            },
            "sudo": SudoConfig {
                // Assign network admin rights.
                key: Some(Self::root_key()),
            },
            "evm": EVMConfig {
                accounts: Self::get_evm_accounts(),
                ..Default::default()
            },
            "baseFee": BaseFeeConfig{
                ..Default::default()
            },
            "evmChainId": EVMChainIdConfig {
                chain_id: Self::evm_chain_id(),
                ..Default::default()
            }
        })
    }
    fn bootnodes() -> Vec<MultiaddrWithPeerId> {
        vec![]
    }

    fn telemetry_endpoints() -> TelemetryEndpoints {
        TelemetryEndpoints::new(vec![(String::from(DEFAULT_TELEMETRY_URL), 0)]).unwrap()
    }

    fn protocol_id() -> &'static str;

    fn fork_id() -> &'static str;

    fn build() -> ChainSpec {
        ChainSpecBuilder::new(Self::wasm_binary(), Self::extension())
            .with_name(Self::chain_name())
            .with_id(Self::chain_identifier())
            .with_chain_type(Self::chain_type())
            .with_genesis_config_patch(Self::runtime_genesis_config())
            .with_boot_nodes(Self::bootnodes())
            .with_telemetry_endpoints(Self::telemetry_endpoints())
            .with_protocol_id(Self::protocol_id())
            .with_fork_id(Self::fork_id())
            .with_properties(Self::chain_spec_prop())
            .build()
    }

    fn get_evm_accounts() -> BTreeMap<H160, fp_evm::GenesisAccount> {
        let accounts = Self::endowed_accounts();
        let mut map = BTreeMap::new();

        for account in accounts {
            let key = H160::from_slice(&account.0);

            let value = GenesisAccount {
                balance: U256::from(Self::initial_balance()),
                nonce: Default::default(),
                code: Default::default(),
                storage: Default::default(),
            };

            map.insert(key, value);
        }

        map
    }
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

pub type AccountPublic = <Signature as Verify>::Signer;

pub struct Account;

impl Account {
    pub fn get_from_seed_with_ecdsa(seed: &str) -> ecdsa::Public {
        ecdsa::Pair::from_string(&format!("//{}", seed), None)
            .expect("internal values are valid; qed")
            .public()
    }

    // truncate the first 20 bytes of the public key
    pub fn to_account_id_from_ecdsa(seed: ecdsa::Public) -> AccountId {
        let mut id = [0u8; 20];
        let seed_bytes: &[u8] = seed.as_ref();
        id.clone_from_slice(&seed_bytes[0..20]);

        mandala_primitives::AccountId20(id)
    }

    pub fn get_evm_compatible_account_id_from_seed(seed: &str) -> AccountId {
        Self::to_account_id_from_ecdsa(Self::get_from_seed_with_ecdsa(seed))
    }

    pub fn get_from_seed_with<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
        TPublic::Pair::from_string(&format!("//{}", seed), None)
            .expect("static values are valid; qed")
            .public()
    }
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> AuraId {
    Account::get_from_seed_with::<AuraId>(s)
}

pub fn authority_keys_from_public(s: &str) -> AuraId {
    AuraId::from_ss58check(s).expect("static values are valid; qed")
}
