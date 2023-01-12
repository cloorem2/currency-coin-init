import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";

describe("currency-coin", () => {
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  // const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

  it("buy bonds1", async () => {
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

    const [ ccb1Mint, ccb1MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccb1_mint_") ], program.programId
      );
    console.log(`ccb1Mint ${ccb1MintBump} ${ccb1Mint}`);

    const [ ccs0Mint, ccs0MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccs0_mint_") ], program.programId
      );
    console.log(`ccs0Mint ${ccs0MintBump} ${ccs0Mint}`);

    const cc_ata = await anchor.utils.token.associatedAddress({
      mint: ccMint,
      owner: mintAuth,
    });
    console.log(`cc_ata ${cc_ata}`);

    const ccb1_ata = await anchor.utils.token.associatedAddress({
      mint: ccb1Mint,
      owner: mintAuth,
    });
    console.log(`ccb1_ata ${ccb1_ata}`);

    const ccs0_ata = await anchor.utils.token.associatedAddress({
      mint: ccs0Mint,
      owner: mintAuth,
    });
    console.log(`ccs0_ata ${ccs0_ata}`);

    const owner_cc_ata = await anchor.utils.token.associatedAddress({
      mint: ccMint,
      owner: payer.publicKey
    });
    console.log(`owner_cc_ata ${owner_cc_ata}`);

    const owner_ccb1_ata = await anchor.utils.token.associatedAddress({
      mint: ccb1Mint,
      owner: payer.publicKey
    });
    console.log(`owner_ccb1_ata ${owner_ccb1_ata}`);

    await program.methods.buyBonds1(
      new anchor.BN(49),
      mintAuthBump,
      ccMintBump,
      ccb1MintBump,
      ccs0MintBump,
    ).accounts({
      mintAuthority: mintAuth,

      ccMintAccount: ccMint,
      ccb1MintAccount: ccb1Mint,
      ccs0MintAccount: ccs0Mint,

      ownerCcAccount: owner_cc_ata,
      ownerCcb1Account: owner_ccb1_ata,
      ccAccount: cc_ata,
      ccb1Account: ccb1_ata,
      ccs0Account: ccs0_ata,
      owner: payer.publicKey,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
  });
});
