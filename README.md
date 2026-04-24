# breath

A terminal-based guide to the breathing exercises from *Breath* by James Nestor.

## Installation

Requires [Rust](https://rustup.rs/).

```sh
git clone https://github.com/ssiewluap/breath
cd breath
cargo install --path .
```

Or run without installing:

```sh
cargo run --release
```

## Usage

Launch the app and pick an exercise by number:

```
  BREATH  ·  James Nestor
  ────────────────────────────────────────────────────

  1   Coherent Breathing                  Beginner
  2   Box Breathing                       Beginner
  3   4-7-8 Breathing                     Beginner
  4   Extended Exhale                     Beginner
  5   Nadi Shodhana                       Beginner
  6   Buteyko — Nasal Unblocking          Beginner
  7   Buteyko — Reduced Breathing         Intermediate
  8   Wim Hof — Bellows Breath            Advanced
  9   Tummo — Inner Fire                  Advanced
  10  Holotropic Breathwork               Advanced
  11  Sudarshan Kriya (SKY)               Intermediate

  Number + Enter to begin  ·  q to quit
```

Type a number, press **Enter** to start. During an exercise:

- A progress bar and countdown timer guide each phase.
- Interactive phases (e.g. breath holds) wait for **Enter** to advance.
- Press **q** or **Esc** at any time to stop and return to the menu.

## Exercises

| # | Exercise | Difficulty | Primary Effect |
|---|----------|------------|----------------|
| 1 | Coherent Breathing | Beginner | HRV, calm |
| 2 | Box Breathing | Beginner | Focus, CO₂ tolerance |
| 3 | 4-7-8 Breathing | Beginner | Relaxation, sleep |
| 4 | Extended Exhale | Beginner | Parasympathetic activation |
| 5 | Nadi Shodhana | Beginner | Hemispheric balance |
| 6 | Buteyko — Nasal Unblocking | Beginner | Clear blocked nose |
| 7 | Buteyko — Reduced Breathing | Intermediate | CO₂ tolerance |
| 8 | Wim Hof — Bellows Breath | Advanced | Energy, immune resilience |
| 9 | Tummo — Inner Fire | Advanced | Internal heat, autonomic control |
| 10 | Holotropic Breathwork | Advanced | Psychological depth |
| 11 | Sudarshan Kriya (SKY) | Intermediate | Cortisol, mood, PTSD |

See [EXERCISES.md](EXERCISES.md) for full descriptions of each technique.
