use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};

use crate::state::config::Config;
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut, has_one = usdc_mint, has_one = nwc_mint, has_one = vault, has_one = treasury)]
    pub config: Account<'info, Config>,

    #[account()]
    pub usdc_mint: Account<'info, Mint>,

    #[account()]
    pub nwc_mint: Account<'info, Mint>,

    /// Conta do usuario que esta enviando USDC
    #[account(mut, token::mint = usdc_mint, token::authority = user)]
    pub user_usdc_ata: Account<'info, TokenAccount>,

    /// Conta do usuario onde $NWC sera mintado
    #[account(mut, token::mint = nwc_mint, token::authority = user)]
    pub user_nwc_ata: Account<'info, TokenAccount>,

    /// Conta onde os 30% de USDC sao armazenados ( Liquidez para Unstake )
    #[account(mut, token::mint = usdc_mint, token::authority = vault_authority)]
    pub vault: Account<'info, TokenAccount>,

    /// CHECK: Wallet Externa 70%
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    /// PDA que controla a Vault
    #[account(seeds = [b"vault"], bump)]

    /// CHECK:  apenas usada como signer
    pub vault_authority: AccountInfo<'info>,

    /// PDA que tem authority para mint NWC
    #[account(seeds = [b"mint-authority"], bump)]
    
    /// CHECK: apenas usada como signer
    pub mint_authority: AccountInfo<'info>, 

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn stake(ctx: Context<Stake>, amount_usdc: u64) -> Result<()> {
    let config = &ctx.accounts.config;

    // Verifica se o contrato esta pausado
    require!(!config.paused, ErrorCode::ContractPaused);

    // Calcula quanto o usuario ira receber em NWC
    let amount_nwc = amount_usdc
        .checked_div(config.rate)
        .ok_or(ErrorCode::MathOverflow)?;

    // Calcula divisao 30 / 70
    let amount_to_vault = amount_usdc
        .checked_mul(30)
        .unwrap()
        .checked_div(100)
        .unwrap();

    let amount_to_treasury = amount_usdc
        .checked_sub(amount_to_vault)
        .ok_or(ErrorCode::MathOverflow)?;

    // Transfere 30% para o vault
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_usdc_ata.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_to_vault,
    )?;

    // Transfere 70% para o treasury
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_usdc_ata.to_account_info(),
                to: ctx.accounts.treasury.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_to_treasury,
    )?;

    // Mint NWC para o usuario
    let bump = ctx.bumps.mint_authority;
    let seeds: &[&[u8]] = &[b"mint-authority", &[bump]];
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nwc_mint.to_account_info(),
                to: ctx.accounts.user_nwc_ata.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[seeds],
        ),
        amount_nwc,
    )?;

    Ok(())
}