# Aion Compatibility Kit

Aion Compatibility Kit (ACK) is an extensive test suite to ensure compatibility between different Aion implementations. Ideally, it should be a centralized place to check all major components and protocols:
- Chain specifications
- Virtal Machine
- P2P protocol

## Aion FastVM

There are currently two FastVM implementations; one is in Rust and the other in Java. Although the two implementations use the same core, Fastvm JIT, they may manifest different behaviour regarding kernel integration.

This test suite specifies a set of integration tests, composed of transactions, which all kernels have to pass. To run the tests,
1. Executes the transactions on a test network;
1. Let all kernels to sync (the state change and receipts check will be enforced by consensus).

### Integration test specs
```
[
  {
    "name": "The name of this test",
    "pipeline": [
      {
        "type": "Transaction type: CREATE or CALL",
        "from": "The address of the sender",
        "to": "The address of the receiver",
        "value": "The value to send, default: 0x0",
        "data": {
          "raw": "Unstructured byte array in hexdecimal, default: EMPTY_BYTES",
          "code": "Contract initialization code, default: EMPTY_BYTES",
          "method": "Pre-hash method signature, default: EMPTY_BYTES",
          "arguments": "Encoded arguments, default: EMPTY_BYTES"
        },
        "nrg": "The energy limit, default: 0xf4240",
        "nrgPrice": "The energy price, default: 0x1",
        "result": "The transaction result: SUCCESS, FAILED or REJECTED"
      }
    ]
  }
]
```

### Managed variables

There are a few system managed variables, which updates dynamically as the tests run:

- `PREMINED_ADDRESS`: the address with premined coins, for test purpose
- `LAST_DEPLOYED_ADDRESS`: the address of the most recently deployed contract
