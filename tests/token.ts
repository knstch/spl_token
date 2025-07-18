import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token } from "../target/types/token";
import { getAssociatedTokenAddress, getAccount, getMint } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import {assert} from "chai";
import { startAnchor } from 'solana-bankrun';
import {BankrunProvider} from "anchor-bankrun";
const IDL = require('../target/idl/token.json');

const PROGRAM_ID = new PublicKey(IDL.address);
const METADATA_PROGRAM_ID = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');

describe("token", () => {
  let context: any;
  let provider: BankrunProvider;
  let program: Program<Token>;
  let admin: anchor.Wallet;
  let secondUser: anchor.Wallet;
  let mintPda: PublicKey;
  let ata: PublicKey;

  before(async () => {
    context = await startAnchor(
        '',
        [
          {name: 'token', programId: PROGRAM_ID},
          {name: 'token_metadata', programId: METADATA_PROGRAM_ID},
        ],
        [],
    );

    provider = new BankrunProvider(context);
    anchor.setProvider(provider);
    program = anchor.workspace.token as Program<Token>;

    const mintAccountSeed = Buffer.from("mint");

    [mintPda] = PublicKey.findProgramAddressSync([mintAccountSeed], program.programId)

    ata = await getAssociatedTokenAddress(mintPda, mintPda, true)

    admin = provider.wallet as anchor.Wallet;
    
    const secondUserKeypair = anchor.web3.Keypair.generate();
    secondUser = new anchor.Wallet(secondUserKeypair);
  })

  it("Initialize program owner", async () => {
    const _ = await program.
    methods.
    initOwner().
    accounts({
      admin: admin.publicKey,
    }).rpc()

    console.log("Admin PubKey is ", admin.publicKey.toBase58());
  })

  it("Initialize program owner again with other credentials", async () => {
    let errorCought = false;
    try {
      const _ = await program.
      methods.
      initOwner().
      accounts({
        admin: secondUser.publicKey,
      }).rpc()
    } catch (_) {
      errorCought = true;
    }

    assert.isTrue(errorCought, "Error should be caught");
  })

  it("Create token with wrong owner credentials", async () => {
    let errorCought = false;
    try {
      const _ = await program.
      methods.
      createToken("test", "TEST", "https://test.com/spl-token.json").
      accounts({
        payer: secondUser.publicKey,
      }).
      rpc()
    } catch (_) {
      errorCought = true;
    }

    assert.isTrue(errorCought, "Error should be caught");
  })
  
  it("Create token", async () => {
    const tx = await program.
    methods.
    createToken("test", "TEST", "https://test.com/spl-token.json").
    accounts({
      payer: admin.publicKey,
    }).
    rpc()
    console.log("Your create token transaction signature", tx);
  });

  it("Mint supply with wrong owner", async () => {
    let errorCought = false;
    try {
      const _ = await program.
      methods.
      mintSupply().
      accounts({
        payer: secondUser.publicKey,
      }).
      rpc()
    } catch (_) {
      errorCought = true;
    }

    assert.isTrue(errorCought, "Error should be caught");
  })

  it("Mint supply", async () => {
    const tx = await program.
    methods.
    mintSupply().
        accounts({
      payer: admin.publicKey,
    }).
    rpc()

    console.log("Your mint token transaction signature", tx);

    const treasuryInfo = await getAccount(provider.connection, ata)
    const tokenAmount = await getMint(provider.connection, mintPda);

    const uiAmount = Number(treasuryInfo.amount) / 10 ** tokenAmount.decimals;

    console.log("raw amount:", treasuryInfo.amount.toString());
    console.log("ui amount:", uiAmount, "tokens");

    assert.equal(uiAmount, 100_000_000)
  })
});
