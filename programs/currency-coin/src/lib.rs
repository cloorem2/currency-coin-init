use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;

declare_id!("A3mhZsPAjroa8oQokfjGKNNNrKwbpz5ZwszHduYs6hz6");

#[program]
pub mod currency_coin {
    use super::*;

    pub fn create_mint_auth(
        ctx: Context<CreateMintAuth>,
    ) -> Result<()> {
        create_mint_auth::create_mint_auth(ctx)
    }

    pub fn create_cc_mint(
        ctx: Context<CreateCcMint>,
        mint_auth_bump: u8,
    ) -> Result<()> {
        create_cc_mint::create_cc_mint(
            ctx, mint_auth_bump
        )
    }

    pub fn create_ccb0_mint(
        ctx: Context<CreateCcb0Mint>,
        mint_auth_bump: u8,
    ) -> Result<()> {
        create_ccb0_mint::create_ccb0_mint(
            ctx, mint_auth_bump
        )
    }

    pub fn create_ccb1_mint(
        ctx: Context<CreateCcb1Mint>,
        mint_auth_bump: u8,
    ) -> Result<()> {
        create_ccb1_mint::create_ccb1_mint(
            ctx, mint_auth_bump
        )
    }

    pub fn create_ccs0_mint(
        ctx: Context<CreateCcs0Mint>,
        mint_auth_bump: u8,
    ) -> Result<()> {
        create_ccs0_mint::create_ccs0_mint(
            ctx, mint_auth_bump
        )
    }

    pub fn create_cc_metadata(
        ctx: Context<CreateCcMetadata>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
    ) -> Result<()> {
        create_cc_metadata::create_cc_metadata(
            ctx,
            metadata_title,
            metadata_symbol,
            metadata_uri,
            mint_auth_bump,
            cc_mint_bump
        )
    }

    pub fn create_ccb0_metadata(
        ctx: Context<CreateCcb0Metadata>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
        mint_auth_bump: u8,
        ccb0_mint_bump: u8,
    ) -> Result<()> {
        create_ccb0_metadata::create_ccb0_metadata(
            ctx,
            metadata_title,
            metadata_symbol,
            metadata_uri,
            mint_auth_bump,
            ccb0_mint_bump
        )
    }

    pub fn create_ccb1_metadata(
        ctx: Context<CreateCcb1Metadata>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
        mint_auth_bump: u8,
        ccb1_mint_bump: u8,
    ) -> Result<()> {
        create_ccb1_metadata::create_ccb1_metadata(
            ctx,
            metadata_title,
            metadata_symbol,
            metadata_uri,
            mint_auth_bump,
            ccb1_mint_bump
        )
    }

    pub fn create_ccs0_metadata(
        ctx: Context<CreateCcs0Metadata>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
        mint_auth_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        create_ccs0_metadata::create_ccs0_metadata(
            ctx,
            metadata_title,
            metadata_symbol,
            metadata_uri,
            mint_auth_bump,
            ccs0_mint_bump
        )
    }

    pub fn init_circulation(
        ctx: Context<InitCirculation>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
    ) -> Result<()> {
        init_circulation::init_circulation(
            ctx,
            mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
        )
    }

    pub fn init_pool0(
        ctx: Context<InitPool0>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        init_pool0::init_pool0(
            ctx,
            mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccs0_mint_bump,
        )
    }
}
