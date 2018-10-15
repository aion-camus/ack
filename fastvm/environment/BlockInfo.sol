pragma solidity ^0.4.0;

contract Test {
    bytes32 blockhash = block.blockhash(0);
    address coinbase = block.coinbase;
    uint timestamp = block.timestamp;
    uint number = block.number;
    uint diff = block.difficulty;
    uint gaslimit = block.gaslimit;
}
