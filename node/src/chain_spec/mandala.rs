use cumulus_primitives_core::ParaId;
use mandala_runtime::{ AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT };
use sc_chain_spec::{ ChainSpecExtension, ChainSpecGroup };
use sc_service::ChainType;
use serde::{ Deserialize, Serialize };
use sp_core::{ sr25519, Pair, Public };
use sp_runtime::traits::{ IdentifyAccount, Verify };
pub use super::*;

pub struct Dev;
// TODO : for @AnggaDanarP
pub struct Live;
pub struct NodeChainSpec<Env>(PhantomData<Env>);

impl CustomChainSpecProperties for NodeChainSpec<Dev> {
    fn extension() -> Extensions {
        Extensions {
            relay_chain: "rococo-local".into(),
            para_id: 2000,
        }
    }

    fn parachain_id() -> u32 {
        2000
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
        ChainType::Local
    }

    fn protocol_id() -> &'static str {
        "template-local"
    }

    fn fork_id() -> &'static str {
        "template-local"
    }
}

// TODO
impl CustomChainSpecProperties for NodeChainSpec<Live> {
    fn token_symbol() -> &'static str {
        "KPGT"
    }

    fn token_decimals() -> u8 {
        18
    }

    fn evm_chain_id() -> u64 {
        6025
    }

    fn initial_balance() -> u128 {
        100_000_000 * UNIT
    }

    fn chain_name() -> &'static str {
        "Niskala"
    }

    fn chain_identifier() -> &'static str {
        "live"
    }
    fn initial_authorities() -> Vec<AuraId> {
        vec![
            // collator 1
            authority_keys_from_public("5HMa8oTYwr5viSwQBSbWgM7vxxiCcgLUgSbcumExjEyJ8sTr"),
            // collator 2
            authority_keys_from_public("5HTaZj7BtHFN5NsK5CYcK99ZPmH8ESz78hybbjmftKsCKyn1")
        ]
    }

    fn chain_type() -> ChainType {
        ChainType::Live
    }

    fn root_key() -> AccountId {
        account_id!("Cea1fA4027315dEfC217054bc16c97C3527d9A0E")
    }

    fn endowed_accounts() -> Vec<AccountId> {
        vec![
            // collator 1
            account_id!("B14fAa1D5a6213BF946C51FCC0097C5E40B7758A"),
            // collator 2
            account_id!("fb8d71863b415DC999C4f475A229aFa147c786e4"),
            // sudo
            account_id!("Cea1fA4027315dEfC217054bc16c97C3527d9A0E"),
            // team
            account_id!("cf34cEfE42aB033Db814639f72EA37baD3e82219"),
            // foundation
            account_id!("e6D8A2F367250bc677a3D566E3Aeb526697C7399")
        ]
    }

    fn extension() -> Extensions {
        Extensions {
            para_id: 4022,
            relay_chain: "paseo".into(),
        }
    }

    fn parachain_id() -> u32 {
        4022
    }

    fn protocol_id() -> &'static str {
        "niskala/live"
    }

    fn fork_id() -> &'static str {
        "niskala/live"
    }
}
// ----------------------------------------- End   dev chainspec custom -----------------------------------------





// /// Specialized `ChainSpec` for the normal parachain runtime.
// pub type ChainSpec = sc_service::GenericChainSpec<(), Extensions>;

// /// The default XCM version to set in genesis config.
// const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

// /// Helper function to generate a crypto pair from seed
// pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
//     TPublic::Pair::from_string(&format!("//{}", seed), None)
//         .expect("static values are valid; qed")
//         .public()
// }

// /// The extensions for the [`ChainSpec`].
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
// #[serde(deny_unknown_fields)]
// pub struct Extensions {
//     /// The relay chain of the Parachain.
//     pub relay_chain: String,
//     /// The id of the Parachain.
//     pub para_id: u32,
// }

// impl Extensions {
//     /// Try to get the extension from the given `ChainSpec`.
//     pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
//         sc_chain_spec::get_extension(chain_spec.extensions())
//     }
// }

// type AccountPublic = <Signature as Verify>::Signer;

// /// Generate collator keys from seed.
// ///
// /// This function's return type must always match the session keys of the chain in tuple format.
// pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
//     get_from_seed::<AuraId>(seed)
// }

// /// Helper function to generate an account ID from seed
// pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
//     where AccountPublic: From<<TPublic::Pair as Pair>::Public>
// {
//     AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
// }

// /// Generate the session keys from individual elements.
// ///
// /// The input must be a tuple of individual keys (a single arg for now since we have just one key).
// pub fn template_session_keys(keys: AuraId) -> mandala_runtime::SessionKeys {
//     mandala_runtime::SessionKeys { aura: keys }
// }

