import * as anchor from "@coral-xyz/anchor";
import { testCreateHyperWallet } from "./create_hyper_wallet";
import { testTransferLamports } from "./transfer_lamports";
import { testTransferSPL } from "./transfer_spl";
import { testWhiteList } from "./whitelist";
import { testOtp } from "./otp";

describe("hyper-wallet-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  testCreateHyperWallet();
  testTransferLamports();
  testTransferSPL();
  testWhiteList();
  testOtp();
});
