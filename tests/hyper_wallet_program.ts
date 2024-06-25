import * as anchor from "@coral-xyz/anchor";
import {
  createMint,
  fundWithSol,
  fundWithSpl,
  getSPLBalance,
  program,
  provider,
  SPL_TOKEN_DECIMALS,
} from "./utils";
import { BN } from "bn.js";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

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
    const voters = [
      ownerKeypair.publicKey,
      deviceKeypair.publicKey,
      keychainKeypair.publicKey,
    ];
    const signature = await program.methods
      .createHyperWallet(voters, 2, bump)
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

    await program.methods
      .createTransferLamportsProposal(
        receiverKeypair.publicKey,
        new BN(0.5 * Math.pow(10, 9))
      )
      .accounts({
        hyperWallet: hyperWalletPDA,
        proposal: proposalKeypair.publicKey,
        rentPayer: anchor.getProvider().publicKey,
        owner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair, proposalKeypair])
      .rpc();

    try {
      await program.methods
        .executeTransferLamportsProposal()
        .accounts({
          hyperWallet: hyperWalletPDA,
          proposal: proposalKeypair.publicKey,
          to: receiverKeypair.publicKey,
        })
        .rpc();
    } catch (error) {
      console.log(error);
    }

    await program.methods
      .approveTransferLamportsProposal()
      .accounts({
        hyperWallet: hyperWalletPDA,
        proposal: proposalKeypair.publicKey,
        voter: deviceKeypair.publicKey,
      })
      .signers([deviceKeypair])
      .rpc();

    await program.methods
      .executeTransferLamportsProposal()
      .accounts({
        hyperWallet: hyperWalletPDA,
        proposal: proposalKeypair.publicKey,
        to: receiverKeypair.publicKey,
      })
      .rpc();

    const hyperWalletBalance = await provider.connection.getBalance(
      hyperWalletPDA
    );
    const receiverBalance = await provider.connection.getBalance(
      receiverKeypair.publicKey
    );
    console.log({ hyperWalletBalance, receiverBalance });
  });

  it("Transfer SPL", async () => {
    const ownerKeypair = anchor.web3.Keypair.generate();
    const deviceKeypair = anchor.web3.Keypair.generate();
    const keychainKeypair = anchor.web3.Keypair.generate();
    const [hyperWalletPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [ownerKeypair.publicKey.toBuffer()],
      program.programId
    );
    const voters = [
      ownerKeypair.publicKey,
      deviceKeypair.publicKey,
      keychainKeypair.publicKey,
    ];
    const signature = await program.methods
      .createHyperWallet(voters, 2, bump)
      .accounts({
        hyperWallet: hyperWalletPDA,
        owner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const sponsorKeypair = anchor.web3.Keypair.generate();
    await fundWithSol(sponsorKeypair.publicKey);
    await fundWithSpl(hyperWalletPDA, sponsorKeypair, splMintAddress);

    const proposalKeypair = anchor.web3.Keypair.generate();
    const receiverKeypair = anchor.web3.Keypair.generate();

    const fromAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      sponsorKeypair,
      splMintAddress,
      hyperWalletPDA,
      true
    );
    const toAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      sponsorKeypair,
      splMintAddress,
      receiverKeypair.publicKey
    );

    const splToSend = 1;
    const amountToSend = new BN(splToSend * Math.pow(10, SPL_TOKEN_DECIMALS));

    await program.methods
      .createTransferSplProposal(fromAta.address, toAta.address, amountToSend)
      .accounts({
        hyperWallet: hyperWalletPDA,
        proposal: proposalKeypair.publicKey,
        rentPayer: anchor.getProvider().publicKey,
        owner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair, proposalKeypair])
      .rpc();

    try {
      await program.methods
        .executeTransferSplProposal()
        .accounts({
          hyperWallet: hyperWalletPDA,
          proposal: proposalKeypair.publicKey,
          fromAta: fromAta.address,
          toAta: toAta.address,
          owner: ownerKeypair.publicKey,
        })
        .signers([ownerKeypair])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    await program.methods
      .approveTransferSplProposal()
      .accounts({
        hyperWallet: hyperWalletPDA,
        proposal: proposalKeypair.publicKey,
        voter: deviceKeypair.publicKey,
      })
      .signers([deviceKeypair])
      .rpc();

    await program.methods
      .executeTransferSplProposal()
      .accounts({
        hyperWallet: hyperWalletPDA,
        proposal: proposalKeypair.publicKey,
        fromAta: fromAta.address,
        toAta: toAta.address,
        owner: ownerKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .rpc();

    const fromBalance = await getSPLBalance(fromAta.address);
    const toBalance = await getSPLBalance(toAta.address);
    console.log({ fromBalance, toBalance });
  });
});
