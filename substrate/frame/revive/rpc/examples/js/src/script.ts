//! Run with bun run script.ts

import { readFileSync } from "fs";
import { Contract, ContractFactory, JsonRpcProvider, parseEther, TransactionResponse } from "ethers";
import { ContractTransaction } from "ethers";
import { Transaction } from "ethers";

const provider = new JsonRpcProvider("http://localhost:8545");
const signer = await provider.getSigner();
console.log(
  `Signer address: ${await signer.getAddress()}, Nonce: ${await signer
    .getNonce()}`,
);

function str_to_bytes(str: string): Uint8Array {
  return new TextEncoder().encode(str);
}

// deploy
async function deploy() {
  console.log(`Deploying Contract...`);

  const bytecode = readFileSync("../rpc_demo.polkavm");
  const contractFactory = new ContractFactory(
    [
      "constructor(bytes memory _data)",
    ],
    bytecode,
    signer,
  );

  const args = str_to_bytes("hello");
  console.log("Deploying contract with args:", args);
  const contract = await contractFactory.deploy(args);
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log(`Contract deployed: ${address}`);
  return address;
}

async function call(address: string) {
  console.log(`Calling Contract at ${address}...`);

  const abi = ["function call(bytes data)"];
  const contract = new Contract(address, abi, signer);

  const value = parseEther("42");
  const tx = await contract.call(str_to_bytes("world"), { value }) as TransactionResponse;
  console.log("Call transaction hash:", tx.hash);
  tx.wait();
}

try {
const address = await deploy();
await call(address);

// check contract balance
 console.log(`Address balance: ${await provider.getBalance(address)}`);
} catch(err) {
	console.error(err);
}
