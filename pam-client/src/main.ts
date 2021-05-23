import { Keypair } from "@solana/web3.js";
import { configs, connection, initAccount, initAccounts } from "./connect";
import { getContract } from "./contract";
async function main() {
  await initAccounts();
  const contract = getContract(connection, configs.progId, configs.userAccount);
  const txInit = contract.initProgDataTx(configs.data_account.publicKey);
  const accessList = Keypair.generate();
  await initAccount(accessList);
  const initAccessListTx = contract.initAccessListTx(
    configs.data_account.publicKey,
    accessList.publicKey
  );
  const ret = await contract.sendTxs([txInit], [configs.data_account]);
  console.log(ret);
}

main()
  .then(() => console.log("DONE"))
  .catch(console.error);
