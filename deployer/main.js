// web3
var Web3 = require('aion-web3')
var web3 = new Web3(new Web3.providers.HttpProvider("http://localhost:8545"));

// readline
var reader = require('readline').createInterface({
  input: require('fs').createReadStream('../transactions.txt')
});

// parse the transactions and send them to the network
reader.on('line', function (line) {
  console.log(line);
  web3.eth.sendRawTransaction(line).then((receipt) => {
    console.log(receipt);
  }, (error) => {
    console.error("Failed to send transaction: ", error);
    process.exit(1);
  });
});
