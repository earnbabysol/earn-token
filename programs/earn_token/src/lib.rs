use anchor_lang::prelude::*;

declare_id!("Earn111111111111111111111111111111111111111");

#[program]
pub mod earn_token {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("EARN token program initialized.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
