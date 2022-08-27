import Head from "next/head";
import styles from "../styles/Home.module.css";
import { useEffect } from "react";
import { ethers } from "ethers";

import * as React from "react";
import "@rainbow-me/rainbowkit/styles.css";

import { useSigner } from "wagmi";
import { ConnectButton, useConnectModal } from "@rainbow-me/rainbowkit";

import Papa, { ParseResult } from "papaparse";

const SERVER_ENDPOINT = "https://ethwatchtower.ngrok.io";

const CHAIN_ID = 5;

const CONTRACT_ADDR =
  CHAIN_ID == 5
    ? "0x8F8F457a0F6BF163af60bC9254E47a44E01AD776"
    : "0xa576edc195c48440fa98840fb3721bf17c1656f6";
const CLI_USER = "--private-key $PRIV_KEY --backup-address $BACKUP_ADDR";
const CLI_RESCUE = `--contract-address ${CONTRACT_ADDR}`;
const CLI_CHAIN = `--chain-id ${CHAIN_ID}`;
const CLI_OUT = "--output-path not-your-private-keys.csv";

const DUMMY_TX_HASHES: string[] = [
  "0x8fcf61d7037ce1fa42e5986eac2f18c0cc65a9e3c1ec899f55467465c8daf0e1",
  "0xcfbfddf83cdffa8566603be2617fc8a351809fc0eeb163946aa612f7e22ef769",
  "0xa54c436c28ac7523dc495dd738c3ec756429b32713b1a72b3ef6ffde6205e891",
  "0x3b6e0ebd4f460b8ebcf7f4040b299dd65e1662db2b4a2d4f5e130e1de198e63d",
  "0xf36979f3f3e3c4858872440e66fdce0439d7cafa7f9a711e4b2da2193170f1c3",
  "0x3a49d1bdc11711fadc33d8474c61d5f4c7e739ac0605094f27438be8b58decc7",
  "0x82dddff278146498c08ad3b8172c481e516fdc006b049a02713be7b85b5f56c7",
  "0x27495a975ad5a9a6296694871cb4f08ea689e6f2a097407eec31b0d479216335",
  "0xf4e88725a872759e392de58f6c2af0673c75014fec3d8ab48534c8b655e3eb5c",
  "0xdf0f8682b99f90d0dcfc558ab67902c7d484b1c9dc254ac8d06161c3de940dee",
];

