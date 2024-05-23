import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";
import {
  createMint,
  fundWithSol,
  fundWithSpl,
  getSPLBalance,
  provider,
  SPL_TOKEN_DECIMALS,
} from "./utils";
import { program } from "./utils";
import BN from "bn.js";
import {
  createTransferInstruction,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";

export const testTransferSPL = () => {
  let splMintAddress: anchor.web3.PublicKey;

  before(async () => {
    splMintAddress = await createMint();
  });

  it("Can receive SPL from another NORMAL wallet", async () => {
    // ACCOUNTS PREPARATION
    const ownerKeypair = new anchor.web3.Keypair();
    const [hyperWalletPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [ownerKeypair.publicKey.toBuffer()],
      program.programId
    );
    await program.methods
      .createHyperWallet()
      .accounts({
        hyperWallet: hyperWalletPDA,
        owner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const normalKeypair = new anchor.web3.Keypair();
    await fundWithSol(normalKeypair.publicKey, 5);
    await fundWithSpl(normalKeypair.publicKey, normalKeypair, splMintAddress);

    const hyperWalletATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      normalKeypair,
      splMintAddress,
      hyperWalletPDA,
      true
    );
    const normalWalletATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      normalKeypair,
      splMintAddress,
      normalKeypair.publicKey
    );

    // TRANSACTION PREPARATION
    const splToSend = 1;
    const amountToSend = splToSend * Math.pow(10, SPL_TOKEN_DECIMALS);
    const tx = new anchor.web3.Transaction();
    tx.add(
      createTransferInstruction(
        normalWalletATA.address,
        hyperWalletATA.address,
        normalKeypair.publicKey,
        amountToSend
      )
    );
    const signature = await provider.sendAndConfirm(tx, [normalKeypair], {
      skipPreflight: true,
    });

    const hyperWalletSPLBalance = await getSPLBalance(hyperWalletATA.address);
    assert.isAtLeast(
      Number(hyperWalletSPLBalance.value.amount),
      1 * Math.pow(10, SPL_TOKEN_DECIMALS)
    );
  });

  it("Can send SPL to another NORMAL wallet", async () => {
    // ACCOUNTS PREPARATION
    const ownerKeypair = new anchor.web3.Keypair();
    const [hyperWalletPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [ownerKeypair.publicKey.toBuffer()],
      program.programId
    );
    await program.methods
      .createHyperWallet()
      .accounts({
        hyperWallet: hyperWalletPDA,
        owner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const sponsorKeypair = new anchor.web3.Keypair();
    await fundWithSol(sponsorKeypair.publicKey, 5);

    await fundWithSpl(hyperWalletPDA, sponsorKeypair, splMintAddress);

    const normalKeypair = new anchor.web3.Keypair();
    await fundWithSol(normalKeypair.publicKey, 5);

    const hyperWalletATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      normalKeypair,
      splMintAddress,
      hyperWalletPDA,
      true
    );
    const normalWalletATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      normalKeypair,
      splMintAddress,
      normalKeypair.publicKey
    );

    // TRANSACTION PREPARATION
    const splToSend = 1;
    const amountToSend = new BN(splToSend * Math.pow(10, SPL_TOKEN_DECIMALS));
    const signature = await program.methods
      .transferSpl({
        rawAmount: amountToSend,
        otpHash: null,
        proofHash: null,
      })
      .accounts({
        fromHyperWallet: hyperWalletPDA,
        fromHyperWalletAta: hyperWalletATA.address,
        to: normalKeypair.publicKey,
        toAta: normalWalletATA.address,
        hyperWalletOwner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const normalWalletSPLBalance = await getSPLBalance(normalWalletATA.address);
    assert.isAtLeast(
      Number(normalWalletSPLBalance.value.amount),
      amountToSend.toNumber()
    );
  });

  it("Can send SPL to another HYPER wallet", async () => {
    // ACCOUNTS PREPARATION
    const senderKeypair = new anchor.web3.Keypair();
    const receiverKeypair = new anchor.web3.Keypair();
    const [senderHyperWalletPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [senderKeypair.publicKey.toBuffer()],
      program.programId
    );
    const [receiverHyperWalletPDA] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [receiverKeypair.publicKey.toBuffer()],
        program.programId
      );

    await program.methods
      .createHyperWallet()
      .accounts({
        hyperWallet: senderHyperWalletPDA,
        owner: senderKeypair.publicKey,
      })
      .signers([senderKeypair])
      .rpc();
    await program.methods
      .createHyperWallet()
      .accounts({
        hyperWallet: receiverHyperWalletPDA,
        owner: receiverKeypair.publicKey,
      })
      .signers([receiverKeypair])
      .rpc();

    const sponsorKeypair = new anchor.web3.Keypair();
    await fundWithSol(sponsorKeypair.publicKey, 5);

    await fundWithSpl(senderHyperWalletPDA, sponsorKeypair, splMintAddress);
    await fundWithSpl(receiverHyperWalletPDA, sponsorKeypair, splMintAddress);

    const senderHyperWalletATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      sponsorKeypair,
      splMintAddress,
      senderHyperWalletPDA,
      true
    );
    const receiverHyperWalletATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      sponsorKeypair,
      splMintAddress,
      receiverHyperWalletPDA,
      true
    );

    // TRANSACTION PREPARATION
    const splToSend = 1;
    const amountToSend = new BN(splToSend * Math.pow(10, SPL_TOKEN_DECIMALS));
    const signature = await program.methods
      .transferSpl({
        rawAmount: amountToSend,
        otpHash: null,
        proofHash: null,
      })
      .accounts({
        fromHyperWallet: senderHyperWalletPDA,
        fromHyperWalletAta: senderHyperWalletATA.address,
        to: receiverHyperWalletPDA,
        toAta: receiverHyperWalletATA.address,
        hyperWalletOwner: senderKeypair.publicKey,
      })
      .signers([senderKeypair])
      .rpc();

    const receiverHyperWalletBalance = await getSPLBalance(
      receiverHyperWalletATA.address
    );
    assert.isAtLeast(
      Number(receiverHyperWalletBalance.value.amount),
      amountToSend.toNumber()
    );
  });
};
