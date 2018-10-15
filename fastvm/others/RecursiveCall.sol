pragma solidity ^0.4.0;

contract A {
    function f(int n) {
        if (n > 0) {
            this.f(n - 1);
        }
    }
}
