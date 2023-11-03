dharitri_sc::imports!();
dharitri_sc::derive_imports!();

use crate::liquidity_pool::State;

pub const MAX_PERCENTAGE: u64 = 100_000;
pub const UNBOND_PERIOD: u64 = 10;

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct UnstakeTokenAttributes<M: ManagedTypeApi> {
    pub delegation_contract: ManagedAddress<M>,
    pub unstake_epoch: u64,
    pub unstake_amount: BigUint<M>,
    pub unbond_epoch: u64,
}

#[dharitri_sc::module]
pub trait ConfigModule:
    dharitri_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[only_owner]
    #[payable("MOAX")]
    #[endpoint(registerLsToken)]
    fn register_ls_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let payment_amount = self.call_value().moax_value().clone_value();
        self.ls_token().issue_and_set_all_roles(
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[only_owner]
    #[payable("MOAX")]
    #[endpoint(registerUnstakeToken)]
    fn register_unstake_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let payment_amount = self.call_value().moax_value().clone_value();
        self.unstake_token().issue_and_set_all_roles(
            DctTokenType::NonFungible,
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[only_owner]
    #[endpoint(setStateActive)]
    fn set_state_active(&self) {
        self.state().set(State::Active);
    }

    #[only_owner]
    #[endpoint(setStateInactive)]
    fn set_state_inactive(&self) {
        self.state().set(State::Inactive);
    }

    #[inline]
    fn is_state_active(&self, state: State) -> bool {
        state == State::Active
    }

    #[view(getState)]
    #[storage_mapper("state")]
    fn state(&self) -> SingleValueMapper<State>;

    #[view(getLsTokenId)]
    #[storage_mapper("lsTokenId")]
    fn ls_token(&self) -> FungibleTokenMapper<Self::Api>;

    #[view(getLsSupply)]
    #[storage_mapper("lsTokenSupply")]
    fn ls_token_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getVirtualMoaxReserve)]
    #[storage_mapper("virtualMoaxReserve")]
    fn virtual_moax_reserve(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewardsReserve)]
    #[storage_mapper("rewardsReserve")]
    fn rewards_reserve(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalWithdrawnMoax)]
    #[storage_mapper("totalWithdrawnMoax")]
    fn total_withdrawn_moax(&self) -> SingleValueMapper<BigUint>;

    #[view(getUnstakeTokenId)]
    #[storage_mapper("unstakeTokenId")]
    fn unstake_token(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[view(getUnstakeTokenSupply)]
    #[storage_mapper("unstakeTokenSupply")]
    fn unstake_token_supply(&self) -> SingleValueMapper<BigUint>;
}
