import * as anchor from "@coral-xyz/anchor";

module.exports = async function (provider: anchor.Provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add deployment logic here
  console.log("Deploying CapsuleUnlock program...");
  // Example: const program = anchor.workspace.CapsuleUnlock;
  // await program.rpc.initialize(...);
};