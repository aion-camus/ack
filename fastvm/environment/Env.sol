pragma solidity ^0.4.0;

contract Test {
    address a = this;
    uint b = this.balance;
    address c = tx.origin;
    address d = msg.sender;
    uint e = msg.value;
    uint f = tx.gasprice;

    uint[] result;
    function calldata() {
        result.push(msg.data.length);
        result.push(uint256(msg.data[0]));
    }

    // CODESIZE, CODECOPY, EXTCODESIZE, EXTCODECOPY are not tested
}
