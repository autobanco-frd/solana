import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NewToken } from "../target/types/new_token";
import { Keypair } from "@solana/web3.js"

describe("new-token", () => {
  // Configure the client to use the local cluster.
const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NewToken as Program<NewToken>;
  const mintKp = new Keypair()

  it("Crea un nuevo token", async () => {
    // Add your test here.
    const tx = await program.methods.CreateToken()
    .accounts({
      payer: provider.wallet.publicKey,
      mintAccount: mintKp.publicKey
    })
    .signers([
      mintKp
    ])
    .rpc();
    console.log ("Tu Token address: ", mintKp.publicKey);
    console.log ("Your transaction signature", tx);
  });
});
