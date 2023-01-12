import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";

describe("currency-coin", () => {
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  // const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

  it("cranking 1", async () => {
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

    const ccb1_ata = await anchor.utils.token.associatedAddress({
      mint: ccb1Mint,
      owner: mintAuth
    });
    console.log(`ccb1_ata ${ccb1_ata}`);

    const ccs0_ata = await anchor.utils.token.associatedAddress({
      mint: ccs0Mint,
      owner: mintAuth
    });
    console.log(`ccs0_ata ${ccs0_ata}`);

    await program.methods.crank1(
      mintAuthBump,
      ccMintBump,
      ccb0MintBump,
      ccb1MintBump,
      ccs0MintBump,
    ).accounts({
      mintAuthority: mintAuth,

      ccMintAccount: ccMint,
      ccb0MintAccount: ccb0Mint,
      ccb1MintAccount: ccb1Mint,
      ccs0MintAccount: ccs0Mint,

      ccTokenAccount: cc_ata,
      ccb0TokenAccount: ccb0_ata,
      ccb1TokenAccount: ccb1_ata,
      ccs0TokenAccount: ccs0_ata,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
  });
});
