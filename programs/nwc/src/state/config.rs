use anchor_lang::prelude::*;

#[account]
pub struct Config {
    /// Dono do programa
    pub owner: Pubkey,

    /// Flag para pausar programa
    pub paused: bool,

    /// Rate USDC -> NWC ( EX: 1 USDC = 1_000_000 ) 
    pub rate: u64,

    /// Mint correto USDC ( validacao de seguranca )
    pub usdc_mint: Pubkey,

    /// Mint correto do NWC 
    pub nwc_mint: Pubkey,

    /// TokenAccount onde USDC de liquidez e guardado
    pub vault: Pubkey,

    /// Wallet externa do projeto ( Recebe 70% do Stake )
    pub treasury: Pubkey,
}

impl Config{
    pub const LEN: usize = 32 + 1 + 8 + 32 + 32 + 32 + 32;
}