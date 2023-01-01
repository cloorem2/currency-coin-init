import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";

describe("currency-coin", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CurrencyCoin as Program<CurrencyCoin>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
