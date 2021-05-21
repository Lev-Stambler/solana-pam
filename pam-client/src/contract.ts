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
  updateAccessList = 1,
}
const instrData = (instr: Instruction): Buffer => {
  return Buffer.from([instr]);
};

export const getContract = (
  connection: Connection,
  programId: PublicKey,
  signerAccount: Signer
) => {
  return {
    initSwapTx: (progDataAccount: PublicKey) => {
      const keys = [
        { pubkey: progDataAccount, isSigner: false, isWritable: true },
      ];
      return new TransactionInstruction({
        keys,
        programId,
        data: instrData(Instruction.init),
      });
    },
    updateAccessListTx: (
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
        data: instrData(Instruction.updateAccessList),
      });
    },
    // TODO: change to init access list account
    // add functionality to add Pubkey user
    // add functionality to remove Pubkey user
    createAccessListAccount: (pks: PublicKey[], access_list_pk?: PublicKey) => {
      const pkBytes = pks.map((pk) => [...pk.toBytes()]);
      const data = new Uint8Array(pkBytes.flat());
      if (access_list_pk) {
      } else {
        const newAccount = Keypair.generate();
        SystemProgram.createAccount({
          fromPubkey: signerAccount.publicKey,
          newAccountPubkey: newAccount.publicKey,
          lamports: 10000000,
          space: 1024,
          programId,
        });
        SystemProgram.
      }
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