export default function Home() {
  const { data: signer } = useSigner();
  const [cliCmd, setCliCmd] = React.useState("");

  const [upApproveStat, setUpApproveStat] = React.useState(0);
  const [upRescueStat, setUpRescueStat] = React.useState(0);
  const { openConnectModal } = useConnectModal();

  // Update the displayed CLI command when wallet is connected
  useEffect(() => {
    signer && constructCliCmd(signer);
  }, [signer]);
  const constructCliCmd = async (signer: ethers.Signer) => {
    const txCt = await signer.getTransactionCount();
    const signerAddr = await signer.getAddress();
    fetch(`${SERVER_ENDPOINT}/heldERC20/${signerAddr}`)
      .then(async (res) => {
        const quantParam = `--min-gas 10 --max-gas 100 --gas-step 10 --nonce ${txCt}`;
        const heldAddresses = await res.json();
        const strAddresses = heldAddresses.join(" ");
        const tokenParam = `--erc20-addresses ${strAddresses}`;
        setCliCmd(
          `veil ${CLI_USER} ${CLI_RESCUE} ${quantParam} ${tokenParam} ${CLI_CHAIN} ${CLI_OUT}`
        );
      })
      .catch((e) => console.error(e));
  };

  // Send signed approval / rescue transactions to backend to be stored
  const uploadSignatures = async (event: any) => {
    const chunk = (a: any[], size: number) =>
      Array.from(new Array(Math.ceil(a.length / size)), (_, i) =>
        a.slice(i * size, i * size + size)
      );

    Papa.parse(event.target.files[0], {
      header: true,
      skipEmptyLines: true,
      complete: async (
        results: ParseResult<{ [property: string]: string | number }>
      ) => {
        // Send approval transactions
        const approvals = results.data.filter(
          (row) => row["type"] === "approve"
        );
        const approveRequestOptions = {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ approveData: approvals }),
        };
        fetch(`${SERVER_ENDPOINT}/postApproveTxs`, approveRequestOptions)
          .then((response) => setUpApproveStat(response.status))
          .catch((e) => console.error(e));

        // Send rescue transactions
        const rescueTxs = results.data
          .filter((row) => row["type"] === "rescue")
          .map((row) => {
            row["gasPrice"] = parseInt(String(row["gasPrice"]));
            return row;
          });
        chunk(rescueTxs, 100).map((rescueChunk) => {
          const rescueRequestOptions = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ signedRescueTxs: rescueChunk }),
          };
          fetch(`${SERVER_ENDPOINT}/postRescueTxs`, rescueRequestOptions)
            .then((response) => {
              if (upRescueStat == 0 || upRescueStat == 200) {
                setUpRescueStat(response.status);
              }
            })
            .catch((e) => console.error(e));
        });
      },
    });
  };

  const [latestTxHash, setLatestTxHash] = React.useState("");
  const [mempool, setMempool] = React.useState([]);

  useEffect(() => {
    const id = setInterval(() => {
      setLatestTxHash(
        DUMMY_TX_HASHES[Math.floor(Math.random() * DUMMY_TX_HASHES.length)]
      );
    }, 100);
    return () => clearInterval(id);
  }, [latestTxHash]);

  return (
    <div>
      <Head>
        <title>Veil ⚡</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <div className="flex px-10 py-10">
        <div className="flex-none">
          <div className="matter-heavy text-3xl">
            Veil
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="currentColor"
              className="w-9 h-9 inline pb-2"
            >
              <path
                fillRule="evenodd"
                d="M14.615 1.595a.75.75 0 01.359.852L12.982 9.75h7.268a.75.75 0 01.548 1.262l-10.5 11.25a.75.75 0 01-1.272-.71l1.992-7.302H3.75a.75.75 0 01-.548-1.262l10.5-11.25a.75.75 0 01.913-.143z"
                clipRule="evenodd"
              />
            </svg>
          </div>
        </div>
        <div className="grow">
          <div className="flex justify-center item-center"></div>
        </div>
        <div className="flex-none">
          <ConnectButton></ConnectButton>
        </div>
      </div>

      <div className="flex justify-center item-center">
        <main className="container max-w-2xl ml-10 matter-regular">
          <div>
            <div className="matter-heavy text-2xl gradient-font">
              Your ultimate defense against private key theft.
            </div>
            <br></br>
            <div>
              Veil protects assets by frontrunning unauthorized transactions and
              transferring all your assets to a secure, pre-specificied backup
              address. It can be set up in seconds, without revealing your
              private key at any point in the process.
            </div>
            <br></br>
            <div className="matter-heavy text-lg">Scanning the mempool...</div>
            {latestTxHash}
            <br></br>
            <br></br>
            <div className="matter-heavy text-lg">Get Started</div>

            <div>
              1) Save your generated watchtower command.
              <br></br>
              <div className="my-3 bg-black text-white px-5 py-3">
                {cliCmd ||
                  `veil --private-key $PRIV_KEY \
              --backup-address $BACKUP_ADDR \
              --contract-address 0x8F8F457a0F6BF163af60bC9254E47a44E01AD776 \
              --min-gas 10 \
              --max-gas 100 \
              --gas-step 10 \
              --nonce 8 \
              --erc20-addresses 0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984 0xdc31Ee1784292379Fbb2964b3B9C4124D8F89C60 \
              --output-path not-your-private-keys.csv`}
              </div>
            </div>
            <div>
              2) Run watchtower from a secure sandbox and upload your signed
              transactions.
              <input
                type="file"
                name="Signed Transactions"
                accept=".csv"
                onChange={uploadSignatures}
                className="my-3"
              />
              <p>
                Approve Upload Status:{" "}
                {upApproveStat == 0 ? (
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="w-5 h-5 inline"
                  >
                    <path
                      fillRule="evenodd"
                      d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zm-1.72 6.97a.75.75 0 10-1.06 1.06L10.94 12l-1.72 1.72a.75.75 0 101.06 1.06L12 13.06l1.72 1.72a.75.75 0 101.06-1.06L13.06 12l1.72-1.72a.75.75 0 10-1.06-1.06L12 10.94l-1.72-1.72z"
                      clipRule="evenodd"
                    />
                  </svg>
                ) : (
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="w-5 h-5 inline"
                  >
                    <path
                      fillRule="evenodd"
                      d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm13.36-1.814a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z"
                      clipRule="evenodd"
                    />
                  </svg>
                )}
              </p>
              <p>
                Rescue Upload Status:{" "}
                {upRescueStat == 0 ? (
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="w-5 h-5 inline"
                  >
                    <path
                      fillRule="evenodd"
                      d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zm-1.72 6.97a.75.75 0 10-1.06 1.06L10.94 12l-1.72 1.72a.75.75 0 101.06 1.06L12 13.06l1.72 1.72a.75.75 0 101.06-1.06L13.06 12l1.72-1.72a.75.75 0 10-1.06-1.06L12 10.94l-1.72-1.72z"
                      clipRule="evenodd"
                    />
                  </svg>
                ) : (
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="w-5 h-5 inline"
                  >
                    <path
                      fillRule="evenodd"
                      d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm13.36-1.814a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z"
                      clipRule="evenodd"
                    />
                  </svg>
                )}
              </p>
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}
