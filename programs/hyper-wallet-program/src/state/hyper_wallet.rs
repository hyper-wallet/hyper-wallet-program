use anchor_lang::prelude::*;

#[account]
pub struct HyperWallet {
    pub owner: Pubkey,
    pub voters: Vec<Pubkey>,
    pub threshold: u8,
    pub bump: u8,
}

impl HyperWallet {
    pub fn size(voters_length: usize) -> usize {
        8 +                     // Discriminator
        32 +                    // owner
        (voters_length * 32) +  // voters
        1 +                     // theshold
        1 // bump
    }
}
