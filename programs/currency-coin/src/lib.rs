use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;

declare_id!("9Xs2Dkn42QaeZwmTq7nN5gRbZXnwiUbCRTpjb4HJy65F");

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

    pub fn init_cc_ata(
        ctx: Context<InitCcAta>,
        mint_auth_bump: u8,
        mint_bump: u8,
    ) -> Result<()> {
        init_cc_ata::init_cc_ata(
            ctx, mint_auth_bump, mint_bump
        )
    }

    pub fn init_ccb0_ata(
        ctx: Context<InitCcb0Ata>,
        mint_auth_bump: u8,
        mint_bump: u8,
    ) -> Result<()> {
        init_ccb0_ata::init_ccb0_ata(
            ctx, mint_auth_bump, mint_bump
        )
    }

    pub fn init_ccb1_ata(
        ctx: Context<InitCcb1Ata>,
        mint_auth_bump: u8,
        mint_bump: u8,
    ) -> Result<()> {
        init_ccb1_ata::init_ccb1_ata(
            ctx, mint_auth_bump, mint_bump
        )
    }

    pub fn init_ccs0_ata(
        ctx: Context<InitCcs0Ata>,
        mint_auth_bump: u8,
        mint_bump: u8,
    ) -> Result<()> {
        init_ccs0_ata::init_ccs0_ata(
            ctx, mint_auth_bump, mint_bump
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

    pub fn init_owner_ata(
        ctx: Context<InitOwnerAta>,
        mint_auth_bump: u8
    ) -> Result<()> {
        init_owner_ata::init_owner_ata(ctx, mint_auth_bump)
    }

    pub fn crank0(
        ctx: Context<Crank0>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        crank0::crank0(
            ctx, mint_auth_bump, cc_mint_bump, ccs0_mint_bump
        )
    }

    pub fn crank1(
        ctx: Context<Crank1>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        crank1::crank1(
            ctx,
            mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn crank3(
        ctx: Context<Crank3>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        crank3::crank3(
            ctx,
            mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn buy_bonds0(
        ctx: Context<BuyBonds0>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        buy_bonds0::buy_bonds0(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn buy_bonds1(
        ctx: Context<BuyBonds1>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        buy_bonds1::buy_bonds1(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn buy_shorts0(
        ctx: Context<BuyShorts0>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        buy_shorts0::buy_shorts0(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccs0_mint_bump,
        )
    }
    pub fn buy_shorts1(
        ctx: Context<BuyShorts1>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        buy_shorts1::buy_shorts1(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn sell_bonds0(
        ctx: Context<SellBonds0>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        sell_bonds0::sell_bonds0(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn sell_bonds1(
        ctx: Context<SellBonds1>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        sell_bonds1::sell_bonds1(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn sell_shorts0(
        ctx: Context<SellShorts0>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        sell_shorts0::sell_shorts0(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn sell_shorts1(
        ctx: Context<SellShorts1>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        sell_shorts1::sell_shorts1(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn redeem_bonds0(
        ctx: Context<RedeemBonds0>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccb1_mint_bump: u8,
    ) -> Result<()> {
        redeem_bonds0::redeem_bonds0(
            ctx,
            mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccb1_mint_bump,
        )
    }

    pub fn redeem_bonds1(
        ctx: Context<RedeemBonds1>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccb1_mint_bump: u8,
    ) -> Result<()> {
        redeem_bonds1::redeem_bonds1(
            ctx,
            mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccb1_mint_bump,
        )
    }

    pub fn mint_to_your_wallet(
        ctx: Context<MintToYourWallet>,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        mint_auth_bump: u8,
    ) -> Result<()> {
        mint_to_your_wallet::mint_to_your_wallet(
            ctx, cc_mint_bump, ccb0_mint_bump, mint_auth_bump,
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
