import * as anchor from "@coral-xyz/anchor";
import {assert} from "chai";
import {fundWithSol, getBalance, logSignature, provider} from "./utils";
import {LAMPORTS_PER_SOL} from "@solana/web3.js";
import {program} from "./utils";
import BN from "bn.js";

export const testTransferLamports = () => {
    it("Can receive SOL from another NORMAL wallet", async () => {
        const ownerKeypair = new anchor.web3.Keypair();
        const [hyperWalletPDA] = anchor.web3.PublicKey.findProgramAddressSync([ownerKeypair.publicKey.toBuffer()], program.programId);
        await program.methods.createHyperWallet().accounts({
            hyperWallet: hyperWalletPDA,
            owner: ownerKeypair.publicKey
        }).signers([ownerKeypair]).rpc();

        const normalKeypair = new anchor.web3.Keypair();
        await fundWithSol(normalKeypair.publicKey, 5);
        const solToSend = 1;
        const lamportsToSend = solToSend * LAMPORTS_PER_SOL;

        const tx = new anchor.web3.Transaction();
        tx.add(
            anchor.web3.SystemProgram.transfer({
                fromPubkey: normalKeypair.publicKey,
                toPubkey: hyperWalletPDA,
                lamports: lamportsToSend
            })
        );
        const signature = await provider.sendAndConfirm(tx, [normalKeypair]);
        console.log(signature);

        const normalWalletBalance = await getBalance(normalKeypair.publicKey);
        const hyperWalletBalance = await getBalance(hyperWalletPDA);
        assert.isAtMost(normalWalletBalance, 4 * LAMPORTS_PER_SOL);
        assert.isAtLeast(hyperWalletBalance, 1 * LAMPORTS_PER_SOL);
    });

    it("Can send SOL to another NORMAL wallet", async () => {
        const ownerKeypair = new anchor.web3.Keypair();
        const [hyperWalletPDA] = anchor.web3.PublicKey.findProgramAddressSync([ownerKeypair.publicKey.toBuffer()], program.programId);
        await program.methods.createHyperWallet().accounts({
            hyperWallet: hyperWalletPDA,
            owner: ownerKeypair.publicKey
        }).signers([ownerKeypair]).rpc();
        await fundWithSol(hyperWalletPDA, 5);

        const normalKeypair = new anchor.web3.Keypair();
        const solToSend = 1;
        const lamportsToSend = new BN(solToSend * LAMPORTS_PER_SOL);

        const signature = await program.methods.transferLamports(lamportsToSend).accounts({
            fromHyperWallet: hyperWalletPDA,
            hyperWalletOwner: ownerKeypair.publicKey,
            to: normalKeypair.publicKey,
        }).signers([ownerKeypair]).rpc();
        console.log(signature);

        const normalWalletBalance = await getBalance(normalKeypair.publicKey);
        const hyperWalletBalance = await getBalance(hyperWalletPDA);
        assert.isAtLeast(normalWalletBalance, 1 * LAMPORTS_PER_SOL);
    });

    it("Can send SOL to another HYPER wallet", async () => {
        const senderKeypair = new anchor.web3.Keypair();
        const receiverKeypair = new anchor.web3.Keypair();
        const [senderHyperWalletPDA] = anchor.web3.PublicKey.findProgramAddressSync([senderKeypair.publicKey.toBuffer()], program.programId);
        const [receiverHyperWalletPDA] = anchor.web3.PublicKey.findProgramAddressSync([receiverKeypair.publicKey.toBuffer()], program.programId);

        await program.methods.createHyperWallet().accounts({
            hyperWallet: senderHyperWalletPDA,
            owner: senderKeypair.publicKey
        }).signers([senderKeypair]).rpc();
        await program.methods.createHyperWallet().accounts({
            hyperWallet: receiverHyperWalletPDA,
            owner: receiverKeypair.publicKey
        }).signers([receiverKeypair]).rpc();

        await fundWithSol(senderHyperWalletPDA, 5);
        const solToSend = 1;
        const lamportsToSend = new BN(solToSend * LAMPORTS_PER_SOL);

        const signature = await program.methods.transferLamports(lamportsToSend).accounts({
            fromHyperWallet: senderHyperWalletPDA,
            hyperWalletOwner: senderKeypair.publicKey,
            to: receiverHyperWalletPDA,
        }).signers([senderKeypair]).rpc();
        console.log(signature);

        const receiveHyperWalletBalance = await getBalance(receiverHyperWalletPDA);
        assert.isAtLeast(receiveHyperWalletBalance, 1 * LAMPORTS_PER_SOL);
    });
}
