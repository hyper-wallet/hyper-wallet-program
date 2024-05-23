import * as anchor from "@coral-xyz/anchor";
import { program } from "./utils";
import { assert, expect } from "chai";

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
      .enableWhitelist()
      .accounts({
        hyperWallet: hyperWalletPDA,
        hyperWalletOwner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const hyperWalletAccount = await program.account.hyperWallet.fetch(
      hyperWalletPDA
    );
    assert.equal(hyperWalletAccount.whitelistEnabled, true);
  });

  it("Can add to white list", async () => {
    await program.methods
      .addToWhitelist(keypair.publicKey)
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
      hyperWalletAccount.whitelistedAddresses.map((a) => a.toString()),
      keypair.publicKey.toString()
    );
  });

  it("Can verify a receiver that is whitelisted", async () => {
    await program.methods
      .verifyReceiver(keypair.publicKey)
      .accounts({
        hyperWallet: hyperWalletPDA,
        hyperWalletOwner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();
  });

  it("Can remove from white list", async () => {
    await program.methods
      .removeFromWhitelist(keypair.publicKey)
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
      hyperWalletAccount.whitelistedAddresses.map((a) => a.toString()),
      keypair.publicKey.toString()
    );
  });

  it("Can verify a receiver that is not whitelisted", async () => {
    try {
      await program.methods
        .verifyReceiver(keypair.publicKey)
        .accounts({
          hyperWallet: hyperWalletPDA,
          hyperWalletOwner: ownerKeypair.publicKey,
        })
        .signers([ownerKeypair])
        .rpc();
    } catch (e) {
      expect(e).to.not.null;
    }
  });

  it("Can disable white list", async () => {
    await program.methods
      .disableWhitelist()
      .accounts({
        hyperWallet: hyperWalletPDA,
        hyperWalletOwner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const hyperWalletAccount = await program.account.hyperWallet.fetch(
      hyperWalletPDA
    );
    assert.equal(hyperWalletAccount.whitelistEnabled, false);
  });
}
