pragma solidity ^0.4.0;

contract Test {

    uint a = 0x567123467891237689439;
    uint b = 0x1232378901726397128396;
    bool[] result;
    function unsigned() {
        result.push(a < b);
        result.push(a > b);
        result.push(a == b);
    }

    int c = 0x567123467891237689439;
    int d = 0x1232378901726397128396;
    bool[] result2;
    function signed() {
        result2.push(c < d);
        result2.push(c > d);
        result2.push(c == d);
    }

    uint[] result3;
    function combo() {
        result3.push(a & b);
        result3.push(a | b);
        result3.push(a ^ b);
        result3.push(~a);
    }

    // BYTE is not tested
}
