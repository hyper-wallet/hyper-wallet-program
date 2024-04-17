import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HyperWalletProgram } from "../target/types/hyper_wallet_program";
import {assert} from "chai";
import {testCreateHyperWallet} from "./create_hyper_wallet";
import {testTransferLamports} from "./transfer_lamports";
import {testTransferSPL} from "./transfer_spl";

describe("hyper-wallet-program", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    testCreateHyperWallet();
    testTransferLamports();
    testTransferSPL();
});
