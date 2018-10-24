pragma solidity ^0.4.0;

contract Test {

    uint result2 = 3;

    function f() {
        result2 = result2 + 5;
    }

    function reset() {
        result2 = 0;
    }
}
