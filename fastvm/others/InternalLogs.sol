pragma solidity ^0.4.0;

contract B {

  event X();

  function f(uint gas) public {
    X();

    this.call.gas(gas).value(0)(bytes4(sha3("g()")));
  }

  event Y();

  function g() public {
    Y();

    uint n = 0;
    for (uint i = 0; i < 1000; i++) {
      n += i;
    }
  }
}
