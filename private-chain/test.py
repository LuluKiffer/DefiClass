
from web3 import Web3, HTTPProvider
from web3.middleware import geth_poa_middleware

web3 = Web3(HTTPProvider('http://localhost:8545'))
web3.middleware_onion.inject(geth_poa_middleware, layer=0)

receipts = []
block_number = web3.eth.blockNumber

add = "0xbE0f9935F9d8BFAC2645A24C1894F085E3A2c79D"
#add = "0x4D0f886E2029222cEE43f5a3e278420BA035388f"
#add = "0x211CCC5CcA1feC4B0d3BE84687dD21C9f8075eE3"

#add = "0x87F180f7E1Ca4F257Fc005D60C569F7083e95850"
#add = "0xd20de27BcE80B94f9b409949731650e6578D1dF4"

#add = "0x211CCC5CcA1feC4B0d3BE84687dD21C9f8075eE3"

for i in range(0, block_number + 1):
    block = web3.eth.getBlock(i, True)

    if block is not None and len(block.transactions) > 0:
        for j in range(0, len(block.transactions)):
            tx = block.transactions[j]

            #if tx.to == add:
            if tx["to"] == add or tx["from"]==add:
                receipt = web3.eth.getTransactionReceipt(tx.hash)
                receipts.append(receipt)

print(receipts)

for receipt in receipts:
    logs = receipt['logs']
    for log in logs:
        print(log)
