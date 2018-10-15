pragma solidity ^0.4.15;

contract Sink {

    event Log(bytes data);

    function() payable {
        Log(msg.data);
    }
}