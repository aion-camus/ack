pragma solidity ^0.4.0;

contract Test {

    function f() {
        suicide(msg.sender);
    }
}
