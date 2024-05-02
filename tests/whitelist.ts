import * as anchor from "@coral-xyz/anchor";
import {
  fundWithSol,
  fundWithSpl,
  getSPLBalance,
  program,
  provider,
  SPL_TOKEN_DECIMALS,
} from "./utils";
import {
  createTransferInstruction,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { assert } from "chai";

export function testWhiteList() {
  let ownerKeypair: anchor.web3.Keypair;
  let hyperWalletPDA: anchor.web3.PublicKey;
  let keypair: anchor.web3.Keypair;
  before(async () => {
    ownerKeypair = new anchor.web3.Keypair();
    hyperWalletPDA = anchor.web3.PublicKey.findProgramAddressSync(
      [ownerKeypair.publicKey.toBuffer()],
      program.programId
    )[0];
    keypair = anchor.web3.Keypair.generate();

    await program.methods
      .createHyperWallet()
      .accounts({
        hyperWallet: hyperWalletPDA,
        owner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();
  });
  it("Can enable white list", async () => {
    await program.methods
      .enableWhiteList()
      .accounts({
        hyperWallet: hyperWalletPDA,
        hyperWalletOwner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const hyperWalletAccount = await program.account.hyperWallet.fetch(
      hyperWalletPDA
    );
    assert.equal(hyperWalletAccount.whiteListEnabled, true);
  });
  it("Can disable white list", async () => {
    await program.methods
      .disableWhiteList()
      .accounts({
        hyperWallet: hyperWalletPDA,
        hyperWalletOwner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const hyperWalletAccount = await program.account.hyperWallet.fetch(
      hyperWalletPDA
    );
    assert.equal(hyperWalletAccount.whiteListEnabled, false);
  });
  it("Can add to white list", async () => {
    await program.methods
      .addToWhiteList(keypair.publicKey)
      .accounts({
        hyperWallet: hyperWalletPDA,
        hyperWalletOwner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const hyperWalletAccount = await program.account.hyperWallet.fetch(
      hyperWalletPDA
    );
    assert.include(
      hyperWalletAccount.whiteListedAddresses.map((a) => a.toString()),
      keypair.publicKey.toString()
    );
  });
  it("Can remove from white list", async () => {
    await program.methods
      .removeFromWhiteList(keypair.publicKey)
      .accounts({
        hyperWallet: hyperWalletPDA,
        hyperWalletOwner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const hyperWalletAccount = await program.account.hyperWallet.fetch(
      hyperWalletPDA
    );
    assert.notInclude(
      hyperWalletAccount.whiteListedAddresses.map((a) => a.toString()),
      keypair.publicKey.toString()
    );
  });
}
