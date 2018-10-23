var Web3 = require('aion-web3');
var web3 = new Web3(new Web3.providers.HttpProvider("http://localhost:8545"));
var fs = require("fs");

var it;

function handler(lines, current) {
    console.log("\n\nCurrent raw transaction: " + lines[current]);

    web3.eth.sendRawTransaction(lines[current], function(err, hash) {
        if (err) {
            console.error("Failed to send transaction: " + err);
            process.exit(1);
        }
        console.log("Transaction hash: " + hash);

        it = setInterval(() => {
            var receipt = web3.eth.getTransactionReceipt(hash);
            if (receipt) {
                console.log(receipt);
                clearInterval(it);

                if (current < lines.length) {
                    handler(lines, current + 1);
                }
            } else {
                console.log("Transaction receipt not yet available");
            }
        }, 2000);
    });
}

fs.readFile("../transactions.txt", "utf8", function(err, contents) {
    var lines = contents.trim().split('\n');
    handler(lines, 0);
});
