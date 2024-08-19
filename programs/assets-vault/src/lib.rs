use anchor_lang::prelude::*;

declare_id!("7YZV5ErKM7o7JS9a4PS2B44cXjpbFqKhaRZHeEqLYC22");

#[program]
pub mod assets_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
