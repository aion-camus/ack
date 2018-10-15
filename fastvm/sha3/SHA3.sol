pragma solidity ^0.4.0;

contract Test {

    bytes32 result;
    function f() {
        result = sha3("abc");
    }
}
