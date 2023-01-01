use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod currency_coin {
    use super::*;

    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
        mint_authority_pda_bump: u8,
    ) -> Result<()> {

        create_token_mint::create_token_mint(
            ctx,
            metadata_title,
            metadata_symbol,
            metadata_uri,
            mint_authority_pda_bump,
        )
    }

    pub fn mint_to_your_wallet(
        ctx: Context<MintToYourWallet>,
        amount: u64,
        mint_bump: u8,
        mint_authority_pda_bump: u8,
    ) -> Result<()> {

        mint_to_your_wallet::mint_to_your_wallet(
            ctx,
            amount,
            mint_bump,
            mint_authority_pda_bump,
        )
    }

    pub fn mint_to_pda_wallet(
        ctx: Context<MintToPdaWallet>,
        amount: u64,
        mint_authority_pda_bump: u8,
    ) -> Result<()> {

        mint_to_auth_pda_ata::mint_to_pda_wallet(
            ctx,
            amount,
            mint_authority_pda_bump,
        )
    }

    pub fn mint_to_another_wallet(
        ctx: Context<MintToAnotherWallet>,
        amount: u64,
        mint_authority_pda_bump: u8,
    ) -> Result<()> {

        mint_to_another_wallet::mint_to_another_wallet(
            ctx,
            amount,
            mint_authority_pda_bump,
        )
    }

    pub fn transfer_to_another_wallet(
        ctx: Context<TransferToAnotherWallet>,
        amount: u64,
    ) -> Result<()> {

        transfer_to_another_wallet::transfer_to_another_wallet(
            ctx,
            amount,
        )
    }
}
