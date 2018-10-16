pragma solidity ^0.4.0;

contract Test {

    event X(uint, bytes, uint[]);

    function Test() {
        uint[] memory temp = new uint[](2);
        temp[0] = 1;
        temp[1] = 2;
        X(1, "abc", temp);
    }
}
