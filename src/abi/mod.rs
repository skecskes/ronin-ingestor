use ethers::contract::abigen;

abigen!(
    ERC20,
    "src/abi/erc20.json"
);

abigen!(
    ERC721,
    "src/abi/erc721.json"
);

abigen!(
    ERC1155,
    "src/abi/erc1155.json"
);
