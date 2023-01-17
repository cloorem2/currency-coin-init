import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";

describe("currency-coin", () => {
  // const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

  it("cranking", async () => {
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

    const ccs0_ata = await anchor.utils.token.associatedAddress({
      mint: ccs0Mint,
      owner: mintAuth
    });
    console.log(`ccs0_ata ${ccs0_ata}`);

    await program.methods.crank0(
      mintAuthBump, ccMintBump, ccs0MintBump
    ).accounts({
      mintAuthority: mintAuth,
      ccMintAccount: ccMint,
      ccs0MintAccount: ccs0Mint,
      ccTokenAccount: cc_ata,
      ccs0TokenAccount: ccs0_ata,
      // payer: payer.publicKey,
      // systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      // associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
  });
});
