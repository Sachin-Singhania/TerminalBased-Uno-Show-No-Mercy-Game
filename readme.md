# UNO Show No Mercy Terminal Game 🎮🃏

A terminal-based multiplayer version of the classic UNO game — **with a twist**!  
This version, "Show No Mercy", includes special cards and unpredictable gameplay that takes the original UNO to the next level!

## Features ✨

- 2 to 6 Players Multiplayer Support 👨‍👩‍👧‍👦
- Special Action Cards 💥
- Terminal-based UI for classic retro fun 💻
- Chain reactions & strategic gameplay ♟️

---

## How to Play ▶️

### Setup 🧰

Each player starts with 7 cards.  
Players take turns to play cards matching either the **color** or the **type** of the top card on the discard pile.

---

### Card Types & Actions 🃏

| Card Type             | Effect Description |
|-----------------------|--------------------|
| `Number(n)` 🔢        | Normal numbered card. Match by number or color. |
| `REVERSE` 🔁         | Reverses the turn order. |
| `SKIP` ⛔            | Skips the next player's turn. |
| `DRAW2` ✌️           | Next player draws 2 cards. |
| `DRAW4` ✴️           | Next player draws 4 cards. |
| `DRAW6` 6️⃣           | Next player draws 6 cards. |
| `DRAW10` 🔟          | Next player draws 10 cards. Brutal! |
| `SKIPEVERYONE` 🚫👥   | Skips all players; you play again. |
| `DISCARDALL` 🗑️       | Discard all cards of the same color. |
| `ZEROPASS` 0️⃣        | All players pass their deck clockwise. |
| `SEVENPASS` 7️⃣      | Current player selects someone to swap decks with. |
| `WILDROULETTE` 🎡     | Keep drawing cards until a declared color is found. |
| `DRAW4REVERSE` ↩️✴️   | Draw 4 + reverse direction. |

---

## Win Condition 🏆

The first player to discard all cards wins the game.  
**Play smart. Chain better. Show no mercy.**

---

## Run the Game 🚀

Make sure you have Rust installed:  
https://www.rust-lang.org/tools/install

```bash
git clone https://github.com/Sachin-Singhania/TerminalBased--Uno-Show-No-Mercy-Game.git
cd TerminalBased--Uno-Show-No-Mercy-Game
cargo run
