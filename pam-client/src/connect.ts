import fs from "fs";
import {
  PublicKey,
  Connection,
  SystemProgram,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
  Keypair,
} from "@solana/web3.js";
//@ts-ignore
import bs58 from "bs58";
const progKey = "8wmMgLo9xBGUKai7eWxF2ziVNVFagGX9bWDgntbx4ifL";
const progId = new PublicKey(progKey);
// import { serializePubkey, trimBuffer } from "../src/utils/utils";

const SolanaNet = "https://devnet.solana.com";

const userAccount = Keypair.generate();
const data_account = Keypair.generate();
export let connection: Connection;
export const configs = {
  data_account,
  userAccount,
  progId,
};

export async function initAccount(account: Keypair) {
  const tx = SystemProgram.createAccount({
    fromPubkey: userAccount.publicKey,
    newAccountPubkey: account.publicKey,
    lamports: 10000000,
    space: 1024 * 100,
    programId: progId,
  });
  try {
    await sendAndConfirmTransaction(
      connection,
      new Transaction().add(tx),
      [userAccount, account],
      {
        skipPreflight: true,
        commitment: "singleGossip",
      }
    );
  } catch (e) {
    console.error(e);
    throw "Failed to init account";
  }
}

async function initDataAccount() {
  await initAccount(data_account);
  console.log("Data account init");
}

export async function initAccounts(): Promise<Connection> {
  connection = new Connection(SolanaNet, "singleGossip");
  const lamports = 10 * 1000000000;
  console.log("new data account:", data_account.publicKey.toBase58());
  await connection.requestAirdrop(userAccount.publicKey, lamports);
  console.log("airdrop done");
  initDataAccount();
  return connection;
}
