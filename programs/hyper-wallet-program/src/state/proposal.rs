use anchor_lang::prelude::*;
use anchor_lang::solana_program::borsh0_10::get_instance_packed_len;

#[account]
pub struct TransferLamportProposal {
    pub hyper_business_wallet: Pubkey,
    pub approved: Vec<Pubkey>,
    pub status: ProposalStatus,
    pub to: Pubkey,
    pub lamports: u64,
}

impl TransferLamportProposal {
    pub fn size(voters_len: usize) -> usize {
        8 +                         // anchor account discriminator
        32 +                        // hyper_business_wallet
        ((voters_len * 32)) +  // approved vec
        1 +                         // status enum
        32 +                        // recipient
        8 // lamports
    }

    /// Register an approval vote.
    pub fn approve(&mut self, voter: Pubkey, threshold: u8) -> Result<()> {
        // // This will be implementd later
        // // If `voter` has previously voted to reject, remove that vote.
        // if let Some(vote_index) = self.has_voted_reject(member.key()) {
        //     self.remove_rejection_vote(vote_index);
        // }

        // Insert the vote of approval.
        match self.approved.binary_search(&voter) {
            Ok(_) => return err!(ProposalError::AlreadyApproved),
            Err(pos) => self.approved.insert(pos, voter),
        };

        // If current number of approvals reaches threshold, mark the transaction as `Approved`.
        if self.approved.len() as u8 >= threshold {
            self.status = ProposalStatus::Approved
        }

        Ok(())
    }
}

#[account]
pub struct TransferSPLProposal {
    pub hyper_business_wallet: Pubkey,
    pub approved: Vec<Pubkey>,
    pub status: ProposalStatus,
    pub from_ata: Pubkey,
    pub to_ata: Pubkey,
    pub raw_amount: u64,
}

impl TransferSPLProposal {
    pub fn size(voters_len: usize) -> usize {
        8 +                         // anchor account discriminator
        32 +                        // hyper_business_wallet
        ((voters_len * 32)) +  // approved vec
        1 +                         // status enum
        32 +                        // from_ata
        32 +                        // to_ata
        8 // raw_amount
    }

    /// Register an approval vote.
    pub fn approve(&mut self, voter: Pubkey, threshold: u8) -> Result<()> {
        // // This will be implementd later
        // // If `voter` has previously voted to reject, remove that vote.
        // if let Some(vote_index) = self.has_voted_reject(member.key()) {
        //     self.remove_rejection_vote(vote_index);
        // }

        // Insert the vote of approval.
        match self.approved.binary_search(&voter) {
            Ok(_) => return err!(ProposalError::AlreadyApproved),
            Err(pos) => self.approved.insert(pos, voter),
        };

        // If current number of approvals reaches threshold, mark the transaction as `Approved`.
        if self.approved.len() as u8 >= threshold {
            self.status = ProposalStatus::Approved
        }

        Ok(())
    }
}

#[account]
pub struct ConfigProposal {
    pub hyper_business_wallet: Pubkey,
    pub approved: Vec<Pubkey>,
    pub status: ProposalStatus,
    pub actions: Vec<ConfigAction>,
}

impl ConfigProposal {
    pub fn size(members_length: usize, actions: &[ConfigAction]) -> usize {
        let actions_size: usize = actions
            .iter()
            .map(|action| get_instance_packed_len(action).unwrap())
            .sum();
        8 +                         // anchor account discriminator
        32 +                        // hyper_business_wallet
        ((members_length * 32)) +   // approved vec
        1 +                         // status enum
        4 +                         // actions vector
        actions_size
    }

    /// Register an approval vote.
    pub fn approve(&mut self, member: Pubkey, threshold: u8) -> Result<()> {
        // // This will be implementd later
        // // If `voter` has previously voted to reject, remove that vote.
        // if let Some(vote_index) = self.has_voted_reject(member.key()) {
        //     self.remove_rejection_vote(vote_index);
        // }

        // Insert the vote of approval.
        match self.approved.binary_search(&member) {
            Ok(_) => return err!(ProposalError::AlreadyApproved),
            Err(pos) => self.approved.insert(pos, member),
        };

        // If current number of approvals reaches threshold, mark the transaction as `Approved`.
        if self.approved.len() as u8 >= threshold {
            self.status = ProposalStatus::Approved
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum ProposalStatus {
    Active,
    Approved,
    Executed,
}

#[error_code]
pub enum ProposalError {
    AlreadyApproved,
    NotApproved,
    InvalidRecipient,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConfigAction {
    /// Add a new member to the Hyper Business Wallet.
    AddMember { new_member: Pubkey },
    /// Remove a member from the Hyper Business Wallet.
    RemoveMember { old_member: Pubkey },
    /// Change the `threshold` of the Hyper Business Wallet.
    ChangeThreshold { new_threshold: u8 },
}
