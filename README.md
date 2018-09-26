# Aion Compatibility Kit

Aion Compatibility Kit (ACK) is an extensive test suite to ensure compatibility between different Aion implementations. Ideally, it should be a centralized place to check all major components and protocols:
- Chain specifications
- Virtual machine
- P2P protocol

## Aion FastVM

There are currently two FastVM implementations; one is in Rust and the other in Java. Although the two implementations use the same core, Fastvm JIT, they may manifest different behaviour regarding kernel integration.

This test suite specifies a set of integration tests, composed of transactions, which all kernels have to pass. To run the tests,
1. Executes the transactions on a test network;
1. Let the kernels to sync.

### Integration test specs

The tests are divides into the following categories:
- basic
- contract
- ddos
- environment
- log
- recursive
- state

For each category, there are a set of JSON files, each of which follows the schema in below:
```
[
  {
    "name": "An unique name to identify the test",
    "description": "A short description of the test",
    "transactions": [
      {
        "type": "Transaction type: CREATE or CALL",
        "receiver": "The receiver's address",
        "value": "(optional) The value to send, in decimal or hex, default: 0",
        "data": {
          "raw": "(optional) Any unstructured byte array, in hex",
          "code": "(optional) Contract initialization code, in hex",
          "method": "(optional) The method signature before hashing",
          "arguments": "(optional) Encoded arguments, in hex"
        },
        "nrg": "(optional) The energy limit, in decimal or hex, default: 1000000",
        "nrgPrice": "(optional) The energy price, in decimal or hex, default: 1"
      }
    ]
  }
]
```

Notes: 

- Fields with default value are optional 
- To fully verify the transaction result, we need to check the state change, gas usage, return data and logs. We're only asserting the transaction `status` and `return_data`, because the other fields can be checked when importing the blocks.
- In a normal Aion transaction, there is only one field `data` for the payload, we're dividing it into different sub-fields based on how the `data` is typically being used. The final payload can be assembled by the following concatenation:
```
data = raw + code + method + arguments
```

### Environment variables

For convenience, the following environment variables are pre-defined and can be referred in the JSON file.

- `ADDRESS_LAST_DEPLOYED`: address of the last deployed contract, reset to zero where a new test starts.
- `ADDRESS_RANDOM`: a random address, generated each time being used.

To use the pre-defined variables, use syntax `${VARIABLE_NAME}`

### Transaction result

The transaction result can be affected by various factors, and most of them are not determinable at the time of writing the tests:
- The block environment
- The transaction environment
- The pre-execution world state
- The code to execute

Transaction results are typically reflected on the following fields:
- Account state (balance, nonce, code hash, storage root)
- Transaction receipt (logs, energy usage)

**Note** that the transaction return data is not hashed within the receipt and can not retrieved via web3. To check the return data of a method, use the event/log infrastructure in solidity.

### Convention

Please use the JSON formatter provided by https://jsonformatter.org/
