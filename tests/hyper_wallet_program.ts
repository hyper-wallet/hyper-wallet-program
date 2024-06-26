import * as anchor from "@coral-xyz/anchor";
import {
  createMint,
  fundWithSol,
  program,
  provider,
  SPL_TOKEN_DECIMALS,
} from "./utils";
import { BN } from "bn.js";

describe("hyper-wallet-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  let splMintAddress: anchor.web3.PublicKey;

  before(async () => {
    splMintAddress = await createMint();
  });

  it("Transfer lamports", async () => {
    const ownerKeypair = anchor.web3.Keypair.generate();
    const deviceKeypair = anchor.web3.Keypair.generate();
    const keychainKeypair = anchor.web3.Keypair.generate();
    const [hyperWalletPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [ownerKeypair.publicKey.toBuffer()],
      program.programId
    );
    const approvers = [deviceKeypair.publicKey, keychainKeypair.publicKey];
    const signature = await program.methods
      .createHyperWallet(approvers)
      .accounts({
        hyperWallet: hyperWalletPDA,
        owner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const hyperWalletAccount = await program.account.hyperWallet.fetchNullable(
      hyperWalletPDA
    );

    await fundWithSol(hyperWalletPDA, 1);

    const proposalKeypair = anchor.web3.Keypair.generate();
    const receiverKeypair = anchor.web3.Keypair.generate();

    const invalidApproverKeypair = anchor.web3.Keypair.generate();

    await program.methods
      .transferLamports(new BN(0.5 * Math.pow(10, 9)))
      .accounts({
        hyperWallet: hyperWalletPDA,
        owner: ownerKeypair.publicKey,
        approver: invalidApproverKeypair.publicKey,
        to: receiverKeypair.publicKey,
      })
      .signers([ownerKeypair, invalidApproverKeypair])
      .rpc();
    const hyperWalletBalance = await provider.connection.getBalance(
      hyperWalletPDA
    );
    const receiverBalance = await provider.connection.getBalance(
      receiverKeypair.publicKey
    );
    console.log({ hyperWalletBalance, receiverBalance });
  });
});
