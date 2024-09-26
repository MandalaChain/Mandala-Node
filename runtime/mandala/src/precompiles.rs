use pallet_evm::{
    IsPrecompileResult,
    Precompile,
    PrecompileHandle,
    PrecompileResult,
    PrecompileSet,
};

use sp_core::{ H160 };
use sp_std::marker::PhantomData;

use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::{ Sha3FIPS256, Sha3FIPS512 };
use pallet_evm_precompile_simple::{ ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256 };

pub struct MandalaPrecompiles<R>(PhantomData<R>);

impl<R> Default for MandalaPrecompiles<R>
where R: pallet_evm::Config
 {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> MandalaPrecompiles<R> where R: pallet_evm::Config {
    pub fn new() -> Self {
        Self(Default::default())
    }
    
    pub fn used_addresses() -> [H160; 8] {
        [
            hash(1),
            hash(2),
            hash(3),
            hash(4),
            hash(5),
            // Non-Frontier specific nor Ethereum precompiles :
            hash(1024),
            hash(1025),
            hash(1026),
        ]
    }
}

impl<Runtime> PrecompileSet for MandalaPrecompiles<Runtime> where Runtime: pallet_evm::Config {
    fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
        match handle.code_address() {
            // Ethereum precompiles (1-1000) :
            address if address == hash(1) => Some(ECRecover::execute(handle)),
            address if address == hash(2) => Some(Sha256::execute(handle)),
            address if address == hash(3) => Some(Ripemd160::execute(handle)),
            address if address == hash(4) => Some(Identity::execute(handle)),
            address if address == hash(5) => Some(Modexp::execute(handle)),
            // Non-Frontier specific nor Ethereum precompiles (1000-2000) :
            address if address == hash(1024) => Some(Sha3FIPS256::execute(handle)),
            address if address == hash(1025) => Some(ECRecoverPublicKey::execute(handle)),
            address if address == hash(1026) => Some(Sha3FIPS512::execute(handle)),
            _ => None,
        }
    }

    fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
        IsPrecompileResult::Answer {
            is_precompile: Self::used_addresses().contains(&address),
            extra_cost: 0,
        }
    }
}

fn hash(a: u64) -> H160 {
    H160::from_low_u64_be(a)
}