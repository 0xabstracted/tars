import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TarsLaunch } from "../target/types/tars_launch";

describe("TarsLaunch", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TarsLaunch as Program<TarsLaunch>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
