use anchor_lang::prelude::*;

use crate::state::*;

pub fn create_config_proposal(
    ctx: Context<CreateConfigProposal>,
    args: CreateConfigProposalArgs,
) -> Result<()> {
    let hyper_business_wallet = &mut ctx.accounts.hyper_business_wallet;
    let proposal = &mut ctx.accounts.proposal;
    let creator = &mut ctx.accounts.creator;

    proposal.hyper_business_wallet = hyper_business_wallet.key();
    proposal.approved = vec![];
    proposal.status = ProposalStatus::Active;
    proposal.actions = args.actions;

    proposal.approve(creator.key(), hyper_business_wallet.threshold)?;

    Ok(())
}

pub fn approve_config_proposal(ctx: Context<ApproveConfigProposal>) -> Result<()> {
    let hyper_business_wallet = &mut ctx.accounts.hyper_business_wallet;
    let proposal = &mut ctx.accounts.proposal;
    let member = &mut ctx.accounts.member;

    proposal.approve(member.key(), hyper_business_wallet.threshold)?;

    Ok(())
}

pub fn execute_config_proposal(ctx: Context<ExecuteConfigProposal>) -> Result<()> {
    let hyper_business_wallet = &mut ctx.accounts.hyper_business_wallet;
    let proposal = &mut ctx.accounts.proposal;

    require!(
        proposal.status == ProposalStatus::Approved,
        ProposalError::NotApproved
    );

    // Execute the actions one by one.
    for action in proposal.actions.iter() {
        match action {
            ConfigAction::AddMember { new_member } => {
                hyper_business_wallet.add_member(new_member.to_owned());
            }

            ConfigAction::RemoveMember { old_member } => {
                hyper_business_wallet.remove_member(old_member.to_owned())?;
            }

            ConfigAction::ChangeThreshold { new_threshold } => {
                hyper_business_wallet.threshold = *new_threshold;
            }
        }
    }

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateConfigProposalArgs {
    pub actions: Vec<ConfigAction>,
}

#[derive(Accounts)]
#[instruction(args: CreateConfigProposalArgs)]
pub struct CreateConfigProposal<'info> {
    pub hyper_business_wallet: Account<'info, HyperBusinessWallet>,
    #[account(
        init,
        payer = rent_payer,
        space = ConfigProposal::size(hyper_business_wallet.members.len(), &args.actions),
    )]
    pub proposal: Account<'info, ConfigProposal>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveConfigProposal<'info> {
    pub hyper_business_wallet: Account<'info, HyperBusinessWallet>,
    #[account(mut)]
    pub proposal: Account<'info, ConfigProposal>,

    pub member: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteConfigProposal<'info> {
    #[account(mut)]
    pub hyper_business_wallet: Account<'info, HyperBusinessWallet>,
    #[account(mut)]
    pub proposal: Account<'info, ConfigProposal>,

    pub system_program: Program<'info, System>,
}
