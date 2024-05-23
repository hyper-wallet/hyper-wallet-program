use crate::errors::*;
use crate::state::hyper_wallet::*;
use anchor_lang::prelude::*;

pub fn enable_whitelist(ctx: Context<EnableWhiteList>) -> Result<()> {
    ctx.accounts.hyper_wallet.whitelist_enabled = true;
    Ok(())
}

pub fn disable_whitelist(ctx: Context<DisableWhiteList>) -> Result<()> {
    ctx.accounts.hyper_wallet.whitelist_enabled = false;
    Ok(())
}

pub fn add_to_whitelist(ctx: Context<AddToWhiteList>, address: Pubkey) -> Result<()> {
    let whitelisted_addresses = &mut ctx.accounts.hyper_wallet.whitelisted_addresses;
    whitelisted_addresses.push(address);
    whitelisted_addresses.sort();
    Ok(())
}

pub fn remove_from_whitelist(ctx: Context<RemoveFromWhiteList>, address: Pubkey) -> Result<()> {
    let whitelisted_addresses = &mut ctx.accounts.hyper_wallet.whitelisted_addresses;
    let index_to_remove = match whitelisted_addresses.binary_search(&address) {
        Ok(index_to_remove) => index_to_remove,
        Err(_e) => return err!(HyperWalletError::AddressNotWhiteListed),
    };
    whitelisted_addresses.remove(index_to_remove);
    Ok(())
}

pub fn verify_receiver(ctx: Context<VerifyReceiver>, address: Pubkey) -> Result<()> {
    let hyper_wallet = &mut ctx.accounts.hyper_wallet;
    if let Err(e) = hyper_wallet.verify_receiver(address) {
        return Err(e);
    }
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

#[derive(Accounts)]
pub struct VerifyReceiver<'info> {
    pub hyper_wallet: Account<'info, HyperWallet>,
    pub hyper_wallet_owner: Signer<'info>,
}
