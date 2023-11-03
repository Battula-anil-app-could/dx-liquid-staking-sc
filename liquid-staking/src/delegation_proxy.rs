dharitri_sc::imports!();

#[dharitri_sc::proxy]
pub trait DelegationProxy {
    #[payable("MOAX")]
    #[endpoint(delegate)]
    fn delegate(&self);

    #[endpoint(unDelegate)]
    fn undelegate(&self, moax_amount: BigUint);

    #[endpoint(withdraw)]
    fn withdraw(&self);

    #[endpoint(claimRewards)]
    fn claim_rewards(&self);
}
