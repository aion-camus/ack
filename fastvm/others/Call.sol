pragma solidity ^0.4.0;

contract Caller {
    function add(uint a, uint b) returns (uint) {
        return a + b;
    }

    uint ret;
    function f() {
        ret = this.add(1, 2);
    }
}
