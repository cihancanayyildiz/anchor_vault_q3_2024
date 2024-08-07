use anchor_lang::prelude::*;
mod contexts;
use contexts::*;
mod state;

declare_id!("5N33BWZRjgQhGCamDsx3pPsgBb5o54APDcvYCsTGQ1bp");

#[program]
mod anchor_vault_q3_2024 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        Ok(())
    }
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;

        Ok(())
    }

    pub fn close(ctx: Context<CloseAccount>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}