// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           30
// Async Callback:                       1
// Promise callbacks:                    5
// Total number of exported functions:  38

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    liquid_staking
    (
        init => init
        upgrade => upgrade
        getLsValueForPosition => get_ls_value_for_position
        registerLsToken => register_ls_token
        registerUnstakeToken => register_unstake_token
        getState => state
        getLsTokenId => ls_token
        getLsSupply => ls_token_supply
        getVirtualRewaReserve => virtual_rewa_reserve
        getRewardsReserve => rewards_reserve
        getUnstakeTokenId => unstake_token
        clearOngoingWhitelistOp => clear_ongoing_whitelist_op
        whitelistDelegationContract => whitelist_delegation_contract
        changeDelegationContractAdmin => change_delegation_contract_admin
        changeDelegationContractParams => change_delegation_contract_params
        getDelegationStatus => get_delegation_status
        getDelegationContractStakedAmount => get_delegation_contract_staked_amount
        getDelegationContractUnstakedAmount => get_delegation_contract_unstaked_amount
        getDelegationContractUnbondedAmount => get_delegation_contract_unbonded_amount
        setStateActive => set_state_active
        setStateInactive => set_state_inactive
        getDelegationAddressesList => delegation_addresses_list
        getAddressesToClaim => addresses_to_claim
        getDelegationClaimStatus => delegation_claim_status
        getDelegationContractData => delegation_contract_data
        claimRewards => claim_rewards
        delegateRewards => delegate_rewards
        recomputeTokenReserve => recompute_token_reserve
        unbondTokens => unbond_tokens
        withdrawAll => withdraw_all
        addLiquidity => add_liquidity
        removeLiquidity => remove_liquidity
        claim_rewards_callback => claim_rewards_callback
        delegate_rewards_callback => delegate_rewards_callback
        withdraw_tokens_callback => withdraw_tokens_callback
        add_liquidity_callback => add_liquidity_callback
        remove_liquidity_callback => remove_liquidity_callback
    )
}

dharitri_sc_wasm_adapter::async_callback! { liquid_staking }
