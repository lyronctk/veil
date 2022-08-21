pub use watchtower::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod watchtower {
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
    #[doc = "Watchtower was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static WATCHTOWER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_marketingWallet\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_liquidityWallet\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"whitelistAddress\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Approval\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"enabled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"BlacklistEnabled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"enabled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"CompoundingEnabled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"isExcluded\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"ExcludeFromFees\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"tokensSwapped\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"SendDividends\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"pair\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bool\",\n        \"name\": \"value\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"SetAutomatedMarketMakerPair\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"tokensSwapped\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"nativeReceived\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"tokensIntoLiquidity\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"SwapAndAddLiquidity\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"enabled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"SwapEnabled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"enabled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"TaxEnabled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Transfer\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"oldAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"UpdateUniswapV2Router\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"accumulativeDividendOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"approve\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"automatedMarketMakerPairs\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"balanceOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"blackList\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"_users\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"blackListMany\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"claim\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"compound\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"compoundingEnabled\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"decimals\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"subtractedValue\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"decreaseAllowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"dividendFeeBPS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"dividendTracker\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract DividendTracker\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"excluded\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"excludeFromDividends\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"excluded\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"excludeFromFees\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"excluded\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"excludeFromMaxTx\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"excluded\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"excludeFromMaxWallet\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getAccountInfo\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getLastClaimTime\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"addedValue\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"increaseAllowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isExcludedFromDividends\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isExcludedFromFees\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isExcludedFromMaxTx\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isExcludedFromMaxWallet\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"lastSwapTime\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"liquidityFeeBPS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"holder\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"manualSendDividend\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"maxTxBPS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"maxWalletBPS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"name\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"openTrading\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"removeFromBlacklist\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"renounceOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"rescueETH\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"rescueToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pair\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"value\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setAutomatedMarketMakerPair\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"_enabled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setCompoundingEnabled\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_treasuryFee\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_liquidityFee\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_dividendFee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setFee\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"bps\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setMaxTxBPS\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"bps\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setMaxWalletBPS\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"_enabled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setSwapEnabled\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"_enabled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setTaxEnabled\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"_marketingWallet\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"_liquidityWallet\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setWallet\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"swapEnabled\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"swapTokensAtAmount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"symbol\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"taxEnabled\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalFeeBPS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalSupply\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transfer\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transferFrom\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"treasuryFeeBPS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"_users\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"unBlackListMany\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"uniswapV2Pair\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"uniswapV2Router\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IUniswapV2Router02\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"_swapEnabled\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_swapTokensAtAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"_swapAllToken\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"updateDividendSettings\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"updateUniswapV2Router\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"withdrawableDividendOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"withdrawnDividendOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"stateMutability\": \"payable\",\n    \"type\": \"receive\"\n  }\n]") . expect ("invalid abi")
        });
    pub struct Watchtower<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Watchtower<M> {
        fn clone(&self) -> Self {
            Watchtower(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Watchtower<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Watchtower<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Watchtower))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Watchtower<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), WATCHTOWER_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `accumulativeDividendOf` (0x27ce0147) function"]
        pub fn accumulative_dividend_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([39, 206, 1, 71], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `automatedMarketMakerPairs` (0xb62496f5) function"]
        pub fn automated_market_maker_pairs(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([182, 36, 150, 245], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blackList` (0x4838d165) function"]
        pub fn black_list(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 56, 209, 101], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blackListMany` (0xc3033aeb) function"]
        pub fn black_list_many(
            &self,
            users: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 3, 58, 235], users)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claim` (0x4e71d92d) function"]
        pub fn claim(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 113, 217, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compound` (0xf69e2046) function"]
        pub fn compound(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 158, 32, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compoundingEnabled` (0x2f4504ae) function"]
        pub fn compounding_enabled(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 69, 4, 174], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            subtracted_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dividendFeeBPS` (0xb80b6e89) function"]
        pub fn dividend_fee_bps(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([184, 11, 110, 137], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dividendTracker` (0x2c1f5216) function"]
        pub fn dividend_tracker(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([44, 31, 82, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `excludeFromDividends` (0x0483f7a0) function"]
        pub fn exclude_from_dividends(
            &self,
            account: ethers::core::types::Address,
            excluded: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 131, 247, 160], (account, excluded))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `excludeFromFees` (0xc0246668) function"]
        pub fn exclude_from_fees(
            &self,
            account: ethers::core::types::Address,
            excluded: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 36, 102, 104], (account, excluded))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `excludeFromMaxTx` (0xd4c989d3) function"]
        pub fn exclude_from_max_tx(
            &self,
            account: ethers::core::types::Address,
            excluded: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 201, 137, 211], (account, excluded))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `excludeFromMaxWallet` (0xd2fcc001) function"]
        pub fn exclude_from_max_wallet(
            &self,
            account: ethers::core::types::Address,
            excluded: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 252, 192, 1], (account, excluded))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAccountInfo` (0x7b510fe8) function"]
        pub fn get_account_info(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([123, 81, 15, 232], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastClaimTime` (0xa680e0bc) function"]
        pub fn get_last_claim_time(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([166, 128, 224, 188], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            added_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isExcludedFromDividends` (0xc705c569) function"]
        pub fn is_excluded_from_dividends(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([199, 5, 197, 105], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isExcludedFromFees` (0x4fbee193) function"]
        pub fn is_excluded_from_fees(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([79, 190, 225, 147], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isExcludedFromMaxTx` (0x658c27a9) function"]
        pub fn is_excluded_from_max_tx(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([101, 140, 39, 169], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isExcludedFromMaxWallet` (0x6dd3d39f) function"]
        pub fn is_excluded_from_max_wallet(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 211, 211, 159], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastSwapTime` (0x0dd87157) function"]
        pub fn last_swap_time(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([13, 216, 113, 87], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidityFeeBPS` (0xebbf1ace) function"]
        pub fn liquidity_fee_bps(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([235, 191, 26, 206], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `manualSendDividend` (0x8e126944) function"]
        pub fn manual_send_dividend(
            &self,
            amount: ethers::core::types::U256,
            holder: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 18, 105, 68], (amount, holder))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxTxBPS` (0x57777d31) function"]
        pub fn max_tx_bps(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([87, 119, 125, 49], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxWalletBPS` (0x744d1591) function"]
        pub fn max_wallet_bps(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([116, 77, 21, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `openTrading` (0xc9567bf9) function"]
        pub fn open_trading(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 86, 123, 249], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeFromBlacklist` (0x537df3b6) function"]
        pub fn remove_from_blacklist(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 125, 243, 182], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rescueETH` (0x9e252f00) function"]
        pub fn rescue_eth(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 37, 47, 0], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rescueToken` (0x33f3d628) function"]
        pub fn rescue_token(
            &self,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 243, 214, 40], (token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAutomatedMarketMakerPair` (0x9a7a23d6) function"]
        pub fn set_automated_market_maker_pair(
            &self,
            pair: ethers::core::types::Address,
            value: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 122, 35, 214], (pair, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setCompoundingEnabled` (0xe4956ce2) function"]
        pub fn set_compounding_enabled(
            &self,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 149, 108, 226], enabled)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFee` (0x5b65b9ab) function"]
        pub fn set_fee(
            &self,
            treasury_fee: ethers::core::types::U256,
            liquidity_fee: ethers::core::types::U256,
            dividend_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [91, 101, 185, 171],
                    (treasury_fee, liquidity_fee, dividend_fee),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMaxTxBPS` (0xaa4e8c4a) function"]
        pub fn set_max_tx_bps(
            &self,
            bps: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 78, 140, 74], bps)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMaxWalletBPS` (0x68c51e35) function"]
        pub fn set_max_wallet_bps(
            &self,
            bps: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 197, 30, 53], bps)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSwapEnabled` (0xe01af92c) function"]
        pub fn set_swap_enabled(
            &self,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 26, 249, 44], enabled)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTaxEnabled` (0xc6af580b) function"]
        pub fn set_tax_enabled(
            &self,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 175, 88, 11], enabled)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setWallet` (0xf1b234ad) function"]
        pub fn set_wallet(
            &self,
            marketing_wallet: ethers::core::types::Address,
            liquidity_wallet: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 178, 52, 173], (marketing_wallet, liquidity_wallet))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapEnabled` (0x6ddd1713) function"]
        pub fn swap_enabled(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 221, 23, 19], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapTokensAtAmount` (0xe2f45605) function"]
        pub fn swap_tokens_at_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([226, 244, 86, 5], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `taxEnabled` (0x870bd30b) function"]
        pub fn tax_enabled(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([135, 11, 211, 11], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalFeeBPS` (0x37eb1528) function"]
        pub fn total_fee_bps(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([55, 235, 21, 40], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            sender: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `treasuryFeeBPS` (0x5937ea6c) function"]
        pub fn treasury_fee_bps(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([89, 55, 234, 108], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unBlackListMany` (0xf4571c49) function"]
        pub fn un_black_list_many(
            &self,
            users: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 87, 28, 73], users)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `uniswapV2Pair` (0x49bd5a5e) function"]
        pub fn uniswap_v2_pair(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([73, 189, 90, 94], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `uniswapV2Router` (0x1694505e) function"]
        pub fn uniswap_v2_router(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([22, 148, 80, 94], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateDividendSettings` (0x5e843ad2) function"]
        pub fn update_dividend_settings(
            &self,
            swap_enabled: bool,
            swap_tokens_at_amount: ethers::core::types::U256,
            swap_all_token: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [94, 132, 58, 210],
                    (swap_enabled, swap_tokens_at_amount, swap_all_token),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateUniswapV2Router` (0x65b8dbc0) function"]
        pub fn update_uniswap_v2_router(
            &self,
            new_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 184, 219, 192], new_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawableDividendOf` (0xa8b9d240) function"]
        pub fn withdrawable_dividend_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([168, 185, 210, 64], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawnDividendOf` (0xaafd847a) function"]
        pub fn withdrawn_dividend_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([170, 253, 132, 122], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BlacklistEnabled` event"]
        pub fn blacklist_enabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BlacklistEnabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CompoundingEnabled` event"]
        pub fn compounding_enabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CompoundingEnabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExcludeFromFees` event"]
        pub fn exclude_from_fees_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExcludeFromFeesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SendDividends` event"]
        pub fn send_dividends_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SendDividendsFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetAutomatedMarketMakerPair` event"]
        pub fn set_automated_market_maker_pair_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetAutomatedMarketMakerPairFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SwapAndAddLiquidity` event"]
        pub fn swap_and_add_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SwapAndAddLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SwapEnabled` event"]
        pub fn swap_enabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SwapEnabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaxEnabled` event"]
        pub fn tax_enabled_filter(&self) -> ethers::contract::builders::Event<M, TaxEnabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateUniswapV2Router` event"]
        pub fn update_uniswap_v2_router_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateUniswapV2RouterFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, WatchtowerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Watchtower<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "BlacklistEnabled", abi = "BlacklistEnabled(bool)")]
    pub struct BlacklistEnabledFilter {
        pub enabled: bool,
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
    #[ethevent(name = "CompoundingEnabled", abi = "CompoundingEnabled(bool)")]
    pub struct CompoundingEnabledFilter {
        pub enabled: bool,
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
    #[ethevent(name = "ExcludeFromFees", abi = "ExcludeFromFees(address,bool)")]
    pub struct ExcludeFromFeesFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub is_excluded: bool,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
    #[ethevent(name = "SendDividends", abi = "SendDividends(uint256,uint256)")]
    pub struct SendDividendsFilter {
        pub tokens_swapped: ethers::core::types::U256,
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
        name = "SetAutomatedMarketMakerPair",
        abi = "SetAutomatedMarketMakerPair(address,bool)"
    )]
    pub struct SetAutomatedMarketMakerPairFilter {
        #[ethevent(indexed)]
        pub pair: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub value: bool,
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
        name = "SwapAndAddLiquidity",
        abi = "SwapAndAddLiquidity(uint256,uint256,uint256)"
    )]
    pub struct SwapAndAddLiquidityFilter {
        pub tokens_swapped: ethers::core::types::U256,
        pub native_received: ethers::core::types::U256,
        pub tokens_into_liquidity: ethers::core::types::U256,
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
    #[ethevent(name = "SwapEnabled", abi = "SwapEnabled(bool)")]
    pub struct SwapEnabledFilter {
        pub enabled: bool,
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
    #[ethevent(name = "TaxEnabled", abi = "TaxEnabled(bool)")]
    pub struct TaxEnabledFilter {
        pub enabled: bool,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
        name = "UpdateUniswapV2Router",
        abi = "UpdateUniswapV2Router(address,address)"
    )]
    pub struct UpdateUniswapV2RouterFilter {
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum WatchtowerEvents {
        ApprovalFilter(ApprovalFilter),
        BlacklistEnabledFilter(BlacklistEnabledFilter),
        CompoundingEnabledFilter(CompoundingEnabledFilter),
        ExcludeFromFeesFilter(ExcludeFromFeesFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SendDividendsFilter(SendDividendsFilter),
        SetAutomatedMarketMakerPairFilter(SetAutomatedMarketMakerPairFilter),
        SwapAndAddLiquidityFilter(SwapAndAddLiquidityFilter),
        SwapEnabledFilter(SwapEnabledFilter),
        TaxEnabledFilter(TaxEnabledFilter),
        TransferFilter(TransferFilter),
        UpdateUniswapV2RouterFilter(UpdateUniswapV2RouterFilter),
    }
    impl ethers::contract::EthLogDecode for WatchtowerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(WatchtowerEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BlacklistEnabledFilter::decode_log(log) {
                return Ok(WatchtowerEvents::BlacklistEnabledFilter(decoded));
            }
            if let Ok(decoded) = CompoundingEnabledFilter::decode_log(log) {
                return Ok(WatchtowerEvents::CompoundingEnabledFilter(decoded));
            }
            if let Ok(decoded) = ExcludeFromFeesFilter::decode_log(log) {
                return Ok(WatchtowerEvents::ExcludeFromFeesFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(WatchtowerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = SendDividendsFilter::decode_log(log) {
                return Ok(WatchtowerEvents::SendDividendsFilter(decoded));
            }
            if let Ok(decoded) = SetAutomatedMarketMakerPairFilter::decode_log(log) {
                return Ok(WatchtowerEvents::SetAutomatedMarketMakerPairFilter(decoded));
            }
            if let Ok(decoded) = SwapAndAddLiquidityFilter::decode_log(log) {
                return Ok(WatchtowerEvents::SwapAndAddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = SwapEnabledFilter::decode_log(log) {
                return Ok(WatchtowerEvents::SwapEnabledFilter(decoded));
            }
            if let Ok(decoded) = TaxEnabledFilter::decode_log(log) {
                return Ok(WatchtowerEvents::TaxEnabledFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(WatchtowerEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UpdateUniswapV2RouterFilter::decode_log(log) {
                return Ok(WatchtowerEvents::UpdateUniswapV2RouterFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for WatchtowerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                WatchtowerEvents::ApprovalFilter(element) => element.fmt(f),
                WatchtowerEvents::BlacklistEnabledFilter(element) => element.fmt(f),
                WatchtowerEvents::CompoundingEnabledFilter(element) => element.fmt(f),
                WatchtowerEvents::ExcludeFromFeesFilter(element) => element.fmt(f),
                WatchtowerEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                WatchtowerEvents::SendDividendsFilter(element) => element.fmt(f),
                WatchtowerEvents::SetAutomatedMarketMakerPairFilter(element) => element.fmt(f),
                WatchtowerEvents::SwapAndAddLiquidityFilter(element) => element.fmt(f),
                WatchtowerEvents::SwapEnabledFilter(element) => element.fmt(f),
                WatchtowerEvents::TaxEnabledFilter(element) => element.fmt(f),
                WatchtowerEvents::TransferFilter(element) => element.fmt(f),
                WatchtowerEvents::UpdateUniswapV2RouterFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `accumulativeDividendOf` function with signature `accumulativeDividendOf(address)` and selector `[39, 206, 1, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "accumulativeDividendOf",
        abi = "accumulativeDividendOf(address)"
    )]
    pub struct AccumulativeDividendOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `automatedMarketMakerPairs` function with signature `automatedMarketMakerPairs(address)` and selector `[182, 36, 150, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "automatedMarketMakerPairs",
        abi = "automatedMarketMakerPairs(address)"
    )]
    pub struct AutomatedMarketMakerPairsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `blackList` function with signature `blackList(address)` and selector `[72, 56, 209, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "blackList", abi = "blackList(address)")]
    pub struct BlackListCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `blackListMany` function with signature `blackListMany(address[])` and selector `[195, 3, 58, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "blackListMany", abi = "blackListMany(address[])")]
    pub struct BlackListManyCall {
        pub users: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `claim` function with signature `claim()` and selector `[78, 113, 217, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claim", abi = "claim()")]
    pub struct ClaimCall;
    #[doc = "Container type for all input parameters for the `compound` function with signature `compound()` and selector `[246, 158, 32, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compound", abi = "compound()")]
    pub struct CompoundCall;
    #[doc = "Container type for all input parameters for the `compoundingEnabled` function with signature `compoundingEnabled()` and selector `[47, 69, 4, 174]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compoundingEnabled", abi = "compoundingEnabled()")]
    pub struct CompoundingEnabledCall;
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub subtracted_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `dividendFeeBPS` function with signature `dividendFeeBPS()` and selector `[184, 11, 110, 137]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dividendFeeBPS", abi = "dividendFeeBPS()")]
    pub struct DividendFeeBPSCall;
    #[doc = "Container type for all input parameters for the `dividendTracker` function with signature `dividendTracker()` and selector `[44, 31, 82, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dividendTracker", abi = "dividendTracker()")]
    pub struct DividendTrackerCall;
    #[doc = "Container type for all input parameters for the `excludeFromDividends` function with signature `excludeFromDividends(address,bool)` and selector `[4, 131, 247, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "excludeFromDividends",
        abi = "excludeFromDividends(address,bool)"
    )]
    pub struct ExcludeFromDividendsCall {
        pub account: ethers::core::types::Address,
        pub excluded: bool,
    }
    #[doc = "Container type for all input parameters for the `excludeFromFees` function with signature `excludeFromFees(address,bool)` and selector `[192, 36, 102, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "excludeFromFees", abi = "excludeFromFees(address,bool)")]
    pub struct ExcludeFromFeesCall {
        pub account: ethers::core::types::Address,
        pub excluded: bool,
    }
    #[doc = "Container type for all input parameters for the `excludeFromMaxTx` function with signature `excludeFromMaxTx(address,bool)` and selector `[212, 201, 137, 211]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "excludeFromMaxTx", abi = "excludeFromMaxTx(address,bool)")]
    pub struct ExcludeFromMaxTxCall {
        pub account: ethers::core::types::Address,
        pub excluded: bool,
    }
    #[doc = "Container type for all input parameters for the `excludeFromMaxWallet` function with signature `excludeFromMaxWallet(address,bool)` and selector `[210, 252, 192, 1]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "excludeFromMaxWallet",
        abi = "excludeFromMaxWallet(address,bool)"
    )]
    pub struct ExcludeFromMaxWalletCall {
        pub account: ethers::core::types::Address,
        pub excluded: bool,
    }
    #[doc = "Container type for all input parameters for the `getAccountInfo` function with signature `getAccountInfo(address)` and selector `[123, 81, 15, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAccountInfo", abi = "getAccountInfo(address)")]
    pub struct GetAccountInfoCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getLastClaimTime` function with signature `getLastClaimTime(address)` and selector `[166, 128, 224, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLastClaimTime", abi = "getLastClaimTime(address)")]
    pub struct GetLastClaimTimeCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub added_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isExcludedFromDividends` function with signature `isExcludedFromDividends(address)` and selector `[199, 5, 197, 105]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "isExcludedFromDividends",
        abi = "isExcludedFromDividends(address)"
    )]
    pub struct IsExcludedFromDividendsCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isExcludedFromFees` function with signature `isExcludedFromFees(address)` and selector `[79, 190, 225, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isExcludedFromFees", abi = "isExcludedFromFees(address)")]
    pub struct IsExcludedFromFeesCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isExcludedFromMaxTx` function with signature `isExcludedFromMaxTx(address)` and selector `[101, 140, 39, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isExcludedFromMaxTx", abi = "isExcludedFromMaxTx(address)")]
    pub struct IsExcludedFromMaxTxCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isExcludedFromMaxWallet` function with signature `isExcludedFromMaxWallet(address)` and selector `[109, 211, 211, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "isExcludedFromMaxWallet",
        abi = "isExcludedFromMaxWallet(address)"
    )]
    pub struct IsExcludedFromMaxWalletCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `lastSwapTime` function with signature `lastSwapTime()` and selector `[13, 216, 113, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lastSwapTime", abi = "lastSwapTime()")]
    pub struct LastSwapTimeCall;
    #[doc = "Container type for all input parameters for the `liquidityFeeBPS` function with signature `liquidityFeeBPS()` and selector `[235, 191, 26, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "liquidityFeeBPS", abi = "liquidityFeeBPS()")]
    pub struct LiquidityFeeBPSCall;
    #[doc = "Container type for all input parameters for the `manualSendDividend` function with signature `manualSendDividend(uint256,address)` and selector `[142, 18, 105, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "manualSendDividend",
        abi = "manualSendDividend(uint256,address)"
    )]
    pub struct ManualSendDividendCall {
        pub amount: ethers::core::types::U256,
        pub holder: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `maxTxBPS` function with signature `maxTxBPS()` and selector `[87, 119, 125, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maxTxBPS", abi = "maxTxBPS()")]
    pub struct MaxTxBPSCall;
    #[doc = "Container type for all input parameters for the `maxWalletBPS` function with signature `maxWalletBPS()` and selector `[116, 77, 21, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maxWalletBPS", abi = "maxWalletBPS()")]
    pub struct MaxWalletBPSCall;
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `openTrading` function with signature `openTrading()` and selector `[201, 86, 123, 249]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "openTrading", abi = "openTrading()")]
    pub struct OpenTradingCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `removeFromBlacklist` function with signature `removeFromBlacklist(address)` and selector `[83, 125, 243, 182]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeFromBlacklist", abi = "removeFromBlacklist(address)")]
    pub struct RemoveFromBlacklistCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `rescueETH` function with signature `rescueETH(uint256)` and selector `[158, 37, 47, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rescueETH", abi = "rescueETH(uint256)")]
    pub struct RescueETHCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `rescueToken` function with signature `rescueToken(address,uint256)` and selector `[51, 243, 214, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rescueToken", abi = "rescueToken(address,uint256)")]
    pub struct RescueTokenCall {
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setAutomatedMarketMakerPair` function with signature `setAutomatedMarketMakerPair(address,bool)` and selector `[154, 122, 35, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "setAutomatedMarketMakerPair",
        abi = "setAutomatedMarketMakerPair(address,bool)"
    )]
    pub struct SetAutomatedMarketMakerPairCall {
        pub pair: ethers::core::types::Address,
        pub value: bool,
    }
    #[doc = "Container type for all input parameters for the `setCompoundingEnabled` function with signature `setCompoundingEnabled(bool)` and selector `[228, 149, 108, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setCompoundingEnabled", abi = "setCompoundingEnabled(bool)")]
    pub struct SetCompoundingEnabledCall {
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `setFee` function with signature `setFee(uint256,uint256,uint256)` and selector `[91, 101, 185, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFee", abi = "setFee(uint256,uint256,uint256)")]
    pub struct SetFeeCall {
        pub treasury_fee: ethers::core::types::U256,
        pub liquidity_fee: ethers::core::types::U256,
        pub dividend_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setMaxTxBPS` function with signature `setMaxTxBPS(uint256)` and selector `[170, 78, 140, 74]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setMaxTxBPS", abi = "setMaxTxBPS(uint256)")]
    pub struct SetMaxTxBPSCall {
        pub bps: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setMaxWalletBPS` function with signature `setMaxWalletBPS(uint256)` and selector `[104, 197, 30, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setMaxWalletBPS", abi = "setMaxWalletBPS(uint256)")]
    pub struct SetMaxWalletBPSCall {
        pub bps: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setSwapEnabled` function with signature `setSwapEnabled(bool)` and selector `[224, 26, 249, 44]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSwapEnabled", abi = "setSwapEnabled(bool)")]
    pub struct SetSwapEnabledCall {
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `setTaxEnabled` function with signature `setTaxEnabled(bool)` and selector `[198, 175, 88, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setTaxEnabled", abi = "setTaxEnabled(bool)")]
    pub struct SetTaxEnabledCall {
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `setWallet` function with signature `setWallet(address,address)` and selector `[241, 178, 52, 173]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setWallet", abi = "setWallet(address,address)")]
    pub struct SetWalletCall {
        pub marketing_wallet: ethers::core::types::Address,
        pub liquidity_wallet: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `swapEnabled` function with signature `swapEnabled()` and selector `[109, 221, 23, 19]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "swapEnabled", abi = "swapEnabled()")]
    pub struct SwapEnabledCall;
    #[doc = "Container type for all input parameters for the `swapTokensAtAmount` function with signature `swapTokensAtAmount()` and selector `[226, 244, 86, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "swapTokensAtAmount", abi = "swapTokensAtAmount()")]
    pub struct SwapTokensAtAmountCall;
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `taxEnabled` function with signature `taxEnabled()` and selector `[135, 11, 211, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "taxEnabled", abi = "taxEnabled()")]
    pub struct TaxEnabledCall;
    #[doc = "Container type for all input parameters for the `totalFeeBPS` function with signature `totalFeeBPS()` and selector `[55, 235, 21, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalFeeBPS", abi = "totalFeeBPS()")]
    pub struct TotalFeeBPSCall;
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub sender: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `treasuryFeeBPS` function with signature `treasuryFeeBPS()` and selector `[89, 55, 234, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "treasuryFeeBPS", abi = "treasuryFeeBPS()")]
    pub struct TreasuryFeeBPSCall;
    #[doc = "Container type for all input parameters for the `unBlackListMany` function with signature `unBlackListMany(address[])` and selector `[244, 87, 28, 73]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unBlackListMany", abi = "unBlackListMany(address[])")]
    pub struct UnBlackListManyCall {
        pub users: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `uniswapV2Pair` function with signature `uniswapV2Pair()` and selector `[73, 189, 90, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "uniswapV2Pair", abi = "uniswapV2Pair()")]
    pub struct UniswapV2PairCall;
    #[doc = "Container type for all input parameters for the `uniswapV2Router` function with signature `uniswapV2Router()` and selector `[22, 148, 80, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "uniswapV2Router", abi = "uniswapV2Router()")]
    pub struct UniswapV2RouterCall;
    #[doc = "Container type for all input parameters for the `updateDividendSettings` function with signature `updateDividendSettings(bool,uint256,bool)` and selector `[94, 132, 58, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "updateDividendSettings",
        abi = "updateDividendSettings(bool,uint256,bool)"
    )]
    pub struct UpdateDividendSettingsCall {
        pub swap_enabled: bool,
        pub swap_tokens_at_amount: ethers::core::types::U256,
        pub swap_all_token: bool,
    }
    #[doc = "Container type for all input parameters for the `updateUniswapV2Router` function with signature `updateUniswapV2Router(address)` and selector `[101, 184, 219, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "updateUniswapV2Router", abi = "updateUniswapV2Router(address)")]
    pub struct UpdateUniswapV2RouterCall {
        pub new_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `withdrawableDividendOf` function with signature `withdrawableDividendOf(address)` and selector `[168, 185, 210, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "withdrawableDividendOf",
        abi = "withdrawableDividendOf(address)"
    )]
    pub struct WithdrawableDividendOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `withdrawnDividendOf` function with signature `withdrawnDividendOf(address)` and selector `[170, 253, 132, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawnDividendOf", abi = "withdrawnDividendOf(address)")]
    pub struct WithdrawnDividendOfCall {
        pub account: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum WatchtowerCalls {
        AccumulativeDividendOf(AccumulativeDividendOfCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        AutomatedMarketMakerPairs(AutomatedMarketMakerPairsCall),
        BalanceOf(BalanceOfCall),
        BlackList(BlackListCall),
        BlackListMany(BlackListManyCall),
        Claim(ClaimCall),
        Compound(CompoundCall),
        CompoundingEnabled(CompoundingEnabledCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        DividendFeeBPS(DividendFeeBPSCall),
        DividendTracker(DividendTrackerCall),
        ExcludeFromDividends(ExcludeFromDividendsCall),
        ExcludeFromFees(ExcludeFromFeesCall),
        ExcludeFromMaxTx(ExcludeFromMaxTxCall),
        ExcludeFromMaxWallet(ExcludeFromMaxWalletCall),
        GetAccountInfo(GetAccountInfoCall),
        GetLastClaimTime(GetLastClaimTimeCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        IsExcludedFromDividends(IsExcludedFromDividendsCall),
        IsExcludedFromFees(IsExcludedFromFeesCall),
        IsExcludedFromMaxTx(IsExcludedFromMaxTxCall),
        IsExcludedFromMaxWallet(IsExcludedFromMaxWalletCall),
        LastSwapTime(LastSwapTimeCall),
        LiquidityFeeBPS(LiquidityFeeBPSCall),
        ManualSendDividend(ManualSendDividendCall),
        MaxTxBPS(MaxTxBPSCall),
        MaxWalletBPS(MaxWalletBPSCall),
        Name(NameCall),
        OpenTrading(OpenTradingCall),
        Owner(OwnerCall),
        RemoveFromBlacklist(RemoveFromBlacklistCall),
        RenounceOwnership(RenounceOwnershipCall),
        RescueETH(RescueETHCall),
        RescueToken(RescueTokenCall),
        SetAutomatedMarketMakerPair(SetAutomatedMarketMakerPairCall),
        SetCompoundingEnabled(SetCompoundingEnabledCall),
        SetFee(SetFeeCall),
        SetMaxTxBPS(SetMaxTxBPSCall),
        SetMaxWalletBPS(SetMaxWalletBPSCall),
        SetSwapEnabled(SetSwapEnabledCall),
        SetTaxEnabled(SetTaxEnabledCall),
        SetWallet(SetWalletCall),
        SwapEnabled(SwapEnabledCall),
        SwapTokensAtAmount(SwapTokensAtAmountCall),
        Symbol(SymbolCall),
        TaxEnabled(TaxEnabledCall),
        TotalFeeBPS(TotalFeeBPSCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        TreasuryFeeBPS(TreasuryFeeBPSCall),
        UnBlackListMany(UnBlackListManyCall),
        UniswapV2Pair(UniswapV2PairCall),
        UniswapV2Router(UniswapV2RouterCall),
        UpdateDividendSettings(UpdateDividendSettingsCall),
        UpdateUniswapV2Router(UpdateUniswapV2RouterCall),
        WithdrawableDividendOf(WithdrawableDividendOfCall),
        WithdrawnDividendOf(WithdrawnDividendOfCall),
    }
    impl ethers::core::abi::AbiDecode for WatchtowerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AccumulativeDividendOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::AccumulativeDividendOf(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <AutomatedMarketMakerPairsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(WatchtowerCalls::AutomatedMarketMakerPairs(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BlackListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::BlackList(decoded));
            }
            if let Ok(decoded) =
                <BlackListManyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::BlackListMany(decoded));
            }
            if let Ok(decoded) = <ClaimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::Claim(decoded));
            }
            if let Ok(decoded) =
                <CompoundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::Compound(decoded));
            }
            if let Ok(decoded) =
                <CompoundingEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::CompoundingEnabled(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <DividendFeeBPSCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::DividendFeeBPS(decoded));
            }
            if let Ok(decoded) =
                <DividendTrackerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::DividendTracker(decoded));
            }
            if let Ok(decoded) =
                <ExcludeFromDividendsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::ExcludeFromDividends(decoded));
            }
            if let Ok(decoded) =
                <ExcludeFromFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::ExcludeFromFees(decoded));
            }
            if let Ok(decoded) =
                <ExcludeFromMaxTxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::ExcludeFromMaxTx(decoded));
            }
            if let Ok(decoded) =
                <ExcludeFromMaxWalletCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::ExcludeFromMaxWallet(decoded));
            }
            if let Ok(decoded) =
                <GetAccountInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::GetAccountInfo(decoded));
            }
            if let Ok(decoded) =
                <GetLastClaimTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::GetLastClaimTime(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <IsExcludedFromDividendsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::IsExcludedFromDividends(decoded));
            }
            if let Ok(decoded) =
                <IsExcludedFromFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::IsExcludedFromFees(decoded));
            }
            if let Ok(decoded) =
                <IsExcludedFromMaxTxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::IsExcludedFromMaxTx(decoded));
            }
            if let Ok(decoded) =
                <IsExcludedFromMaxWalletCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::IsExcludedFromMaxWallet(decoded));
            }
            if let Ok(decoded) =
                <LastSwapTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::LastSwapTime(decoded));
            }
            if let Ok(decoded) =
                <LiquidityFeeBPSCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::LiquidityFeeBPS(decoded));
            }
            if let Ok(decoded) =
                <ManualSendDividendCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::ManualSendDividend(decoded));
            }
            if let Ok(decoded) =
                <MaxTxBPSCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::MaxTxBPS(decoded));
            }
            if let Ok(decoded) =
                <MaxWalletBPSCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::MaxWalletBPS(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(WatchtowerCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <OpenTradingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::OpenTrading(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RemoveFromBlacklistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::RemoveFromBlacklist(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RescueETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::RescueETH(decoded));
            }
            if let Ok(decoded) =
                <RescueTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::RescueToken(decoded));
            }
            if let Ok(decoded) =
                <SetAutomatedMarketMakerPairCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(WatchtowerCalls::SetAutomatedMarketMakerPair(decoded));
            }
            if let Ok(decoded) =
                <SetCompoundingEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::SetCompoundingEnabled(decoded));
            }
            if let Ok(decoded) = <SetFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::SetFee(decoded));
            }
            if let Ok(decoded) =
                <SetMaxTxBPSCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::SetMaxTxBPS(decoded));
            }
            if let Ok(decoded) =
                <SetMaxWalletBPSCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::SetMaxWalletBPS(decoded));
            }
            if let Ok(decoded) =
                <SetSwapEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::SetSwapEnabled(decoded));
            }
            if let Ok(decoded) =
                <SetTaxEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::SetTaxEnabled(decoded));
            }
            if let Ok(decoded) =
                <SetWalletCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::SetWallet(decoded));
            }
            if let Ok(decoded) =
                <SwapEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::SwapEnabled(decoded));
            }
            if let Ok(decoded) =
                <SwapTokensAtAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::SwapTokensAtAmount(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TaxEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::TaxEnabled(decoded));
            }
            if let Ok(decoded) =
                <TotalFeeBPSCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::TotalFeeBPS(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TreasuryFeeBPSCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::TreasuryFeeBPS(decoded));
            }
            if let Ok(decoded) =
                <UnBlackListManyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::UnBlackListMany(decoded));
            }
            if let Ok(decoded) =
                <UniswapV2PairCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::UniswapV2Pair(decoded));
            }
            if let Ok(decoded) =
                <UniswapV2RouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::UniswapV2Router(decoded));
            }
            if let Ok(decoded) =
                <UpdateDividendSettingsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::UpdateDividendSettings(decoded));
            }
            if let Ok(decoded) =
                <UpdateUniswapV2RouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::UpdateUniswapV2Router(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableDividendOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::WithdrawableDividendOf(decoded));
            }
            if let Ok(decoded) =
                <WithdrawnDividendOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WatchtowerCalls::WithdrawnDividendOf(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for WatchtowerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                WatchtowerCalls::AccumulativeDividendOf(element) => element.encode(),
                WatchtowerCalls::Allowance(element) => element.encode(),
                WatchtowerCalls::Approve(element) => element.encode(),
                WatchtowerCalls::AutomatedMarketMakerPairs(element) => element.encode(),
                WatchtowerCalls::BalanceOf(element) => element.encode(),
                WatchtowerCalls::BlackList(element) => element.encode(),
                WatchtowerCalls::BlackListMany(element) => element.encode(),
                WatchtowerCalls::Claim(element) => element.encode(),
                WatchtowerCalls::Compound(element) => element.encode(),
                WatchtowerCalls::CompoundingEnabled(element) => element.encode(),
                WatchtowerCalls::Decimals(element) => element.encode(),
                WatchtowerCalls::DecreaseAllowance(element) => element.encode(),
                WatchtowerCalls::DividendFeeBPS(element) => element.encode(),
                WatchtowerCalls::DividendTracker(element) => element.encode(),
                WatchtowerCalls::ExcludeFromDividends(element) => element.encode(),
                WatchtowerCalls::ExcludeFromFees(element) => element.encode(),
                WatchtowerCalls::ExcludeFromMaxTx(element) => element.encode(),
                WatchtowerCalls::ExcludeFromMaxWallet(element) => element.encode(),
                WatchtowerCalls::GetAccountInfo(element) => element.encode(),
                WatchtowerCalls::GetLastClaimTime(element) => element.encode(),
                WatchtowerCalls::IncreaseAllowance(element) => element.encode(),
                WatchtowerCalls::IsExcludedFromDividends(element) => element.encode(),
                WatchtowerCalls::IsExcludedFromFees(element) => element.encode(),
                WatchtowerCalls::IsExcludedFromMaxTx(element) => element.encode(),
                WatchtowerCalls::IsExcludedFromMaxWallet(element) => element.encode(),
                WatchtowerCalls::LastSwapTime(element) => element.encode(),
                WatchtowerCalls::LiquidityFeeBPS(element) => element.encode(),
                WatchtowerCalls::ManualSendDividend(element) => element.encode(),
                WatchtowerCalls::MaxTxBPS(element) => element.encode(),
                WatchtowerCalls::MaxWalletBPS(element) => element.encode(),
                WatchtowerCalls::Name(element) => element.encode(),
                WatchtowerCalls::OpenTrading(element) => element.encode(),
                WatchtowerCalls::Owner(element) => element.encode(),
                WatchtowerCalls::RemoveFromBlacklist(element) => element.encode(),
                WatchtowerCalls::RenounceOwnership(element) => element.encode(),
                WatchtowerCalls::RescueETH(element) => element.encode(),
                WatchtowerCalls::RescueToken(element) => element.encode(),
                WatchtowerCalls::SetAutomatedMarketMakerPair(element) => element.encode(),
                WatchtowerCalls::SetCompoundingEnabled(element) => element.encode(),
                WatchtowerCalls::SetFee(element) => element.encode(),
                WatchtowerCalls::SetMaxTxBPS(element) => element.encode(),
                WatchtowerCalls::SetMaxWalletBPS(element) => element.encode(),
                WatchtowerCalls::SetSwapEnabled(element) => element.encode(),
                WatchtowerCalls::SetTaxEnabled(element) => element.encode(),
                WatchtowerCalls::SetWallet(element) => element.encode(),
                WatchtowerCalls::SwapEnabled(element) => element.encode(),
                WatchtowerCalls::SwapTokensAtAmount(element) => element.encode(),
                WatchtowerCalls::Symbol(element) => element.encode(),
                WatchtowerCalls::TaxEnabled(element) => element.encode(),
                WatchtowerCalls::TotalFeeBPS(element) => element.encode(),
                WatchtowerCalls::TotalSupply(element) => element.encode(),
                WatchtowerCalls::Transfer(element) => element.encode(),
                WatchtowerCalls::TransferFrom(element) => element.encode(),
                WatchtowerCalls::TransferOwnership(element) => element.encode(),
                WatchtowerCalls::TreasuryFeeBPS(element) => element.encode(),
                WatchtowerCalls::UnBlackListMany(element) => element.encode(),
                WatchtowerCalls::UniswapV2Pair(element) => element.encode(),
                WatchtowerCalls::UniswapV2Router(element) => element.encode(),
                WatchtowerCalls::UpdateDividendSettings(element) => element.encode(),
                WatchtowerCalls::UpdateUniswapV2Router(element) => element.encode(),
                WatchtowerCalls::WithdrawableDividendOf(element) => element.encode(),
                WatchtowerCalls::WithdrawnDividendOf(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for WatchtowerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                WatchtowerCalls::AccumulativeDividendOf(element) => element.fmt(f),
                WatchtowerCalls::Allowance(element) => element.fmt(f),
                WatchtowerCalls::Approve(element) => element.fmt(f),
                WatchtowerCalls::AutomatedMarketMakerPairs(element) => element.fmt(f),
                WatchtowerCalls::BalanceOf(element) => element.fmt(f),
                WatchtowerCalls::BlackList(element) => element.fmt(f),
                WatchtowerCalls::BlackListMany(element) => element.fmt(f),
                WatchtowerCalls::Claim(element) => element.fmt(f),
                WatchtowerCalls::Compound(element) => element.fmt(f),
                WatchtowerCalls::CompoundingEnabled(element) => element.fmt(f),
                WatchtowerCalls::Decimals(element) => element.fmt(f),
                WatchtowerCalls::DecreaseAllowance(element) => element.fmt(f),
                WatchtowerCalls::DividendFeeBPS(element) => element.fmt(f),
                WatchtowerCalls::DividendTracker(element) => element.fmt(f),
                WatchtowerCalls::ExcludeFromDividends(element) => element.fmt(f),
                WatchtowerCalls::ExcludeFromFees(element) => element.fmt(f),
                WatchtowerCalls::ExcludeFromMaxTx(element) => element.fmt(f),
                WatchtowerCalls::ExcludeFromMaxWallet(element) => element.fmt(f),
                WatchtowerCalls::GetAccountInfo(element) => element.fmt(f),
                WatchtowerCalls::GetLastClaimTime(element) => element.fmt(f),
                WatchtowerCalls::IncreaseAllowance(element) => element.fmt(f),
                WatchtowerCalls::IsExcludedFromDividends(element) => element.fmt(f),
                WatchtowerCalls::IsExcludedFromFees(element) => element.fmt(f),
                WatchtowerCalls::IsExcludedFromMaxTx(element) => element.fmt(f),
                WatchtowerCalls::IsExcludedFromMaxWallet(element) => element.fmt(f),
                WatchtowerCalls::LastSwapTime(element) => element.fmt(f),
                WatchtowerCalls::LiquidityFeeBPS(element) => element.fmt(f),
                WatchtowerCalls::ManualSendDividend(element) => element.fmt(f),
                WatchtowerCalls::MaxTxBPS(element) => element.fmt(f),
                WatchtowerCalls::MaxWalletBPS(element) => element.fmt(f),
                WatchtowerCalls::Name(element) => element.fmt(f),
                WatchtowerCalls::OpenTrading(element) => element.fmt(f),
                WatchtowerCalls::Owner(element) => element.fmt(f),
                WatchtowerCalls::RemoveFromBlacklist(element) => element.fmt(f),
                WatchtowerCalls::RenounceOwnership(element) => element.fmt(f),
                WatchtowerCalls::RescueETH(element) => element.fmt(f),
                WatchtowerCalls::RescueToken(element) => element.fmt(f),
                WatchtowerCalls::SetAutomatedMarketMakerPair(element) => element.fmt(f),
                WatchtowerCalls::SetCompoundingEnabled(element) => element.fmt(f),
                WatchtowerCalls::SetFee(element) => element.fmt(f),
                WatchtowerCalls::SetMaxTxBPS(element) => element.fmt(f),
                WatchtowerCalls::SetMaxWalletBPS(element) => element.fmt(f),
                WatchtowerCalls::SetSwapEnabled(element) => element.fmt(f),
                WatchtowerCalls::SetTaxEnabled(element) => element.fmt(f),
                WatchtowerCalls::SetWallet(element) => element.fmt(f),
                WatchtowerCalls::SwapEnabled(element) => element.fmt(f),
                WatchtowerCalls::SwapTokensAtAmount(element) => element.fmt(f),
                WatchtowerCalls::Symbol(element) => element.fmt(f),
                WatchtowerCalls::TaxEnabled(element) => element.fmt(f),
                WatchtowerCalls::TotalFeeBPS(element) => element.fmt(f),
                WatchtowerCalls::TotalSupply(element) => element.fmt(f),
                WatchtowerCalls::Transfer(element) => element.fmt(f),
                WatchtowerCalls::TransferFrom(element) => element.fmt(f),
                WatchtowerCalls::TransferOwnership(element) => element.fmt(f),
                WatchtowerCalls::TreasuryFeeBPS(element) => element.fmt(f),
                WatchtowerCalls::UnBlackListMany(element) => element.fmt(f),
                WatchtowerCalls::UniswapV2Pair(element) => element.fmt(f),
                WatchtowerCalls::UniswapV2Router(element) => element.fmt(f),
                WatchtowerCalls::UpdateDividendSettings(element) => element.fmt(f),
                WatchtowerCalls::UpdateUniswapV2Router(element) => element.fmt(f),
                WatchtowerCalls::WithdrawableDividendOf(element) => element.fmt(f),
                WatchtowerCalls::WithdrawnDividendOf(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AccumulativeDividendOfCall> for WatchtowerCalls {
        fn from(var: AccumulativeDividendOfCall) -> Self {
            WatchtowerCalls::AccumulativeDividendOf(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for WatchtowerCalls {
        fn from(var: AllowanceCall) -> Self {
            WatchtowerCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for WatchtowerCalls {
        fn from(var: ApproveCall) -> Self {
            WatchtowerCalls::Approve(var)
        }
    }
    impl ::std::convert::From<AutomatedMarketMakerPairsCall> for WatchtowerCalls {
        fn from(var: AutomatedMarketMakerPairsCall) -> Self {
            WatchtowerCalls::AutomatedMarketMakerPairs(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for WatchtowerCalls {
        fn from(var: BalanceOfCall) -> Self {
            WatchtowerCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BlackListCall> for WatchtowerCalls {
        fn from(var: BlackListCall) -> Self {
            WatchtowerCalls::BlackList(var)
        }
    }
    impl ::std::convert::From<BlackListManyCall> for WatchtowerCalls {
        fn from(var: BlackListManyCall) -> Self {
            WatchtowerCalls::BlackListMany(var)
        }
    }
    impl ::std::convert::From<ClaimCall> for WatchtowerCalls {
        fn from(var: ClaimCall) -> Self {
            WatchtowerCalls::Claim(var)
        }
    }
    impl ::std::convert::From<CompoundCall> for WatchtowerCalls {
        fn from(var: CompoundCall) -> Self {
            WatchtowerCalls::Compound(var)
        }
    }
    impl ::std::convert::From<CompoundingEnabledCall> for WatchtowerCalls {
        fn from(var: CompoundingEnabledCall) -> Self {
            WatchtowerCalls::CompoundingEnabled(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for WatchtowerCalls {
        fn from(var: DecimalsCall) -> Self {
            WatchtowerCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for WatchtowerCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            WatchtowerCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<DividendFeeBPSCall> for WatchtowerCalls {
        fn from(var: DividendFeeBPSCall) -> Self {
            WatchtowerCalls::DividendFeeBPS(var)
        }
    }
    impl ::std::convert::From<DividendTrackerCall> for WatchtowerCalls {
        fn from(var: DividendTrackerCall) -> Self {
            WatchtowerCalls::DividendTracker(var)
        }
    }
    impl ::std::convert::From<ExcludeFromDividendsCall> for WatchtowerCalls {
        fn from(var: ExcludeFromDividendsCall) -> Self {
            WatchtowerCalls::ExcludeFromDividends(var)
        }
    }
    impl ::std::convert::From<ExcludeFromFeesCall> for WatchtowerCalls {
        fn from(var: ExcludeFromFeesCall) -> Self {
            WatchtowerCalls::ExcludeFromFees(var)
        }
    }
    impl ::std::convert::From<ExcludeFromMaxTxCall> for WatchtowerCalls {
        fn from(var: ExcludeFromMaxTxCall) -> Self {
            WatchtowerCalls::ExcludeFromMaxTx(var)
        }
    }
    impl ::std::convert::From<ExcludeFromMaxWalletCall> for WatchtowerCalls {
        fn from(var: ExcludeFromMaxWalletCall) -> Self {
            WatchtowerCalls::ExcludeFromMaxWallet(var)
        }
    }
    impl ::std::convert::From<GetAccountInfoCall> for WatchtowerCalls {
        fn from(var: GetAccountInfoCall) -> Self {
            WatchtowerCalls::GetAccountInfo(var)
        }
    }
    impl ::std::convert::From<GetLastClaimTimeCall> for WatchtowerCalls {
        fn from(var: GetLastClaimTimeCall) -> Self {
            WatchtowerCalls::GetLastClaimTime(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for WatchtowerCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            WatchtowerCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<IsExcludedFromDividendsCall> for WatchtowerCalls {
        fn from(var: IsExcludedFromDividendsCall) -> Self {
            WatchtowerCalls::IsExcludedFromDividends(var)
        }
    }
    impl ::std::convert::From<IsExcludedFromFeesCall> for WatchtowerCalls {
        fn from(var: IsExcludedFromFeesCall) -> Self {
            WatchtowerCalls::IsExcludedFromFees(var)
        }
    }
    impl ::std::convert::From<IsExcludedFromMaxTxCall> for WatchtowerCalls {
        fn from(var: IsExcludedFromMaxTxCall) -> Self {
            WatchtowerCalls::IsExcludedFromMaxTx(var)
        }
    }
    impl ::std::convert::From<IsExcludedFromMaxWalletCall> for WatchtowerCalls {
        fn from(var: IsExcludedFromMaxWalletCall) -> Self {
            WatchtowerCalls::IsExcludedFromMaxWallet(var)
        }
    }
    impl ::std::convert::From<LastSwapTimeCall> for WatchtowerCalls {
        fn from(var: LastSwapTimeCall) -> Self {
            WatchtowerCalls::LastSwapTime(var)
        }
    }
    impl ::std::convert::From<LiquidityFeeBPSCall> for WatchtowerCalls {
        fn from(var: LiquidityFeeBPSCall) -> Self {
            WatchtowerCalls::LiquidityFeeBPS(var)
        }
    }
    impl ::std::convert::From<ManualSendDividendCall> for WatchtowerCalls {
        fn from(var: ManualSendDividendCall) -> Self {
            WatchtowerCalls::ManualSendDividend(var)
        }
    }
    impl ::std::convert::From<MaxTxBPSCall> for WatchtowerCalls {
        fn from(var: MaxTxBPSCall) -> Self {
            WatchtowerCalls::MaxTxBPS(var)
        }
    }
    impl ::std::convert::From<MaxWalletBPSCall> for WatchtowerCalls {
        fn from(var: MaxWalletBPSCall) -> Self {
            WatchtowerCalls::MaxWalletBPS(var)
        }
    }
    impl ::std::convert::From<NameCall> for WatchtowerCalls {
        fn from(var: NameCall) -> Self {
            WatchtowerCalls::Name(var)
        }
    }
    impl ::std::convert::From<OpenTradingCall> for WatchtowerCalls {
        fn from(var: OpenTradingCall) -> Self {
            WatchtowerCalls::OpenTrading(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for WatchtowerCalls {
        fn from(var: OwnerCall) -> Self {
            WatchtowerCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RemoveFromBlacklistCall> for WatchtowerCalls {
        fn from(var: RemoveFromBlacklistCall) -> Self {
            WatchtowerCalls::RemoveFromBlacklist(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for WatchtowerCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            WatchtowerCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RescueETHCall> for WatchtowerCalls {
        fn from(var: RescueETHCall) -> Self {
            WatchtowerCalls::RescueETH(var)
        }
    }
    impl ::std::convert::From<RescueTokenCall> for WatchtowerCalls {
        fn from(var: RescueTokenCall) -> Self {
            WatchtowerCalls::RescueToken(var)
        }
    }
    impl ::std::convert::From<SetAutomatedMarketMakerPairCall> for WatchtowerCalls {
        fn from(var: SetAutomatedMarketMakerPairCall) -> Self {
            WatchtowerCalls::SetAutomatedMarketMakerPair(var)
        }
    }
    impl ::std::convert::From<SetCompoundingEnabledCall> for WatchtowerCalls {
        fn from(var: SetCompoundingEnabledCall) -> Self {
            WatchtowerCalls::SetCompoundingEnabled(var)
        }
    }
    impl ::std::convert::From<SetFeeCall> for WatchtowerCalls {
        fn from(var: SetFeeCall) -> Self {
            WatchtowerCalls::SetFee(var)
        }
    }
    impl ::std::convert::From<SetMaxTxBPSCall> for WatchtowerCalls {
        fn from(var: SetMaxTxBPSCall) -> Self {
            WatchtowerCalls::SetMaxTxBPS(var)
        }
    }
    impl ::std::convert::From<SetMaxWalletBPSCall> for WatchtowerCalls {
        fn from(var: SetMaxWalletBPSCall) -> Self {
            WatchtowerCalls::SetMaxWalletBPS(var)
        }
    }
    impl ::std::convert::From<SetSwapEnabledCall> for WatchtowerCalls {
        fn from(var: SetSwapEnabledCall) -> Self {
            WatchtowerCalls::SetSwapEnabled(var)
        }
    }
    impl ::std::convert::From<SetTaxEnabledCall> for WatchtowerCalls {
        fn from(var: SetTaxEnabledCall) -> Self {
            WatchtowerCalls::SetTaxEnabled(var)
        }
    }
    impl ::std::convert::From<SetWalletCall> for WatchtowerCalls {
        fn from(var: SetWalletCall) -> Self {
            WatchtowerCalls::SetWallet(var)
        }
    }
    impl ::std::convert::From<SwapEnabledCall> for WatchtowerCalls {
        fn from(var: SwapEnabledCall) -> Self {
            WatchtowerCalls::SwapEnabled(var)
        }
    }
    impl ::std::convert::From<SwapTokensAtAmountCall> for WatchtowerCalls {
        fn from(var: SwapTokensAtAmountCall) -> Self {
            WatchtowerCalls::SwapTokensAtAmount(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for WatchtowerCalls {
        fn from(var: SymbolCall) -> Self {
            WatchtowerCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TaxEnabledCall> for WatchtowerCalls {
        fn from(var: TaxEnabledCall) -> Self {
            WatchtowerCalls::TaxEnabled(var)
        }
    }
    impl ::std::convert::From<TotalFeeBPSCall> for WatchtowerCalls {
        fn from(var: TotalFeeBPSCall) -> Self {
            WatchtowerCalls::TotalFeeBPS(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for WatchtowerCalls {
        fn from(var: TotalSupplyCall) -> Self {
            WatchtowerCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for WatchtowerCalls {
        fn from(var: TransferCall) -> Self {
            WatchtowerCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for WatchtowerCalls {
        fn from(var: TransferFromCall) -> Self {
            WatchtowerCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for WatchtowerCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            WatchtowerCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<TreasuryFeeBPSCall> for WatchtowerCalls {
        fn from(var: TreasuryFeeBPSCall) -> Self {
            WatchtowerCalls::TreasuryFeeBPS(var)
        }
    }
    impl ::std::convert::From<UnBlackListManyCall> for WatchtowerCalls {
        fn from(var: UnBlackListManyCall) -> Self {
            WatchtowerCalls::UnBlackListMany(var)
        }
    }
    impl ::std::convert::From<UniswapV2PairCall> for WatchtowerCalls {
        fn from(var: UniswapV2PairCall) -> Self {
            WatchtowerCalls::UniswapV2Pair(var)
        }
    }
    impl ::std::convert::From<UniswapV2RouterCall> for WatchtowerCalls {
        fn from(var: UniswapV2RouterCall) -> Self {
            WatchtowerCalls::UniswapV2Router(var)
        }
    }
    impl ::std::convert::From<UpdateDividendSettingsCall> for WatchtowerCalls {
        fn from(var: UpdateDividendSettingsCall) -> Self {
            WatchtowerCalls::UpdateDividendSettings(var)
        }
    }
    impl ::std::convert::From<UpdateUniswapV2RouterCall> for WatchtowerCalls {
        fn from(var: UpdateUniswapV2RouterCall) -> Self {
            WatchtowerCalls::UpdateUniswapV2Router(var)
        }
    }
    impl ::std::convert::From<WithdrawableDividendOfCall> for WatchtowerCalls {
        fn from(var: WithdrawableDividendOfCall) -> Self {
            WatchtowerCalls::WithdrawableDividendOf(var)
        }
    }
    impl ::std::convert::From<WithdrawnDividendOfCall> for WatchtowerCalls {
        fn from(var: WithdrawnDividendOfCall) -> Self {
            WatchtowerCalls::WithdrawnDividendOf(var)
        }
    }
    #[doc = "Container type for all return fields from the `accumulativeDividendOf` function with signature `accumulativeDividendOf(address)` and selector `[39, 206, 1, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AccumulativeDividendOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `automatedMarketMakerPairs` function with signature `automatedMarketMakerPairs(address)` and selector `[182, 36, 150, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AutomatedMarketMakerPairsReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `compoundingEnabled` function with signature `compoundingEnabled()` and selector `[47, 69, 4, 174]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CompoundingEnabledReturn(pub bool);
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `dividendFeeBPS` function with signature `dividendFeeBPS()` and selector `[184, 11, 110, 137]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DividendFeeBPSReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `dividendTracker` function with signature `dividendTracker()` and selector `[44, 31, 82, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DividendTrackerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getAccountInfo` function with signature `getAccountInfo(address)` and selector `[123, 81, 15, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAccountInfoReturn(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `getLastClaimTime` function with signature `getLastClaimTime(address)` and selector `[166, 128, 224, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetLastClaimTimeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IncreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `isExcludedFromDividends` function with signature `isExcludedFromDividends(address)` and selector `[199, 5, 197, 105]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsExcludedFromDividendsReturn(pub bool);
    #[doc = "Container type for all return fields from the `isExcludedFromFees` function with signature `isExcludedFromFees(address)` and selector `[79, 190, 225, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsExcludedFromFeesReturn(pub bool);
    #[doc = "Container type for all return fields from the `isExcludedFromMaxTx` function with signature `isExcludedFromMaxTx(address)` and selector `[101, 140, 39, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsExcludedFromMaxTxReturn(pub bool);
    #[doc = "Container type for all return fields from the `isExcludedFromMaxWallet` function with signature `isExcludedFromMaxWallet(address)` and selector `[109, 211, 211, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsExcludedFromMaxWalletReturn(pub bool);
    #[doc = "Container type for all return fields from the `lastSwapTime` function with signature `lastSwapTime()` and selector `[13, 216, 113, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LastSwapTimeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `liquidityFeeBPS` function with signature `liquidityFeeBPS()` and selector `[235, 191, 26, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LiquidityFeeBPSReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `maxTxBPS` function with signature `maxTxBPS()` and selector `[87, 119, 125, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MaxTxBPSReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `maxWalletBPS` function with signature `maxWalletBPS()` and selector `[116, 77, 21, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MaxWalletBPSReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NameReturn(pub String);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `swapEnabled` function with signature `swapEnabled()` and selector `[109, 221, 23, 19]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SwapEnabledReturn(pub bool);
    #[doc = "Container type for all return fields from the `swapTokensAtAmount` function with signature `swapTokensAtAmount()` and selector `[226, 244, 86, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SwapTokensAtAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `taxEnabled` function with signature `taxEnabled()` and selector `[135, 11, 211, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TaxEnabledReturn(pub bool);
    #[doc = "Container type for all return fields from the `totalFeeBPS` function with signature `totalFeeBPS()` and selector `[55, 235, 21, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TotalFeeBPSReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferFromReturn(pub bool);
    #[doc = "Container type for all return fields from the `treasuryFeeBPS` function with signature `treasuryFeeBPS()` and selector `[89, 55, 234, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TreasuryFeeBPSReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `uniswapV2Pair` function with signature `uniswapV2Pair()` and selector `[73, 189, 90, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UniswapV2PairReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `uniswapV2Router` function with signature `uniswapV2Router()` and selector `[22, 148, 80, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UniswapV2RouterReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `withdrawableDividendOf` function with signature `withdrawableDividendOf(address)` and selector `[168, 185, 210, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct WithdrawableDividendOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `withdrawnDividendOf` function with signature `withdrawnDividendOf(address)` and selector `[170, 253, 132, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct WithdrawnDividendOfReturn(pub ethers::core::types::U256);
}
