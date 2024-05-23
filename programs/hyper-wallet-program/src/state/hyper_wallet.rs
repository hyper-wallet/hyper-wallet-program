use crate::errors::HyperWalletError;
use anchor_lang::prelude::*;
use rs_merkle::{algorithms::Sha256, MerkleProof};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SpendingLimit {
    pub ata: Pubkey,
    pub raw_amount: u64,
    pub total_spent: u64,
    pub last_reset: i64,
    pub reset_period: i64,
}

#[account]
pub struct HyperWallet {
    pub owner: Pubkey,
    pub whitelist_enabled: bool,
    pub whitelisted_addresses: Vec<Pubkey>,
    pub otp_enabled: bool,
    pub otp_root: [u8; 32],
    pub otp_init_time: u32,
    pub spending_limit_enabled: bool,
    pub spending_limits: Vec<SpendingLimit>,
}

impl HyperWallet {
    pub fn new(owner: Pubkey) -> Self {
        HyperWallet {
            owner,
            whitelist_enabled: false,
            whitelisted_addresses: Vec::new(),
            otp_enabled: false,
            otp_root: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            otp_init_time: 0,
            spending_limit_enabled: false,
            spending_limits: Vec::new(),
        }
    }
    pub fn set_spending_limit(
        &mut self,
        ata: Pubkey,
        raw_amount: u64,
        reset_period: i64,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let limit = SpendingLimit {
            ata,
            raw_amount,
            total_spent: 0,
            last_reset: now,
            reset_period,
        };
        self.spending_limits.push(limit);

        Ok(())
    }

    pub fn remove_spending_limit(&mut self, ata: Pubkey) -> Result<()> {
        if let Some(index) = self.spending_limits.iter().position(|v| v.ata == ata) {
            self.spending_limits.remove(index);
        }

        Ok(())
    }

    pub fn verify_and_record_payment(&mut self, ata: Pubkey, raw_amount: u64) -> Result<()> {
        for limit in &mut self.spending_limits {
            if limit.ata == ata {
                let now = Clock::get()?.unix_timestamp;
                if (now - limit.last_reset) >= limit.reset_period {
                    limit.total_spent = 0;
                    limit.last_reset = now;
                }
                require!(
                    (limit.total_spent + raw_amount) > limit.raw_amount,
                    HyperWalletError::SpendingLimitExceeded
                );
                limit.total_spent += raw_amount;
                break;
            }
        }

        Ok(())
    }
    pub fn verify_receiver(&self, receiver: Pubkey) -> Result<()> {
        if self.whitelist_enabled == false {
            return Ok(());
        }

        let receiver_is_whitelisted =
            match self.whitelisted_addresses.iter().find(|&v| *v == receiver) {
                None => false,
                Some(_v) => true,
            };

        require!(
            receiver_is_whitelisted,
            HyperWalletError::AddressNotWhiteListed
        );

        Ok(())
    }
    pub fn verify_otp(
        &self,
        otp_hash: Option<[u8; 32]>,
        proof_hash: Option<Vec<[u8; 32]>>,
    ) -> Result<()> {
        if self.otp_enabled == false {
            return Ok(());
        }

        require!(otp_hash.is_some(), HyperWalletError::OtpIsRequired);
        require!(
            proof_hash.is_some(),
            HyperWalletError::OtpProofPathIsRequired,
        );

        let otp_hash = otp_hash.unwrap();
        let proof_hash = proof_hash.unwrap();

        let current_time = Clock::get()?.unix_timestamp;
        let init_time = self.otp_init_time;
        let interval = ((current_time - init_time as i64) / 30) as usize;

        let proof_hash_copy = proof_hash.clone();
        let proof = MerkleProof::<Sha256>::new(proof_hash_copy);
        let root = self.otp_root;
        let indices_to_prove = vec![interval];
        let leave_hash = otp_hash;

        require!(
            proof.verify(root, &indices_to_prove, &[leave_hash], usize::pow(2, 10)),
            HyperWalletError::OtpIsInvalid
        );

        Ok(())
    }
}
