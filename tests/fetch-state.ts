import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";

describe("currency-coin", () => {
  // const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

  it("fetcher says:", async () => {
    const [ mintAuth, mintAuthBump ] =
      await anchor.web3.PublicKey.findProgramAddress(
        [ Buffer.from("mint_auth_") ], program.programId
      );
    const res = await program.account.mintAuth.fetch(mintAuth);
    console.log('cc0_amount ' + res.cc0Amount.toString());
    console.log('ccb_amount ' + res.ccbAmount.toString());
    console.log('cc1_amount ' + res.cc1Amount.toString());
    console.log('ccs_amount ' + res.ccsAmount.toString());
    console.log();
    console.log('imod  ' + res.imod.toExponential(5));
    console.log('rmod  ' + res.rmod.toExponential(5));
    console.log();
    console.log('ima0  ' + (res.ima0*60*60*24*365).toExponential(5));
    console.log('ima1  ' + (res.ima1*60*60*24*365).toExponential(5));
    console.log('ima2  ' + (res.ima2*60*60*24*365).toExponential(5));
    console.log();
    console.log('timestamp ' + res.timestamp.toString());
    console.log('maturity  ' + res.maturityState.toString());
  });
});
