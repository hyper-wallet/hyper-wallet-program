use anchor_lang::prelude::*;

#[account]
pub struct HyperWallet {
    pub owner: Pubkey,
}