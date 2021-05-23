import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Signer,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

enum Instruction {
  init = 0,
  initAccessList = 1,
  addToAccessList = 2,
  removeToAccessList = 2,
}
const instrData = (instr: Instruction, data: number[] = []): Buffer => {
  return Buffer.from([instr, ...data]);
};

export const getContract = (
  connection: Connection,
  programId: PublicKey,
  signerAccount: Signer
) => {
  return {
    initProgDataTx: (progDataAccount: PublicKey) => {
      const keys = [
        { pubkey: progDataAccount, isSigner: true, isWritable: true },
      ];
      return new TransactionInstruction({
        keys,
        programId,
        data: instrData(Instruction.init),
      });
    },
    removeFromAccessList: (
      progDataAccount: PublicKey,
      accessListPk: PublicKey,
      remove: PublicKey
    ) => {
      return new TransactionInstruction({
        keys: [
          {
            pubkey: progDataAccount,
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: accessListPk,
            isSigner: true,
            isWritable: true,
          },
        ],
        programId,
        data: instrData(Instruction.removeToAccessList, [...remove.toBytes()]),
      });
    },
    addToAccessListTx: (
      progDataAccount: PublicKey,
      accessListPk: PublicKey,
      add: PublicKey
    ) => {
      return new TransactionInstruction({
        keys: [
          {
            pubkey: progDataAccount,
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: accessListPk,
            isSigner: true,
            isWritable: true,
          },
        ],
        programId,
        data: instrData(Instruction.addToAccessList, [...add.toBytes()]),
      });
    },
    initAccessListTx: (
      progDataAccount: PublicKey,
      newAccessListAccount: PublicKey
    ) => {
      return new TransactionInstruction({
        keys: [
          {
            pubkey: progDataAccount,
            isSigner: false,
            isWritable: true,
          },
          {
            pubkey: newAccessListAccount,
            isSigner: true,
            isWritable: true,
          },
        ],
        programId,
        data: instrData(Instruction.initAccessList),
      });
    },
    // TODO: change to init access list account
    // add functionality to add Pubkey user
    // add functionality to remove Pubkey user
    createAccessListAccountTx: (pks: PublicKey[]) => {
      const pkBytes = pks.map((pk) => [...pk.toBytes()]);
      const data = new Uint8Array(pkBytes.flat());
      const newAccount = Keypair.generate();
      const tx = SystemProgram.createAccount({
        fromPubkey: signerAccount.publicKey,
        newAccountPubkey: newAccount.publicKey,
        lamports: 10000000,
        space: 1024,
        programId,
      });
      return { newAccount, tx };
    },
    sendTxs: async (
      instructions: TransactionInstruction[],
      signers: Signer[] = []
    ) => {
      try {
        const tx = new Transaction();
        instructions.forEach((inst) => {
          tx.add(inst);
        });
        const txRet = await sendAndConfirmTransaction(
          connection,
          tx,
          [signerAccount, ...signers],
          {
            skipPreflight: true,
            commitment: "singleGossip",
          }
        );
        return txRet;
      } catch (e) {
        console.error("Error with send general", e);
        throw e;
      }
    },
  };
};
