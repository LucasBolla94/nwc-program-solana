use anchor_lang::prelude::*;

// Módulos internos
pub mod state;
pub mod instructions;
pub mod errors;

// Importa os tipos
use instructions::*;

declare_id!("2gLr5H4kndTC3wyP2UxN7fNbedfVxJygSHEwydLBh1N6");

#[program]
pub mod nwc {
    use super::*;

    // Inicializa o Programa
    pub fn initialize(ctx: Context<Initialize>, initial_rate: u64) -> Result<()> {
        // Chama a função real do arquivo instructions/initialize.rs
        instructions::initialize::initialize(ctx, initial_rate)
    }

    // Stake: Usuario envia USDC e recebe NWC
    pub fn stake(ctx: Context<Stake>, amount_usdc: u64) -> Result<()> {
    instructions::stake::stake(ctx, amount_usdc)
    }

    // Unstake: Usuario envia NWC e recebe USDC
    pub fn unstake(ctx: Context<Unstake>, amount_nwc: u64) -> Result<()> {
    instructions::unstake::unstake(ctx, amount_nwc)
    }

    // Atualiza o rate ( Somente o Owner pode fazer )
    pub fn update_rate(ctx: Context<UpdateRate>, new_rate: u64) -> Result<()> {
        instructions::update_rate::handler(ctx, new_rate)
    }
}
