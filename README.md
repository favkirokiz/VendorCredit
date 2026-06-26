# VendorCredit

A Soroban-based credit ledger system that digitizes the traditional “lista” notebook used by small neighborhood stores, enabling transparent and tamper-proof credit tracking on Stellar.

---

# 🧩 Problem

A sari-sari store owner in Bulacan, Philippines manually tracks customer debts using a handwritten notebook, leading to frequent errors, disputes, and lost income because outstanding balances are difficult to manage and verify accurately.

---

# 💡 Solution

VendorCredit allows store owners to register customers, record credit purchases, and track repayments through a Soroban smart contract. Every transaction is stored on Stellar, ensuring an immutable, transparent, and verifiable credit history.

---

# ⏱️ Timeline

- Day 1: Smart contract design (Soroban)
- Day 2: Contract implementation + tests
- Day 3: Frontend integration (React + Freighter wallet)
- Day 4: Testing + deployment on Stellar testnet
- Day 5: Demo preparation

---

# ⚙️ Stellar Features Used

- Soroban Smart Contracts
- XLM for transaction fees
- USDC-ready architecture (future extension)

---

# 🎯 Vision & Purpose

To modernize informal credit systems in developing economies by providing small businesses with a simple, transparent, and blockchain-powered ledger that reduces disputes and improves trust between merchants and customers.

---

# 📦 Prerequisites

- Rust (latest stable)
- Soroban CLI
- Node.js (for frontend)
- Freighter Wallet extension

Install Soroban CLI:
--bash
cargo install --locked soroban-cli

## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CDVSPIAJWAGVSZEJ33DHRZLSZZS2S5PFTMKUYBZ6P4QS2S4Y3Q7I5CX6` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CDVSPIAJWAGVSZEJ33DHRZLSZZS2S5PFTMKUYBZ6P4QS2S4Y3Q7I5CX6) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/86bb4d51d2902e5d93737abaa8b7fccd0d67bffd3e21f09c6128d4c7699deb71) |
| Deployed | 2026-06-26 08:04:07 UTC |
| Wallet | freighter (`GCFS…2LOX`) |
![Screenshot 2026-06-26 160952_1.png](./assets/Screenshot%202026-06-26%20160952_1.png)
