import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorTemplate } from "../target/types/anchor_template";

describe("anchor_template", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorTemplate as Program<AnchorTemplate>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
