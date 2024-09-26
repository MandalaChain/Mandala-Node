use frame_support::traits::{
    fungible::{Balanced, Credit},
    Get, OnUnbalanced, OriginTrait,
};
use sp_runtime::traits::TryConvert;
use sp_std::convert::TryInto;
use xcm::latest::{Junction::AccountKey20, Location, NetworkId};

/// Instructs how to convert a 20 byte accountId into a Location
pub struct AccountIdToLocation<AccountId>(sp_std::marker::PhantomData<AccountId>);
impl<AccountId> sp_runtime::traits::Convert<AccountId, Location> for AccountIdToLocation<AccountId>
where
    AccountId: Into<[u8; 20]>,
{
    fn convert(account: AccountId) -> Location {
        Location {
            parents: 0,
            interior: [AccountKey20 {
                network: None,
                key: account.into(),
            }]
            .into(),
        }
    }
}

// Convert a local Origin (i.e., a signed 20 byte account Origin)  to a Multilocation
pub struct SignedToAccountId20<Origin, AccountId, Network>(
    sp_std::marker::PhantomData<(Origin, AccountId, Network)>,
);
impl<Origin: OriginTrait + Clone, AccountId: Into<[u8; 20]>, Network: Get<NetworkId>>
    TryConvert<Origin, Location> for SignedToAccountId20<Origin, AccountId, Network>
where
    Origin::PalletsOrigin: From<frame_system::RawOrigin<AccountId>>
        + TryInto<frame_system::RawOrigin<AccountId>, Error = Origin::PalletsOrigin>,
{
    fn try_convert(o: Origin) -> Result<Location, Origin> {
        o.try_with_caller(|caller| match caller.try_into() {
            Ok(frame_system::RawOrigin::Signed(who)) => Ok((AccountKey20 {
                key: who.into(),
                network: Some(Network::get()),
            })
            .into()),
            Ok(other) => Err(other.into()),
            Err(other) => Err(other),
        })
    }
}

// right now we give everything to the author
pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);
impl<R> OnUnbalanced<Credit<R::AccountId, pallet_balances::Pallet<R>>> for DealWithFees<R>
where
    R: pallet_balances::Config + pallet_authorship::Config,
{
    // this seems to be called for substrate-based transactions
    fn on_unbalanceds<B>(
        mut fees_then_tips: impl Iterator<Item = Credit<R::AccountId, pallet_balances::Pallet<R>>>,
    ) {
        let Some(author) = <pallet_authorship::Pallet<R>>::author() else {
            return;
        };

        let Some(fees) = fees_then_tips.next() else {
            return;
        };

        let _ = <pallet_balances::Pallet<R>>::resolve(&author, fees);

        let Some(tip) = fees_then_tips.next() else {
            return;
        };

        let _ = <pallet_balances::Pallet<R>>::resolve(&author, tip);
    }

    // this is called from pallet_evm for Ethereum-based transactions
    // (technically, it calls on_unbalanced, which calls this when non-zero)
    fn on_nonzero_unbalanced(amount: Credit<R::AccountId, pallet_balances::Pallet<R>>) {
        if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
            let _ = <pallet_balances::Pallet<R>>::resolve(&author, amount);
        }
    }
}
