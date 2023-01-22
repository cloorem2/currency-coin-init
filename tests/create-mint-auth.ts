import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";

describe("currency-coin", () => {
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  // const provider = anchor.AnchorProvider.env();
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

    const res = await program.account.mintAuth.fetch(mintAuth);
    console.log('mintAuth fetched:');
    console.log(res);
  });
});
