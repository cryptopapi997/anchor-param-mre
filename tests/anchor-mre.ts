import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorMre } from "../target/types/anchor_mre";
import { ASSOCIATED_TOKEN_PROGRAM_ID, MINT_SIZE, TOKEN_PROGRAM_ID, createAssociatedTokenAccountIdempotentInstruction, createInitializeMint2Instruction, createMintToInstruction, getAssociatedTokenAddressSync, getMinimumBalanceForRentExemptMint, transfer } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";

export const TOKEN_MINT = Keypair.fromSecretKey(new Uint8Array([
  233, 132, 53, 39, 177, 254, 146, 147, 56, 5, 201,
  25, 151, 108, 175, 134, 226, 255, 11, 184, 116, 200,
  236, 178, 88, 203, 30, 213, 123, 29, 34, 101, 160,
  125, 200, 55, 211, 178, 66, 27, 149, 22, 219, 191,
  28, 218, 171, 113, 92, 216, 236, 165, 124, 20, 89,
  205, 119, 106, 175, 166, 185, 155, 69, 242
]))


describe("Demo", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.AnchorMre as Program<AnchorMre>;
  const provider = anchor.getProvider();
  const fstAccOffset = new anchor.BN(2);
  const sndAccOffset = new anchor.BN(1);

  const [signer] = [Keypair.generate()];
  const [signerATA] = [signer]
    .map((a) => getAssociatedTokenAddressSync(TOKEN_MINT.publicKey, a.publicKey));
  const fstAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("FstAcc"), fstAccOffset.toBuffer('le', 4)],
    program.programId
  )[0];
  const sndAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("SndAcc"), sndAccOffset.toBuffer('le', 4)],
    program.programId
  )[0];
  const thrdAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("ThrdAcc")],
    program.programId
  )[0];
  const frthAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("FrthAcc"), signer.publicKey.toBuffer()],
    program.programId
  )[0];
  const fifthAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("FifthAcc"), frthAccount.toBuffer()],
    program.programId
  )[0];
  const padding = PublicKey.findProgramAddressSync(
    [Buffer.from("Padding")],
    program.programId
  )[0];

  const thrdAccountATA = getAssociatedTokenAddressSync(TOKEN_MINT.publicKey, thrdAccount, true);

  before("Initialize accounts", async () => {
    const mint_tx = new Transaction();
    let lamports = await getMinimumBalanceForRentExemptMint(provider.connection);
    mint_tx.instructions = [
      SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: TOKEN_MINT.publicKey,
        lamports,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
      }),
      createInitializeMint2Instruction(TOKEN_MINT.publicKey, 6, provider.publicKey, null),
    ];
    await provider.sendAndConfirm(mint_tx, [TOKEN_MINT]);

    let tx = new Transaction();
    tx.instructions = [
      ...[signer].map((k) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: k.publicKey,
          lamports: LAMPORTS_PER_SOL,
        })
      ),
      ...[
        [signer.publicKey, signerATA],
      ].flatMap((x) => [
        createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, x[1], x[0], TOKEN_MINT.publicKey),
        createMintToInstruction(TOKEN_MINT.publicKey, x[1], provider.publicKey, 1e9),
      ]),
    ];
    await provider.sendAndConfirm(tx);

    await program.methods.initOne().accounts({
      signer: signer.publicKey,
      thrdAcc: thrdAccount,
      mint: TOKEN_MINT.publicKey,
      thrdAccAta: thrdAccountATA,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    }).signers([signer]).rpc();
    await program.methods.initTwo().accounts({
      signer: signer.publicKey,
      frthAcc: frthAccount,
      fifthAcc: fifthAccount,
      padding: padding,
      systemProgram: SystemProgram.programId,
    }).signers([signer]).rpc();

    await program.methods.initThree(fstAccOffset.toNumber(), sndAccOffset.toNumber()).accounts({
      signer: signer.publicKey,
      fstAcc: fstAccount,
      sndAcc: sndAccount,
      systemProgram: SystemProgram.programId,
    }).signers([signer]).rpc();
  });

  describe("Demo", () => {
    it('Demo the bug', async () => {
      let current_claim = {
        buf: Buffer.from(Array(32).fill(0)),
        buggyField: new anchor.BN(1),
      };

      console.log("We send with buggy field value as ", current_claim.buggyField);
      await program.methods.demo(fstAccOffset.toNumber(), sndAccOffset.toNumber(), current_claim).accounts({
        signerAgain: signer.publicKey,
        signerAta: signerATA,
        padding: padding,
        signer: signer.publicKey,
        sndAcc: sndAccount,
        frthAcc: frthAccount,
        thrdAcc: thrdAccount,
        thrdAccAta: thrdAccountATA,
        tokenProgram: TOKEN_PROGRAM_ID,
        fstAcc: fstAccount,
      }).signers([signer]).rpc().catch((e) => {
        console.log(e);
      });
    });

  });
});
