// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

// For uniswap call
struct UniswapCall {
	address sender;
	address receiver;
	uint256 amount;
}

// For uniswap send
struct UniswapSend {
	address sender;
	uint256 amount;
}
