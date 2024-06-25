use anchor_lang::prelude::*;
use instructions::*;
pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("HYPERhd7VFrTzbRLyGsRcGQZkSfaKUGKAY8XDbaY5AgL");

#[program]
pub mod hyper_wallet_program {
    use super::*;

    pub fn create_hyper_wallet(
        ctx: Context<CreateHyperWallet>,
        voters: Vec<Pubkey>,
        threshold: u8,
        bump: u8,
    ) -> Result<()> {
        instructions::hyper_wallet::create_hyper_wallet(ctx, voters, threshold, bump)
    }

    pub fn close_hyper_wallet(ctx: Context<CloseHyperWallet>) -> Result<()> {
        instructions::hyper_wallet::close_hyper_wallet(ctx)
    }

    pub fn create_transfer_lamports_proposal(
        ctx: Context<CreateTransferLamportsProposal>,
        to: Pubkey,
        lamports: u64,
    ) -> Result<()> {
        instructions::lamports_proposal::create_transfer_lamports_proposal(ctx, to, lamports)
    }

    pub fn approve_transfer_lamports_proposal(
        ctx: Context<ApproveTransferLamportsProposal>,
    ) -> Result<()> {
        instructions::lamports_proposal::approve_transfer_lamports_proposal(ctx)
    }

    pub fn execute_transfer_lamports_proposal(
        ctx: Context<ExecuteTransferLamportsProposal>,
    ) -> Result<()> {
        instructions::lamports_proposal::execute_transfer_lamports_proposal(ctx)
    }

    pub fn create_transfer_spl_proposal(
        ctx: Context<CreateTransferSPLProposal>,
        from_ata: Pubkey,
        to_ata: Pubkey,
        raw_amount: u64,
    ) -> Result<()> {
        instructions::spl_proposal::create_transfer_spl_proposal(ctx, from_ata, to_ata, raw_amount)
    }

    pub fn approve_transfer_spl_proposal(ctx: Context<ApproveTransferSPLProposal>) -> Result<()> {
        instructions::spl_proposal::approve_transfer_spl_proposal(ctx)
    }

    pub fn execute_transfer_spl_proposal(ctx: Context<ExecuteTransferSPLProposal>) -> Result<()> {
        instructions::spl_proposal::execute_transfer_spl_proposal(ctx)
    }
}
