import { SecretNetworkClient, Wallet } from "secretjs";
import { readFileSync } from "fs";
import { bech32 } from "bech32";
import dotenv from 'dotenv';
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC); 
const contractWasm = readFileSync('../contract.wasm');
//Replace with your actual values 
let codeId =  8492; 
let contractCodeHash = "5b7be48ab557e2476f004a20ab31acef6fb93bac0dbafc43f32bf3f692a3cf52";
let contractAddress = "secret1getmue73adp03szndykhzprr2pjuvqekd3nm09"; 


const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
    url: "https://api.pulsar3.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
  });

  let upload_contract = async () => {
    try {
    let tx = await secretjs.tx.compute.storeCode(
      {
        sender: wallet.address,
        wasm_byte_code: contractWasm,
        source: "",
        builder: "",
      },
      {
        gasLimit: 4_000_000,
      }
    );
  
    const codeId = Number(
      tx.arrayLog.find((log) => log.type === "message" && log.key === "code_id")
        .value
    );
  
    console.log("codeId: ", codeId);
  
    const contractCodeHash = (
      await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
    ).code_hash;
    console.log(`Contract hash: ${contractCodeHash}`);
    } catch (error) {
    console.error("Error uploading contract:", error);
    if (tx) console.log("Transaction logs:", tx.arrayLog); // For debugging
  }

    
  };

async function instantiateContract() {
  try {
    const initMsg = { }; 
    let tx = await secretjs.tx.compute.instantiateContract(
      {
        code_id: codeId,
        sender: wallet.address,
        code_hash: contractCodeHash, 
        init_msg: initMsg,
        label: "Secret Wallet" + Math.ceil(Math.random() * 10000),
      },
      {
        gasLimit: 400_000,
      }
    );
  
    // Check if the transaction was successful
    if (tx.code !== 0) {
      throw new Error(`Failed to instantiate contract: ${tx.rawLog}`);
    }

    //Find the contract_address in the logs
    const contractAddress = tx.arrayLog.find(
        (log) => log.type === "message" && log.key === "contract_address"
      ).value;
    
      console.log(contractAddress);
  } catch (error) {
    console.error("Error instantiating contract:", error);
  }
}

// Init Wallet Function
async function initWallet() {
  try{ 
    const tx = await secretjs.tx.compute.executeContract(
      {
          sender: wallet.address,
          contract_Address: contractAddress,
          code_hash: contractCodeHash, 
          msg: { 
              init_wallet: { 
                  address: wallet.address,
              } 
          },      
          sentFunds: []
      },
      {
          gasLimit: 100_000
      }
    );

    if (tx.code !== 0) {
      console.error("Transaction failed with code:", tx.code);
      console.log("Raw log:", tx.rawLog); // Log the raw transaction log for more details
      throw new Error("InitWallet transaction failed");
    }
  
    console.log('Wallet initialized:', tx);

  } catch(err){
    console.error("Error initializing wallet: ", err);
  }
}

async function getBalance() {
  try{
    const response = await secretjs.query.compute.queryContract({
        contractAddress: contractAddress,
        codeHash: contractCodeHash,
        query: {
            get_balance: {
                address: wallet.address
            }
        }
    })
    console.log("Balance is: ", response.balance);
  } catch(err){
    console.error("Error getting the balance: ", err);
  }

}

async function main() {
//   await upload_contract();
//   await instantiateContract();

  //Interact with your contract
  await initWallet(); 
//   await getBalance();

}

main().catch((err) => {
  console.error(err);
});

