import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BlueshiftAnchorVault } from "../target/types/blueshift_anchor_vault";
import { assert } from "chai";

describe("blueshift_anchor_vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BlueshiftAnchorVault as Program<BlueshiftAnchorVault>;

  const authority = provider.wallet;

  const [vaultPda, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), authority.publicKey.toBuffer()],
    program.programId
  );

  it("Initializes the program", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("✅ Init tx:", tx);
  });

  it("Deposits lamports into the vault", async () => {
    const amount = anchor.web3.LAMPORTS_PER_SOL / 100; // 0.01 SOL

    const tx = await program.methods
      .deposit(new anchor.BN(amount))
      .accounts({
        signer: authority.publicKey,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("✅ Deposit tx:", tx);

    const vaultBalance = await provider.connection.getBalance(vaultPda);
    assert.equal(vaultBalance, amount);
  });

  it("Withdraws lamports from the vault", async () => {
    const authorityBalanceBefore = await provider.connection.getBalance(authority.publicKey);

    const tx = await program.methods
      .withdraw()
      .accounts({
        signer: authority.publicKey,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("✅ Withdraw tx:", tx);

    const vaultBalanceAfter = await provider.connection.getBalance(vaultPda);
    const authorityBalanceAfter = await provider.connection.getBalance(authority.publicKey);

    assert.equal(vaultBalanceAfter, 0);
    assert.isAbove(
      authorityBalanceAfter,
      authorityBalanceBefore,
      "Authority balance did not increase after withdrawal"
    );
  });
});
