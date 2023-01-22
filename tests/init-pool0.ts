import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";

describe("currency-coin", () => {
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  // const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

  it("init pool", async () => {
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

    const [ ccs0Mint, ccs0MintBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("ccs0_mint_") ], program.programId
      );
    console.log(`ccs0Mint ${ccs0MintBump} ${ccs0Mint}`);

    const cc_ata = await anchor.utils.token.associatedAddress({
      mint: ccMint,
      owner: mintAuth
    });
    console.log(`cc_ata ${cc_ata}`);

    const ccb0_ata = await anchor.utils.token.associatedAddress({
      mint: ccb0Mint,
      owner: mintAuth
    });
    console.log(`ccb0_ata ${ccb0_ata}`);

    const ccs0_ata = await anchor.utils.token.associatedAddress({
      mint: ccs0Mint,
      owner: mintAuth
    });
    console.log(`ccs0_ata ${ccs0_ata}`);

    await program.methods.initPool0(
      mintAuthBump, ccMintBump, ccb0MintBump, ccs0MintBump
    ).accounts({
      mintAuthority: mintAuth,
      ccMintAccount: ccMint,
      ccb0MintAccount: ccb0Mint,
      ccs0MintAccount: ccs0Mint,
      ccTokenAccount: cc_ata,
      ccb0TokenAccount: ccb0_ata,
      ccs0TokenAccount: ccs0_ata,
      // payer: payer.publicKey,
      // systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      // associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    }).signers([payer.payer]).rpc();

    const res = await program.account.mintAuth.fetch(mintAuth);
    console.log('mintAuth fetched:');
    console.log(res);
  });
});
