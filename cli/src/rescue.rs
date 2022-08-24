pub use rescue::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod rescue {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Rescue was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static RESCUE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"erc20Contract\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"AssetTransfer\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"erc20Contract\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"AssetTransferFailure\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"erc20Addresses\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"backupAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"rescueAssets\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    pub struct Rescue<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Rescue<M> {
        fn clone(&self) -> Self {
            Rescue(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Rescue<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Rescue<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Rescue))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Rescue<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), RESCUE_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `rescueAssets` (0x45b0502a) function"]
        pub fn rescue_assets(
            &self,
            erc_20_addresses: ::std::vec::Vec<ethers::core::types::Address>,
            backup_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 176, 80, 42], (erc_20_addresses, backup_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AssetTransfer` event"]
        pub fn asset_transfer_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AssetTransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AssetTransferFailure` event"]
        pub fn asset_transfer_failure_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AssetTransferFailureFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, RescueEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Rescue<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "AssetTransfer", abi = "AssetTransfer(address,address,uint256)")]
    pub struct AssetTransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        pub erc_20_contract: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "AssetTransferFailure",
        abi = "AssetTransferFailure(address,address,uint256)"
    )]
    pub struct AssetTransferFailureFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        pub erc_20_contract: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum RescueEvents {
        AssetTransferFilter(AssetTransferFilter),
        AssetTransferFailureFilter(AssetTransferFailureFilter),
    }
    impl ethers::contract::EthLogDecode for RescueEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AssetTransferFilter::decode_log(log) {
                return Ok(RescueEvents::AssetTransferFilter(decoded));
            }
            if let Ok(decoded) = AssetTransferFailureFilter::decode_log(log) {
                return Ok(RescueEvents::AssetTransferFailureFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for RescueEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                RescueEvents::AssetTransferFilter(element) => element.fmt(f),
                RescueEvents::AssetTransferFailureFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `rescueAssets` function with signature `rescueAssets(address[],address)` and selector `[69, 176, 80, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rescueAssets", abi = "rescueAssets(address[],address)")]
    pub struct RescueAssetsCall {
        pub erc_20_addresses: ::std::vec::Vec<ethers::core::types::Address>,
        pub backup_address: ethers::core::types::Address,
    }
}
