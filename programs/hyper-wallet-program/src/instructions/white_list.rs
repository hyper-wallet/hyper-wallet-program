use crate::errors::*;
use crate::state::hyper_wallet::*;
use anchor_lang::prelude::*;

pub fn enable_white_list(ctx: Context<EnableWhiteList>) -> Result<()> {
    ctx.accounts.hyper_wallet.white_list_enabled = true;
    Ok(())
}

pub fn disable_white_list(ctx: Context<DisableWhiteList>) -> Result<()> {
    ctx.accounts.hyper_wallet.white_list_enabled = false;
    Ok(())
}

pub fn add_to_white_list(ctx: Context<AddToWhiteList>, address: Pubkey) -> Result<()> {
    let mut white_listed_addresses = &mut ctx.accounts.hyper_wallet.white_listed_addresses;
    white_listed_addresses.push(address);
    white_listed_addresses.sort();
    Ok(())
}

pub fn remove_from_white_list(ctx: Context<RemoveFromWhiteList>, address: Pubkey) -> Result<()> {
    let mut white_listed_addresses = &mut ctx.accounts.hyper_wallet.white_listed_addresses;
    let index_to_remove = match white_listed_addresses.binary_search(&address) {
        Ok(index_to_remove) => index_to_remove,
        Err(_e) => return err!(HyperWalletError::AddressNotWhiteListed),
    };
    white_listed_addresses.remove(index_to_remove);
    Ok(())
}

#[derive(Accounts)]
pub struct EnableWhiteList<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DisableWhiteList<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddToWhiteList<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveFromWhiteList<'info> {
    #[account(mut, seeds = [hyper_wallet_owner.key().as_ref()], bump)]
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}
