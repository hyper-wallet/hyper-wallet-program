use anchor_lang::prelude::*;
use instructions::*;
pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("HWPtsy7nbqCiszLppXDqizw7w5CQVLhvHY7j9mCnA63R");

#[program]
pub mod hyper_wallet_program {
    use super::*;

    pub fn create_hyper_wallet(
        ctx: Context<CreateHyperWallet>,
        approvers: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::hyper_wallet::create_hyper_wallet(ctx, approvers)
    }

    pub fn close_hyper_wallet(ctx: Context<CloseHyperWallet>) -> Result<()> {
        instructions::hyper_wallet::close_hyper_wallet(ctx)
    }

    pub fn transfer_lamports(ctx: Context<TransferLamports>, lamports: u64) -> Result<()> {
        instructions::transfer_lamports::transfer_lamports(ctx, lamports)
    }

    pub fn transfer_spl(ctx: Context<TransferSPL>, raw_amount: u64) -> Result<()> {
        instructions::transfer_spl::transfer_spl(ctx, raw_amount)
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
