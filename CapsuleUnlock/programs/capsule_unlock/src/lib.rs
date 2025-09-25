use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod capsule_unlock {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let capsule = &mut ctx.accounts.capsule;
        capsule.owner = *ctx.accounts.user.key;
        capsule.unlock_time = Clock::get()?.unix_timestamp + 86400; // 1 day
        capsule.message = "Default message".to_string();
        Ok(())
    }

    pub fn unlock(ctx: Context<Unlock>) -> Result<()> {
        let capsule = &mut ctx.accounts.capsule;
        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time >= capsule.unlock_time, ErrorCode::NotUnlockedYet);
        // Logic to unlock
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 4 + 200)]
    pub capsule: Account<'info, Capsule>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unlock<'info> {
    #[account(mut, has_one = owner)]
    pub capsule: Account<'info, Capsule>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Capsule {
    pub owner: Pubkey,
    pub unlock_time: i64,
    pub message: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Capsule is not unlocked yet")]
    NotUnlockedYet,
}