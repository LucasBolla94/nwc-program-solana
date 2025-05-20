use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Burn, Transfer, Mint};

use crate::state::config::Config;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        mut,
        has_one = nwc_mint,
        has_one = vault,
        has_one = usdc_mint
    )]
    pub config: Account<'info, Config>,

    pub nwc_mint: Account<'info, Mint>,
    pub usdc_mint: Account<'info, Mint>,

    /// Conta onde os USDC serão enviados
    #[account(mut, token::mint = usdc_mint, token::authority = user)]
    pub user_usdc_ata: Account<'info, TokenAccount>,

    /// Conta onde os NWC serão queimados
    #[account(mut, token::mint = nwc_mint, token::authority = user)]
    pub user_nwc_ata: Account<'info, TokenAccount>,

    /// Vault do programa (guarda liquidez para unstake)
    #[account(mut, token::mint = usdc_mint, token::authority = vault_authority)]
    pub vault: Account<'info, TokenAccount>,

    /// PDA que controla a vault
    #[account(seeds = [b"vault"], bump)]
    /// CHECK: apenas signer
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn unstake(ctx: Context<Unstake>, amount_nwc: u64) -> Result<()> {
    let config = &ctx.accounts.config;

    require!(!config.paused, ErrorCode::ContractPaused);

    // Calcula quanto USDC o usuário deve receber
    let amount_usdc = amount_nwc
        .checked_mul(config.rate)
        .ok_or(ErrorCode::MathOverflow)?;

    // Verifica liquidez no vault
    require!(
        ctx.accounts.vault.amount >= amount_usdc,
        ErrorCode::InsufficientLiquidity
    );

    // Queima NWC do usuário
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.nwc_mint.to_account_info(),
                from: ctx.accounts.user_nwc_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_nwc,
    )?;

    // Transfere USDC do vault para o usuário
    let bump = ctx.bumps.vault_authority;
    let seeds: &[&[u8]] = &[b"vault", &[bump]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.user_usdc_ata.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            &[seeds],
        ),
        amount_usdc,
    )?;

    Ok(())
}
