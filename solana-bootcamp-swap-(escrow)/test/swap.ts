import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Swap } from "../target/types/swap";
import { randomBytes } from "node:crypto";
import {
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync
} from "@solana/spl-token";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

import {
  confirmTransaction,
  createAccountsMintsAndTokenAccounts,
  makeKeypairs
} from '@solana-developers/helpers';

const TOKEN_PROGRAM: typeof TOKEN_2022_PROGRAM_ID | typeof TOKEN_PROGRAM_ID =
  TOKEN_2022_PROGRAM_ID;

const SECONDS = 1000;

const ANCHOR_SLOW_TEST_THRESHOLD = 40 * SECONDS;

const getRandomBigNumber = (size = 8) => {
  return new BN(randomBytes(size));
}

describe("swap", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const user = (provider.wallet as anchor.Wallet).payer;
  const payer = user;
  const connection = provider.connection;

  const program = anchor.workspace.swap as Program<Swap>;

  const accounts: Record<string, PublicKey> = {
    tokenProgram: TOKEN_PROGRAM
  }

  let alice: anchor.web3.Keypair;
  let bob: anchor.web3.Keypair;
  let tokenMintA: anchor.web3.Keypair;
  let tokenMintB: anchor.web3.Keypair;

  [alice, bob, tokenMintA, tokenMintB] = makeKeypairs(4);

  const tokenAOfferedAmount = new BN(1_000_000);
  const tokenBWantedAmount = new BN(1_000_000);

  before("setup accounts, mints and token accounts", async () => {
  const userMintsAndTokenAccounts = await createAccountsMintsAndTokenAccounts([
    [1_000_000_000, 0],
    [0, 1_000_000_000],
  ], 1 * LAMPORTS_PER_SOL, connection, payer);

  const users = userMintsAndTokenAccounts.users;
  alice = users[0];
  bob = users[1];

  const mints = userMintsAndTokenAccounts.mints;
  tokenMintA = mints[0];
  tokenMintB = mints[1];

  const tokenAccounts = userMintsAndTokenAccounts.tokenAccounts;

  const aliceTokenAccountA = tokenAccounts[0][0];
  const aliceTokenAccountB = tokenAccounts[0][1];

  const bobTokenAccountA = tokenAccounts[1][0];
  const bobTokenAccountB = tokenAccounts[1][1];

  // Save the accounts for later use
  accounts.maker = alice.publicKey;
  accounts.taker = bob.publicKey;
  accounts.tokenMintA = tokenMintA.publicKey;
  accounts.makerTokenAccountA = aliceTokenAccountA;
  accounts.takerTokenAccountA = bobTokenAccountA;
  accounts.tokenMintB = tokenMintB.publicKey;
  accounts.makerTokenAccountB = aliceTokenAccountB;
  accounts.takerTokenAccountB = bobTokenAccountB;
});


  it("Puts the tokens Alice offers into vault makes an offer", async () => {
    // Add your test here.
    const offerId = getRandomBigNumber();
    const offer = PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        accounts.maker.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    )[0];

    const vault = getAssociatedTokenAddressSync(
      accounts.tokenMintA,
      offer,
      true,
      TOKEN_PROGRAM
    );

    accounts.offer = offer;
    accounts.vault = vault;

    const transactionSignature = await program.methods.
      makeOffer(offerId, tokenAOfferedAmount, tokenBWantedAmount)
      .accounts({ ...accounts })
      .signers([alice])
      .rpc();

    await confirmTransaction(connection, transactionSignature);

    //check
    const vaultBalanceResponse = await connection.getTokenAccountBalance(vault);
    const vaultBalance = new BN(vaultBalanceResponse.value.amount);
    assert(vaultBalance.eq(tokenAOfferedAmount));

    const offerAccount = await program.account.offer.fetch(offer);

    assert(offerAccount.maker.equals(alice.publicKey));
    assert(offerAccount.tokenMintA.equals(accounts.tokenMintA));
    assert(offerAccount.tokenMintB.equals(accounts.tokenMintB));
    assert(offerAccount.tokenBWantedAmount.eq(tokenBWantedAmount));
  }).slow(ANCHOR_SLOW_TEST_THRESHOLD);

  it("puts the token from vault to bob's and gives alice bob's token(when bob takes the offer)", async () => {
    const transactionSignature = await program.methods
      .takeOffer()
      .accounts({ ...accounts })
      .signers([bob])
      .rpc();

    await confirmTransaction(connection,transactionSignature);

    const bobTokenAccountBalanceAfterResponse =
    await connection.getTokenAccountBalance(accounts.takerTokenAccountA);
    const bobTokenAccountBalanceAfter = new BN(
      bobTokenAccountBalanceAfterResponse.value.amount
    );

    assert(bobTokenAccountBalanceAfter.eq(tokenAOfferedAmount));

    const aliceTokenAccountBalanceAfterResponse =
    await connection.getTokenAccountBalance(accounts.makerTokenAccountB);
    const aliceTokenAccountBalanceAfter = new BN(
      bobTokenAccountBalanceAfterResponse.value.amount
    );

    assert(aliceTokenAccountBalanceAfter.eq(tokenBWantedAmount));
  }).slow(ANCHOR_SLOW_TEST_THRESHOLD);
});

