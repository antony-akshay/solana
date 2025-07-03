import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Lottery } from "../target/types/lottery";

describe("lottery", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.lottery as Program<Lottery>;

  it("Is initialized!", async () => {
    const intiConfigTx = await program.methods.initializeConfig(
      new anchor.BN(0),
      new anchor.BN(1722712025),
      new anchor.BN(10000)
    ).instruction();

    const blockhashWithContext = await provider.connection.getLatestBlockhash();

    const tx = new anchor.web3.Transaction({
      feePayer:provider.wallet.publicKey,
      blockhash:blockhashWithContext.blockhash,
      lastValidBlockHeight:blockhashWithContext.lastValidBlockHeight
    }).add(intiConfigTx)
    console.log("Your transaction signature", tx);

    const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection,tx,[wallet.payer]);

    console.log("transaction signature:",signature);
  });
});
