## Making Sense of LDK & LDK Node
I like to think about the architecture of software systems in a particular way, 
i.e. hexagonal architecture with clean separation between driving and driven ports, 
and with adaptable interfaces to make systems modular and components interchangeable.
This note contains the representation of LDK in this style and captures what I have
learned studying the development kit in the past few months.

**Note**: I am uncertain if this style applies in the way that I have used it but it has
been helpful in organizing my thoughts about the layout and the direction of the flow 
of data.

### Architectural Overview of LDK
LDK is composed of four major structures
- `ChannelManager` to handle off-chain channel management
- `ChainMonitor` to handle on-chain monitoring of relevant transactions
- `PeerManager` to manage peers, (dis)connecting to, and forwarding/receiving messages
- `P2PGossipSync` for the latest view of the network graph and the propagation of gossip across the network.

The architecture is requires careful management of state, is event-driven, having
structures that can generate events and messages, and those with the tools to handle
the generated events/messages.

![ldk](/obsidian.images/ldk.and.ldk.node/ldk.overview.jpg)

### LDK Node
LDK Node is the lightning node based on LDK, integrating the development kit and other pluggable/adaptable tools:
- block/transaction-sync: to sync channel and wallet operations, as well as fee rate estimation to the chain, 
- invoice generation, 
- persistence: to save channel state, 
- networking, 
- rapid gossip, and 
- background processing.

**Note**: Red dotted lines showing how LDK Node integrates LDK.

![ldk-node](/obsidian.images/ldk.and.ldk.node/ldk.and.ldk.node.2.jpg)