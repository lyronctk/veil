import Head from "next/head";
import styles from "../styles/Home.module.css";
import { useEffect } from "react";
import { ethers } from "ethers";

import * as React from "react";
import "@rainbow-me/rainbowkit/styles.css";

import { useSigner } from "wagmi";
import { ConnectButton, useConnectModal } from "@rainbow-me/rainbowkit";

import Papa, { ParseResult } from "papaparse";

const SERVER_ENDPOINT = "http://localhost:8000";

const CLI_USER = "--private-key $PRIV_KEY --backup-address $BACKUP_ADDR";
const CLI_RESCUE =
  "--contract-address 0x8F8F457a0F6BF163af60bC9254E47a44E01AD776";
const CLI_OUT = "--output-path not-your-private-keys.csv";

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
          `watchtower ${CLI_USER} ${CLI_RESCUE} ${quantParam} ${tokenParam} ${CLI_OUT}`
        );
      })
      .catch((e) => console.error(e));
  };

  // Send signed approval / rescue transactions to backend to be stored
  const uploadSignatures = async (
    event: any
  ) => {
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

  var url =
    "wss://eth-goerli.g.alchemy.com/v2/zg3KpYV3WSXDhGlkzW3KdupZfG40nW2e";
  var customWsProvider = new ethers.providers.WebSocketProvider(url);
  customWsProvider.on("pending", (tx) => {
    setLatestTxHash(tx);
  });

  return (
    <div>
      <Head>
        <title>Watchtower ⚡</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div style={{ height: "20px", backgroundColor: "yellow" }}></div>
      <main className="container max-w-2xl ml-10 matter-regular">
        <br></br>
        <div>
          <div className="matter-heavy text-3xl">
            Watchtower
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
          <div>Your ultimate defense against private key theft.</div>
          <br></br>
          <div>
            Watchtower protects assets by frontrunning unauthorized transactions
            and transferring all your assets to a secure, pre-specificied backup
            address. It can be set up in seconds, without revealing your private
            key at any point in the process.
          </div>
          <br></br>
          <div className="matter-heavy text-lg">Scanning the mempool...</div>
          {latestTxHash}
          <br></br>
          <br></br>
          <div className="matter-heavy text-lg">How do I get started?</div>
          <div>
            1) Connect your wallet.
            <div className="my-3">
              <ConnectButton></ConnectButton>
            </div>
          </div>

          <div>
            2) Save your generated watchtower command.
            <br></br>
            <div className="my-3 bg-black text-white px-5 py-3">
              {cliCmd ||
                `watchtower --private-key $PRIV_KEY \
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
            3) Run watchtower from a secure sandbox and upload your signed
            transactions.
            <input
              type="file"
              name="Signed Transactions"
              accept=".csv"
              onChange={uploadSignatures}
              className="my-3"
            />
            <p>Approve Upload Status: {upApproveStat}</p>
            <p>Rescue Upload Status: {upRescueStat}</p>
          </div>
          <br></br>

          <div>4) Sleep soundly now that your assets are protected :)</div>
        </div>
      </main>
    </div>
  );
}
