pragma solidity ^0.4.0;
    
contract DynamicArray {

    function create(uint n) {
        new uint[](n);
    }

    uint ret;
    function createAndAccess(uint n) {
        uint[] memory tmp = new uint[](n);
        tmp[n - 1] = 7;
        ret = tmp[n - 1];
    }
}