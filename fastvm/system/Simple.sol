pragma solidity ^0.4.0;

contract simple {
  function f(uint n) constant returns(uint) {
    uint sum = 0x12345678;

    for (uint i = 0; i < n; i++) {
      sum = sum + i;
    }
    
    return sum;
  }
}
