use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

pub fn create_transfer_spl_proposal(
    ctx: Context<CreateTransferSPLProposal>,
    from_ata: Pubkey,
    to_ata: Pubkey,
    raw_amount: u64,
) -> Result<()> {
    let hyper_business_wallet = &mut ctx.accounts.hyper_business_wallet;
    let proposal = &mut ctx.accounts.proposal;
    let owner = &mut ctx.accounts.owner;

    proposal.hyper_business_wallet = hyper_business_wallet.key();
    proposal.approved = vec![];
    proposal.status = ProposalStatus::Active;
    proposal.from_ata = from_ata;
    proposal.to_ata = to_ata;
    proposal.raw_amount = raw_amount;

    proposal.approve(owner.key(), hyper_business_wallet.threshold)?;

    Ok(())
}

pub fn approve_transfer_spl_proposal(ctx: Context<ApproveTransferSPLProposal>) -> Result<()> {
    let hyper_business_wallet = &mut ctx.accounts.hyper_business_wallet;
    let proposal = &mut ctx.accounts.proposal;
    let voter = &mut ctx.accounts.voter;

    proposal.approve(voter.key(), hyper_business_wallet.threshold)?;

    Ok(())
}

pub fn execute_transfer_spl_proposal(ctx: Context<ExecuteTransferSPLProposal>) -> Result<()> {
    let hyper_business_wallet = &mut ctx.accounts.hyper_business_wallet;
    let proposal = &mut ctx.accounts.proposal;
    let to_ata = &mut ctx.accounts.to_ata;
    let from_ata = &mut ctx.accounts.from_ata;

    require!(
        proposal.status == ProposalStatus::Approved,
        ProposalError::NotApproved
    );
    require!(
        proposal.to_ata == to_ata.key(),
        ProposalError::InvalidRecipient
    );

    let bump_vector = ctx.bumps.hyper_business_wallet.to_le_bytes();
    let binding = ctx.accounts.owner.key();
    let inner = vec![binding.as_ref(), bump_vector.as_ref()];
    let outer = vec![inner.as_slice()];

    let transfer_instruction = anchor_spl::token::Transfer {
        from: from_ata.to_account_info(),
        to: to_ata.to_account_info(),
        authority: hyper_business_wallet.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice(),
    );
    anchor_spl::token::transfer(cpi_ctx, proposal.raw_amount)?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTransferSPLProposal<'info> {
    pub hyper_business_wallet: Account<'info, HyperBusinessWallet>,
    #[account(
        init,
        payer = rent_payer,
        space = TransferSPLProposal::size(3),
    )]
    pub proposal: Account<'info, TransferSPLProposal>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveTransferSPLProposal<'info> {
    pub hyper_business_wallet: Account<'info, HyperBusinessWallet>,
    #[account(mut)]
    pub proposal: Account<'info, TransferSPLProposal>,

    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTransferSPLProposal<'info> {
    #[account(
        mut,
        seeds = [owner.key().as_ref()],
        bump
    )]
    pub hyper_business_wallet: Account<'info, HyperBusinessWallet>,
    #[account(mut)]
    pub proposal: Account<'info, TransferSPLProposal>,

    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK:
    pub to_ata: Account<'info, TokenAccount>,

    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
