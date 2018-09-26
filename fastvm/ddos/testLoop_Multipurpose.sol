pragma solidity ^0.4.15;

contract Multipurpose {

    event Log(bytes data);

    function logLoop(uint n) {
        for (uint i = 0; i < n; i++) {
            Log("0x1122");
        }
    }

    uint a;
    function ssetLoop(uint n) {
        for (uint i = 0; i < n; i++) {
            a = a + 1;
        } 
    }
}