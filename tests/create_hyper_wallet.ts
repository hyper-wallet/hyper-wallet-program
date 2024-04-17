import * as anchor from "@coral-xyz/anchor";
import {program} from "./utils";
import {assert} from "chai";

export const testCreateHyperWallet = () => {
    it("Can create a hyper wallet", async () => {
        const ownerKeypair = anchor.web3.Keypair.generate();
        const [hyperWalletPDA] = anchor.web3.PublicKey.findProgramAddressSync([ownerKeypair.publicKey.toBuffer()], program.programId);
        const signature = await program.methods.createHyperWallet().accounts({
            hyperWallet: hyperWalletPDA,
            owner: ownerKeypair.publicKey
        }).signers([ownerKeypair]).rpc();
        console.log("Signature: ", signature);

        const hyperWalletAccount = await program.account.hyperWallet.fetchNullable(hyperWalletPDA);
        assert.isNotNull(hyperWalletAccount);
        assert.equal(hyperWalletAccount.owner.toString(), ownerKeypair.publicKey.toString());
    });
}
