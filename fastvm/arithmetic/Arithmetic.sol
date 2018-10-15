pragma solidity ^0.4.0;

contract Test {

    uint a = 0x567123467891237689439;
    uint b = 0x1232378901726397128396;
    uint[] result;
    function unsigned() {
        result.push(a + b);
        result.push(a - b);
        result.push(a * b);
        result.push(a / b);
        result.push(a % b);
        result.push(a ** b);
    }

    int c = 0x567123467891237689439;
    int d = 0x1232378901726397128396;
    int[] result2;
    function signed() {
        result2.push(c + d);
        result2.push(c - d);
        result2.push(c * d);
        result2.push(c / d);
        result2.push(c % d);
    }

    uint e = 0x12378012734;
    uint am;
    uint mm;
    function combo() {
        am = addmod(a, b, e);
        mm = mulmod(a, b, e);
    }

    // STOP and SIGNEXTEND not tested
}
