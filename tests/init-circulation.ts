import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";

const sleep = require('sleep');
describe("currency-coin", () => {
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  // const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

  it("init circulation", async () => {
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

    const cc_ata = await anchor.utils.token.associatedAddress({
      mint: ccMint,
      owner: payer.publicKey
    });
    console.log(`cc_ata ${cc_ata}`);

    const ccb0_ata = await anchor.utils.token.associatedAddress({
      mint: ccb0Mint,
      owner: payer.publicKey
    });
    console.log(`ccb0_ata ${ccb0_ata}`);

    await program.methods.initCirculation(
      mintAuthBump, ccMintBump, ccb0MintBump
    ).accounts({
      mintAuthority: mintAuth,
      ccMintAccount: ccMint,
      ccb0MintAccount: ccb0Mint,
      ccTokenAccount: cc_ata,
      ccb0TokenAccount: ccb0_ata,
      payer: payer.publicKey,
      // rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      // systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      // associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
  });
});
