import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("currency-coin", () => {
  // const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

  it("initialzed cc ata", async () => {
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
      owner: mintAuth
    });
    console.log(`cc_ata ${cc_ata}`);

    const tx = await program.methods.initCcAta(
      mintAuthBump, ccMintBump,
    ).accounts({
      mintAuthority: mintAuth,
      mintAccount: ccMint,
      tokenAccount: cc_ata,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    console.log(`tx ${tx}`);
  });

  it("initialzed ccb0 ata", async () => {
    const [ mintAuth, mintAuthBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("mint_auth_") ], program.programId
      );

    const [ ccb0Mint, ccb0MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccb0_mint_") ], program.programId
      );
    console.log(`ccb0Mint ${ccb0MintBump} ${ccb0Mint}`);

    const ccb0_ata = await anchor.utils.token.associatedAddress({
      mint: ccb0Mint,
      owner: mintAuth
    });
    console.log(`ccb0_ata ${ccb0_ata}`);

    const tx = await program.methods.initCcb0Ata(
      mintAuthBump, ccb0MintBump,
    ).accounts({
      mintAuthority: mintAuth,
      mintAccount: ccb0Mint,
      tokenAccount: ccb0_ata,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    console.log(`tx ${tx}`);
  });

  it("initialzed ccb1 ata", async () => {
    const [ mintAuth, mintAuthBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("mint_auth_") ], program.programId
      );

    const [ ccb1Mint, ccb1MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccb1_mint_") ], program.programId
      );
    console.log(`ccb1Mint ${ccb1MintBump} ${ccb1Mint}`);

    const ccb1_ata = await anchor.utils.token.associatedAddress({
      mint: ccb1Mint,
      owner: mintAuth
    });
    console.log(`ccb1_ata ${ccb1_ata}`);

    const tx = await program.methods.initCcb1Ata(
      mintAuthBump, ccb1MintBump,
    ).accounts({
      mintAuthority: mintAuth,
      mintAccount: ccb1Mint,
      tokenAccount: ccb1_ata,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    console.log(`tx ${tx}`);
  });

  it("initialzed ccs0 ata", async () => {
    const [ mintAuth, mintAuthBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("mint_auth_") ], program.programId
      );

    const [ ccs0Mint, ccs0MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccs0_mint_") ], program.programId
      );
    console.log(`ccs0Mint ${ccs0MintBump} ${ccs0Mint}`);

    const ccs0_ata = await anchor.utils.token.associatedAddress({
      mint: ccs0Mint,
      owner: mintAuth
    });
    console.log(`ccs0_ata ${ccs0_ata}`);

    const tx = await program.methods.initCcs0Ata(
      mintAuthBump, ccs0MintBump,
    ).accounts({
      mintAuthority: mintAuth,
      mintAccount: ccs0Mint,
      tokenAccount: ccs0_ata,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    console.log(`tx ${tx}`);
  });
});
