import * as anchor from "@coral-xyz/anchor";
import { program } from "./utils";
import { createHash, randomBytes } from "crypto";
import * as base32 from "base32-ts";
import * as totp from "totp-generator";
import { MerkleTree } from "merkletreejs";
import { SHA256 } from "crypto-js";

export function testOtp() {
  let keypair: anchor.web3.Keypair;
  let hyperWalletPda: anchor.web3.PublicKey;

  let secretKey: string; // When implemented on mobile, this will be replaced by authenticator app
  let tree: MerkleTree; // When implemented on mobile, this will be replaced by storing the tree in secure store
  const PERIOD_IN_SECONDS = 1; // a period of 30 seconds for generating otp codes
  const OTP_CODE_AMOUNT = Math.pow(2, 10);
  let initTimeInSeconds: number;

  before(async () => {
    // Create a new hyper wallet
    keypair = anchor.web3.Keypair.generate();
    hyperWalletPda = anchor.web3.PublicKey.findProgramAddressSync(
      [keypair.publicKey.toBuffer()],
      program.programId
    )[0];

    await program.methods
      .createHyperWallet()
      .accounts({
        hyperWallet: hyperWalletPda,
        owner: keypair.publicKey,
      })
      .signers([keypair])
      .rpc();
  });

  it("Can set up otp", async () => {
    // Generate random secret
    initTimeInSeconds = Math.floor(Date.now() / 1000);
    secretKey = base32.Base32.encode(randomBytes(20), "RFC4648");

    // Generate OTP codes for 6 months + build Merkle tree
    const leave_values = [];
    for (let i = 0; i < OTP_CODE_AMOUNT; i++) {
      const otp = totp.TOTP.generate(secretKey, {
        period: PERIOD_IN_SECONDS,
        timestamp: (initTimeInSeconds + i * PERIOD_IN_SECONDS) * 1000,
      }).otp.toString();
      leave_values.push(otp);
    }
    const leaves = leave_values.map((x) =>
      createHash("sha256").update(x).digest()
    );
    tree = new MerkleTree(leaves, (data) =>
      createHash("sha256").update(data).digest()
    );
    const root = tree.getRoot();

    // Generate QR code/link
    const otpLink = `otpauth://totp/Hyper%Wallet:${hyperWalletPda}?secret=${secretKey}&issuer=Hyper%20Wallet&algorithm=SHA1&digits=6&period=30`;

    await program.methods
      .setUpOtp({
        initTime: initTimeInSeconds,
        root: [...root],
      })
      .accounts({
        hyperWallet: hyperWalletPda,
        hyperWalletOwner: keypair.publicKey,
      })
      .signers([keypair])
      .rpc();

    const hyperWalletAccount = await program.account.hyperWallet.fetch(
      hyperWalletPda
    );
  });

  it("can confirm otp", async () => {
    await new Promise((resolve) => {
      setTimeout(() => {
        resolve(true);
      }, 3000);
    });
    // The interval is fixed to 1
    const currentTimestamp = Math.floor(Date.now() / 1000) * 1000;
    const otp = totp.TOTP.generate(secretKey, {
      period: PERIOD_IN_SECONDS,
      timestamp: currentTimestamp,
    }).otp.toString();
    const otpHash = createHash("sha256").update(otp).digest();
    const proofPath = tree.getProof(otpHash);
    const proofHash = proofPath.map((path) => Array.from(path.data));

    await program.methods
      .confirmOtp({
        otpHash: [...otpHash],
        proofHash: [...proofHash],
      })
      .accounts({
        hyperWallet: hyperWalletPda,
        hyperWalletOwner: keypair.publicKey,
      })
      .signers([keypair])
      .rpc();
  });
}
