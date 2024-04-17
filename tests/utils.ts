import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import {LAMPORTS_PER_SOL} from "@solana/web3.js";
import { HyperWalletProgram } from "../target/types/hyper_wallet_program";
import {Program} from "@coral-xyz/anchor";

export const SPL_TOKEN_DECIMALS = 6;
export const connection = new anchor.web3.Connection(
    "http://127.0.0.1:8899",
    "confirmed")
export const provider = anchor.AnchorProvider.env();
export const program = anchor.workspace.HyperWalletProgram as Program<HyperWalletProgram>;

export const createMint = async (): Promise<anchor.web3.PublicKey> => {
    const tokenMint = new anchor.web3.Keypair();
    const lamportsForMint = await connection.getMinimumBalanceForRentExemption(
        spl.MintLayout.span
    );
    const tx = new anchor.web3.Transaction();
    tx.add(
        anchor.web3.SystemProgram.createAccount({
            programId: spl.TOKEN_PROGRAM_ID,
            space: spl.MintLayout.span,
            fromPubkey: provider.wallet.publicKey,
            newAccountPubkey: tokenMint.publicKey,
            lamports: lamportsForMint,
        })
    );
    tx.add(
        spl.createInitializeMintInstruction(
            tokenMint.publicKey,
            SPL_TOKEN_DECIMALS,
            provider.wallet.publicKey,
            provider.wallet.publicKey
        )
    );
    const signature = await provider.sendAndConfirm(tx, [tokenMint]);
    console.log(
        `Created new mint account at ${tokenMint.publicKey}. Sig: ${signature}`
    );
    return tokenMint.publicKey;
};

export const fundWithSol = async (publicKey: anchor.web3.PublicKey, solAmount = 1) => {
    const signature = await connection.requestAirdrop(
        publicKey,
        solAmount * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(signature);
    logSignature(
        signature,
        `Funded ${publicKey.toString()} with ${solAmount} SOL`
    );
};

export const fundWithSpl = async (
    accountAddress: anchor.web3.PublicKey,
    feePayer: anchor.web3.Keypair,
    mint: anchor.web3.PublicKey
) => {
    const ata = await spl.getOrCreateAssociatedTokenAccount(
        provider.connection,
        feePayer,
        mint,
        accountAddress,
        true
    );
    const tx = new anchor.web3.Transaction();
    tx.add(
        spl.createMintToInstruction(
            mint,
            ata.address,
            provider.wallet.publicKey,
            5000000
        )
    );
    const signature = await provider.sendAndConfirm(tx);
    logSignature(
        signature,
        `Funded ${accountAddress.toString()} with 5 SPL: ${signature}`
    );
};

export const logSignature = (signature: string, message: string) => {
    console.log(`---
${message}
${signature}`);
}

export const getBalance = async (accountAddress: anchor.web3.PublicKey)=> {
    return provider.connection.getBalance(accountAddress);
}

export const getSPLBalance = async (tokenAccountAddress: anchor.web3.PublicKey)=> {
    return provider.connection.getTokenAccountBalance(tokenAccountAddress);
}