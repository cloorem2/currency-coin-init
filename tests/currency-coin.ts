import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import cc_metadata from '../cc_metadata.json';
import ccb_metadata from '../ccb_metadata.json';
import ccs_metadata from '../ccs_metadata.json';

const fs = require('fs');
const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

describe("currency-coin", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

  it("Created the mint_auth account", async () => {
    const [ mintAuth, mintAuthBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("mint_auth_") ], program.programId
      );
    console.log(`mintAuth ${mintAuthBump} ${mintAuth}`);
    await program.methods.createMintAuth().accounts({
      mintAuthority: mintAuth,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([payer.payer]).rpc();
  });

  it("Created CC Mint", async () => {
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
    const [ ccMeta, ccMetaBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          ccMint.toBuffer()
        ], TOKEN_METADATA_PROGRAM_ID
      );
    console.log(`ccMeta ${ccMetaBump} ${ccMeta}`);

    const cc_uri = "https://githubraw.com/cloorem2/currency-coin/main/cc_metadata.json";
    const tx = await program.methods.createCcMint(
      cc_metadata.name, cc_metadata.symbol,
      cc_uri, mintAuthBump
    ).accounts({
      metadataAccount: ccMeta,
      mintAccount: ccMint,
      mintAuthority: mintAuth,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    console.log(`tx ${tx}`);
  });

  it("Created CCB0 Mint", async () => {
    const [ mintAuth, mintAuthBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("mint_auth_") ], program.programId
      );
    console.log(`mintAuth ${mintAuthBump} ${mintAuth}`);
    const [ ccb0Mint, ccb0MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccb0_mint_") ], program.programId
      );
    console.log(`ccb0Mint ${ccb0MintBump} ${ccb0Mint}`);
    const [ ccb0Meta, ccb0MetaBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          ccb0Mint.toBuffer()
        ], TOKEN_METADATA_PROGRAM_ID
      );
    console.log(`ccb0Meta ${ccb0MetaBump} ${ccb0Meta}`);

    const ccb0_uri = "https://githubraw.com/cloorem2/currency-coin/main/ccb_metadata.json";
    const tx = await program.methods.createCcb0Mint(
      ccb_metadata.name, ccb_metadata.symbol,
      ccb0_uri, mintAuthBump
    ).accounts({
      metadataAccount: ccb0Meta,
      mintAccount: ccb0Mint,
      mintAuthority: mintAuth,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    console.log(`tx ${tx}`);
  });

  /*
  it("mint me some cc", async () => {
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

    const amountToMint = 71;
    const tokenAddress = await anchor.utils.token.associatedAddress({
      mint: ccMint,
      owner: payer.publicKey
    });
    console.log(`my ata: ${tokenAddress}`);

    const tx = await program.methods.mintToYourWallet(
      new anchor.BN(amountToMint), ccMintBump, mintAuthBump
    ).accounts({
      mintAccount: ccMint,
      mintAuthority: mintAuth,
      tokenAccount: tokenAddress,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    console.log(`tx ${tx}`);
  })
  */

    /*
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
  */
});
