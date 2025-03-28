# Chain Sourcing Lightning Nodes

## Introduction
Channels on the Lightning Network (LN) are anchored on the base layer. These turbo-charged transactions are high-speed transaction conduits whose lifecycle, from creation/opening to destruction/closing, is managed by LN nodes. The nodes have two primary responsibilities:
- The off-chain management of transaction in the conduit
- The tracking/monitoring of onchain transaction including those for channel
funding, commitment, second-stage timeout/success, and closure that show up on chain. 

To effectively do this, the LN nodes rely on a chain data sourcer that connects to block sources, like bitcoind, esplora and/or electrum servers, etc and provides notifications about onchain transaction state. This article explores the properties of such a chain data sourcer in two distinct operational modes/orientation:
- Block
- Transaction

Before proceeding, we take a short detour to establish the particular responsibilities of LN nodes with respect to the channels they manage:

Firstly, for every distinct channel a LN node (plans to) operate with a peer/counterparty (another LN node), the node must monitor all transactions related to the channel. This is crucial because each node must determine the precise moment to consider a channel open and ready for normal operations, or alternatively, breached, unresponsive, and/or closing, and requiring onchain fund claims.

<div style="text-align: center;">
<img src="./obsidian.images/chain.sourcing/chain.sourcing.channel.monitors.jpg" width=500px>
</div>

Secondly, nodes must monitor the current/instantaneous onchain fee rates for their wallets (on-/off-chain) to calculate the right transaction fees for funding channels or for claims. Additionally, nodes that offer liquidity services, i.e., Liquidity Service Providers (LSPs), require onchain monitoring for special channels, such as Just-In-Time (JIT) channels they maintain with other nodes.

These requirements uniquely identify a channel manager (offchain logic and state management), a chain monitor and its collection of channel monitors, on-/off-chain wallets, and liquidity managers as node objects/structures that observe the chain data sourcer, expecting (listening for) notifications on transactions of interest.

As previously stated, the high-level logic for chain data sourcing is two-fold. The first regularly polls the blockchain source for chain data. The second notifies/updates interested parties (wallets, chain monitor, channel manager, channel monitors) of any important updates.

## Block-oriented Chain Data Sourcing
LN nodes that plan to operate without major resource constraints may consider fetching block data from a full bitcoin node.
The collection of channel monitors, on-/off-chain wallets, and liquidity managers can `listen` for block connection/disconnection events of note on the blockchain.When these events occur, the chain data source can poll the blockchain for updates, and then update/notify the listeners interested in the block events. A typical use case for such a sourcer is the continuous synchronization of listeners to the most-Proof-of-Work (POW) chain tip. This typically involves:
1. Syncing wallets so they are aware of relevant transactions.
2. Updating the fee rate estimates so that transactions/claims can be broadcast and bumped with the right fees.
3. Broadcasting transactions.

<div style="text-align: center;">
<img src="./obsidian.images/chain.sourcing/block.oriented.chain.sourcer.2.jpg" width=500px>
</div>

<div style="margin-left:60px;">
<h3> Implementation Aside: Block-syncing with LDK </h3>
LDK, in its `lightning-block-sync` crate, provides a helpful client to keep listeners "in-sync" with the blockchain. In-sync here meaning that the listeners all have the same view of the blockchain. 

<br>

Having the same view is important because at any time T<sub><i>i</i></sub>, the state of the blockchain will change from when it was last viewed. Blocks are connected and disconnected as mining happens. The best chain tip with the most-POW chain changes over time, thus, the chain data source must synchronize all listeners to the best chain tip as it changes.

<div style="text-align: center;">
<img src="./obsidian.images/chain.sourcing/listeners.view.of.blockchain.jpg" width=500px>
</div>

To achieve this synchronization, LDK provides a lightweight Simple Payment Verification (SPV) client that polls a trusted blocksource with its chain poller (via a blocksource client), and then updates the provided listeners with its chain notifier (see [image]() below).

<div style="text-align: center;">
<img src="./obsidian.images/chain.sourcing/spv.client.jpg" width=500px>
</div>

<h3> Implementation Aside: Chain Sourcing with LDK-Node </h3>

LDK users like LDK-Node that are reliant on synchronization utilities from the library first conducts an initial one-time sync to bring all listeners to the same view of the best tip on the trusted block source, and then continuously does so the the hitherto describe SPV client, as the node runs.  With the view updating to reflect the "instantaneous" state of the blockchain, and listeners notified of changes, the chain data sourcer can update wallets with relevant transactions, compute appropriate fee rates for transactions, and broadcast transactions as need be. The image below depicts the primary objects in the sourcer (objects not overly important to this article are greyed out).

<div style="text-align: center;">
<img src="./obsidian.images/chain.sourcing/ldk.node.chain.sourcer.bitcoinrpc.jpg" width=500px>
</div>

</div>

## Transaction-oriented Chain Data Sourcing
For resource-constrained devices it is better to use a transaction-oriented Application Programming Interface (API) for chain data sourcing. The chain data sourcer should provide a `filter` interface so objects/structures can register interest in transactions or transaction outputs. In essence, subscribing to updates on the block objects when they are ready. In turn, these objects provide a `confirm`ation interface via which the chain data sourcer can notify them when there are confirmations on monitored filtered transactions.

Block sources for these kinds of operations are typically indexed blockchain servers like esplora and electrum that permit light clients to "subscribe" to transactions and/or transaction outputs, instead of blocks, as this minimizes the computational and storage requirements the clients have.

<div style="text-align: center;">
<img src="./obsidian.images/chain.sourcing/transaction.oriented.chain.sourcer.jpg" width=500px>
</div>

<div style="margin-left:60px;">
<h3>Implementation Aside: Transaction Syncing with LDK</h3>

LDK in its `lightning-sync-sync` crate, provides esplora/electrum sync clients that implement a `filter` interface with which "clients" indicate interest in; the registered transactions and outputs added to a filter queue. The sync client, processes these registrations and based on the blockchain state, e.g the addition of a new chain tip, notifies the clients of unconfirmed or confirmed transactions, bringing itself, and the confirmation clients into sync with the current best tip.
<div style="text-align: center;">
<img src="./obsidian.images/chain.sourcing/transaction.oriented.syncing.jpg" width=500px>
</div>

The chain sourcing responsibilites remain as stated [here](#L30)

</div>

## Conclusion
Chain data sourcing is a critical architecture component in LN nodes, representing the node's ability to maintain accurate real-time blockchain state. The two primary sourcing methods offer distinct approaches to solving onchain synchronization for LN nodes, both tailored to different operational requirements and constraints.

Block-oriented sourcing with enough resources (computation and memory) can leverage direct access to bitcoin nodes, and LDK's lightweight client to maintain a continuously updating view of the blockchain. Conversely, transaction-oriented sourcing for nodes on resource-constrained devices, with their sourcer's filteration interface, can rely on indexed blockchain servers, and LDK's transaction-syncing clients to monitor only specific transactions of interest.

In this article we explored how LN nodes can source for chain data considering orientations and resource constraints.

## References
1. [Lightning Development Kit (LDK)](https://github.com/lightningdevkit/rust-lightning)
2. [LDK Node](https://github.com/lightningdevkit/ldk-node)