import binascii
from web3.auto import w3

with open("../DefiPrivateNetwork/keystore/UTC--2023-02-27T16-25-22.473750789Z--69a82375a3399bc233f44bbb8aa76f1a16b37b40") as keyfile:
    encrypted_key = keyfile.read()
    private_key = w3.eth.account.decrypt(encrypted_key, '12345')
    print(binascii.b2a_hex(private_key))
