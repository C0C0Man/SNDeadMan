import { SecretNetworkClient, Wallet } from 'secretjs';
import { readFileSync } from 'fs';
import { bech32 } from 'bech32';
import dotenv from 'dotenv';
dotenv.config();

const wallet = new Wallet("crouch cruel frame dizzy actual saddle harvest patch giant enable piece hunt"); 
const contract_wasm = readFileSync('../contract.wasm');

// Replace with your actual values after deployment
let codeId = 8490;
let contractCodeHash = "5e32ad58eaee785d8d28852a57687c1ec3ad042590500be3bb508664c64c9a95";
let contractAddress = "secret145tyf4t6jaqpjpshsa2glylgm54tvmygldzmj9";

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
    url: "https://api.pulsar3.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
  });

  let uploadContract = async () => {
    try {
    let tx = await secretjs.tx.compute.storeCode(
      {
        sender: wallet.address,
        wasm_byte_code: contract_wasm,
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


  let instantiateContract = async () => {
    let tx = await secretjs.tx.compute.instantiateContract(
      {
        code_id: codeId,
        sender: wallet.address,
        code_hash: contractCodeHash, 
        init_msg: {},
        label: "Secret Wallet" + Math.ceil(Math.random() * 10000),
      },
      {
        gasLimit: 400_000,
      }
    );
  
    //Find the contract_address in the logs
    const contractAddress = tx.arrayLog.find(
      (log) => log.type === "message" && log.key === "contract_address"
    ).value;
  
    console.log(contractAddress);
};

function encodeAddress(address) {
    const words = bech32.toWords(Buffer.from(address));
    return bech32.encode('secret', words);
  }

let initWallet = async (password) => {
    console.log("Initializing wallet for address:", wallet.address.toString());
   
  const tx = await secretjs.tx.compute.executeContract(
    {
        sender: wallet.address,
        contractAddress: contractAddress,
        msg: { 
            initWallet: { 
                address: wallet.address,
                password: password
            } 
        },         
        code_hash: contractCodeHash,
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
}
// Add other functions to interact with your contract
async function setPassword(oldPassword, newPassword) {
  try {
      const tx = await secretjs.tx.compute.executeContract(
          {
              sender: wallet.address,
              contractAddress: contractAddress,
              codeHash: contractCodeHash,
              msg: {
                  set_password: {
                      current_password: oldPassword ? oldPassword : null,
                      new_password: newPassword
                  }
              }
          },
          {
              gasLimit: 100_000
          }
      );
      console.log('Password updated:', tx);
  } catch (error) {
      console.error('Failed to update password:', error);
      throw error; // Rethrow the error to stop execution
  }
}

async function getBalance() {
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
}
async function main() {
//   await uploadContract();
//   await instantiateContract();

await initWallet("test"); 
//   await getBalance();
//   await setPassword("test", "test2");
//   await getBalance();
}

main().catch((err) => {
  console.error(err);
});
