use std::collections::{BTreeSet};
use anchor_lang::prelude::*;

#[account]
pub struct HyperWallet {
    pub owner: Pubkey,
    pub white_list_enabled: bool,
    pub white_listed_addresses: Vec<Pubkey>,
}