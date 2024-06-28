use std::{ collections::BTreeMap, marker::PhantomData };

use fp_evm::GenesisAccount;
use niskala_runtime::{
    AccountId,
    AuraConfig,
    AuraExtConfig,
    BalancesConfig,
    CollatorSelectionConfig,
    EVMChainIdConfig,
    EVMConfig,
    EthereumConfig,
    ParachainInfoConfig,
    PolkadotXcmConfig,
    RuntimeGenesisConfig,
    SS58Prefix,
    SessionConfig,
    Signature,
    SudoConfig,
    SystemConfig,
    EXISTENTIAL_DEPOSIT,
    WASM_BINARY,
    UNIT,
};
use sc_chain_spec::{ ChainSpecExtension, ChainSpecGroup };
use sc_network::config::MultiaddrWithPeerId;
use sc_service::{ ChainType, GenericChainSpec, Properties };
use sc_telemetry::TelemetryEndpoints;
use serde::{ Serialize, Deserialize };
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{ ecdsa, sr25519, Pair, Public, TypedGet, H160, U256 };
use sp_runtime::{ traits::{ IdentifyAccount, Verify }, MultiAddress };

// The URL for the telemetry server.
const DEFAULT_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

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


pub fn template_session_keys(keys: AuraId) -> niskala_runtime::SessionKeys {
    niskala_runtime::SessionKeys { aura: keys }
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

    // default prefunded pandi accounts, override this if you have some custom prefunded accounts
    fn endowed_accounts() -> Vec<AccountId> {
        vec![
            // Pandi 1
            AccountId::from(hex_literal::hex!("4ef0028bb468B75B146228613d29CEFBb5181AF1")),
            // Pandi 2
            AccountId::from(hex_literal::hex!("dE242D3DA911275663C05F4A925d506edb40d8Db")),
            // Pandi 3
            AccountId::from(hex_literal::hex!("bd8b90Bf8B40E45FA9dfe27e0d8E22b3E2B2996F")),
            // Pandi 4
            AccountId::from(hex_literal::hex!("CF45e0788bB1b2F93ea188415f4C1f16e0a97c27")),
            // Pandi 5
            AccountId::from(hex_literal::hex!("ED509Bb407a82e23CB8135401406C18841725Da3")),
            // below are developments accounts

            // Balthar
            AccountId::from(hex_literal::hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")),
            // Charleth
            AccountId::from(hex_literal::hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")),
            // Dorothy
            AccountId::from(hex_literal::hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")),
            // Ethan
            AccountId::from(hex_literal::hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")),
            // Faith
            AccountId::from(hex_literal::hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d"))
        ]
    }

    fn initial_authorities() -> Vec<AuraId> {
        vec![authority_keys_from_seed("Alice")]
    }

    fn enable_println() -> bool {
        true
    }

    fn root_key() -> AccountId {
        // Balthar
        AccountId::from(hex_literal::hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0"))
    }

    fn runtime_genesis_config() -> RuntimeGenesisConfig {
        Self::testnet_genesis()
    }

    /// Configure initial storage state for FRAME modules.
    fn testnet_genesis() -> RuntimeGenesisConfig {
        RuntimeGenesisConfig {
            parachain_info: ParachainInfoConfig {
                parachain_id: Self::parachain_id().into(),
                ..Default::default()
            },
            aura_ext: AuraExtConfig {
                ..Default::default()
            },
            collator_selection: CollatorSelectionConfig {
                candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
                invulnerables: Self::initial_authorities()
                    .iter()
                    .map(|x| x.clone().into_inner().0.into())
                    .collect(),
                // TODO : idk bout this
                desired_candidates: 20,
            },
            parachain_system: Default::default(),
            polkadot_xcm: PolkadotXcmConfig {
                safe_xcm_version: Some(SAFE_XCM_VERSION),
                ..Default::default()
            },
            session: SessionConfig {
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
            system: SystemConfig {
                ..Default::default()
            },
            balances: BalancesConfig {
                balances: Self::endowed_accounts()
                    .iter()
                    .cloned()
                    .map(|k| (k, Self::initial_balance()))
                    .collect(),
            },
            aura: AuraConfig {
                authorities: Self::initial_authorities()
                    .iter()
                    .map(|x| x.clone())
                    .collect(),
            },
            sudo: SudoConfig {
                // Assign network admin rights.
                key: Some(Self::root_key()),
            },
            transaction_payment: Default::default(),
            ethereum: Default::default(),
            evm: EVMConfig {
                accounts: Self::get_evm_accounts(),
                ..Default::default()
            },
            base_fee: Default::default(),
            evm_chain_id: EVMChainIdConfig {
                chain_id: Self::evm_chain_id(),
                ..Default::default()
            },
        }
    }
    fn bootnodes() -> Vec<MultiaddrWithPeerId> {
        vec![]
    }

    fn telemetry_endpoints() -> Option<TelemetryEndpoints> {
        Some(TelemetryEndpoints::new(vec![(String::from(DEFAULT_TELEMETRY_URL), 0)]).unwrap())
    }

    fn protocol_id() -> Option<&'static str> {
        None
    }

    fn fork_id() -> Option<&'static str> {
        None
    }

    fn build() -> ChainSpec {
        ChainSpec::from_genesis(
            Self::chain_name(),
            Self::chain_identifier(),
            Self::chain_type(),
            move || Self::runtime_genesis_config(),
            Self::bootnodes(),
            Self::telemetry_endpoints(),
            Self::protocol_id(),
            Self::fork_id(),
            Some(Self::chain_spec_prop()),
            Self::extension(),
            Self::wasm_binary()
        )
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

// ----------------------------------------- Begin dev chainspec custom -----------------------------------------
pub struct Dev;
pub struct Live;
pub struct NodeChainSpec<Env>(PhantomData<Env>);

impl CustomChainSpecProperties for NodeChainSpec<Dev> {
    fn extension() -> Extensions {
        Extensions {
            relay_chain: "rococo-local".into(),
            para_id: 1000,
        }
    }

    fn parachain_id() -> u32 {
        1000
    }

    fn token_symbol() -> &'static str {
        "KPGD"
    }

    fn token_decimals() -> u8 {
        18
    }

    fn evm_chain_id() -> u64 {
        895670
    }

    fn initial_balance() -> u128 {
        1_000_000 * UNIT
    }

    fn chain_name() -> &'static str {
        "Niskala Dev"
    }

    fn chain_identifier() -> &'static str {
        "dev"
    }

    fn chain_type() -> ChainType {
        ChainType::Development
    }
}

// TODO
// impl CustomChainSpecProperties for NodeChainSpec<Live> {
//     fn token_symbol() -> &'static str {
//         "KPGD"
//     }

//     fn token_decimals() -> u8 {
//         18
//     }

//     fn evm_chain_id() -> u64 {
//         6025
//     }

//     fn initial_balance() -> u128 {
//         1_000_000 * KPGD
//     }

//     fn chain_name() -> &'static str {
//         "ID chain local"
//     }

//     fn chain_identifier() -> &'static str {
//         "local"
//     }

//     fn initial_authorities() -> Vec<(AuraId, GrandpaId)> {
//         vec![
//             authority_keys_from_seed("Alice"),
//             authority_keys_from_seed("Bob"),
//             authority_keys_from_seed("Charlie"),
//             authority_keys_from_seed("Ferdie")
//         ]
//     }

//     fn chain_type() -> ChainType {
//         ChainType::Local
//     }
// }
// ----------------------------------------- End   dev chainspec custom -----------------------------------------

/// we locked using sr25519 and derive the evm key from it
pub struct Account;

impl Account {
    pub fn get_from_seed_with_ecdsa(seed: &str) -> ecdsa::Public {
        ecdsa::Pair
            ::from_string(&format!("//{}", seed), None)
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

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> AuraId {
    Account::get_from_seed_with::<AuraId>(s)
}
