import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";

const sleep = require('sleep');
describe("currency-coin", () => {
  // const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

  it("redeem bonds0", async () => {
    const [ mintAuth, mintAuthBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("mint_auth_") ], program.programId
      );
    console.log(`mintAuth ${mintAuthBump} ${mintAuth}`);

    const [ ccMint, ccMintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("cc_mint_") ], program.programId
      );
    console.log(`ccMint ${ccMintBump} ${ccMint}`);

    const [ ccb0Mint, ccb0MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccb0_mint_") ], program.programId
      );
    console.log(`ccb0Mint ${ccb0MintBump} ${ccb0Mint}`);

    const [ ccb1Mint, ccb1MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccb1_mint_") ], program.programId
      );
    console.log(`ccb1Mint ${ccb1MintBump} ${ccb1Mint}`);

    const owner_cc_ata = await anchor.utils.token.associatedAddress({
      mint: ccMint,
      owner: payer.publicKey
    });
    console.log(`owner_cc_ata ${owner_cc_ata}`);

    const owner_ccb0_ata = await anchor.utils.token.associatedAddress({
      mint: ccb0Mint,
      owner: payer.publicKey
    });
    console.log(`owner_ccb0_ata ${owner_ccb0_ata}`);

    const owner_ccb1_ata = await anchor.utils.token.associatedAddress({
      mint: ccb1Mint,
      owner: payer.publicKey
    });
    console.log(`owner_ccb1_ata ${owner_ccb1_ata}`);

    await program.methods.redeemBonds0(
      mintAuthBump,
      ccMintBump,
      ccb0MintBump,
      ccb1MintBump,
    ).accounts({
      mintAuthority: mintAuth,

      ccMintAccount: ccMint,
      ccb0MintAccount: ccb0Mint,
      ccb1MintAccount: ccb1Mint,

      ownerCcAccount: owner_cc_ata,
      ownerCcb0Account: owner_ccb0_ata,
      ownerCcb1Account: owner_ccb1_ata,

      owner: payer.publicKey,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
  });
});