// pub fn development_config() -> ChainSpec {
//     // Give your base currency a unit name and decimal places
//     let mut properties = sc_chain_spec::Properties::new();
//     properties.insert("tokenSymbol".into(), "KPGD".into());
//     properties.insert("tokenDecimals".into(), (18).into());
//     properties.insert("ss58Format".into(), (6629).into());

//     ChainSpec::builder(
//         mandala_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
//         Extensions {
//             relay_chain: "rococo-local".into(),
//             // You MUST set this to the correct network!
//             para_id: 1000,
//         }
//     )
//         .with_name("Development")
//         .with_id("dev")
//         .with_chain_type(ChainType::Development)
//         .with_genesis_config_patch(
//             testnet_genesis(
//                 // initial collators.
//                 vec![
//                     (
//                         get_account_id_from_seed::<sr25519::Public>("Alice"),
//                         get_collator_keys_from_seed("Alice"),
//                     ),
//                     (
//                         get_account_id_from_seed::<sr25519::Public>("Bob"),
//                         get_collator_keys_from_seed("Bob"),
//                     )
//                 ],
//                 vec![
//                     get_account_id_from_seed::<sr25519::Public>("Alice"),
//                     get_account_id_from_seed::<sr25519::Public>("Bob"),
//                     get_account_id_from_seed::<sr25519::Public>("Charlie"),
//                     get_account_id_from_seed::<sr25519::Public>("Dave"),
//                     get_account_id_from_seed::<sr25519::Public>("Eve"),
//                     get_account_id_from_seed::<sr25519::Public>("Ferdie"),
//                     get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
//                     get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
//                     get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
//                     get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
//                     get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
//                     get_account_id_from_seed::<sr25519::Public>("Ferdie//stash")
//                 ],
//                 get_account_id_from_seed::<sr25519::Public>("Alice"),
//                 (1000).into()
//             )
//         )
//         .with_properties(properties)
//         .build()
// }

// pub fn local_testnet_config() -> ChainSpec {
//     let mut properties = sc_chain_spec::Properties::new();
//     properties.insert("tokenSymbol".into(), "KPGD".into());
//     properties.insert("tokenDecimals".into(), (18).into());
//     properties.insert("ss58Format".into(), (6629).into());

//     #[allow(deprecated)]
//     ChainSpec::builder(
//         mandala_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
//         Extensions {
//             relay_chain: "rococo-local".into(),
//             // You MUST set this to the correct network!
//             para_id: 1000,
//         }
//     )
//         .with_name("Local Testnet")
//         .with_id("local_testnet")
//         .with_chain_type(ChainType::Local)
//         .with_genesis_config_patch(
//             testnet_genesis(
//                 // initial collators.
//                 vec![
//                     (
//                         get_account_id_from_seed::<sr25519::Public>("Alice"),
//                         get_collator_keys_from_seed("Alice"),
//                     ),
//                     (
//                         get_account_id_from_seed::<sr25519::Public>("Bob"),
//                         get_collator_keys_from_seed("Bob"),
//                     )
//                 ],
//                 vec![
//                     get_account_id_from_seed::<sr25519::Public>("Alice"),
//                     get_account_id_from_seed::<sr25519::Public>("Bob"),
//                     get_account_id_from_seed::<sr25519::Public>("Charlie"),
//                     get_account_id_from_seed::<sr25519::Public>("Dave"),
//                     get_account_id_from_seed::<sr25519::Public>("Eve"),
//                     get_account_id_from_seed::<sr25519::Public>("Ferdie"),
//                     get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
//                     get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
//                     get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
//                     get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
//                     get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
//                     get_account_id_from_seed::<sr25519::Public>("Ferdie//stash")
//                 ],
//                 get_account_id_from_seed::<sr25519::Public>("Alice"),
//                 (1000).into()
//             )
//         )
//         .with_protocol_id("template-local")
//         .with_properties(properties)
//         .build()
// }

// fn testnet_genesis(
//     invulnerables: Vec<(AccountId, AuraId)>,
//     endowed_accounts: Vec<AccountId>,
//     root: AccountId,
//     id: ParaId
// ) -> serde_json::Value {
//     serde_json::json!({
// 		"balances": {
// 			"balances": endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 60)).collect::<Vec<_>>(),
// 		},
// 		"parachainInfo": {
// 			"parachainId": id,
// 		},
// 		"collatorSelection": {
// 			"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
// 			"candidacyBond": EXISTENTIAL_DEPOSIT * 16,
// 		},
// 		"session": {
// 			"keys": invulnerables
// 				.into_iter()
// 				.map(|(acc, aura)| {
// 					(
// 						acc.clone(),                 // account id
// 						acc,                         // validator id
// 						template_session_keys(aura), // session keys
// 					)
// 				})
// 			.collect::<Vec<_>>(),
// 		},
// 		"polkadotXcm": {
// 			"safeXcmVersion": Some(SAFE_XCM_VERSION),
// 		},
// 		"sudo": { "key": Some(root) }
// 	})
// }
