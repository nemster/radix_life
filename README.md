# RadixLife blueprnt

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
    Address("component_tdx_2_1cpyr294csm672ekfcyu6u9fjn8stjcma6snjpz2wdn0eef72psah9x")
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
    Address("resource_tdx_2_1nta73wetyu8jz4yn2m0femd532u3l4th7lutf645te4leqjhpmlwud")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;
CALL_METHOD
    Address("component_tdx_2_1cpyr294csm672ekfcyu6u9fjn8stjcma6snjpz2wdn0eef72psah9x")
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
    Address("component_tdx_2_1cpyr294csm672ekfcyu6u9fjn8stjcma6snjpz2wdn0eef72psah9x")
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
If the user doesn't have any in game coin, he can combine this transaction with the previous one.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1t5h2z9l3fcg74s0yqgluwft47v8ktj5u95uz3lh9w0rus838exp6cs")
    Decimal("<COIN_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1t5h2z9l3fcg74s0yqgluwft47v8ktj5u95uz3lh9w0rus838exp6cs")
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1cpyr294csm672ekfcyu6u9fjn8stjcma6snjpz2wdn0eef72psah9x")
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

### Make choice with payment

A choice can be anything.  
Some choices require a payment, most choices don't. This is the transaction to execute to make a choice that requires a payment.  
Some choices require a number to be specified, some don't.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1nta73wetyu8jz4yn2m0femd532u3l4th7lutf645te4leqjhpmlwud")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("resource_tdx_2_1t5h2z9l3fcg74s0yqgluwft47v8ktj5u95uz3lh9w0rus838exp6cs")
    Decimal("<COIN_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1t5h2z9l3fcg74s0yqgluwft47v8ktj5u95uz3lh9w0rus838exp6cs")
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1cpyr294csm672ekfcyu6u9fjn8stjcma6snjpz2wdn0eef72psah9x")
    "make_choice"
    Proof("people_proof")
    "<CHOICE>"
    Some(Bucket("coin_bucket"))
    <NUMBER>u64
;
```

`<ACCOUNT_ADDRESS>` The account address of the buyer.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the NFT.  
`<COIN_AMOUNT>` The number of coin to spend.  
`<NUMBER>` The meaning of this number depends in the choice; it can be zero if not required.  
`<CHOICE>` Is a string representing what the radixian wants to do.  

### Make choice without payment

A choice can be anything.
Some choices require a payment, most choices don't. This is the transaction to execute to make a choice that doesn't require a payment.  
Some choices require a number to be specified, some don't.

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1nta73wetyu8jz4yn2m0femd532u3l4th7lutf645te4leqjhpmlwud")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;
CALL_METHOD
    Address("component_tdx_2_1cpyr294csm672ekfcyu6u9fjn8stjcma6snjpz2wdn0eef72psah9x")
    "make_choice"
    Proof("people_proof")
    "<CHOICE>"
    None
    <NUMBER>u64
;
```

`<ACCOUNT_ADDRESS>` The account address of the buyer.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the NFT.  
`<NUMBER>` The meaning of this number depends in the choice; it can be zero if not required.  
`<CHOICE>` Is a string representing what the radixian wants to do.  

### Withdraw from bank account

This is the transaction to request a withdraw from a bank account; the bank account is managed by the backend so you can have to wait a little bit before the coins come to your wallet.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_non_fungibles"
    Address("resource_tdx_2_1nta73wetyu8jz4yn2m0femd532u3l4th7lutf645te4leqjhpmlwud")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#<NON_FUNGIBLE_ID>#"))
;
POP_FROM_AUTH_ZONE
    Proof("people_proof")
;
CALL_METHOD
    Address("component_tdx_2_1cpyr294csm672ekfcyu6u9fjn8stjcma6snjpz2wdn0eef72psah9x")
    "withdraw_from_bank_account"
    Proof("people_proof")
    Decimal("<COIN_AMOUNT>")
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
    Address("resource_tdx_2_1t5h2z9l3fcg74s0yqgluwft47v8ktj5u95uz3lh9w0rus838exp6cs")
    Decimal("<COIN_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1t5h2z9l3fcg74s0yqgluwft47v8ktj5u95uz3lh9w0rus838exp6cs")
    Bucket("coin_bucket")
;
CALL_METHOD
    Address("component_tdx_2_1cpyr294csm672ekfcyu6u9fjn8stjcma6snjpz2wdn0eef72psah9x")
    "deposit_to_bank_account"
    <NON_FUNGIBLE_ID>u64
    Bucket("coin_bucket")
;
```

`<ACCOUNT_ADDRESS>` The account containing the radixian who wants to deposit.  
`<COIN_AMOUNT>` The number of coin to deposit in the radixian's bank account.  
`<NON_FUNGIBLE_ID>` The numeric identifier of the radixian NFT.  
