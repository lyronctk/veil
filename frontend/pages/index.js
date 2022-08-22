import Head from 'next/head'
import Image from 'next/image'
import Link from 'next/link'
import styles from '../styles/Home.module.css'

import Papa from 'papaparse'

export default function Home() {

  const uploadSignedTx = async (event) => {
    Papa.parse(event.target.files[0], {
      header: true,
      skipEmptyLines: true,
      complete: function (results) {
        const requestOptions = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ approvals: results.data })
        };
        console.log(requestOptions);
        fetch('http://localhost:8000/upload_signed_tx', requestOptions)
            .then(response => response.json());
        console.log("SENT FETCH");
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

        <p className={styles.description}>
          <code className={styles.code}>Managing your Watchtower</code> 
        </p>

        <h3>Upload Signed Transactions</h3>
        <input
          type="file"
          name="Signed Transacstions "
          accept=".csv"
          onChange={uploadSignedTx}
          style={{ display: "block", margin: "10px auto" }}
        />
        <h3></h3>

        <h3>Alerts</h3>
        <p>Assets safe, no thefts detected</p>
        <h3></h3>

        <h3>Covered Assets</h3>
        <ul>
          <li>UNI</li>
        </ul>
        <h3></h3>
      </main>

      <footer className={styles.footer}>
        <a href="https://nextjs.org" target="_blank" rel="noopener noreferrer">
          Built with Next.js | Powered by Watchtower
        </a>
      </footer>
    </div>
  )
}