use crate::state::*;
use anchor_lang::prelude::*;

pub fn create_transfer_lamports_proposal(
    ctx: Context<CreateTransferLamportsProposal>,
    to: Pubkey,
    lamports: u64,
) -> Result<()> {
    let hyper_business_wallet = &mut ctx.accounts.hyper_business_wallet;
    let proposal = &mut ctx.accounts.proposal;
    let owner = &mut ctx.accounts.owner;

    proposal.hyper_business_wallet = hyper_business_wallet.key();
    proposal.approved = vec![];
    proposal.status = ProposalStatus::Active;
    proposal.to = to;
    proposal.lamports = lamports;

    proposal.approve(owner.key(), hyper_business_wallet.threshold)?;

    Ok(())
}

pub fn approve_transfer_lamports_proposal(
    ctx: Context<ApproveTransferLamportsProposal>,
) -> Result<()> {
    let hyper_business_wallet = &mut ctx.accounts.hyper_business_wallet;
    let proposal = &mut ctx.accounts.proposal;
    let voter = &mut ctx.accounts.voter;

    proposal.approve(voter.key(), hyper_business_wallet.threshold)?;

    Ok(())
}

pub fn execute_transfer_lamports_proposal(
    ctx: Context<ExecuteTransferLamportsProposal>,
) -> Result<()> {
    let hyper_business_wallet = &mut ctx.accounts.hyper_business_wallet;
    let proposal = &mut ctx.accounts.proposal;
    let to = &mut ctx.accounts.to;

    require!(
        proposal.status == ProposalStatus::Approved,
        ProposalError::NotApproved
    );
    require!(proposal.to == to.key(), ProposalError::InvalidRecipient);

    hyper_business_wallet.sub_lamports(ctx.accounts.proposal.lamports)?;
    to.add_lamports(ctx.accounts.proposal.lamports)?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateTransferLamportsProposal<'info> {
    pub hyper_business_wallet: Account<'info, HyperBusinessWallet>,
    #[account(
        init,
        payer = rent_payer,
        space = TransferLamportProposal::size(3),
    )]
    pub proposal: Account<'info, TransferLamportProposal>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveTransferLamportsProposal<'info> {
    pub hyper_business_wallet: Account<'info, HyperBusinessWallet>,
    #[account(mut)]
    pub proposal: Account<'info, TransferLamportProposal>,

    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTransferLamportsProposal<'info> {
    #[account(mut)]
    pub hyper_business_wallet: Account<'info, HyperBusinessWallet>,
    #[account(mut)]
    pub proposal: Account<'info, TransferLamportProposal>,

    #[account(mut)]
    /// CHECK:
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
