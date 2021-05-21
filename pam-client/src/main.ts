import { configs, connection, initAccounts } from "./connect";
import { getContract } from "./contract";
async function main() {
  await initAccounts();
  const contract = getContract(connection, configs.progId, configs.userAccount);
  const tx = contract.initSwapTx(configs.data_account.publicKey);
  console.log(tx)
  const ret = await contract.sendTxs([tx]);
  console.log(ret)
}

main()
  .then(() => console.log("DONE"))
  .catch(console.error);
