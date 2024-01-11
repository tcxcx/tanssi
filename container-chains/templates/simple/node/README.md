# Foresta Node Documentation

![Foresta Banner](media/foresta-banner.png)

**An enterprise-level carbon-credit issuance and retiring platform**
</br>
🔎 For more about Foresta Protocol, head to our [website](https://www.foresta.network)</br>
📢 Follow our latest updates on [Twitter](https://twitter.com/Foresta)</br>
🤝 Engage with fellow developers on our [Discord server](https://discord.com/invite/)</br>

## Build the Foresta Node

To build the Foresta node, you will need a proper Substrate development environment.

If you need a refresher setting up your Substrate environment, see [Substrate's Getting Started Guide](https://substrate.dev/docs/en/knowledgebase/getting-started/).

```bash
# Fetch the code
git clone https://github.com/tcxcx/foresta-node.git
cd foresta-node

# Build the node (The first build will be long (~30min depending on machine specs)
cargo build -p container-chain-template-simple-node --release

# Run the node 
./target/release/container-chain-template-simple-node --dev --sealing 6000
```

## Runtime Architecture

The Tanssi Runtime is built using FRAME and consists of pallets from substrate, frontier, cumulus, and `pallets/` which already provide these key block production and collator functionality.

From substrate:

- _Balances_: Tracks token balances
- _Sudo_: Allows a privileged account to make arbitrary runtime changes - will be removed before
  launch
- _Timestamp_: On-Chain notion of time
- _Transaction Payment_: Transaction payment (fee) management
- _Authorship_: A pallet where authorship information for orchestrator is stored
- _Invulnerables_: A pallet that selects invulnerable collators to be assigned to author in container-chains and orchestrator
- _Session_: A pallet that handles session-changes and keys
- _AuthorityMapping_: A pallet that handles a mapping between collator accounts and authority keys

From cumulus:

- _ParachainSystem_: A helper to perform relay-storage verifications and injection of cross-chain messages
- _ParachainInfo_: A place to store parachain-relevant constants like parachain id

The following pallets are stored in `pallets/`. They are designed for Tanssi's specific requirements:

- _Registrar_: A pallet that stores all registered container-chains
- _Configuration_: A pallet storing the current configuration from which several other components depend
- _CollatorAssignment_: A pallet implementing collator account to orchestrator/container-chain assignment
- _AuthorityAssignment_: A pallet implementing collator authority key to orchestrator/container-chain assignment
- _Initializer_: A pallet that handles everything that happens on a session-change
- _AuthorNoting_: A pallet that stores the latest author of each of the container-chains

## Container-chain template

This repo uses the Simple Template offered within the main Tanssi repository at [Tanssi repository](https://github.com/moondance-labs/tanssi).

- _Simple template_: Which ressembles the parachain-template node from cumulus and substrate, and only basic pallet like _pallet-balances_, _parachain-system_ and basic configuration.

### Build container-chain nodes (full nodes only, not collators)

These nodes will only act as full nodes, but not as collators since these are offered by Tanssi:

```bash
# Build the simple-template node
cargo build -p container-chain-template-simple-node --release
```
