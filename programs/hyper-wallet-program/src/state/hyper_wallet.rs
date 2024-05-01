use anchor_lang::prelude::*;

#[account]
pub struct HyperWallet {
    pub owner: Pubkey,
    pub whitelist_enabled: bool,
    pub whitelisted_addresses: Vec<Pubkey>,
    pub otp_enabled: bool,
    pub spending_limit: u64,
}
