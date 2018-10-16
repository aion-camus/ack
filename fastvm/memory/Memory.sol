pragma solidity ^0.4.0;

contract Test {

    uint result;
    function f() {
        uint[] memory a = new uint[](5);
        a[0] = 1;
        result = a[0];
    }
}
