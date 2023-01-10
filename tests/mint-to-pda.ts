import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("currency-coin", () => {
  // const provider = anchor.AnchorProvider.env();
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

  it("initialzed cc pda ata", async () => {
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

    const cc_ata = await anchor.utils.token.associatedAddress({
      mint: ccMint,
      // owner: payer.publicKey
      owner: mintAuth
    });
    console.log(`cc_ata: ${cc_ata}`);

    const tx = await program.methods.mintToPdaWallet(
      new anchor.BN(1), mintAuthBump,
    ).accounts({
      mintAccount: ccMint,
      mintAuthority: mintAuth,
      tokenAccount: cc_ata,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    console.log(`tx ${tx}`);
  });

  /*
  it("initialzed program atas", async () => {
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

    const [ ccs0Mint, ccs0MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccs0_mint_") ], program.programId
      );
    console.log(`ccs0Mint ${ccs0MintBump} ${ccs0Mint}`);

    const [ ccs1Mint, ccs1MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccs1_mint_") ], program.programId
      );
    console.log(`ccs1Mint ${ccs1MintBump} ${ccs1Mint}`);

    const ccb0_ata = await anchor.utils.token.associatedAddress({
      mint: ccb0Mint,
      owner: payer.publicKey
    });
    console.log(`ccb0_ata: ${ccb0_ata}`);

    const ccb1_ata = await anchor.utils.token.associatedAddress({
      mint: ccb1Mint,
      owner: payer.publicKey
    });
    console.log(`ccb1_ata: ${ccb1_ata}`);

    const ccs0_ata = await anchor.utils.token.associatedAddress({
      mint: ccs0Mint,
      owner: payer.publicKey
    });
    console.log(`ccs0_ata: ${ccs0_ata}`);

    const ccs1_ata = await anchor.utils.token.associatedAddress({
      mint: ccs1Mint,
      owner: payer.publicKey
    });
    console.log(`ccs1_ata: ${ccs1_ata}`);

    const tx = await program.methods.initAtas(
      mintAuthBump,
      ccMintBump,
      ccb0MintBump,
      ccb1MintBump,
      ccs0MintBump,
      ccs1MintBump,
    ).accounts({
      mintAuthority: mintAuth,
      ccMintAccount: ccMint,
      ccb0MintAccount: ccb0Mint,
      ccb1MintAccount: ccb1Mint,
      ccs0MintAccount: ccb0Mint,
      ccs1MintAccount: ccb1Mint,
      ccAta: cc_ata,
      ccb0Ata: ccb0_ata,
      ccb1Ata: ccb1_ata,
      ccs0Ata: ccs0_ata,
      ccs1Ata: ccs1_ata,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      // tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    console.log(`tx ${tx}`);
  });
  */
});
