use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Transfer, transfer};
use crate::state::config::Config;

#[derive(Accounts)]
pub struct VaultWithdraw<'info> {
    /// Config principal
    #[account(has_one = owner, has_one = vault, has_one = treasury)]
    pub config: Account<'info, Config>,

    /// CHECK: verificado via `has_one`, e signer
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    /// Vault TokenAccount (PDA do programa)
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
        token::mint = config.usdc_mint,
        token::authority = vault_authority
    )]
    pub vault: Account<'info, TokenAccount>,

    /// Treasury que recebe o USDC
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,

    /// CHECK: PDA autoridade da Vault
    #[account(seeds = [b"vault"], bump)]
    pub vault_authority: AccountInfo<'info>,

    /// SPL Token Program
    pub token_program: Program<'info, Token>,
}

pub fn vault_withdraw_handler(ctx: Context<VaultWithdraw>) -> Result<()> {
    let amount = ctx.accounts.vault.amount;

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.vault_authority.to_account_info(),
    };

    let bump = ctx.bumps.vault_authority;
    let seeds: &[&[u8]] = &[b"vault", &[bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );

    transfer(cpi_ctx, amount)?;

    Ok(())
}
