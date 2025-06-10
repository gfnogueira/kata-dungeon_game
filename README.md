# Dungeon Game - Rust Solution

This repository contains a Rust implementation for [Leetcode Problem #174 - Dungeon Game](https://leetcode.com/problems/dungeon-game/).

## 🧩 Problem Summary

A knight is trapped in a dungeon and must reach the princess at the bottom-right corner. Each room may have demons (negative values), be empty (0), or contain health orbs (positive values). The knight can only move **right** or **down**. You must compute the **minimum initial health** required for the knight to complete the journey without dying.

---

## 🚀 How to Run Locally

### 1. Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then check:
```bash
rustc --version
```

### 2. Clone this repository

```bash
git clone https://github.com/gfnogueira/kata-dungeon_game.git
cd kata-dungeon_game
```

### 3. Run the project
```bash
cargo run
```
You should see:
 ```bash
 Minimum initial health required: 7
 ```


---
## 📁 Project Structure

```bash
kata-dungeon_game/
├── Cargo.toml
├── src/
│   └── main.rs       # Main logic and example usage
├── .gitignore
└── README.md
```

---

## 🧠 Solution Approach

We use dynamic programming (bottom-up) to compute the minimum health needed to enter each cell, starting from the bottom-right (princess’s position) and going backward. The knight must always maintain at least 1 HP to stay alive.