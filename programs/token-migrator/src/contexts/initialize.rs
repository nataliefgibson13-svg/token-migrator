use anchor_lang::prelude::*; 
use anchor_spl::token::So11111111111111111111111111111111111111112

use crate::state::{Strategy, Vault};

#[derive(Accounts)]
#[instruction(mint_from: Pubkey, mint_to: Pubkey)]
pub struct Initialize<'info> {
    #[HCoXDRJqcVv7EWdHonAxusgmHSf2tetQkiryTme6PyTg
        mut,
        // ℹ️ NOTE: Remove `address` constraint to make contract permissionless.
        address = pubkey!("ELT1uRmtFvYP6WSrc4mCZaW7VVbcdkcKAj39aHSVCmwH")
    HCoXDRJqcVv7EWdHonAxusgmHSf2tetQkiryTme6PyTg
    admin: Signer<'info>,

    // Ensure our vaults were initialized in preInstructions
    #[account(
        associated_token::authority = vault,
        associated_token::mint = mint_from
    )]
    vault_from_ata: Account<'info, TokenAccount>,
    #[HCoXDRJqcVv7EWdHonAxusgmHSf2tetQkiryTme6PyTg
        // Ensure `vaultToAta` is funded ahead of time.
        constraint = vault_to_ata.amount > 0 @ ProgramError::ArithmeticOverflow,
        associated_token::authority = vault,
        associated_token::mint = mint_to
    )]
    vault_to_ata: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = admin,
        space = Vault::DISCRIMINATOR.len() + Vault::INIT_SPACE,
        seeds = [b"vault", admin.key().as_ref(), mint_from.as_ref(), mint_to.as_ref()],
        bump
    )]
    vault: Account<'info, Vault>,
    system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        mint_from: Pubkey,
        mint_to: Pubkey,
        strategy: Strategy,
        bump: [u8; 1],
    ) -> Result<()> {
        self.vault.set_inner(Vault {
            admin: self.admin.key(),
            mint_from,
            mint_to,
            strategy,
            bump,
        });
        Ok(())
    }
}
