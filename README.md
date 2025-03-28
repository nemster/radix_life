# RadixLife blueprint

## Transaction manifests (Stokenet)

### Buy an egg

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("<EGG_PRICE>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Bucket("xrd_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "buy_egg"
    Bucket("xrd_bucket")
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

`<ACCOUNT_ADDRESS>` The account address of the buyer.  
`<EGG_PRICE>` Price of one egg in XRD.  

### Give name

Assign a name to an NFT you own.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1nfsa03aejkcq52vl7lhvsq6aqtdlzf5crjjn3cu05phk3kfa40dek0")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "give_name"
    Proof("people_proof")
    "<NAME>"
;
```

`<ACCOUNT_ADDRESS>` The account address of the NFT owner.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the NFT.  
`<NAME>` The name to assign to the NFT.  

### Buy coins

Exchange XRD for in game coins.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("<XRD_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Bucket("xrd_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "buy_coins"
    Bucket("xrd_bucket")
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

`<ACCOUNT_ADDRESS>` The account address of the buyer.  
`<XRD_AMOUNT>` The number of XRD to exchange.  

### Buy objects

Buy one or more objects using in game coins.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Decimal("<COIN_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "buy_objects"
    Bucket("coin_bucket")
    "<OBJECT_NAME>"
    <NUMBER_OF_OBJECTS>u8
    <MORTGAGE>
    <OWNER_ID>u64
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

`<ACCOUNT_ADDRESS>` The account address of the buyer.  
`<COIN_AMOUNT>` The number of coin to spend.  
`<OBJECT_NAME>` The name of the object(s) to buy.  
`<NUMBER_OF_OBJECTS>` The number of objects to buy.  
`<MORTGAGE>` Whether to mortgage the object and pay just half its price or not (`true` or `false`).  
`<OWNER_ID>` Numeric NFT id of the owner of the object(s).  

### Buy objects with XRD

It is possible to combine the XRD -> in game coin exchange and the buy objects calls to buy objects paying in XRD.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("<XRD_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Bucket("xrd_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "buy_coins"
    Bucket("xrd_bucket")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "buy_objects"
    Bucket("coin_bucket")
    "<OBJECT_NAME>"
    <NUMBER_OF_OBJECTS>u8
    <MORTGAGE>
    <OWNER_ID>u64
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

`<ACCOUNT_ADDRESS>` The account address of the buyer.  
`<XRD_AMOUNT>` The number of XRD to exchange.  
`<OBJECT_NAME>` The name of the object(s) to buy.  
`<NUMBER_OF_OBJECTS>` The number of objects to buy.  
`<MORTGAGE>` Whether to mortgage the object and pay just half its price or not (`true` or `false`).  
`<OWNER_ID>` Numeric NFT id of the owner of the object(s).  

### Make choice with payment

A choice can be anything.  
Some choices require a payment, most choices don't. This is the transaction to execute to make a choice that requires a payment.  
Some choices require a number to be specified, some don't.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1nfsa03aejkcq52vl7lhvsq6aqtdlzf5crjjn3cu05phk3kfa40dek0")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Decimal("<COIN_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "make_choice"
    Proof("people_proof")
    "<CHOICE>"
    Some(Bucket("coin_bucket"))
    <NUMBER>u64
;
```

`<ACCOUNT_ADDRESS>` The account address owning the radixian.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the NFT.  
`<COIN_AMOUNT>` The number of coin to spend.  
`<CHOICE>` Is a string representing what the radixian wants to do.  
`<NUMBER>` The meaning of this number depends on the choice; it can be zero if not required.  

### Make choice with XRD payment

It is possible to combine the XRD -> in game coin exchange and the make choice calls to make a choice paying in XRD.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1nfsa03aejkcq52vl7lhvsq6aqtdlzf5crjjn3cu05phk3kfa40dek0")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("<XRD_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Bucket("xrd_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "buy_coins"
    Bucket("xrd_bucket")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "make_choice"
    Proof("people_proof")
    "<CHOICE>"
    Some(Bucket("coin_bucket"))
    <NUMBER>u64
;
```

`<ACCOUNT_ADDRESS>` The account address owning the radixian.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the NFT.  
`<XRD_AMOUNT>` The number of XRD to exchange.  
`<CHOICE>` Is a string representing what the radixian wants to do.  
`<NUMBER>` The meaning of this number depends on the choice; it can be zero if not required.  

### Make choice without payment

A choice can be anything.  
Some choices require a payment, most choices don't. This is the transaction to execute to make a choice that doesn't require a payment.  
Some choices require a number to be specified, some don't.

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1nfsa03aejkcq52vl7lhvsq6aqtdlzf5crjjn3cu05phk3kfa40dek0")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "make_choice"
    Proof("people_proof")
    "<CHOICE>"
    None
    <NUMBER>u64
;
```

`<ACCOUNT_ADDRESS>` The account address owning the radixian.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the NFT.  
`<CHOICE>` Is a string representing what the radixian wants to do.  
`<NUMBER>` The meaning of this number depends on the choice; it can be zero if not required.  

### Withdraw from bank account

This is the transaction to request a withdraw from a bank account; the bank account is managed by the backend so you can have to wait a little bit before the coins come to your wallet.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1nfsa03aejkcq52vl7lhvsq6aqtdlzf5crjjn3cu05phk3kfa40dek0")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "withdraw_from_bank_account"
    Proof("people_proof")
    <COIN_AMOUNT>u32
;
```

`<ACCOUNT_ADDRESS>` The account containing the radixian who wants to withdraw.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the radixian NFT.  
`<COIN_AMOUNT>` The number of coin to withdraw from the radixian's bank account.  

### Deposit to bank account

This is the transaction to deposit coins to a bank account; the bank account is managed by the backend so you can have to wait a little bit before the bank account is updated.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Decimal("<COIN_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "deposit_to_bank_account"
    <NON_FUNGIBLE_ID>u64
    Bucket("coin_bucket")
;
```

`<ACCOUNT_ADDRESS>` The account containing the radixian who wants to deposit.  
`<COIN_AMOUNT>` The number of coin to deposit in the radixian's bank account.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the radixian NFT.  

### Rent an object

It is possible for a radixian to rent an object belonging to another radixian.  
The object stays in the owner account but the `rent_to` field is updated.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1nfsa03aejkcq52vl7lhvsq6aqtdlzf5crjjn3cu05phk3kfa40dek0")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "rent"
    Proof("people_proof")
    "<OBJECT_NAME>"
    <OBJECT_ID>u64
;
```

`<ACCOUNT_ADDRESS>` The account containing the radixian who wants to rent an object.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the radixian NFT.  
`<OBJECT_NAME>` The name of the object to rent.  
`<OBJECT_ID>` The numeric ID of the object to rent.  

### Sell an object

Place an object for sale on the second-hand market.  
This method returns a receipt that can be later used to withdraw the proceeds of the sale or the object (if no one bought it).  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw_non_fungibles"
    Address("resource_tdx_2_1n2m9vnd08kvfsyl2dlk8hy5ula5745m2q4xdrkxy3slttsw5mu4rhf")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<OBJECT_ID>#"))
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1n2m9vnd08kvfsyl2dlk8hy5ula5745m2q4xdrkxy3slttsw5mu4rhf")
    Bucket("object_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "sell_object"
    Bucket("object_bucket")
    <PRICE>u32
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

`<ACCOUNT_ADDRESS>` The account containing the radixian who wants to deposit.  
`<OBJECT_ID>` The numeric ID of the object to rent.  
`<PRICE>` The price at which the object is to be sold.  

### Buy an used object

Buy an object from the second-hand market using in game coins.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Decimal("<COIN_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "buy_used_object"
    <OBJECT_ID>u64
    <OWNER_ID>u64
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

`<ACCOUNT_ADDRESS>` The account address of the buyer.  
`<COIN_AMOUNT>` The number of coin to spend.  
`<OBJECT_ID>` Numeric NFT id of the object.  
`<OWNER_ID>` Numeric NFT id of the owner of the object.  

### Buy an used object with XRD

It is possible to combine the XRD -> in game coin exchange and the buy used object calls to buy an object paying in XRD.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("<XRD_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Bucket("xrd_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "buy_coins"
    Bucket("xrd_bucket")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1tkumff7k6gs92vszzcclqe3sj76ax706uswpumeufh5f8md0c49xgx")
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "buy_used_object"
    <OBJECT_ID>u64
    <OWNER_ID>u64
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

`<ACCOUNT_ADDRESS>` The account address of the buyer.  
`<XRD_AMOUNT>` The number of XRD to exchange.  
`<OBJECT_ID>` Numeric NFT id of the object.  
`<OWNER_ID>` Numeric NFT id of the owner of the object.  

### Allow renting an object

Use this transaction to allow or disallow other users to rent an object you own.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1n2m9vnd08kvfsyl2dlk8hy5ula5745m2q4xdrkxy3slttsw5mu4rhf")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<OBJECT_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("object_proof")
;
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "allow_rent"
    Proof("object_proof")
    <ALLOW>
    Some(<DAILY_PRICE>u32)
    Some(<OWNER_ID>u64)
;
```

`<ACCOUNT_ADDRESS>` The account address of the buyer.  
`<OBJECT_ID>` Numeric NFT id of the object.  
`<ALLOW>` True to allow rent or false to disallow it.  
`<DAILY_PRICE>` The daily fee amount. `None` if disallowing the rent.  
`<OWNER_ID>` Numeric NFT ID of the owner of the object (who will receive the rent fee?). `None` if disallowing it.   

### Terminate a rent

A renter can terminate the rent contract anytime by executing this transaction manifest.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1nfsa03aejkcq52vl7lhvsq6aqtdlzf5crjjn3cu05phk3kfa40dek0")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;   
CALL_METHOD
    Address("component_tdx_2_1crkhanphyasmvgpdshsw39mg6dderhjayvdkmpzh7ssa9ald0tuucr")
    "terminate_rent"
    Proof("people_proof")
    <OBJECT_ID>u64
;
```

`<ACCOUNT_ADDRESS>` The account containing the radixian who wants to terminate the rent.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the radixian NFT.  
`<OBJECT_ID>` The numeric ID of the object to stop renting.  

