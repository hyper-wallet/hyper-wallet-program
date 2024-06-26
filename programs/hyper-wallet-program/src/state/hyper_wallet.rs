use anchor_lang::prelude::*;

#[account]
pub struct HyperWallet {
    pub owner: Pubkey,
    pub approvers: Vec<Pubkey>,
}

impl HyperWallet {
    pub fn size(approvers_length: usize) -> usize {
        8 +                     // Discriminator
        32 +                    // owner
        4 +                     // approvers vec
        (approvers_length * 32) // approvers
    }

    pub fn is_valid_approver(&self, approver: Pubkey) -> bool {
        let approver_is_valid = match self.approvers.iter().find(|&v| *v == approver.key()) {
            None => false,
            Some(_v) => true,
        };
        return approver_is_valid;
    }
}
