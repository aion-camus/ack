// web3
var Web3 = require('aion-web3')
var web3 = new Web3(new Web3.providers.HttpProvider("http://localhost:8545"));

// readline
var reader = require('readline').createInterface({
  input: require('fs').createReadStream('../transactions.txt')
});

// parse the transactions and send them to the network
var processed = 0;
reader.on('line', function (line) {
  web3.eth.sendRawTransaction(line, function(err, hash) {
    if (err) {
      console.error("Failed to send transaction: " + err);
      process.exit(1);
    }
    var wait = 60000 + processed * 1000;
    console.log("Tx hash: " + hash + ", wait for " + wait/1000 + " seconds");

    setTimeout(() => {
      var receipt = web3.eth.getTransactionReceipt(hash);
      if (receipt) {
        console.log(receipt);
      } else {
        console.error("Failed to retrieve the receipt for transaction: " + hash);
      }
    }, wait);

    processed++;
  });
});
