var Web3 = require('aion-web3');
var web3 = new Web3(new Web3.providers.HttpProvider("http://localhost:8545"));
var fs = require("fs");

function handler(lines, current) {
    console.log("Current raw tx: " + lines[current]);

    web3.eth.sendRawTransaction(lines[current], function(err, hash) {
        if (err) {
            console.error("Failed to send transaction: " + err);
            process.exit(1);
        }
        var wait = 10000;
        console.log("Tx hash: " + hash + ", wait for " + wait / 1000 + " seconds");

        setTimeout(() => {
            var receipt = web3.eth.getTransactionReceipt(hash);
            if (receipt) {
                console.log(receipt);
                if (current < lines.length) {
                    handler(lines, current + 1);
                }
            } else {
                console.error("Failed to retrieve the receipt for transaction: " + hash);
            }
        }, wait);
    });
}

fs.readFile("../transactions.txt", "utf8", function(err, contents) {
    var lines = contents.trim().split('\n');
    handler(lines, 0);
});
