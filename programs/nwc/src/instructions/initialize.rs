use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};

use crate::state::config::Config;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [b"config"],
        bump,
        space = 8 + Config::LEN,
    )]
    pub config: Account<'info, Config>,

    /// Mint do USDC - validacao para garantir token correto
    pub usdc_mint: Account<'info, Mint>,

    /// Mint do NWC -  nosso Token que sera mintado pelo PDA
    pub nwc_mint: Account<'info, Mint>,

    /// Wallet externa do projeto que recebe os 70% do Stake
    /// CHECK: Apenas valida como Pubkey
    #[account()]
    pub treasury: AccountInfo<'info>,

    /// TokenAccount (vault) que armazena USDC para resgate ( unstake )
    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = vault_authority,
    )]
    pub vault: Account<'info, TokenAccount>,

    /// PDA  que sera usado como authority do vault
    /// seeds fixo = [b"vault"]
    #[account(
        seeds = [b"vault"],
        bump,
    )]
    
    /// CHECK: Nao armazena dados, apenas PDA usada com Signer
    pub vault_authority: AccountInfo<'info>,

    /// PDA que sera usada para mintar NWC
    #[account(
        seeds = [b"mint-authority"],
        bump,
    )]

    /// CHECK: Apenas usada como signer para mint_to
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    initial_rate: u64,) -> Result<()> {
        let config = &mut ctx.accounts.config;

        config.owner = ctx.accounts.payer.key();
        config.paused = false;
        config.rate = initial_rate;
        config.usdc_mint = ctx.accounts.usdc_mint.key();
        config.nwc_mint = ctx.accounts.nwc_mint.key();
        config.vault = ctx.accounts.vault.key();
        config.treasury = ctx.accounts.treasury.key();

        Ok(())
    }