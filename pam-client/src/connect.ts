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
const progKey = "Bn1E3nNvUbPtA9hTVj3GrnLyQMEAvdX1Ld9yNUVDNtub";
const progId = new PublicKey(progKey);
// import { serializePubkey, trimBuffer } from "../src/utils/utils";

const SolanaNet = "https://devnet.solana.com";

const userAccount = Keypair.generate();
const data_account = Keypair.generate();
export let connection: Connection;
export const configs = {
  data_account,
  userAccount,
  progId
}

async function initDataAccount() {
  const createAccountTransaction = SystemProgram.createAccount({
    fromPubkey: userAccount.publicKey,
    newAccountPubkey: data_account.publicKey,
    lamports: 1000000000,
    space: 1024 * 1024, // this is like bytes, wholly poop, so this is 1 meg
    programId: progId,
  });
  try {
    await sendAndConfirmTransaction(
      connection,
      new Transaction().add(createAccountTransaction),
      [userAccount, data_account],
      {
        skipPreflight: true,
        commitment: "singleGossip",
      }
    );
    console.log("Data account init");
  } catch (e) {
    console.error(e);
    throw "Failed to initDataAccount";
  }
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
