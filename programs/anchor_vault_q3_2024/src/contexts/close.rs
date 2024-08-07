use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use crate::state::*;

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close = user,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> CloseAccount<'info> {
    pub fn close(&mut self) -> Result<()> {
        let transfer_accs = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let pda_signer = &[&seeds[..]];

        let transfer_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            transfer_accs,
            pda_signer,
        );
        
        let amount = self.vault.lamports();

        transfer(transfer_ctx, amount)?;

        Ok(())
    }
}
