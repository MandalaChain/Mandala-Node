pub use super::*;

pub struct Dev;
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



