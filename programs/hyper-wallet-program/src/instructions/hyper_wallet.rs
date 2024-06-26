use crate::{errors::HyperWalletError, state::*};
use anchor_lang::prelude::*;

pub fn create_hyper_wallet(ctx: Context<CreateHyperWallet>, approvers: Vec<Pubkey>) -> Result<()> {
    let hyper_wallet = &mut ctx.accounts.hyper_wallet;
    hyper_wallet.owner = ctx.accounts.owner.key();
    hyper_wallet.approvers = approvers;

    Ok(())
}

pub fn close_hyper_wallet(_ctx: Context<CloseHyperWallet>) -> Result<()> {
    Ok(())
}

pub fn change_approvers(ctx: Context<ChangeApprovers>, new_approver: Pubkey) -> Result<()> {
    let hyper_wallet = &mut ctx.accounts.hyper_wallet;
    let approver = &mut ctx.accounts.approver;

    require!(
        hyper_wallet.is_valid_approver(approver.key()),
        HyperWalletError::InvalidApprover
    );

    hyper_wallet.approvers = vec![approver.key(), new_approver];

    Ok(())
}

#[derive(Accounts)]
pub struct CreateHyperWallet<'info> {
    #[account(init, payer = rent_payer, space = HyperWallet::size(5), seeds = [owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    #[account(mut)]
    pub rent_payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseHyperWallet<'info> {
    #[account(mut, close = owner, seeds = [owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeApprovers<'info> {
    #[account(mut, seeds = [owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub owner: Signer<'info>,
    pub approver: Signer<'info>,
    pub system_program: Program<'info, System>,
}
