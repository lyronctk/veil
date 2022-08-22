import "../styles/globals.css";
import type { AppProps } from "next/app";

import { chain, configureChains, createClient, WagmiConfig } from "wagmi";
import { getDefaultWallets, RainbowKitProvider } from "@rainbow-me/rainbowkit";
import { publicProvider } from "wagmi/providers/public";
import { QueryClient, QueryClientProvider, useQuery } from "react-query";

// Connect to Ethereum via wagmi
const { chains, provider } = configureChains(
  [chain.goerli],
  [publicProvider()]
);
const { connectors } = getDefaultWallets({ appName: "Watchtower", chains });
const wagmiClient = createClient({ autoConnect: true, connectors, provider });
const queryClient = new QueryClient();

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <QueryClientProvider client={queryClient}>
      <WagmiConfig client={wagmiClient}>
        <RainbowKitProvider chains={chains}>
          <Component {...pageProps} />
        </RainbowKitProvider>
      </WagmiConfig>
    </QueryClientProvider>
  );
}

export default MyApp;
