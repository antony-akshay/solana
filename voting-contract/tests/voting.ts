//file inside tests/voting.ts

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Voting } from "../target/types/voting";
import { Keypair, PublicKey, SystemProgram, VoteInit, VoteProgram } from '@solana/web3.js';
import { expect } from "chai";

describe("voting", () => {
  // Configure the client to use the local cluster.
  let provider: anchor.AnchorProvider;
  let program: Program<Voting>;

  before(() => {
    provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    program = anchor.workspace.voting as Program<Voting>;
  });

  it("initialize polling", async () => {
    // Add your test here.
    const pollID = new anchor.BN(1);

    const [pollPda] = PublicKey.findProgramAddressSync(
      [pollID.toArrayLike(Buffer, "be", 8)],
      program.programId
    );

    const accounts: {
      signer: PublicKey;
      pollAccount: PublicKey;
      systemProgram: PublicKey;
    } = {
      signer: provider.wallet.publicKey,
      pollAccount: pollPda,
      systemProgram: SystemProgram.programId,
    };

    const tx = await program.methods.initializePoll(
      pollID,
      "what is your favorite type of peanut butter?",
      new anchor.BN(0),
      new anchor.BN(1721246480)
    )
      .accounts(accounts)
      .rpc();
    console.log("Your transaction signature", tx);

    const pollTransaction = await program.account.pollAccount.fetch(pollPda);
    console.log("POLL:", pollTransaction);

    expect(pollTransaction.pollId.toNumber()).equal(1);
    expect(pollTransaction.description).equal("what is your favorite type of peanut butter?");
    expect(pollTransaction.pollStart.toNumber()).lessThan(pollTransaction.pollEnd.toNumber());

  });


  it("initialize candidate", async () => {

    const pollID = new anchor.BN(1);
    const candidate1 = "Akshay";
    const candidate2 = "Antony";

    const [pollPda] = PublicKey.findProgramAddressSync(
      [pollID.toArrayLike(Buffer, "be", 8)],
      program.programId
    );

    const [CandidatePda1] = PublicKey.findProgramAddressSync(
      [pollID.toArrayLike(Buffer, "be", 8), Buffer.from(candidate1)],
      program.programId
    );

    const [CandidatePda2] = PublicKey.findProgramAddressSync(
      [pollID.toArrayLike(Buffer, "be", 8), Buffer.from(candidate2)],
      program.programId
    );


    const account1: {
      signer: PublicKey;
      pollAccount: PublicKey;
      candidateAccount: PublicKey
      systemProgram: PublicKey;
    } = {
      signer: provider.wallet.publicKey,
      pollAccount: pollPda,
      candidateAccount: CandidatePda1,
      systemProgram: SystemProgram.programId,
    };

    const account2: {
      signer: PublicKey;
      pollAccount: PublicKey;
      candidateAccount: PublicKey
      systemProgram: PublicKey;
    } = {
      signer: provider.wallet.publicKey,
      pollAccount: pollPda,
      candidateAccount: CandidatePda2,
      systemProgram: SystemProgram.programId,
    };

    await program.methods.initializeCandidate(
      "Akshay",
      pollID
    ).accounts(account1).rpc();
    await program.methods.initializeCandidate(
      "Antony",
      pollID
    ).accounts(account2).rpc();

    const candidate1p = await program.account.candidateAccount.fetch(CandidatePda1);
    const candidate2p = await program.account.candidateAccount.fetch(CandidatePda2);
    const pollTransaction = await program.account.pollAccount.fetch(pollPda);

    expect(candidate1p.candidaiteVotes.toNumber()).to.equal(0);
    expect(candidate2p.candidaiteVotes.toNumber()).to.equal(0);



    console.log("candidate1:", candidate1p);
    console.log("candidate2:", candidate2p);
    console.log("poll:", pollTransaction)

  });

  it("vote test", async () => {

    const pollID = new anchor.BN(1);
    const candidate1 = "Akshay";

    const [pollPda] = PublicKey.findProgramAddressSync(
      [pollID.toArrayLike(Buffer, "be", 8)],
      program.programId
    );

    const [CandidatePda1] = PublicKey.findProgramAddressSync(
      [pollID.toArrayLike(Buffer, "be", 8), Buffer.from(candidate1)],
      program.programId
    );


    const account: {
      signer: PublicKey;
      pollAccount: PublicKey;
      candidateAccount: PublicKey
      systemProgram: PublicKey;
    } = {
      signer: provider.wallet.publicKey,
      pollAccount: pollPda,
      candidateAccount: CandidatePda1,
      systemProgram: SystemProgram.programId,
    };


    await program.methods.vote(
      "Akshay",
      pollID
    ).accounts(account)
    .rpc();

    const candidate1p = await program.account.candidateAccount.fetch(CandidatePda1);
    console.log("candidate1:", candidate1p);
    expect(candidate1p.candidaiteVotes.toNumber()).to.equal(1);
  });


});
