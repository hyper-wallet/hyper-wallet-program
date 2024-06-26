use anchor_lang::prelude::*;

use crate::errors::HyperBusinessWalletError;

#[account]
pub struct HyperBusinessWallet {
    pub creator: Pubkey,
    pub members: Vec<Pubkey>,
    pub threshold: u8,
    pub bump: u8,
}

impl HyperBusinessWallet {
    pub fn size(members_length: usize) -> usize {
        8 +                     // Discriminator
        32 +                    // creator
        (members_length * 32) + // members
        1 +                     // theshold
        1 // bump
    }

    /// Add `new_member` to the Hyper Business Wallet `members` vec and sort the vec.
    pub fn add_member(&mut self, new_member: Pubkey) {
        self.members.push(new_member);
        self.members.sort();
    }

    /// Remove `member_pubkey` from the Hyper Business Wallet `members` vec.
    pub fn remove_member(&mut self, member_pubkey: Pubkey) -> Result<()> {
        let old_member_index = match self.is_member(member_pubkey) {
            Some(old_member_index) => old_member_index,
            None => return err!(HyperBusinessWalletError::NotAMember),
        };

        self.members.remove(old_member_index);

        Ok(())
    }

    pub fn is_member(&self, member_pubkey: Pubkey) -> Option<usize> {
        self.members.binary_search(&member_pubkey).ok()
    }
}
