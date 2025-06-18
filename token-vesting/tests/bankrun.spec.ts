import *  as anchor from '@coral-xyz/anchor';
import { SYSTEM_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/native/system';
import { Keypair, PublicKey } from "@solana/web3.js";
import { BankrunProvider } from 'anchor-bankrun';
import { BanksClient, Clock, ProgramTestContext, startAnchor } from 'solana-bankrun';
import { Vesting } from '../target/types/vesting';
import { Program } from '@coral-xyz/anchor';
import { createMint } from 'spl-token-bankrun'
import { publicKey } from '@coral-xyz/anchor/dist/cjs/utils';
import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';
import { Token, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { mintTo } from 'spl-token-bankrun';
import { BN } from 'bn.js';


const IDL = require("../target/idl/vesting.json");

describe("vesting Smart Contract", async () => {
    const companyName = "companyName"
    let beneficiary: Keypair;
    let context: ProgramTestContext;
    let provider: BankrunProvider;
    let program: anchor.Program<Vesting>;
    let banksClient: BanksClient;
    let employer: Keypair;
    let mint: PublicKey;
    let beneficiaryProvider: BankrunProvider;
    let program2: Program<Vesting>;
    let vestingAccountKey: PublicKey;
    let treasuryTokenAccount: PublicKey;
    let employeeAccount: PublicKey;



    beforeEach(async () => {


        beneficiary = new anchor.web3.Keypair();


        context = await startAnchor(
            "", // Empty string for default workspace
            [{ name: "vesting", programId: new PublicKey(IDL.address) }],
            [
                {
                    address: beneficiary.publicKey,
                    info: {
                        lamports: 1_000_000_000, // 1 SOL
                        data: Buffer.alloc(0),
                        owner: SYSTEM_PROGRAM_ID,
                        executable: false
                    }
                },
            ]
        );

        provider = new BankrunProvider(context);

        anchor.setProvider(provider);

        program = new Program<Vesting>(IDL as Vesting, provider);

        banksClient = context.banksClient;

        employer = provider.wallet.payer;

        mint = await createMint(banksClient, employer, employer.publicKey, null, 2);

        beneficiaryProvider = new BankrunProvider(context);
        beneficiaryProvider.wallet = new NodeWallet(beneficiary);

        program2 = new Program<Vesting>(IDL as Vesting, beneficiaryProvider);

        [vestingAccountKey] = PublicKey.findProgramAddressSync(
            [Buffer.from(companyName)],
            program.programId,
        ),

            [treasuryTokenAccount] = PublicKey.findProgramAddressSync(
                [Buffer.from("vesting_treasury"),
                Buffer.from(companyName)],
                program.programId
            ),

            [employeeAccount] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("employee_vesting"),
                    beneficiary.publicKey.toBuffer(),
                    vestingAccountKey.toBuffer()
                ],
                program.programId
            )
    })

    it("create vesting account", async () => {
        const tx = await program.methods.createVestingAccount(companyName).accounts({
            signer: employer.publicKey,
            mint: mint,
            tokenProgram: TOKEN_PROGRAM_ID,
        }).rpc({ commitment: "confirmed" });

        const vesting_account_data = await program.account.vestingAccount.fetch(vestingAccountKey, "confirmed");

        console.log("vesting account:", vesting_account_data);
        console.log("create vesting accounts:", tx);


        const amount = 10_000 * 10 ** 9;
        const mintTx = await mintTo(
            banksClient,
            employer,
            mint,
            treasuryTokenAccount,
            employer,
            amount
        );

        console.log("mint tx:", mintTx);
    })

    // it("fund the treasury token account",async()=>{
    //     const amount = 10_000 * 10 ** 9;
    //     const mintTx = await mintTo(
    //         banksClient,
    //         employer,
    //         mint,
    //         treasuryTokenAccount,
    //         employer,
    //         amount
    //     );

    //     console.log("mint tx:",mintTx);
    // })

    it("create employee vesting account", async () => {
        const tx2 = program.methods.createEmployeeAccount(
            new BN(0),
            new BN(200),
            new BN(100),
            new BN(0)
        ).accounts({
            beneficery: beneficiary.publicKey,
            vestingAccount: vestingAccountKey
        }).rpc({ commitment: 'confirmed', skipPreflight: true });


        console.log("create employee account tx:", tx2);

        console.log("employee account", employeeAccount.toBase58());


    });

    it("should claim tokens", async () => {
        await new Promise(resolve => setTimeout(resolve, 1000));
        const currentClock = await banksClient.getClock();
        context.setClock(
            new Clock(
                currentClock.slot,
                currentClock.epochStartTimestamp,
                currentClock.epoch,
                currentClock.leaderScheduleEpoch,
                BigInt(1000)
            )
        )

        const tx3 = program2.methods.claimTokens(companyName).accounts({
            tokenProgram: TOKEN_PROGRAM_ID
        }).rpc({ commitment: "confirmed" });

        console.log("claim tx:", tx3);

    })


})