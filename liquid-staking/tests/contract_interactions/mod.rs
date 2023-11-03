use crate::contract_setup::LiquidStakingContractSetup;
use dharitri_sc::types::Address;
use dharitri_sc_scenario::{managed_address, num_bigint, rust_biguint, DebugApi};
use liquid_staking::config::{ConfigModule, UnstakeTokenAttributes};
use liquid_staking::LiquidStaking;

use delegation_mock::*;
use liquid_staking::delegation::DelegationModule;

impl<LiquidStakingContractObjBuilder> LiquidStakingContractSetup<LiquidStakingContractObjBuilder>
where
    LiquidStakingContractObjBuilder: 'static + Copy + Fn() -> liquid_staking::ContractObj<DebugApi>,
{
    pub fn deploy_staking_contract(
        &mut self,
        owner_address: &Address,
        moax_balance: u64,
        total_staked: u64,
        delegation_contract_cap: u64,
        nr_nodes: u64,
        apy: u64,
    ) -> Address {
        let rust_zero = rust_biguint!(0u64);
        let moax_balance_biguint = &Self::exp18(moax_balance);
        let total_staked_biguint = Self::exp18(total_staked);
        let delegation_contract_cap_biguint = Self::exp18(delegation_contract_cap);

        self.b_mock
            .set_moax_balance(owner_address, moax_balance_biguint);

        let delegation_wrapper = self.b_mock.create_sc_account(
            &rust_zero,
            Some(owner_address),
            delegation_mock::contract_obj,
            "delegation-mock.wasm",
        );

        self.b_mock
            .execute_tx(owner_address, &delegation_wrapper, &rust_zero, |sc| {
                sc.init();
            })
            .assert_ok();

        self.b_mock
            .execute_tx(
                owner_address,
                &delegation_wrapper,
                moax_balance_biguint,
                |sc| {
                    sc.deposit_moax();
                },
            )
            .assert_ok();

        self.b_mock
            .execute_tx(owner_address, &self.sc_wrapper, &rust_zero, |sc| {
                sc.whitelist_delegation_contract(
                    managed_address!(delegation_wrapper.address_ref()),
                    managed_address!(owner_address),
                    Self::to_managed_biguint(total_staked_biguint),
                    Self::to_managed_biguint(delegation_contract_cap_biguint),
                    nr_nodes,
                    apy,
                );
            })
            .assert_ok();

        delegation_wrapper.address_ref().clone()
    }

    pub fn update_staking_contract_params(
        &mut self,
        owner_address: &Address,
        contract_address: &Address,
        total_staked: u64,
        delegation_contract_cap: u64,
        nr_nodes: u64,
        apy: u64,
    ) {
        let rust_zero = rust_biguint!(0u64);
        let total_staked_biguint = Self::exp18(total_staked);
        let delegation_contract_cap_biguint = Self::exp18(delegation_contract_cap);

        self.b_mock
            .execute_tx(owner_address, &self.sc_wrapper, &rust_zero, |sc| {
                sc.change_delegation_contract_params(
                    managed_address!(contract_address),
                    Self::to_managed_biguint(total_staked_biguint),
                    Self::to_managed_biguint(delegation_contract_cap_biguint),
                    nr_nodes,
                    apy,
                );
            })
            .assert_ok();
    }

    pub fn add_liquidity(&mut self, caller: &Address, payment_amount: u64) {
        self.b_mock
            .execute_tx(
                caller,
                &self.sc_wrapper,
                &Self::exp18(payment_amount),
                |sc| {
                    sc.add_liquidity();
                },
            )
            .assert_ok();
    }

    pub fn remove_liquidity(
        &mut self,
        caller: &Address,
        payment_token: &[u8],
        payment_amount: u64,
    ) {
        self.b_mock
            .execute_dct_transfer(
                caller,
                &self.sc_wrapper,
                payment_token,
                0,
                &Self::exp18(payment_amount),
                |sc| {
                    sc.remove_liquidity();
                },
            )
            .assert_ok();
    }

    pub fn claim_rewards(&mut self, caller: &Address) {
        let rust_zero = rust_biguint!(0u64);
        self.b_mock
            .execute_tx(caller, &self.sc_wrapper, &rust_zero, |sc| {
                sc.claim_rewards();
            })
            .assert_ok();
    }

    pub fn recompute_token_reserve(&mut self, caller: &Address) {
        let rust_zero = rust_biguint!(0u64);
        self.b_mock
            .execute_tx(caller, &self.sc_wrapper, &rust_zero, |sc| {
                sc.recompute_token_reserve();
            })
            .assert_ok();
    }

    pub fn delegate_rewards(&mut self, caller: &Address) {
        let rust_zero = rust_biguint!(0u64);
        self.b_mock
            .execute_tx(caller, &self.sc_wrapper, &rust_zero, |sc| {
                sc.delegate_rewards();
            })
            .assert_ok();
    }

    pub fn delegate_rewards_check_insufficient(&mut self, caller: &Address) {
        let rust_zero = rust_biguint!(0u64);
        self.b_mock
            .execute_tx(caller, &self.sc_wrapper, &rust_zero, |sc| {
                sc.delegate_rewards();
            })
            .assert_error(4, "Old claimed rewards must be greater than 1 MOAX");
    }

    pub fn unbond_tokens(&mut self, caller: &Address, payment_token: &[u8], token_nonce: u64) {
        self.b_mock
            .execute_dct_transfer(
                caller,
                &self.sc_wrapper,
                payment_token,
                token_nonce,
                &num_bigint::BigUint::from(1u64), // NFT
                |sc| {
                    sc.unbond_tokens();
                },
            )
            .assert_ok();
    }

    pub fn setup_new_user(&mut self, moax_token_amount: u64) -> Address {
        let rust_zero = rust_biguint!(0);

        let new_user = self.b_mock.create_user_account(&rust_zero);
        self.b_mock
            .set_moax_balance(&new_user, &Self::exp18(moax_token_amount));
        new_user
    }

    pub fn check_user_balance(&self, address: &Address, token_id: &[u8], token_balance: u64) {
        self.b_mock
            .check_dct_balance(address, token_id, &Self::exp18(token_balance));
    }

    pub fn check_user_balance_denominated(
        &self,
        address: &Address,
        token_id: &[u8],
        token_balance: u128,
    ) {
        self.b_mock.check_dct_balance(
            address,
            token_id,
            &num_bigint::BigUint::from(token_balance),
        );
    }

    pub fn check_user_moax_balance(&self, address: &Address, token_balance: u64) {
        self.b_mock
            .check_moax_balance(address, &Self::exp18(token_balance));
    }

    pub fn check_user_moax_balance_denominated(&self, address: &Address, token_balance: u128) {
        self.b_mock
            .check_moax_balance(address, &num_bigint::BigUint::from(token_balance));
    }

    pub fn check_contract_storage(
        &mut self,
        ls_token_supply: u64,
        virtual_moax_reserve: u64,
        rewards_reserve: u64,
        withdrawn_moax: u64,
    ) {
        self.b_mock
            .execute_query(&self.sc_wrapper, |sc| {
                assert_eq!(
                    sc.ls_token_supply().get(),
                    Self::to_managed_biguint(Self::exp18(ls_token_supply))
                );
                assert_eq!(
                    sc.virtual_moax_reserve().get(),
                    Self::to_managed_biguint(Self::exp18(virtual_moax_reserve))
                );
                assert_eq!(
                    sc.rewards_reserve().get(),
                    Self::to_managed_biguint(Self::exp18(rewards_reserve))
                );
                assert_eq!(
                    sc.total_withdrawn_moax().get(),
                    Self::to_managed_biguint(Self::exp18(withdrawn_moax))
                );
            })
            .assert_ok();
    }

    pub fn check_contract_rewards_storage_denominated(&mut self, rewards_reserve: u128) {
        self.b_mock
            .execute_query(&self.sc_wrapper, |sc| {
                assert_eq!(
                    sc.rewards_reserve().get(),
                    Self::to_managed_biguint(num_bigint::BigUint::from(rewards_reserve))
                );
            })
            .assert_ok();
    }

    pub fn check_delegation_contract_values(
        &mut self,
        delegation_contract: &Address,
        total_staked: u64,
    ) {
        self.b_mock
            .execute_query(&self.sc_wrapper, |sc| {
                assert_eq!(
                    sc.delegation_contract_data(&managed_address!(delegation_contract))
                        .get()
                        .total_staked_from_ls_contract,
                    Self::to_managed_biguint(Self::exp18(total_staked))
                );
            })
            .assert_ok();
    }

    pub fn get_ls_value_for_position(&mut self, token_amount: u64) -> u128 {
        let mut ls_value = 0u64;
        self.b_mock
            .execute_query(&self.sc_wrapper, |sc| {
                let ls_value_biguint = sc
                    .get_ls_value_for_position(Self::to_managed_biguint(Self::exp18(token_amount)));
                ls_value = ls_value_biguint.to_u64().unwrap();
            })
            .assert_ok();

        u128::from(ls_value)
    }

    pub fn check_delegation_contract_values_denominated(
        &mut self,
        delegation_contract: &Address,
        total_staked: u128,
    ) {
        self.b_mock
            .execute_query(&self.sc_wrapper, |sc| {
                assert_eq!(
                    sc.delegation_contract_data(&managed_address!(delegation_contract))
                        .get()
                        .total_staked_from_ls_contract,
                    Self::to_managed_biguint(num_bigint::BigUint::from(total_staked))
                );
            })
            .assert_ok();
    }

    pub fn check_user_nft_balance_denominated(
        &self,
        address: &Address,
        token_id: &[u8],
        token_nonce: u64,
        token_balance: u64,
    ) {
        self.b_mock
            .check_nft_balance::<UnstakeTokenAttributes<DebugApi>>(
                address,
                token_id,
                token_nonce,
                &num_bigint::BigUint::from(token_balance),
                None,
            );
    }
}
