use anchor_lang::prelude::*;

#[account]
pub struct TransferLamportProposal {
    pub hyper_wallet: Pubkey,
    pub approved: Vec<Pubkey>,
    pub status: ProposalStatus,
    pub to: Pubkey,
    pub lamports: u64,
}

impl TransferLamportProposal {
    pub fn size(voters_len: usize) -> usize {
        8 +                         // anchor account discriminator
        32 +                        // hyper_wallet
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
    pub hyper_wallet: Pubkey,
    pub approved: Vec<Pubkey>,
    pub status: ProposalStatus,
    pub from_ata: Pubkey,
    pub to_ata: Pubkey,
    pub raw_amount: u64,
}

impl TransferSPLProposal {
    pub fn size(voters_len: usize) -> usize {
        8 +                         // anchor account discriminator
        32 +                        // hyper_wallet
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
