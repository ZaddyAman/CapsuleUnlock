import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { CapsuleUnlock } from "../target/types/capsule_unlock";
import { expect } from "chai";

describe("capsule_unlock", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CapsuleUnlock as Program<CapsuleUnlock>;

  it("Initializes a capsule", async () => {
    const capsuleKeypair = anchor.web3.Keypair.generate();
    await program.methods
      .initialize()
      .accounts({
        capsule: capsuleKeypair.publicKey,
        user: program.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([capsuleKeypair])
      .rpc();

    const capsuleAccount = await program.account.capsule.fetch(capsuleKeypair.publicKey);
    expect(capsuleAccount.owner.toString()).to.equal(program.provider.publicKey.toString());
  });
});