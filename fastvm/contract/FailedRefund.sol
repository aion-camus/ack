pragma solidity ^0.4.0;

contract FailedRefund {

    address public recipient;
    mapping (address => uint) public balances;
    address[] private refundAddresses;
    mapping (address => uint) public refunds;

    event Sent(address from, address to, uint amount);

    function FailedRefund() {
        balances[msg.sender] = 100000;
        refundAddresses.push(1);
        refundAddresses.push(2);
    }

    function send(address leader, uint amount) public {
        if (balances[msg.sender] < amount) {
            return;
        }

        balances[msg.sender] -= amount;
        balances[recipient] += amount;
        return Sent(msg.sender, recipient, amount);
    }

    function refundAll() public {
        for(uint x; x < refundAddresses.length; x++) {
            if(refunds[refundAddresses[x]] > 0) {
                refundAddresses[x].transfer(refunds[refundAddresses[x]]);
            }
        }

    }

}
