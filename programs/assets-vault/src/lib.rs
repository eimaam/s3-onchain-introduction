use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

declare_id!("FZvbeh26adPXUF4PQYs6LyeXmZtnMScmFSAa3SSAhD7R");

#[program]
pub mod vault_program {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>, name: String) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.creator.key;
        vault.total_deposited = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let depositor_account = &mut ctx.accounts.depositor_token_account;
        let vault_token_account = &ctx.accounts.vault_token_account;

        // Perform the transfer from depositor to vault
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: depositor_account.to_account_info(),
                to: vault_token_account.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        );

        transfer(cpi_ctx, amount)?;

        vault.total_deposited += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let withdrawer_account = &mut ctx.accounts.withdrawer_token_account;
        let vault_token_account = &ctx.accounts.vault_token_account;

        // Check if the vault has sufficient funds
        if vault.total_deposited < amount {
            return Err(VaultError::InsufficientFunds.into());
        }

        // Perform the transfer from vault to withdrawer
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: vault_token_account.to_account_info(),
                to: withdrawer_account.to_account_info(),
                authority: vault.to_account_info(),
            },
        );

        transfer(cpi_ctx, amount)?;

        vault.total_deposited -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(init, payer = creator, space = Vault::LEN)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
    )]
    pub depositor_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
        associated_token::authority = vault,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub mint_account: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = vault,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = withdrawer,
        associated_token::mint = mint_account,
        associated_token::authority = withdrawer,
    )]
    pub withdrawer_token_account: Account<'info, TokenAccount>,

    pub mint_account: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub total_deposited: u64,
}

impl Vault {
    const LEN: usize = 32 + 8; // Pubkey + u64
}

#[error_code]
pub enum VaultError {
    #[msg("Insufficient funds in the vault.")]
    InsufficientFunds,
}
