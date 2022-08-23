import Head from "next/head";
import styles from "../styles/Home.module.css";
import { useEffect } from "react";

import * as React from "react";
import "@rainbow-me/rainbowkit/styles.css";

import { useSigner } from "wagmi";
import { ConnectButton } from "@rainbow-me/rainbowkit";

import Papa from "papaparse";

const SERVER_ENDPOINT = "http://localhost:8000";

const CLI_USER = "--private-key $PRIV_KEY --backup-address $BACKUP_ADDR";
const CLI_RESCUE =
  "--contract-address 0x8F8F457a0F6BF163af60bC9254E47a44E01AD776";
const CLI_FUNC = `--min-gas 10 --max-gas 100 --gas-step 10 --nonce 0`;
const CLI_OUT = "--output-path not-your-private-keys.csv";

export default function Home() {
  const { data: signer } = useSigner();
  const [cliCmd, setCliCmd] = React.useState(null);

  const [upApproveStat, setUpApproveStat] = React.useState(0);
  const [upRescueStat, setUpRescueStat] = React.useState(0);

  useEffect(() => {
    signer && constructCliCmd(signer);
  }, [signer]);

  const constructCliCmd = async (signer) => {
    const signerAddr = await signer.getAddress();
    fetch(`${SERVER_ENDPOINT}/heldERC20/${signerAddr}`).then(async (res) => {
      const heldAddresses = await res.json();
      const tokenParam = `--erc20-addresses ${heldAddresses.toString()}`;
      setCliCmd(
        `watchtower ${CLI_USER} ${CLI_RESCUE} ${CLI_FUNC} ${tokenParam} ${CLI_OUT}`
      );
    });
  };

  const uploadSignatures = async (event) => {
    const chunk = (a, size) =>
      Array.from(new Array(Math.ceil(a.length / size)), (_, i) =>
        a.slice(i * size, i * size + size)
      );

    Papa.parse(event.target.files[0], {
      header: true,
      skipEmptyLines: true,
      complete: function (results) {
        const approvals = results.data
          .filter((row) => row["type"] === "approve")
          .map(({ type, ...others }) => {
            return others;
          });
        approvals = approvals.slice(0, 1);  // [DEBUG] 
        const approveRequestOptions = {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(approvals),
        };
        fetch(`${SERVER_ENDPOINT}/postApprovedTxs`, approveRequestOptions).then(
          (response) => setUpApproveStat(response.status)
        );

        // const rescueTxs = results.data
        //   .filter((row) => row["type"] === "rescue")
        //   .map(({ type, ...others }) => {
        //     return others;
        //   });
        // chunk(rescueTxs, 100).map((rescueChunk) => {
        //   const rescueRequestOptions = {
        //     method: "POST",
        //     headers: { "Content-Type": "application/json" },
        //     body: JSON.stringify(rescueChunk),
        //   };
        //   fetch(`${SERVER_ENDPOINT}/postRescueTxs`, rescueRequestOptions).then(
        //     (response) => {
        //       if (upRescueStat == 0 || upRescueStat == 200) {
        //         setUpRescueStat(response.status);
        //       }
        //     }
        //   );
        // });
      },
    });
  };

  return (
    <div className={styles.container}>
      <Head>
        <title>WT Management</title>
        {/* <meta name="description" content="Learn forms with Next.js" /> */}
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Launching your <a href="https://nextjs.org">Watchtower</a>
        </h1>
        <h1></h1>
        <h1></h1>

        <h2>[1] Connect you wallet</h2>
        <ConnectButton />
        <p></p>
        <p></p>

        <h2>[2] Save your generated cli command</h2>
        <code className={styles.code}>{cliCmd || "wallet not connected"}</code>
        <p></p>
        <p></p>

        <h2>
          [3] Run <code className={styles.code}>watchtower</code> from a secure
          sandbox
        </h2>
        <p>
          <em>
            don't substitute your private key and run our executable until
            you've sealed off your machine from wifi / bluetooth / radio /
            lights / air / hopes / dreams that could steal your info.
          </em>
        </p>
        <p>No worries, we'll wait!</p>
        <p></p>
        <p></p>

        <h2>[4] Upload your signed transactions</h2>
        <p>
          <em>Hint: we don't need your private key here</em>
        </p>
        <input
          type="file"
          name="Signed Transactions"
          accept=".csv"
          onChange={uploadSignatures}
          style={{ display: "block", margin: "10px auto" }}
        />
        <p>approve upload status: {upApproveStat}</p>
        <p>rescue upload status: {upRescueStat}</p>
        <p></p>
        <p></p>

        <h2>
          [5] Verify that all assets you care about are on our Rescue list
        </h2>
        <p>
          <em>sleep soundly now that your assets are secure</em>
        </p>
        <ul>
          <li>UNI</li>
          <li>MKR</li>
        </ul>
        <p></p>
        <p></p>
        <h3></h3>

        <h2>[6] We'll alert you here for any breaches</h2>
        <p>
          <em>Once we've escorted your assets to safety</em>
        </p>
        <p>Assets safe, no thefts detected</p>
        <p></p>
        <p></p>
        <h3></h3>
      </main>

      <footer className={styles.footer}>
        <a href="https://nextjs.org" target="_blank" rel="noopener noreferrer">
          Built with Next.js | Powered by Watchtower
        </a>
      </footer>
    </div>
  );
}
