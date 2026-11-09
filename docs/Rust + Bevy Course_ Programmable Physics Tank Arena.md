- [From Zero Rust to a Programmable Physics Tank Arena](#from-zero-rust-to-a-programmable-physics-tank-arena)
    - [A self-directed Rust + Bevy + Avian2D course](#a-self-directed-rust--bevy--avian2d-course)
  - [0. Orientation](#0-orientation)
    - [0.1 What you will build](#01-what-you-will-build)
    - [0.2 How the course is shaped, and why](#02-how-the-course-is-shaped-and-why)
    - [0.3 Structure of every unit](#03-structure-of-every-unit)
    - [0.4 Assumptions this course makes](#04-assumptions-this-course-makes)
    - [0.5 Resuming after a break](#05-resuming-after-a-break)
  - [1. Technology Baseline](#1-technology-baseline)
    - [1.1 Pinned versions (as of September 2026)](#11-pinned-versions-as-of-september-2026)
    - [1.2 Physics engine choice: Avian2D, with justification](#12-physics-engine-choice-avian2d-with-justification)
    - [1.3 Toolchain setup (done once, in Unit 1.0)](#13-toolchain-setup-done-once-in-unit-10)
    - [1.4 Resource shelf (referenced throughout)](#14-resource-shelf-referenced-throughout)
- [PHASE 1: Rust Fundamentals (no engine)](#phase-1-rust-fundamentals-no-engine)
  - [Unit 1.0: Toolchain, Cargo, and the Edit–Check Loop](#unit-10-toolchain-cargo-and-the-editcheck-loop)
  - [Unit 1.1: Syntax Speed-Run for Polyglots](#unit-11-syntax-speed-run-for-polyglots)
  - [Unit 1.2: Ownership and Moves](#unit-12-ownership-and-moves)
  - [Unit 1.3: Borrowing, References, and Slices](#unit-13-borrowing-references-and-slices)
  - [Unit 1.4: Lifetimes Without Fear](#unit-14-lifetimes-without-fear)
  - [Unit 1.5: Structs, Enums, and Pattern Matching](#unit-15-structs-enums-and-pattern-matching)
  - [Unit 1.6: `Option`, `Result`, and Error Handling](#unit-16-option-result-and-error-handling)
  - [Unit 1.7: Traits and Generics (Static Dispatch)](#unit-17-traits-and-generics-static-dispatch)
  - [Unit 1.8: Trait Objects, `dyn`, and the `Send`/`Sync`/`'static` Trio](#unit-18-trait-objects-dyn-and-the-sendsyncstatic-trio)
  - [Unit 1.9: Closures and `Fn` / `FnMut` / `FnOnce`](#unit-19-closures-and-fn--fnmut--fnonce)
  - [Unit 1.10: Iterators and the Adapter Style](#unit-110-iterators-and-the-adapter-style)
  - [Unit 1.11: Collections and the "Mutate While Iterating" Problem](#unit-111-collections-and-the-mutate-while-iterating-problem)
  - [Unit 1.12: Modules, Crates, Workspaces, Features, and Tests](#unit-112-modules-crates-workspaces-features-and-tests)
  - [Unit 1.13: Smart Pointers and Interior Mutability (Reading Level)](#unit-113-smart-pointers-and-interior-mutability-reading-level)
  - [Bridge Units: Engine Patterns in Plain Rust](#bridge-units-engine-patterns-in-plain-rust)
  - [Unit 1.14 (Bridge A): The Game Loop and Enum State Machines](#unit-114-bridge-a-the-game-loop-and-enum-state-machines)
  - [Unit 1.15 (Bridge B): Systems Operating on Data (a Toy ECS)](#unit-115-bridge-b-systems-operating-on-data-a-toy-ecs)
  - [Unit 1.16 (Bridge C): The Terminal Bot Duel and the First `BotBrain` Contract](#unit-116-bridge-c-the-terminal-bot-duel-and-the-first-botbrain-contract)
- [PHASE 2: Introduction to Bevy](#phase-2-introduction-to-bevy)
  - [Unit 2.1: The Bevy Mental Model](#unit-21-the-bevy-mental-model)
  - [Unit 2.2: Hello Bevy: App, Schedules, Window, Camera, Sprite](#unit-22-hello-bevy-app-schedules-window-camera-sprite)
  - [Unit 2.3: Queries Meet the Borrow Checker](#unit-23-queries-meet-the-borrow-checker)
  - [Unit 2.4: Resources, Commands, Messages, and Observers](#unit-24-resources-commands-messages-and-observers)
  - [Unit 2.5: Plugins, System Ordering, Sets, Run Conditions, and States](#unit-25-plugins-system-ordering-sets-run-conditions-and-states)
  - [Unit 2.6: Transforms, Hierarchies, Sprites, Cameras, and Gizmos](#unit-26-transforms-hierarchies-sprites-cameras-and-gizmos)
  - [Unit 2.7: Input, Time, and Frame-Rate Independence](#unit-27-input-time-and-frame-rate-independence)
  - [Unit 2.8: Fixed Timestep and the Physics Boundary](#unit-28-fixed-timestep-and-the-physics-boundary)
- [PHASE 3: Capstone, a Programmable 2D Tank Combat Arena](#phase-3-capstone-a-programmable-2d-tank-combat-arena)
    - [The architecture you are building toward](#the-architecture-you-are-building-toward)
    - [Project layout](#project-layout)
  - [Unit 3.1: Skeleton, Arena, and Round States](#unit-31-skeleton-arena-and-round-states)
  - [Unit 3.2: One Manually Driven Tank (No Physics Yet)](#unit-32-one-manually-driven-tank-no-physics-yet)
  - [Unit 3.3: Integrate Avian2D: Bodies, Colliders, Walls](#unit-33-integrate-avian2d-bodies-colliders-walls)
  - [Unit 3.4: Physics-Driven Projectiles](#unit-34-physics-driven-projectiles)
  - [Unit 3.5: The `BotBrain` Contract, Version 2 (Pure Rust)](#unit-35-the-botbrain-contract-version-2-pure-rust)
  - [Unit 3.6: Brains Inside the ECS, and the Action Adapter](#unit-36-brains-inside-the-ecs-and-the-action-adapter)
  - [Unit 3.7: Collisions, Damage, Death, and `last_hit`](#unit-37-collisions-damage-death-and-last_hit)
  - [Unit 3.8: Many Bots, Round Lifecycle, and a Winner](#unit-38-many-bots-round-lifecycle-and-a-winner)
  - [Unit 3.9: Real Sensors: Radar Cone, Proximity, and Wall Rays](#unit-39-real-sensors-radar-cone-proximity-and-wall-rays)
  - [Unit 3.10: Timing, Determinism, and Headless Simulation](#unit-310-timing-determinism-and-headless-simulation)
  - [Unit 3.11: Bot Personalities and Strategy](#unit-311-bot-personalities-and-strategy)
  - [Unit 3.12 (Stretch within the Capstone): Tournaments, Instrumentation, and Balancing](#unit-312-stretch-within-the-capstone-tournaments-instrumentation-and-balancing)
  - [Unit 3.13: Capstone Completion Gate and Retrospective](#unit-313-capstone-completion-gate-and-retrospective)
- [PHASE 4: Optional Stretch Modules](#phase-4-optional-stretch-modules)
  - [Unit 4.1 (Optional): Game Feel, HUD, and Visual Polish](#unit-41-optional-game-feel-hud-and-visual-polish)
  - [Unit 4.2 (Optional): Runtime-Scripted Bots with Rhai, and a Real Lesson in Sandboxing](#unit-42-optional-runtime-scripted-bots-with-rhai-and-a-real-lesson-in-sandboxing)
    - [Why Rhai (and when to override that)](#why-rhai-and-when-to-override-that)
    - [Threat model first (the security teaching moment)](#threat-model-first-the-security-teaching-moment)
    - [Exercise](#exercise)
  - [Unit 4.3 (Optional): Doors to Your Other Interests](#unit-43-optional-doors-to-your-other-interests)
- [Appendices](#appendices)
  - [Appendix A: Compiler and Runtime Error Field Guide](#appendix-a-compiler-and-runtime-error-field-guide)
  - [Appendix B: Version-Drift Checklist (use when resuming after months away)](#appendix-b-version-drift-checklist-use-when-resuming-after-months-away)
  - [Appendix C: Go / C++ / Python → Rust and Bevy Map](#appendix-c-go--c--python--rust-and-bevy-map)
  - [Appendix D: Mastery Gates at a Glance](#appendix-d-mastery-gates-at-a-glance)

# From Zero Rust to a Programmable Physics Tank Arena

### A self-directed Rust + Bevy + Avian2D course

---

## 0. Orientation

### 0.1 What you will build

A 2D, top-down, RoboCode-style arena. Several tanks share a walled arena with real rigid-body physics (mass, momentum, collisions, fast projectiles). Nobody drives the tanks. Each tank is controlled by Rust code that implements one shared trait:

```
pub trait BotBrain: Send + Sync {
    fn name(&self) -> &str;
    fn tick(&mut self, sensors: &Sensors) -> Action;
}
```

Every simulation tick the game builds a `Sensors` snapshot for each tank (own health, heading, radar contacts, proximity warnings, who hit it last), hands it to that tank's brain, receives an `Action` (throttle, turn, turret turn, fire), validates it, and turns it into physics. Collisions become damage, damage becomes destroyed tanks, and the last tank standing wins. By the end you will be able to run tournaments between bot "personalities" and, optionally, load bots as runtime scripts.

### 0.2 How the course is shaped, and why

The capstone combines three mental models, each of which is hard on first contact:

| Layer    | The question it keeps asking you                                          |
| -------- | ------------------------------------------------------------------------- |
| Rust     | _Who owns this data, and who may borrow it right now?_                    |
| Bevy ECS | _Which systems touch which data, and in what order do they run?_          |
| Physics  | _What is simulated, at what timestep, and which system is authoritative?_ |

Most people who bounce off "Rust via Bevy" are fighting all three simultaneously and cannot tell which one produced a given error. This course separates them on purpose. Phase 1 is plain Rust with no engine. Phase 2 is Bevy with no physics and no AI. Phase 3 adds physics before AI, and AI before sensors. Every capstone unit introduces **one** major new idea. When something breaks, you'll know which layer to suspect.

There is also a continuity thread you should watch for: the pure-Rust `arena_core` crate you create in the Phase 1 bridge units (holding `Sensors`, `Action`, `BotBrain`, and a couple of brains) is the same crate the Bevy game depends on in Phase 3. The game is built _around_ the domain logic you already wrote and tested, rather than the other way round.

### 0.3 Structure of every unit

Each unit has: **Effort** (S ≈ under 1 hour, M ≈ 1–3 hours, L ≈ 3–6 hours, XL ≈ several sittings), **Prerequisites** (so you can resume after a break), **Objectives**, **Resources**, **Exercise** with **Success criteria**, **Why it works this way** (the mechanism underneath), and **Checkpoints** (2–4 questions). The Why section contains the material needed to answer the checkpoints; if you can't answer them without rereading, the concept hasn't landed yet.

Callouts marked **From Go/C++/Python** map a Rust or Bevy idea onto something you already know, and say where the analogy breaks. Callouts marked **Security note** flag places where the material touches safety, robustness, or trust boundaries, since those are areas you've said you want to strengthen.

### 0.4 Assumptions this course makes

- **Top-down arena, zero gravity.** Tanks drive on a floor seen from above; projectiles fly straight. (A side-view artillery game would need gravity and a different design; this course does not do that.)
- **Native desktop only** (Linux, macOS, or Windows), single process, no networking.
- **Plain toolchain.** `rustup` + `cargo` + an editor with rust-analyzer. No containers; GPU access and incremental compile caches are both simpler natively.
- **Tanks are simple shapes** (rectangles and circles) until the optional polish module.

### 0.5 Resuming after a break

1. Find the last unit you completed. Reread only its Objectives, Why, and Checkpoints.
2. Re-run that unit's code and confirm its success criteria still hold (`cargo test`, `cargo run`).
3. Answer its checkpoints without looking. If you can't, redo that unit's exercise quickly, not the whole phase.
4. Check the Prerequisites line of the next unit; if it names something you've forgotten, spend 15 minutes on that unit's Why section first.
5. Keep a `GOTCHAS.md` in your course folder: every compiler error that confused you for more than ten minutes, with one sentence explaining what it actually meant. By Phase 3 it becomes your most valuable personal reference.

---

## 1. Technology Baseline

### 1.1 Pinned versions (as of September 2026)

| Component           | Version                                    | Notes                                  |
| ------------------- | ------------------------------------------ | -------------------------------------- |
| Rust                | current stable (1.98.x at time of writing) | `rustup update stable` before starting |
| Edition             | 2024                                       | Default for `cargo new`                |
| Bevy                | `0.19` (0.19.1 is the latest patch)        | Released June 2026                     |
| Avian2D             | `0.7`                                      | The Avian release built for Bevy 0.19  |
| Rhai (Phase 4 only) | latest 1.x                                 | Check crates.io when you get there     |

```
[dependencies]
bevy = "0.19"
avian2d = "0.7"
```

**Why `"0.19"` and not `"=0.19.1"`?** Cargo's default requirement is a _caret_ requirement. For versions below 1.0, Cargo treats the minor number as the breaking-change boundary, so `"0.19"` means `>=0.19.0, <0.20.0`. You get bug-fix patches but never an API-breaking upgrade. Your committed `Cargo.lock` then freezes the exact resolved versions until you choose to run `cargo update`. An exact `=` pin would also block patch releases that fix bugs, which is rarely what you want.

**Check before you start, and whenever you resume after months away:**

- **Bevy 0.20 is imminent.** Its first release candidate appeared in mid-September 2026. Bevy ships breaking releases roughly every three to four months, and each needs a matching Avian release. **Stay on 0.19 for the whole course.** Upgrading is a great post-capstone exercise (see 4.3), but doing it mid-milestone mixes "learning" failures with "migration" failures.
- Verify the Bevy ↔︎ Avian pairing in the compatibility table in the Avian README before adding either dependency.
- When reading any Bevy tutorial, blog, or video, **check which Bevy version it targets.** Bevy's API has changed substantially across releases (for example, 0.17 split the old "events" into _messages_ and _observer events_, and 0.19 introduced the BSN scene system). Code from 2023 will often not compile.
- The code in this document is written against Bevy 0.19 / Avian 0.7 and labelled as **sketches**: the structure and concepts are what matter. If a name doesn't resolve, search that version's docs.rs page; renames between releases are common and the migration guides list them.

### 1.2 Physics engine choice: Avian2D, with justification

Both mature options for Bevy are viable. Here is the trade-off as it applies to _this_ learner and _this_ project.

**Avian2D** is an ECS-native physics engine written for Bevy. Its state is ordinary Bevy components (`Position`, `Rotation`, `LinearVelocity`, `Collider`...), and its pipeline is ordinary Bevy systems running in Bevy schedules. There is no separate physics world to synchronise.

**Rapier2D** (via `bevy_rapier2d`) is a mature general-purpose engine by Dimforge with an optional cross-platform determinism feature, strong performance, and a long track record. The Bevy plugin mirrors Rapier's own internal world into and out of Bevy components.

**Pick Avian here**, for three reasons:

1. **One mental model instead of two.** You'll just have learned that ECS components are the source of truth. With Avian that stays true for physics. With Rapier there are two worlds and a synchronisation layer, which introduces questions ("I changed the component; why didn't the body move yet?") that are about the plugin, not about physics or Bevy.
2. **The source is readable with Phase 2 knowledge.** When you want to know _why_ something happens, Avian's code is Bevy systems and queries. That makes it a learning resource in its own right, which suits your interest in how things work.
3. **It covers everything the capstone needs**: dynamic/static bodies, colliders, damping, collision layers, collision messages, continuous collision detection (for fast projectiles), and spatial queries (raycasts, for sensors).

What you give up: Avian is younger. Its CCD is serviceable and a Box2D-inspired overhaul is planned for 0.8. Rapier is also the better choice if you later need cross-platform bit-identical determinism (e.g. networked lockstep). None of that matters for a local arena. Nothing in the course's _architecture_ depends on Avian specifically: the "Action adapter" pattern in Unit 3.6 means swapping engines would touch one module.

### 1.3 Toolchain setup (done once, in Unit 1.0)

- Install via `rustup` (rust-lang.org/tools/install). On Windows, accept the MSVC build tools prompt.
- Editor: VS Code with **rust-analyzer**, or RustRover, or Neovim/Helix with rust-analyzer. Turn on inlay type hints; seeing inferred types and lifetimes is extremely helpful while learning.
- **Linux only:** Bevy needs system libraries for audio, input, and windowing (on Debian/Ubuntu roughly `pkg-config libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev`). Use Bevy's `docs/linux_dependencies.md` in the Bevy repo for your distro.
- Useful extras: `cargo install cargo-edit cargo-audit` (and optionally `cargo-watch` or `bacon` for continuous checking).

### 1.4 Resource shelf (referenced throughout)

**Rust**

- _The Rust Programming Language_ ("the Book"): [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/) (the current edition matches Rust 2024; note that trait objects moved to **Chapter 18** when an async chapter was added as Chapter 17, so older chapter numbers you see online may be off by one).
- _Rust by Example_: [https://doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)
- _Rustlings_ small exercises: [https://github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)
- _Rust for the Polyglot Programmer_ (Ian Jackson), a dense tour for people who already know several languages: [https://www.chiark.greenend.org.uk/\~ianmdlvl/rust-polyglot/](https://www.chiark.greenend.org.uk/~ianmdlvl/rust-polyglot/)
- _Common Rust Lifetime Misconceptions_ (pretzelhammer): [https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)
- Jon Gjengset's _Crust of Rust_ videos (YouTube, channel @jonhoo): "Lifetime Annotations", "Iterators", "Smart Pointers and Interior Mutability", "Send, Sync, and their implementors". Long, deep, excellent for "why".
- Standard library docs: [https://doc.rust-lang.org/std/](https://doc.rust-lang.org/std/) and the Cargo Book: [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/)

**Game architecture (engine-independent)**

- Robert Nystrom, _Game Programming Patterns_ (free online): [https://gameprogrammingpatterns.com/](https://gameprogrammingpatterns.com/) ; chapters "Game Loop", "Update Method", "State", "Component", "Event Queue".
- Glenn Fiedler, _Fix Your Timestep!_: [https://gafferongames.com/post/fix_your_timestep/](https://gafferongames.com/post/fix_your_timestep/)
- Catherine West, RustConf 2018 closing keynote on why Rust pushes games toward ECS (blog version): [https://kyren.github.io/2018/09/14/rustconf-talk.html](https://kyren.github.io/2018/09/14/rustconf-talk.html)

**Bevy**

- Official Learn section and Quick Start: [https://bevy.org/learn/](https://bevy.org/learn/)
- Official examples, pinned to your version: [https://github.com/bevyengine/bevy/tree/v0.19.1/examples](https://github.com/bevyengine/bevy/tree/v0.19.1/examples) (browsable with live demos at [https://bevy.org/examples/](https://bevy.org/examples/)). Bevy's maintainers treat the examples as primary documentation; they compile against the release they're tagged with.
- API docs: [https://docs.rs/bevy/0.19.1/bevy/](https://docs.rs/bevy/0.19.1/bevy/)
- Error code explanations (B0001 etc.): [https://bevy.org/learn/errors/](https://bevy.org/learn/errors/)
- Migration guides: [https://bevy.org/learn/migration-guides/](https://bevy.org/learn/migration-guides/)
- Tainted Coders Bevy guides (community, generally kept current): [https://taintedcoders.com/](https://taintedcoders.com/)
- _Unofficial Bevy Cheat Book_: [https://bevy-cheatbook.github.io/](https://bevy-cheatbook.github.io/) (conceptual explanations are still good, but much of its code targets older Bevy versions; read for ideas, not for copy-paste).

**Avian**

- Docs: [https://docs.rs/avian2d/0.7](https://docs.rs/avian2d/0.7) (the crate-level page is a real guide with a table of contents)
- Examples: in the Avian repository ([https://github.com/avianphysics/avian](https://github.com/avianphysics/avian)), under the 2D crate's `examples` folder; check out the `v0.7.0` tag.
- Release posts by the author (Joona Aalto): [https://joonaa.dev/](https://joonaa.dev/)

---

# PHASE 1: Rust Fundamentals (no engine)

**Phase goal:** reach the point where ownership, borrowing, trait, and lifetime errors are things you can _reason about_, not things you work around by adding `.clone()` until the compiler goes quiet. Syntax and control flow get little time; the new mental model gets most of it.

**Working setup for Phase 1:** one scratch package with one binary per unit, so nothing gets deleted:

```
rust-lab/
  Cargo.toml
  src/bin/u01_basics.rs
  src/bin/u02_ownership.rs
  ...
```

Run any of them with `cargo run --bin u02_ownership`. Unit 1.12 then creates the real workspace that the bridge units and the capstone grow out of.

---

## Unit 1.0: Toolchain, Cargo, and the Edit–Check Loop

**Effort:** S · **Prerequisites:** none

**Objectives**

- Install and verify `rustup`, `rustc`, `cargo`, rust-analyzer.
- Understand package vs crate vs binary; `Cargo.toml` vs `Cargo.lock`.
- Adopt the fast loop: `cargo check` while editing, `cargo clippy` before committing, `cargo fmt` always.

**Resources**

- The Book, Ch. 1 (Getting Started).
- Cargo Book, "Getting Started" and "Cargo Guide → Cargo.toml vs Cargo.lock".

**Exercise**

1. Install stable Rust. Run `rustc --version`, `cargo --version`.
2. `cargo new rust-lab`, move `main.rs` to `src/bin/u00_hello.rs`, and make it read one command-line argument (`std::env::args()`) and print a different message depending on it (including when it's missing).
3. Run `cargo check`, `cargo run --bin u00_hello -- tank`, `cargo fmt`, `cargo clippy`. Fix anything clippy suggests.
4. `git init` and commit, including `Cargo.lock`.

**Success criteria:** the program runs with and without an argument; clippy is clean; you can say in one sentence each what `rustc`, `cargo`, a _package_, and a _crate_ are.

**Why it works this way** `rustc` compiles one crate (one compilation unit, either a library or an executable). Cargo is the package manager and build orchestrator: it reads `Cargo.toml`, resolves dependency versions, writes the resolution to `Cargo.lock`, and invokes `rustc` once per crate in dependency order. A _package_ is a directory with a `Cargo.toml`; it can contain one library crate and any number of binary crates. `cargo check` runs the compiler front end (parsing, type checking, **borrow checking**) but skips code generation and linking. Since nearly all of your errors during this course will come from type and borrow checking, `check` gives you them in a fraction of the time. You commit `Cargo.lock` for applications so that "it built last month" stays true after a long break.

> **From Go:** `Cargo.toml` + `Cargo.lock` ≈ `go.mod` + `go.sum`; a crate ≈ a Go package in the sense of a compilation unit, but Rust's _modules_ (Unit 1.12) are finer-grained than Go packages and don't map to directories one-to-one.

**Checkpoints**

1. Why is `cargo check` so much faster than `cargo build`, and which kinds of errors does it still catch?
2. What belongs in `Cargo.toml` that doesn't belong in source code? What does `Cargo.lock` add?
3. Why commit `Cargo.lock` for this course's projects?

---

## Unit 1.1: Syntax Speed-Run for Polyglots

**Effort:** S · **Prerequisites:** 1.0

**Objectives** (fast, focusing only on what differs from Go/C++/Python)

- Everything-is-an-expression: blocks, `if`, and `match` produce values; the last expression without `;` is the value.
- `let` vs `let mut`; **shadowing** (`let x = x.trim();`).
- Explicit numeric types and **no implicit conversions** (`as`, `From`/`Into`); `usize` for indexing.
- Integer overflow: panics in debug builds, wraps in release (use `checked_*`/`wrapping_*`/`saturating_*` when it matters).
- Tuples, arrays `[T; N]`, ranges, `loop`/`while`/`for`, labelled `break`, `loop` returning a value.
- Macros look like functions with `!` (`println!`, `vec!`, `format!`).

**Resources**

- The Book, Ch. 3 (skim; stop on anything surprising).
- _Rust for the Polyglot Programmer_, sections on syntax and types.

**Exercise:** `u01_basics`: read whitespace-separated integers from stdin, print min, max, sum, mean (as `f64`), and a line per value saying odd/even. Use `if` as an expression at least once, shadowing at least once, and a labelled `break` to stop at the first negative number.

**Success criteria:** compiles with zero `as` casts except where you're deliberately converting integer to float; clippy clean.

**Why it works this way** Rust inherits expression orientation from ML-family languages: because blocks yield values, you rarely need uninitialised variables or ternaries, and `match` can be used anywhere a value is expected. No implicit numeric conversion exists because silent widening/narrowing is a classic source of C bugs; Rust makes you write the conversion where it happens, so it's visible in review. Debug-mode overflow panics are a deliberate compromise: they catch bugs while you develop, then are removed in release builds for speed, with explicit methods available when wrapping is actually intended.

**Checkpoints**

1. What is the value of a block whose last line ends with a semicolon?
2. How does shadowing differ from `let mut`, and when is it preferable?
3. What happens on `u8` overflow in a debug build versus a release build?

---

## Unit 1.2: Ownership and Moves

**Effort:** L · **Prerequisites:** 1.1

**Objectives**

- The three rules: every value has one owner; ownership can move; the value is dropped when its owner goes out of scope.
- Move vs `Copy` vs `Clone`; why `String`/`Vec` move but `i32`/`f32` copy.
- Stack vs heap at a useful level: a `String` is a small stack struct (pointer, length, capacity) that owns a heap buffer.
- Drop order and RAII.

**Resources**

- The Book, Ch. 4.1 ("What Is Ownership?"), read slowly, including the stack/heap discussion.
- Rust by Example, "Scoping rules → Ownership and moves".

**Exercise:** `u02_ownership`.

1. Write `fn consume(s: String) -> usize` and call it twice with the same variable. Read the full error (`E0382`, use of moved value). Write, as a comment above the call, _what_ moved, _where_ it moved to, and _who_ tried to use it afterwards.
2. Fix it three different ways: (a) have `consume` return the `String` back; (b) change it to borrow (`&str`); (c) `.clone()` at the call site. Write one sentence on the cost of each.
3. Repeat with a `Vec<i32>`, then with a plain `i32` and explain why the `i32` version compiles unchanged.
4. Define `struct Noisy(&'static str)` with an `impl Drop` that prints its name. Create several in nested scopes, move one into a function, and predict the print order before running.

**Success criteria:** your printed drop order matches your prediction; given any short function, you can predict whether each value is moved, copied, cloned, or borrowed.

**Why it works this way** Ownership is how Rust answers "who frees this memory?" at compile time. In Go and Python a garbage collector answers it at runtime by tracing reachable objects; in C++ you answer it with conventions (RAII, smart pointers) that the compiler doesn't fully enforce. Rust enforces a single owner statically. A _move_ copies the small stack part of the value (for a `String`, three machine words) to the new location and **statically marks the old binding as unusable**, so there is never a second owner who might free the heap buffer again (a double free) or read it after it's freed (use-after-free). Types whose bits _are_ the whole value and that own no resources (integers, floats, `bool`, and small structs of those) can implement `Copy`; for them, "moving" leaves the original valid because duplicating the bits is always safe. `Clone` is the explicit, potentially expensive deep copy. When the owner goes out of scope, `Drop` runs, in reverse order of declaration, which is RAII exactly as in C++, but with the compiler guaranteeing no one still holds a pointer.

> **From C++:** Rust moves are like `std::move` except (1) they're the default for non-`Copy` types, and (2) the moved-from object can't be touched at all, which the compiler checks, rather than being left in a "valid but unspecified" state.

**Checkpoints**

1. Why does `let b = a;` invalidate `a` when `a` is a `String` but not when it's an `i32`?
2. What physically gets copied during a move of a `Vec<u8>` holding a million bytes?
3. Why is `.clone()` usually _not_ the right first response to E0382?
4. In what order are local variables dropped at the end of a function, and why that order?

---

## Unit 1.3: Borrowing, References, and Slices

**Effort:** L · **Prerequisites:** 1.2

**Objectives**

- Shared references `&T` and exclusive references `&mut T`.
- The core rule: **many readers or one writer, never both at once.**
- Non-lexical lifetimes: a borrow lasts until its _last use_, not until the end of the block.
- Slices (`&str`, `&[T]`) as borrowed views; why `&str` beats `&String` in parameters.
- Recognising E0499 (two mutable borrows) and E0502 (mutable while shared).

**Resources**

- The Book, Ch. 4.2 (References and Borrowing) and 4.3 (The Slice Type).
- Rust by Example, "Scoping rules → Borrowing".

**Exercise:** `u03_borrowing`.

1. Write `fn word_count(text: &str) -> usize`, `fn longest_word(text: &str) -> Option<&str>`, and `fn normalize(text: &mut String)` (lowercase and collapse whitespace).
2. Write `fn index_of_max(xs: &[f32]) -> Option<usize>` and `fn double_all(xs: &mut [f32])`.
3. **Produce the classic C++ iterator-invalidation bug and watch Rust reject it:** take `let first = &v[0];`, then `v.push(9.0);`, then print `first`. Read E0502. Explain in a comment what would go wrong at runtime if it were allowed.
4. Move the `println!` of `first` to _before_ the `push` and observe that it now compiles. Explain why (NLL).
5. Try holding two `&mut` to the same `Vec` and read E0499.

**Success criteria:** all functions pass a couple of `assert!`s in `main`; for each error you provoked, you can explain which invariant it protects.

**Why it works this way** A reference is a pointer the compiler has proved can't outlive its referent and can't observe a data race or invalidation. The rule "shared XOR mutable" (often called _aliasing XOR mutability_) is what makes that proof possible. In the exercise, `push` may reallocate the vector's buffer; `first` would then point into freed memory. C++ compiles that and gives you undefined behaviour; Rust rejects it because a shared borrow (`first`) is alive while a mutable borrow (`push` takes `&mut self`) is requested. Since Rust 2018, the checker computes borrows from actual uses (non-lexical lifetimes), so a borrow whose last use has passed no longer blocks anything. That's why reordering the print fixes the error.

The same rule also rules out data races at compile time: a data race needs two accesses to the same memory, at least one a write, without synchronisation, and "one writer XOR many readers" can't express that. **Keep this in mind: in Phase 2, Bevy's scheduler applies exactly this rule to whole systems, to decide which ones can run in parallel.**

**Checkpoints**

1. Why can many `&T` coexist but not a `&mut T` alongside any other reference to the same value?
2. Why does moving the print before the `push` make the program compile?
3. Why is `fn f(s: &str)` more flexible than `fn f(s: &String)`?
4. How does "shared XOR mutable" also prevent data races between threads?

---

## Unit 1.4: Lifetimes Without Fear

**Effort:** M · **Prerequisites:** 1.3

**Objectives**

- Read `'a` annotations as _relationships between references_, not as instructions to keep something alive.
- Know the three elision rules well enough to predict when annotations are needed.
- Understand what `'static` means as a **bound** (`T: 'static`: "contains no non-static borrows") versus as a reference (`&'static str`: "valid for the whole program").
- Read E0106 (missing lifetime specifier) and E0597 (borrowed value doesn't live long enough).

**Resources**

- The Book, Ch. 10.3 (Validating References with Lifetimes). Stop before "Generic Type Parameters, Trait Bounds, and Lifetimes Together" if it gets dense.
- _Common Rust Lifetime Misconceptions_: read misconceptions 1–5 now; come back to the rest after Phase 2.
- Optional: Crust of Rust, "Lifetime Annotations".

**Exercise:** `u04_lifetimes`.

1. Write `fn longest(a: &str, b: &str) -> &str`, read E0106, then add `<'a>` annotations. Build a caller where `b` is a `String` in an inner block and the result is used outside that block; read E0597 and explain it.
2. Write `struct Tokenizer<'a> { src: &'a str, pos: usize }` with `fn next_token(&mut self) -> Option<&'a str>`. Note that tokens borrow from `src`, not from the tokenizer, so they can outlive a `&mut self` call. Try changing the return type to `Option<&str>` (which elision ties to `&mut self`) and observe what stops compiling in a loop that collects tokens.
3. Write `fn store<T: 'static>(x: T) -> Box<dyn std::any::Any>` and pass it a `String` (works) and a `&str` borrowed from a local `String` (fails). Explain why an owned `String` satisfies `'static` even though it certainly won't live forever.

**Success criteria:** you can explain in plain English what the `'a` in `longest` promises, and why exercise 2's `&'a str` return type is more useful than `&str`.

**Why it works this way** Lifetimes are compile-time labels for regions of code in which a reference must stay valid. An annotation doesn't change how long anything lives; it tells the compiler how the lifetimes of inputs and outputs are _related_, so it can check callers. `longest<'a>(a: &'a str, b: &'a str) -> &'a str` says "the result is valid only as long as _both_ inputs are," which is why the caller in exercise 1 is rejected. Elision rules fill in the obvious cases: each input reference gets its own lifetime; if there's exactly one input lifetime, outputs get it; if there's a `&self`/`&mut self`, outputs get `self`'s. Exercise 2 is exactly where the third rule gives the wrong relationship for your intent.

`T: 'static` means "`T` holds no borrowed references that could expire," so owned data like `String`, `Vec<f32>`, or your own structs without references all qualify, no matter how briefly they actually exist. **This bound shows up constantly in Bevy:** every component and resource must be `'static`, because the engine stores them in the `World` indefinitely and can't track borrows into your stack frames. That's one reason Bevy code rarely has explicit lifetime annotations: you store owned data and let the engine lend it to your systems.

**Checkpoints**

1. What does `'a` in `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` promise to callers?
2. Why can't marking a reference `'static` rescue a reference to a local variable?
3. Does a `String` satisfy `T: 'static`? Does a `&str` sliced from a local `String`? Why?
4. Which elision rule applies to `fn name(&self) -> &str`?

---

## Unit 1.5: Structs, Enums, and Pattern Matching

**Effort:** M · **Prerequisites:** 1.4

**Objectives**

- Structs (named, tuple, unit) and `impl` blocks: associated functions (`new`) vs methods (`&self`, `&mut self`, `self`).
- Enums with data (sum types). `match` with destructuring, guards, bindings (`x @ 1..=5`), `if let`, `let else`, `matches!`.
- Exhaustiveness and "make illegal states unrepresentable".
- Deriving common traits: `Debug`, `Clone`, `Copy`, `PartialEq`, `Default`.

**Resources**

- The Book, Ch. 5 (Structs), Ch. 6 (Enums and Pattern Matching), Ch. 19 (Patterns and Matching), skimming the latter.

**Exercise:** `u05_robot_commands`. A turn-based robot interpreter:

```
#[derive(Debug, Clone, Copy, PartialEq)]
enum Command { Move(f32), Turn(f32), Fire, Wait }

#[derive(Debug)]
enum RobotStatus { Idle, Reloading { ticks_left: u32 }, Destroyed }

struct Robot { x: f32, y: f32, heading: f32, health: i32, status: RobotStatus }
```

Implement `fn apply(&mut self, cmd: Command) -> Result<(), &'static str>` using a `match` on the **tuple** `(&self.status, cmd)`: a destroyed robot rejects everything, a reloading robot can move but not fire, and firing moves an idle robot into `Reloading { ticks_left: 3 }`. Add `fn tick(&mut self)` that counts reloads down. Then add a new variant `Command::Scan`, and see how the compiler lists every `match` you must update.

**Success criteria:** no status is represented with strings or booleans that could contradict each other (e.g. no `is_reloading: bool` alongside a separate `ticks_left`); adding a variant produces compile errors at every place that needs a decision.

**Why it works this way** A Rust enum is a _tagged union_: the value stores a discriminant plus the data for whichever variant is active, and the type system won't let you read a variant's data without checking the tag (which is what `match` does). Exhaustiveness checking means the compiler knows the complete set of cases, so adding a variant becomes a compile-time to-do list rather than a runtime surprise. This is why you'll see state machines in Rust modelled as enums rather than as flag fields: `Reloading { ticks_left }` makes it impossible for "ticks left" to exist when the robot isn't reloading.

> **From Go:** Go has no sum types; you'd emulate this with an interface and a type switch (not exhaustive) or an int constant plus separate fields (nothing stops contradictions). **From C++:** like `std::variant` + `std::visit`, but built in and ergonomic.

**Checkpoints**

1. What does "exhaustive" buy you when a new variant is added?
2. Why is an enum with data preferable to a struct with a `kind: String` field plus optional fields?
3. When would you use `if let` or `let else` instead of a full `match`?

---

## Unit 1.6: `Option`, `Result`, and Error Handling

**Effort:** L · **Prerequisites:** 1.5

**Objectives**

- `Option<T>` for "might be absent"; `Result<T, E>` for "might fail, and here's why".
- Combinators: `map`, `and_then`, `ok_or`, `unwrap_or`, `?` on both `Option` and `Result`.
- Custom error enums implementing `Display` and `std::error::Error`; `From` conversions so `?` converts errors automatically.
- When `unwrap`/`expect` are acceptable (tests, truly impossible states, prototype code) and when they're not.
- Awareness of `thiserror` (for library error types) and `anyhow` (for application-level errors).

**Resources**

- The Book, Ch. 9 (Error Handling).
- Rust by Example, "Error handling" (especially "Wrapping errors").

**Exercise:** `u06_config`. Parse a simulation config:

```
# arena config
arena_width = 800
arena_height = 600
max_tanks = 8
tick_rate_hz = 60
```

Return `Result<ArenaConfig, ConfigError>` where

```
#[derive(Debug)]
enum ConfigError {
    Io(std::io::Error),
    Malformed { line: usize, text: String },
    BadNumber { key: String, source: std::num::ParseIntError },
    Duplicate(String),
    Missing(&'static str),
}
```

Implement `Display`, `std::error::Error` (with `source()`), and `From<std::io::Error>` and use `?` for all propagation. Blank lines and `#` comments are ignored. Write one test per error variant. Then rewrite it using `thiserror` and compare the amount of code.

**Success criteria:** no `unwrap` outside tests; every malformed input yields a specific, human-readable error that names the line or key.

**Why it works this way** Rust has no null and no exceptions. Absence and failure are ordinary values in the return type, so the signature tells the caller everything that can happen, and the compiler forces them to handle it (a `Result` you ignore produces a warning; you can't use the `T` inside without dealing with the `E`). The `?` operator is sugar for "if this is `Err(e)`, return `Err(From::from(e))` from the current function; otherwise unwrap the `Ok`." The `From::from` step is why implementing `From<io::Error> for ConfigError` lets `?` convert errors automatically.

> **From Go:** `?` is `if err != nil { return nil, err }` done for you, with typed errors and automatic wrapping. Unlike Go, you can't accidentally use the value when there was an error. **From Python:** it's exceptions, except that the possible errors are part of the function's type and propagation is visible at each `?`.

**Bevy connection:** since 0.16, Bevy systems may return `Result`, so `let tank = query.single()?;` inside a system is idiomatic. What you learn here carries straight over.

**Checkpoints**

1. Roughly what does `let v = parse(x)?;` expand to?
2. When is `Option<T>` the right return type and when is `Result<T, E>`?
3. Why implement `From<io::Error> for ConfigError`?
4. Name one situation where `expect("...")` is a reasonable choice.

---

## Unit 1.7: Traits and Generics (Static Dispatch)

**Effort:** L · **Prerequisites:** 1.6

**Objectives**

- Define and implement traits; default methods; implementing std traits (`Display`, `Default`, `From`).
- Generic functions and structs with trait bounds; `where` clauses; `impl Trait` in argument and return position.
- Associated types (e.g. `Iterator::Item`) at a reading level.
- Monomorphisation: what the compiler actually generates.
- Coherence / the orphan rule, briefly.

**Resources**

- The Book, Ch. 10.1 (Generic Data Types) and 10.2 (Traits).
- Rust by Example, "Traits" and "Generics".

**Exercise:** `u07_traits`.

1. Define `trait Scorer { fn score(&self, distance: f32, health: f32) -> f32; }` with two implementations (`PreferClose`, `PreferWeak`) and a default method `fn describe(&self) -> String`.
2. Write `fn best_target<S: Scorer>(scorer: &S, targets: &[(f32, f32)]) -> Option<usize>`.
3. Write a generic `struct Ring<T> { items: Vec<T>, cap: usize }` with `push` (evicting the oldest item) and a method `fn average(&self) -> f32 where T: Into<f32> + Copy` that only exists for suitable `T`.
4. Implement `Display` for one of your types and use it in `println!("{}", x)`.

**Success criteria:** `best_target` works with both scorers without any runtime type checks; `Ring<String>` compiles but calling `.average()` on it doesn't, and you can explain the error message.

**Why it works this way** A trait is a named set of capabilities; a trait bound is a requirement that a type provide them. Generic functions are compiled by **monomorphisation**: the compiler stamps out a separate copy of `best_target` for each concrete `S` you use, with the method calls resolved statically and usually inlined. You get abstraction with no runtime cost, at the price of more compiled code and longer compile times (a big part of why Bevy compiles slowly: it's heavily generic). Methods in a `where`-constrained `impl` exist only for types meeting the bound, which is how `Ring<String>::average` can be absent without any runtime check.

> **From Go:** Go interfaces are satisfied _implicitly_ (structurally) and calls through them are always dynamic. Rust traits are implemented _explicitly_ (`impl Trait for Type`), and with generics the calls are static. Go generics (1.18+) are implemented with a mix of dictionaries and stenciling; Rust fully monomorphises. **From C++:** generics are like templates, except that bounds are checked at the definition, so errors show up where you wrote the bound, not ten layers deep in an instantiation (roughly what C++20 concepts aim for).

**Checkpoints**

1. What does monomorphisation mean for runtime speed, binary size, and compile time?
2. What does the bound in `fn f<T: Scorer>(t: &T)` let the body do that it otherwise couldn't?
3. Why can't you `impl Display for Vec<MyType>` directly? (Hint: orphan rule.)

---

## Unit 1.8: Trait Objects, `dyn`, and the `Send`/`Sync`/`'static` Trio

**Effort:** L · **Prerequisites:** 1.7

**Objectives**

- `Box<dyn Trait>` and `&dyn Trait`: dynamic dispatch through a vtable.
- When to choose `dyn` (heterogeneous collections, plugin-style extensibility, smaller code) vs generics (performance, one concrete type at a time).
- **Dyn compatibility** (formerly "object safety"): which traits can be used as `dyn`, and why.
- Supertraits (`trait BotBrain: Send + Sync`).
- What `Send` and `Sync` mean and why engines that run code on thread pools require them.

**Resources**

- The Book, **Ch. 18.2** ("Using Trait Objects That Allow for Values of Different Types").
- The Book, Ch. 16.4 ("Extensible Concurrency with the Send and Sync Traits").
- Rust Reference, "Trait objects" and "Dyn compatibility" sections.
- Optional: Crust of Rust, "Send, Sync, and their implementors".

**Exercise:** `u08_brains`. The first appearance of the capstone's central abstraction:

```
#[derive(Debug, Clone, Default)]
struct RobotView { enemy_distance: Option<f32>, enemy_bearing: f32, health: f32 }

trait RobotBrain {
    fn name(&self) -> &str;
    fn decide(&mut self, view: &RobotView) -> Command; // Command from Unit 1.5
}
```

1. Implement `Aggressive`, `Cautious`, and `Random` (the last with a tiny xorshift RNG struct you write yourself, holding its own state, so `decide` genuinely needs `&mut self`).
2. Store all three in `Vec<Box<dyn RobotBrain>>` and call `decide` in a loop.
3. Write the same loop generically (`fn run<B: RobotBrain>(b: &mut B)`) and explain why you _can't_ put three different `B`s in one `Vec<B>`.
4. Add a method `fn clone_brain<T>(&self) -> T;` with a generic parameter and read the dyn-compatibility error. Remove it.
5. Add `use std::rc::Rc; use std::cell::RefCell;` and give `Cautious` a field `log: Rc<RefCell<Vec<String>>>`. Then write `fn needs_send<T: Send>(_: T) {}` and call it with a `Box<dyn RobotBrain + Send>` built from `Cautious`. Read the error, then fix it by using `Arc<Mutex<...>>` instead. This error will look familiar in Unit 3.6.

**Success criteria:** the heterogeneous `Vec` works; you can draw the memory layout of a `Box<dyn RobotBrain>` (two pointers: data and vtable); you can explain why `Rc` is not `Send`.

**Why it works this way** A generic `Vec<T>` holds values of _one_ type, all the same size, laid out contiguously. To hold different types, you store pointers to them, and to call methods without knowing the concrete type you need a table of function pointers for that type's implementation: the **vtable**. `Box<dyn RobotBrain>` is a _fat pointer_: one pointer to the heap data, one pointer to the vtable for its concrete type. A call is an indirect jump through the vtable, so it can't be inlined, but code is generated only once. Dyn compatibility rules exist because a vtable must contain a finite, fixed set of concrete function pointers: a generic method would need infinitely many entries (one per possible `T`), and a method returning `Self` by value can't work when the size of `Self` is unknown.

`Send` means "safe to move to another thread"; `Sync` means "safe to share `&T` between threads." They are _auto traits_: the compiler implements them for your type automatically if all its fields have them. `Rc` isn't `Send` because its reference count is updated non-atomically, so two threads could corrupt it; `Arc` uses atomic counts. **Bevy runs your systems on a thread pool and can hand components to any thread,** so every component and resource must be `Send + Sync + 'static`. That's why the capstone's trait will be declared `trait BotBrain: Send + Sync` and stored as `Box<dyn BotBrain>`.

> **From Go:** a Go interface value is exactly a fat pointer (type/method table + data pointer). `dyn Trait` is the same machinery made explicit and opt-in. Go has no compile-time equivalent of `Send`/`Sync`; data races are found at runtime by `-race`, if at all.

**Checkpoints**

1. What are the two pointers inside a `Box<dyn RobotBrain>`?
2. Give one reason to prefer `dyn` and one reason to prefer generics.
3. Why can't a dyn-compatible trait have a generic method?
4. Why does storing an `Rc<RefCell<..>>` inside your brain make it unusable as a Bevy component?

---

## Unit 1.9: Closures and `Fn` / `FnMut` / `FnOnce`

**Effort:** M · **Prerequisites:** 1.8

**Objectives**

- Closure syntax and capture; `move` closures.
- The three closure traits and how the compiler chooses among them.
- Storing closures: `impl Fn`, `Box<dyn FnMut>`, generics.

**Resources**

- The Book, Ch. 13.1 (Closures).
- Rust by Example, "Closures" (especially "Capturing" and "As input parameters").

**Exercise:** `u09_rules`. Build a priority rule engine:

```
type Rule = Box<dyn FnMut(&RobotView) -> Option<Command>>;
```

1. Create rules: "if health \< 20, turn away" (captures a threshold by value), "fire when enemy within range" (captures a `range` variable), and "every 10th call, scan" (captures and **mutates** a counter, so it must be `FnMut`).
2. Put them in a `Vec<Rule>`; for each `RobotView`, return the first `Some`.
3. Write a closure that moves a `String` out of its environment (e.g. returns it) and try calling it twice. Read the `FnOnce` error.
4. Write `fn apply_n<F: FnMut()>(n: usize, mut f: F)` and observe what happens when you pass a closure that only reads (it still works; explain why).

**Success criteria:** the rule engine returns the expected commands; you can state, for each closure you wrote, which traits it implements and why.

**Why it works this way** Each closure is compiled into an anonymous struct holding its captures, plus an implementation of one or more call traits. The compiler picks the capture mode per variable from how the body uses it (by shared reference if it only reads, by mutable reference if it mutates, by value if it moves out), and `move` forces capture by value. The traits then follow: `Fn` if the body only reads captures (callable any number of times, even concurrently), `FnMut` if it mutates them (callable repeatedly but needs exclusive access), `FnOnce` if it consumes them (callable once). They form a hierarchy (every `Fn` is also `FnMut`, and every `FnMut` is also `FnOnce`), which is why a read-only closure can be passed where `FnMut` is required.

**Bevy connection:** Bevy systems are plain functions (or closures) whose parameters describe the data they need. Bevy uses trait implementations on function types to turn `fn move_tanks(q: Query<&mut Transform>)` into a scheduled system. This is the same idea of "the type of a callable encodes what it's allowed to touch."

**Checkpoints**

1. What is actually generated when you write a closure that captures two variables?
2. Why is a closure that increments a captured counter `FnMut` and not `Fn`?
3. Why can a closure that returns a captured `String` by value be called only once?

---

## Unit 1.10: Iterators and the Adapter Style

**Effort:** M · **Prerequisites:** 1.9

**Objectives**

- The `Iterator` trait (`next`, associated `Item`); laziness.
- `iter()` (yields `&T`), `iter_mut()` (yields `&mut T`), `into_iter()` (yields `T`, consuming the collection).
- Adapters (`map`, `filter`, `filter_map`, `enumerate`, `zip`, `take_while`) and consumers (`collect`, `sum`, `fold`, `find`, `any`, `all`, `min_by`, `max_by`).
- Comparing floats: why `max()` doesn't work on `f32`, and `max_by(|a, b| a.total_cmp(b))`.
- Implementing `Iterator` for your own type.

**Resources**

- The Book, Ch. 13.2–13.4 (Iterators, and the performance comparison with loops).
- `std::iter::Iterator` docs; skim the method list so you know what exists.
- Optional: Crust of Rust, "Iterators".

**Exercise:** `u10_radar`. Given

```
#[derive(Debug, Clone, Copy)]
struct Contact { id: u32, distance: f32, bearing: f32, health: f32, team: u8 }
```

write iterator pipelines (no index loops) for: nearest enemy (`team != my_team`); all enemies within range sorted by distance; weakest enemy in a firing cone of ±10°; whether any enemy is closer than 50; mean enemy health. Then implement a `ScanArc` iterator that yields bearings from `start` to `end` in fixed steps.

**Success criteria:** each query is one expression; you've used `total_cmp` for float ordering; your `ScanArc` works in a `for` loop.

**Why it works this way** Iterator adapters are lazy structs wrapping other iterators. Nothing runs until a consumer (`collect`, `sum`, a `for` loop) calls `next()`. Because each adapter is a concrete generic type, monomorphisation plus inlining typically compiles a chain like `filter().map().sum()` into the same tight loop you'd write by hand; that's the "zero-cost abstraction" claim, and The Book's Ch. 13.4 shows the benchmark. Floats implement only `PartialOrd` rather than `Ord` because `NaN` isn't comparable to anything, so `max()` (which needs a total order) is unavailable; `total_cmp` defines a total order that includes NaN, and you choose it explicitly.

**Bevy connection:** a Bevy `Query` is iterated exactly like this (`for (a, b) in &query`, `query.iter().filter(...)`), and sensor code in Phase 3 is essentially this exercise.

**Checkpoints**

1. Why does `v.iter().map(f)` do no work by itself?
2. What's the difference in ownership between `for x in &v`, `for x in &mut v`, and `for x in v`?
3. Why can't you call `.max()` on an iterator of `f32`, and what do you do instead?

---

## Unit 1.11: Collections and the "Mutate While Iterating" Problem

**Effort:** M · **Prerequisites:** 1.10

**Objectives**

- `Vec<T>`, `HashMap<K, V>`, `HashSet<T>`, `BTreeMap<K, V>`, `VecDeque<T>`: when each fits.
- The `entry` API (`*map.entry(k).or_insert(0) += 1`).
- Borrowing elements out of collections; why you can't hold `&mut` to two elements at once, and the escape hatches (`split_at_mut`, indices, two-phase updates).
- `HashMap` iteration order is not stable between runs.

**Resources**

- The Book, Ch. 8 (Common Collections).
- `std::collections` module docs, which include a good "when to use which collection" section.

**Exercise:** `u11_stats`.

1. Tournament statistics: a `HashMap<String, BotStats>` keyed by bot name, where `BotStats` has wins, losses, shots_fired, shots_hit, damage_dealt. Use `entry` to update it from a list of match events.
2. **The two-tanks problem:** given `tanks: Vec<Tank>` and a list of hits `(attacker_idx, target_idx, damage)`, apply each hit so that the target loses health _and_ the attacker's `damage_dealt` increases. First try `let a = &mut tanks[i]; let t = &mut tanks[j];` and read E0499. Then fix it two ways: (a) `split_at_mut` or indexing one at a time in sequence; (b) two phases: first compute a `Vec` of changes while only reading, then apply them.
3. Print your `HashMap` a few times across separate runs and notice the order changes. Switch to `BTreeMap` and notice it doesn't.

**Success criteria:** both fixes to the two-tanks problem work; you can explain why the naive version is rejected even when `i != j`.

**Why it works this way** `tanks[i]` goes through `IndexMut::index_mut(&mut self, i)`, which borrows the **whole** vector mutably; the compiler can't prove `i != j` from indices known only at runtime, so it refuses two simultaneous whole-vector mutable borrows. `split_at_mut` is a standard-library function (implemented with a small amount of carefully reviewed `unsafe`) that proves disjointness by construction. The two-phase approach (read everything, compute changes, then write) sidesteps the problem entirely and is the pattern you'll use most in ECS code: **Bevy's `Commands` and messages are a formalised two-phase update.**

`HashMap` uses a randomly seeded SipHash by default so that attackers can't craft keys that all collide and degrade the map to linear time (a "HashDoS" attack). **Security note:** this is a real-world defence baked into the standard library, and its side effect (unpredictable iteration order) will matter again in Unit 3.10 when you want reproducible simulations.

**Checkpoints**

1. Why does the compiler reject `&mut v[i]` and `&mut v[j]` alive at the same time, even if `i != j`?
2. Describe the two-phase update pattern and why it avoids borrow conflicts.
3. Why is `HashMap` iteration order different between runs, and when does that matter?

---

## Unit 1.12: Modules, Crates, Workspaces, Features, and Tests

**Effort:** L · **Prerequisites:** 1.7–1.11

**Objectives**

- Module tree: `mod`, files and folders, `pub`, `pub(crate)`, `use`, `crate::`, `super::`, re-exports (`pub use`).
- Library vs binary crates; a **workspace** with a shared `Cargo.lock` and `target/`.
- Cargo **features** (optional functionality), `[dev-dependencies]`, profiles.
- Unit tests (`#[cfg(test)] mod tests`), integration tests (`tests/`), doc tests.
- Reading dependency docs on docs.rs.

**Resources**

- The Book, Ch. 7 (Packages, Crates, and Modules) and Ch. 11 (Writing Automated Tests); Ch. 14.3 (Cargo Workspaces).
- Cargo Book, "Workspaces" and "Features".

**Exercise:** create the workspace that the rest of the course grows from:

```
arena/
  Cargo.toml            # [workspace] members = ["crates/*"], resolver = "3"
  crates/
    arena_core/         # library: domain types + brains, no engine
      src/lib.rs
      src/command.rs
      src/brain.rs
      src/brains/{mod.rs, aggressive.rs, cautious.rs}
    arena_cli/          # binary: depends on arena_core via path
      src/main.rs
```

Move your Unit 1.5 `Command` and Unit 1.8 brains into `arena_core`, exposing a small public API through `pub use` in `lib.rs`; keep helpers `pub(crate)`. Add a feature `serde` to `arena_core` that, when enabled, derives `Serialize`/`Deserialize` on your types (optional dependency on `serde`). Write unit tests for each brain (given this view, expect this command) and one integration test in `arena_core/tests/`. Run `cargo test --workspace` and `cargo test -p arena_core --features serde`.

**Success criteria:** `arena_cli` can only use what `arena_core` exports; all tests pass with and without the feature; you can explain what goes in the workspace root `Cargo.toml` versus member manifests.

**Why it works this way** The crate is the unit of compilation and of privacy-by-default: items are private unless marked `pub`, and a module can see its ancestors' private items but not its children's. Splitting domain logic into a library crate with no engine dependency buys you three things you'll rely on in Phase 3: brains can be unit-tested in milliseconds without starting Bevy; compile times stay low while iterating on bot logic (changing `arena_core` doesn't recompile Bevy); and the boundary makes it _structurally impossible_ for bot code to reach into engine internals. Features are additive compile-time switches (Cargo unifies them across the dependency graph, so they must only add functionality, never remove it); Bevy and Avian both use features heavily, which is why you'll see them in their docs.

**Checkpoints**

1. What's the difference between a package, a crate, and a module?
2. Why must Cargo features be additive?
3. What does putting `BotBrain` in a crate with no Bevy dependency guarantee about bot code?
4. Where do unit tests live versus integration tests, and what can each access?

---

## Unit 1.13: Smart Pointers and Interior Mutability (Reading Level)

**Effort:** M · **Prerequisites:** 1.8, 1.11

**Objectives**

- `Box<T>` (single-owner heap), `Rc<T>`/`Arc<T>` (shared ownership), `RefCell<T>`/`Mutex<T>`/`RwLock<T>` (interior mutability), atomics.
- Understand the trade: interior mutability moves borrow checking from compile time to runtime (`RefCell` panics on violation; `Mutex` blocks).
- Know _why_ idiomatic Bevy code rarely uses these, so you recognise it as a smell when you reach for them.

**Resources**

- The Book, Ch. 15 (Smart Pointers), sections 15.1, 15.4, 15.5; Ch. 16.3 (Shared-State Concurrency).
- Optional: Crust of Rust, "Smart Pointers and Interior Mutability".

**Exercise:** `u13_shared`. Build a tiny "shared scoreboard" three ways: (1) pass `&mut Scoreboard` to each function that updates it; (2) `Rc<RefCell<Scoreboard>>` shared by several `Bot` structs, then deliberately trigger a `BorrowMutError` panic by holding a `borrow()` while calling `borrow_mut()`; (3) `Arc<Mutex<Scoreboard>>` updated from four `std::thread::scope` threads. Write a short comparison: which catches mistakes at compile time, which at runtime, and which could deadlock?

**Success criteria:** you've seen a runtime borrow panic and can explain why the compiler couldn't catch it.

**Why it works this way** Sometimes shared ownership or mutation-through-a-shared-reference is genuinely needed (graphs, caches, cross-thread state). `RefCell` keeps a borrow counter and enforces the same "shared XOR mutable" rule at runtime; `Mutex` enforces exclusive access by blocking. Both work, but you lose compile-time guarantees and gain runtime failure modes. An ECS avoids most of this by design: data lives in one place (the `World`), and the _scheduler_ hands each system exactly the borrows it declared. This moves the "who can mutate what" question back to something checked before the frame runs. When you're tempted to put an `Arc<Mutex<_>>` in a Bevy component, the right answer is usually a resource, a message, or a different split of components.

**Checkpoints**

1. What does `RefCell` check, and when?
2. Why is `Arc<Mutex<T>>` thread-safe while `Rc<RefCell<T>>` is not?
3. Why does an ECS reduce the need for interior mutability?

---

## Bridge Units: Engine Patterns in Plain Rust

These three units deliberately build the _shapes_ Bevy will formalise (a loop, a state machine, data-plus-systems, and a brain contract) with nothing but the standard library. Do them inside the `arena/` workspace as new binaries in `arena_cli` (e.g. `src/bin/`), depending on `arena_core`.

---

## Unit 1.14 (Bridge A): The Game Loop and Enum State Machines

**Effort:** M · **Prerequisites:** 1.12

**Objectives**

- Structure a program as _input → update → render_ repeated.
- Separate a fixed simulation step from wall-clock time (first contact with fixed timestep).
- Model program phases as an enum state machine with `match`-driven transitions.

**Resources**

- _Game Programming Patterns_: "Game Loop" and "State".
- _Fix Your Timestep!_ (Fiedler), read now for concepts, again in Unit 2.8.

**Exercise:** `arena_cli/src/bin/loop_demo.rs`. A terminal "round" runner:

```
enum Phase { Countdown { ticks_left: u32 }, Running { tick: u64 }, Finished { winner: Option<u32> } }
```

- The loop runs at a fixed simulation rate of 10 ticks/sec using an **accumulator**: measure elapsed real time with `std::time::Instant`, add it to an accumulator, and run `update()` as many whole steps as fit, carrying the remainder. Render (print one status line) once per loop iteration.
- `update` is a function `fn update(phase: Phase, world: &mut World) -> Phase` that consumes the old phase and returns the next one, using `match`.
- Simulate a slow frame by sleeping 350 ms once; observe that the simulation catches up with several updates rather than slowing down or skipping.

**Success criteria:** the round passes through all three phases; tick count after N real seconds is ≈ 10·N regardless of the artificial stall.

**Why it works this way** Separating _simulation time_ from _real time_ is the key idea. If each update advanced the simulation by "however long the last frame took," results would depend on machine speed and frame hiccups; physics in particular becomes unstable with large, variable steps. The accumulator converts real elapsed time into a whole number of constant-size simulation steps. Taking `Phase` by value and returning a new `Phase` makes illegal transitions hard to write and ensures data belonging to a finished phase (like `ticks_left`) is gone once it's over. Bevy's `States` (Unit 2.5) and `FixedUpdate` (Unit 2.8) are these two ideas, built into the engine.

**Checkpoints**

1. What problem does the accumulator solve that `sleep(100ms)` between updates doesn't?
2. Why might `update` run zero times in one loop iteration and three times in another?
3. What does consuming and returning `Phase` prevent?

---

## Unit 1.15 (Bridge B): Systems Operating on Data (a Toy ECS)

**Effort:** L · **Prerequisites:** 1.14, 1.11

**Objectives**

- Model entities as IDs and components as separate storages ("struct of arrays").
- Write behaviour as free functions ("systems") that declare what they read and write through their parameters.
- Experience the borrow problems that motivate Bevy's design, and solve them by hand.

**Resources**

- _Game Programming Patterns_: "Component" and "Update Method".
- Catherine West's RustConf 2018 talk/blog (see shelf). **This is the single best explanation of why Rust pushes game code toward ECS.** Read it now.

**Exercise:** `arena_cli/src/bin/toy_ecs.rs`.

```
type Entity = usize;
struct World {
    alive: Vec<bool>,
    position: Vec<Option<(f32, f32)>>,
    velocity: Vec<Option<(f32, f32)>>,
    health: Vec<Option<i32>>,
    projectile: Vec<Option<ProjectileData>>, // owner: Entity, damage: i32
}
```

Write systems as free functions, each taking only the fields it needs:

- `fn movement(pos: &mut [Option<(f32,f32)>], vel: &[Option<(f32,f32)>], dt: f32)`
- `fn projectile_hits(pos: &[...], proj: &[...], health: &[...]) -> Vec<HitEvent>` (read-only; returns _events_ describing hits)
- `fn apply_damage(health: &mut [...], hits: &[HitEvent])`
- `fn despawn_dead(world: &mut World, dead: &[Entity])` (structural change, done last)

Call them from a `tick(world: &mut World)` in order, borrowing different fields of `World` in the same statement (e.g. `movement(&mut world.position, &world.velocity, dt)`). Then try the object-oriented alternative for comparison: give a `Tank` struct a method `fn fire_at(&mut self, other: &mut Tank)` and try to call it on two elements of one `Vec<Tank>`. Note how quickly this runs back into Unit 1.11's problem.

Finally, try **spawning a projectile from inside a loop that iterates over positions** (pushing to the vectors while iterating them). Read the error, then fix it by collecting "spawn requests" in a `Vec` and applying them after the loop.

**Success criteria:** projectiles move, hit, and remove health; dead entities are removed after all systems run; you did not need `Rc`, `RefCell`, or `unsafe`.

**Why it works this way** Rust lets you mutably borrow _different fields_ of a struct at the same time, because the compiler can see they're disjoint (this is called _borrow splitting_). Storing each component type in its own collection turns "one tank mutating another tank" (two `&mut Tank` into one `Vec`) into "a system reading one column and writing another" (disjoint borrows of different fields), which the borrow checker accepts. Behaviour becomes free functions whose signatures state exactly what they touch. Structural changes (spawning, despawning) would invalidate iteration of the very vectors being iterated, so they're queued and applied afterward.

You have now hand-built the four things Bevy gives you: **components** (the columns), **queries** (the borrowed columns a system asks for), **commands** (queued structural changes), and **messages** (the `HitEvent` list passed from one system to the next). What Bevy adds on top: automatic storage (archetype tables, not `Vec<Option<T>>` with holes), a scheduler that runs non-conflicting systems in parallel, change detection, and generation-checked entity IDs so that a stale ID can't silently refer to a new entity that reused the slot.

**Checkpoints**

1. Why does `movement(&mut world.position, &world.velocity, dt)` compile while `fire_at(&mut tanks[0], &mut tanks[1])` doesn't?
2. Why must despawning happen after the other systems rather than inside them?
3. What's the danger of reusing an entity index after despawn, and how could you guard against it?
4. Which Bevy concept corresponds to your `Vec<HitEvent>`?

---

## Unit 1.16 (Bridge C): The Terminal Bot Duel and the First `BotBrain` Contract

**Effort:** L · **Prerequisites:** 1.15, 1.8

**Objectives**

- Design the sensor → brain → action contract the capstone will use, in pure Rust.
- Keep decision logic ignorant of world storage.
- Validate untrusted outputs at a boundary.

**Resources:** none new. Reread your Unit 1.8 notes.

**Exercise:** in `arena_core`, add a `bot` module containing the _first version_ of the capstone contract:

```
pub struct BotId(pub u32);

pub struct Sensors {
    pub tick: u64,
    pub own_health: f32,
    pub own_heading: f32,            // radians
    pub contacts: Vec<Contact>,      // enemies currently visible
    pub last_hit_by: Option<BotId>,
}
pub struct Contact { pub id: BotId, pub distance: f32, pub bearing: f32 } // bearing relative to own heading

pub struct Action { pub throttle: f32, pub turn: f32, pub fire: bool }   // throttle, turn in [-1, 1]

pub trait BotBrain: Send + Sync {
    fn name(&self) -> &str;
    fn tick(&mut self, sensors: &Sensors) -> Action;
}
```

(Derive `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` as appropriate.)

Note the change from Unit 1.5's `enum Command` to `struct Action`: a tank should be able to drive, turn, _and_ fire in the same tick, and an enum allows exactly one of those per tick. That's a real design decision, and you'll see it again when we discuss RoboCode-style blocking vs non-blocking bot APIs.

In `arena_cli`, build a 2D terminal duel (a plain `Vec` of tank structs is fine here; keep the toy ECS for learning, not for this): each tick, **build a `Sensors` snapshot for each tank from the world state**, call `brain.tick(&sensors)`, **sanitise the returned `Action`** (clamp throttle and turn to \[-1, 1\], replace NaN or infinity with 0, enforce a fire cooldown the brain can't bypass), apply movement, resolve instant-hit shots with a simple cone check, and print a status line. End when one tank is at zero health or after 2,000 ticks. Port two brains to the new trait and add unit tests (`Sensors` in → expected `Action` out).

**Success criteria:** a full match runs to completion; the brains never see the `Vec` of tanks or any other tank's struct; a deliberately broken brain that returns `f32::NAN` throttle and fires every tick cannot move the tank or bypass the cooldown.

**Why it works this way** The `Sensors → Action` boundary is an _information boundary_ (the brain knows only what the game chooses to expose) and a _capability boundary_ (the brain can only request things the game knows how to validate). Snapshotting sensors as **owned data** instead of references into the world means the brain holds no borrows, so it can't interfere with world updates, can be tested with hand-built inputs, and, crucially for Phase 3, can be stored and called from inside an ECS system without lifetime entanglements.

**Security note:** the game treats brain output as _untrusted input_, clamping ranges and rejecting non-finite floats. This is the same principle as validating HTTP request bodies in a web app: validate at the trust boundary, never deeper inside. A NaN that reaches a physics solver can propagate into every body it touches. Be aware, though, that a _compiled-in_ Rust brain is not sandboxed at all: it runs as part of your program and could, say, loop forever or open files. The trait is an **architectural** boundary, not a **security** boundary. That distinction becomes the heart of Unit 4.2.

**Checkpoints**

1. Why is `Action` a struct of simultaneous intents rather than an enum of single commands?
2. Why are `Sensors` owned snapshots rather than references into the world?
3. Where exactly should `Action` values be validated, and why there?
4. What can a malicious compiled-in `BotBrain` still do that validation doesn't prevent?

**Phase 1 exit gate.** Before starting Bevy, you should be able to explain without notes: move vs copy vs borrow; why "shared XOR mutable" exists; what `'a` and `T: 'static` mean; `Option`/`Result`/`?`; generics vs `dyn`; `Send`/`Sync`; the three closure traits; why the toy ECS avoids borrow conflicts. You don't need mastery, but the borrow checker should no longer feel arbitrary.

---

# PHASE 2: Introduction to Bevy

**Phase goal:** learn Bevy as an **ECS runtime** that schedules your functions and lends them borrowed data, and only secondarily as a renderer. Every unit connects back to Phase 1. By the end, you should be able to read any system's signature and say what data it touches, whether it can run in parallel with another system, and when its changes become visible.

**Setup used throughout Phase 2:** a separate scratch project, `bevy-lab/`, one binary per unit (`src/bin/u21_app.rs`, ...), all sharing one `target/` so Bevy compiles once. Put this in its `Cargo.toml`:

```
[dependencies]
bevy = "0.19"

# Bevy's own recommendation: optimise dependencies even in debug builds,
# so the engine runs at usable speed while your code stays fast to compile.
[profile.dev]
opt-level = 1
[profile.dev.package."*"]
opt-level = 3
```

The first build takes several minutes; after that, incremental builds of your code are much faster. The Quick Start's "Setup" page lists further compile-time options (such as the `dynamic_linking` feature for development and faster linkers). Try them if iteration feels slow, but read the caveats.

---

## Unit 2.1: The Bevy Mental Model

**Effort:** M (reading and note-taking) · **Prerequisites:** Phase 1 complete, especially 1.15

**Objectives**

- Explain Entities, Components, Systems, Resources, Queries, Commands, Messages, Observers, Schedules, Plugins, and the `App`/`World` split.
- Map each onto something you built in the toy ECS.

**Resources**

- Bevy Quick Start: "Introduction" and "Getting Started" (Apps, ECS, Plugins, Resources).
- Tainted Coders: the ECS, Systems, and Queries guides.
- Reread your Unit 1.15 code with the Bevy docs beside it.

**Exercise:** write a one-page `BEVY_MODEL.md` with a two-column table: _toy ECS concept_ ↔︎ _Bevy concept_. Then answer in a paragraph each: (1) How does a Bevy system get `&mut` access to components without violating the borrow checker? (2) Why do `Commands` exist? (3) What's the difference between a resource and a component?

**Success criteria:** your table covers every item in the objectives; your answer to (1) mentions _declared access_ and _scheduling_.

**Why it works this way** The `World` is the one place all ECS data lives: entities, their components (stored in _archetype tables_, one table per unique combination of component types, each column a densely packed array), and resources (singletons). The `App` owns a `World` plus the schedules and plugins that act on it. A **system** is an ordinary function whose parameters (`Query<...>`, `Res<T>`, `ResMut<T>`, `Commands`, `MessageReader<T>`...) each implement Bevy's `SystemParam` trait. From those types, Bevy computes each system's **access set** (which component and resource types it reads and which it writes) _once_, when the system is initialised. The scheduler then uses those sets like a borrow checker for whole functions: two systems whose access sets don't conflict (no write overlapping another read or write) can run on different threads at the same time; conflicting systems are run one after another.

So the answer to (1) is: Bevy doesn't break the borrow rules; it lifts them from individual references to entire systems and checks them before running anything. It relies on some carefully audited `unsafe` internally to hand out the borrows it has proven disjoint, just as `split_at_mut` did in Unit 1.11.

`Commands` exist for the same reason as Unit 1.15's spawn queue: spawning, despawning, and adding or removing components move entities between archetype tables, which would invalidate every in-progress query iteration. Commands are recorded into a buffer and applied at a synchronisation point when no system is running.

**Checkpoints**

1. Where do components physically live, and why does an archetype layout make iterating a query fast?
2. When does Bevy detect that two parameters within one system conflict? (Compile time, startup, or every frame?)
3. What problem do `Commands` solve, and what's the cost?
4. Resource vs component: give one example of each from the capstone.

---

## Unit 2.2: Hello Bevy: App, Schedules, Window, Camera, Sprite

**Effort:** S–M · **Prerequisites:** 2.1

**Objectives**

- Build an `App` with `DefaultPlugins`; register systems in `Startup` and `Update`.
- Spawn a 2D camera and a coloured rectangle sprite; move it every frame.
- Know what `DefaultPlugins` adds and that it's just a group of ordinary plugins.

**Resources**

- Quick Start, "Apps" and "ECS" pages.
- Examples: `examples/hello_world.rs`, `examples/2d/sprite.rs`, `examples/2d/move_sprite.rs` (at tag `v0.19.1`).

**Exercise:** `u22_hello.rs`

```
use bevy::prelude::*;

#[derive(Component)]
struct Spinner { speed: f32 } // radians per second

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (spin, drift))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.7, 0.4), Vec2::new(60.0, 40.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Spinner { speed: 1.5 },
    ));
}

fn spin(time: Res<Time>, mut q: Query<(&Spinner, &mut Transform)>) {
    for (spinner, mut transform) in &mut q {
        transform.rotate_z(spinner.speed * time.delta_secs());
    }
}

fn drift(time: Res<Time>, mut q: Query<&mut Transform, With<Spinner>>) {
    for mut t in &mut q {
        t.translation.x = 200.0 * time.elapsed_secs().sin();
    }
}
```

Get it running, then: set the window title and size via `WindowPlugin` (`DefaultPlugins.set(...)`); spawn five spinners at different speeds; comment out the `.add_systems(Update, ...)` line and observe that nothing moves and nothing warns you.

**Success criteria:** window opens, sprites spin and drift smoothly; you can explain each system parameter in your own words.

**Why it works this way** `App::new()` creates an empty `World` and default schedules. Plugins register resources, systems, and other plugins; `DefaultPlugins` is a _plugin group_ bundling windowing, input, time, transforms, rendering, assets, audio and more, each of which you could replace or disable. `Startup` runs once; `Update` runs every frame. A system is only a function until it's registered in a schedule; nothing calls it otherwise. Spawning a tuple of components creates one entity with all of them. Components can declare _required components_, which is why spawning `Camera2d` alone produces a working camera: it pulls in the other components a camera needs with defaults. `spin` and `drift` both write `Transform`, so the scheduler won't run them simultaneously; they'll run one after another in an unspecified order (more on ordering in 2.5).

**Checkpoints**

1. What does `DefaultPlugins` actually add, and how would you remove one piece of it?
2. Why is `Transform` a component and `Time` a resource?
3. `spin` and `drift` both mutate `Transform`: can they run in parallel? In which order do they run?

---

## Unit 2.3: Queries Meet the Borrow Checker

**Effort:** L · **Prerequisites:** 2.2; firm grasp of 1.3 and 1.11

**Objectives**

- `Query<D, F>`: data (`&T`, `&mut T`, `Option<&T>`, `Entity`, tuples) and filters (`With<T>`, `Without<T>`, `Changed<T>`, `Added<T>`, `Or<...>`).
- `iter`, `iter_mut`, `get(entity)`, `get_mut(entity)`, `single()`/`single_mut()` (which return `Result`), `iter_combinations`.
- Intra-system access conflicts (error **B0001**) and their three fixes: `Without` filters to prove disjointness, `ParamSet`, or restructuring into separate components or systems.
- Fallible systems returning `Result`.

**Resources**

- Quick Start, ECS page (queries); Tainted Coders, "Queries".
- Bevy error reference: **B0001** and **B0002** at [https://bevy.org/learn/errors/](https://bevy.org/learn/errors/).
- `Query` API docs on docs.rs (read the method list and the long doc comment at the top).
- Example: `examples/ecs/ecs_guide.rs` (a long, well-commented tour).

**Exercise:** `u23_queries.rs`. Spawn six entities with combinations of `Tank`, `Health(f32)`, `Team(u8)`, `Transform`, and a `Projectile` marker. Write:

1. `report(q: Query<(Entity, &Health, Option<&Team>)>)`, printing everything once per second (use a `Local<Timer>` or check elapsed time).
2. `nearest_enemy(q: Query<(Entity, &Transform, &Team), With<Tank>>)` using `iter_combinations` to compute all pairwise distances.
3. **Provoke B0001:** `fn bad(mut tanks: Query<&mut Transform, With<Tank>>, projectiles: Query<&Transform, With<Projectile>>)`. Run it (it compiles!) and read the panic. Fix it with `Without<Tank>` on the second query and explain why that works. Then write a version where the two sets genuinely overlap and fix it with `ParamSet`.
4. Rewrite `report` using `let (e, h, _) = q.single()?;` for a query you know should match exactly one entity, returning `Result` from the system; spawn a second matching entity and see what happens.

**Success criteria:** you can look at any query signature and predict (a) which entities it matches and (b) whether it conflicts with another query in the same system.

**Why it works this way** A query's type is a declaration of access. `Query<&mut Transform, With<Tank>>` means "write access to `Transform` on entities that have `Tank`." The type system can't know at compile time whether two queries' entity sets overlap; that depends on filters and on what's been spawned. So Bevy checks at **system initialisation (startup)**: if two parameters could produce a `&mut` and another reference to the same component on the same entity, it panics with B0001 instead of risking aliased mutable references. Adding `Without<Tank>` to the second query proves disjointness through the filter types (no entity can be both "with Tank" and "without Tank"), so the check passes. `ParamSet` is the escape hatch for truly overlapping access: it lets you use only one of its inner queries at a time, which is exactly the "sequential `&mut`, not simultaneous" rule from Phase 1 applied to queries. It is also why the toy ECS separated data into columns: in Phase 3, making sensor output a _separate component_ from the data it's computed from will avoid needing `ParamSet` at all.

**Checkpoints**

1. Why does the B0001 example compile but panic at startup, rather than failing to compile?
2. Why does adding `Without<Tank>` make the conflict go away?
3. When would you reach for `ParamSet` instead of a filter?
4. What does `single()` return when zero or two entities match, and why is that better than panicking?

---

## Unit 2.4: Resources, Commands, Messages, and Observers

**Effort:** L · **Prerequisites:** 2.3

**Objectives**

- Resources: `#[derive(Resource)]`, `insert_resource`/`init_resource`, `Res<T>`/`ResMut<T>`, `Option<Res<T>>`.
- `Commands`: spawn, despawn, insert/remove components, and **when** those changes become visible.
- **Messages** (buffered, pull-based; `#[derive(Message)]`, `app.add_message::<T>()`, `MessageWriter`, `MessageReader`) versus **observer events** (push-based; `commands.trigger(...)`, `app.add_observer(...)`). Since Bevy 0.17 these are separate concepts; older tutorials call both "events."
- `Local<T>` for per-system private state.

**Resources**

- Quick Start, "Resources" page.
- Examples: `examples/ecs/message.rs` (or the closest-named message example in your tag), `examples/ecs/observers.rs`, `examples/ecs/removal_detection.rs`.
- Bevy 0.17 release notes, section on events/messages, to understand the split: [https://bevy.org/news/bevy-0-17/](https://bevy.org/news/bevy-0-17/)

**Exercise:** `u24_combat_toy.rs`. No physics yet; distances are checked by hand.

- Resources: `Scoreboard { hits: u32, kills: u32 }`, `SpawnTimer(Timer)`.
- Every 0.5 s, spawn a "bullet" entity (small sprite, velocity component) aimed at one of three stationary "tank" sprites with `Health`.
- `detect_hits` writes a `Hit { target: Entity, damage: f32 }` message and **despawns the bullet** via `Commands`.
- `apply_hits` reads `Hit` messages and reduces `Health`; if health ≤ 0 it writes `Destroyed { entity }`.
- `on_destroyed` reads `Destroyed`, despawns the tank, and bumps `Scoreboard`.
- Separately, add an **observer**: when a tank is destroyed, `commands.trigger(...)` a custom event and have an observer log it. Compare the two mechanisms.
- Experiment: in `detect_hits`, after queuing the despawn, try to `get` the bullet in a later statement of the _same_ system. It still exists. Explain why.

**Success criteria:** all tanks are destroyed over time, the scoreboard is correct, no warnings about despawning missing entities; you can state when a queued command takes effect.

**Why it works this way** Five mechanisms, five different jobs:

| Mechanism      | Scope               | Timing                                   | Use it for                               |
| -------------- | ------------------- | ---------------------------------------- | ---------------------------------------- |
| Component      | per entity          | read/written in place                    | tank health, velocity, brain             |
| Resource       | one per world       | read/written in place                    | round state, scoreboard, config          |
| Command        | structural change   | deferred to the next sync point          | spawn projectile, despawn tank           |
| Message        | many-to-many signal | buffered; readers pull at their own pace | "hit happened", "tank destroyed"         |
| Observer event | targeted signal     | handlers run immediately when triggered  | "react right now to this specific thing" |

Messages are stored in a double buffer; each `MessageReader` keeps its own cursor, so every reader sees every message once, and messages are dropped after they've survived two update cycles. That decoupling is what lets the physics collision code say "these two things touched" without knowing anything about damage, while the damage system says "this tank lost 10 health" without knowing about collisions. Observers are better for immediate, entity-targeted reactions (and power many of Bevy's own internals); messages are better for batch processing inside a schedule, which is what a simulation tick is. Commands are applied at _sync points_; Bevy automatically inserts one between two systems when the second is ordered after the first and the first has `Commands`. That's why the bullet still exists for the rest of the same system.

**Checkpoints**

1. Why is `RoundState` a resource and not a component on each tank?
2. Why is sending a `Hit` message better than having the hit-detection system directly edit health, scoreboard, and sound?
3. When does `commands.entity(e).despawn()` actually take effect?
4. Give one situation where an observer fits better than a message.

---

## Unit 2.5: Plugins, System Ordering, Sets, Run Conditions, and States

**Effort:** L · **Prerequisites:** 2.4

**Objectives**

- Write your own `Plugin`s; organise code by feature.
- Explicit ordering: `.before()`, `.after()`, `.chain()`, `SystemSet`s configured with `configure_sets`.
- Run conditions: `run_if(...)`, `in_state(...)`, `resource_exists::<T>`, `on_timer(...)`.
- `States`: `init_state`, `OnEnter`/`OnExit`, `NextState<T>`; entities cleaned up automatically on state exit (state-scoped entities).

**Resources**

- Quick Start, "Plugins" page.
- Examples: `examples/ecs/system_sets.rs` (or ordering examples in `examples/ecs/`), `examples/state/states.rs`, `examples/ecs/run_conditions.rs`.

**Exercise:** refactor Unit 2.4 into `ArenaPlugin`, `CombatPlugin`, and `HudPlugin` (log-only for now). Define

```
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum RoundState { #[default] Setup, Running, Finished }

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum CombatSet { Detect, Resolve, Cleanup }
```

Configure `CombatSet::Detect → Resolve → Cleanup` with `configure_sets(Update, (...).chain())`, run all combat systems only `in_state(RoundState::Running)`, spawn tanks `OnEnter(RoundState::Running)` with state-scoped cleanup, and transition to `Finished` when all tanks are gone, then back to `Setup` after 3 seconds. Then deliberately remove the ordering and run it several times: can you make it show a one-frame delay (a hit whose damage applies a frame later)?

**Success criteria:** rounds loop indefinitely with no leftover entities between rounds (check with a system counting entities); you can say which ordering constraints are _necessary_ and which you added "just in case."

**Why it works this way** A schedule is a **dependency graph** of systems. Bevy runs independent systems in parallel and respects only the orderings you declare, plus the conflicts it detects. Without an ordering, two conflicting systems run one after another in an arbitrary but valid order, which can differ between builds. That's why the unordered experiment may show a one-frame lag: if `apply_hits` happens to run before `detect_hits`, messages from this frame are read next frame. Declare ordering where the _meaning_ requires it ("apply damage after detecting hits") and nowhere else: over-constraining a schedule serialises work that could have been parallel and hides which dependencies are real. System sets let you order groups by role rather than listing individual functions. `States` is Unit 1.14's enum state machine built into the engine, with transition schedules (`OnEnter`/`OnExit`) and run conditions.

**Checkpoints**

1. When is an explicit ordering constraint necessary?
2. Why is adding `.chain()` to every system in the game a bad idea?
3. What's the advantage of ordering _sets_ instead of individual systems?
4. How does `in_state(RoundState::Running)` relate to your Unit 1.14 `Phase` enum?

---

## Unit 2.6: Transforms, Hierarchies, Sprites, Cameras, and Gizmos

**Effort:** M · **Prerequisites:** 2.5

**Objectives**

- `Transform` (local) vs `GlobalTransform` (computed world transform); parent/child hierarchies (`ChildOf`, `with_children`, the `children![]` macro).
- 2D rotation as a `Quat` about Z; converting heading angles to and from direction vectors (`transform.up()`, `rotation * Vec3::Y`).
- Draw order via `translation.z`; camera zoom via the camera's projection scale.
- **Gizmos** for immediate-mode debug drawing (lines, circles, arcs), which you'll use constantly for sensors.

**Resources**

- Examples: `examples/2d/move_sprite.rs`, `examples/2d/rotation.rs` (rotating toward a target: very relevant), `examples/transforms/` folder, `examples/gizmos/2d_gizmos.rs`.
- `Transform` and `Gizmos` API docs.

**Exercise:** `u26_turret.rs`. Spawn a tank body (rectangle) with a child turret (thin rectangle offset along +Y). Rotate the body slowly with one system; rotate the turret (local rotation only) to track the mouse cursor with another (convert cursor position to world coordinates via `Camera::viewport_to_world_2d`). Draw with gizmos: a line along the body's forward direction, a line along the turret's world direction (from its `GlobalTransform`), and a 30° arc representing a radar cone.

**Success criteria:** the turret stays attached as the body turns without you computing its world position; the turret aims correctly regardless of body rotation; gizmo lines match the sprites.

**Why it works this way** Each entity's `Transform` is relative to its parent; Bevy's transform propagation system multiplies down the hierarchy each frame (in `PostUpdate`) to produce `GlobalTransform`. So you express the turret's rotation _relative to the body_, and world placement falls out automatically. To aim at a world point you need the _difference_ between the desired world angle and the parent's world angle, which is the kind of reasoning the sensor system in Phase 3 does for bearings. Convention for this course: **forward is +Y** (`transform.up()`), and angles are radians counter-clockwise, matching `Quat::from_rotation_z`. Gizmos are redrawn each frame from scratch, which makes them ideal for visualising transient data like radar sweeps.

**Checkpoints**

1. Why does `GlobalTransform` lag behind `Transform` if you read it in `Update` right after changing `Transform`?
2. How do you compute a turret's local angle so that it points at a world-space target?
3. Why is drawing sensors with gizmos better than spawning sprite entities for them?

---

## Unit 2.7: Input, Time, and Frame-Rate Independence

**Effort:** M · **Prerequisites:** 2.6

**Objectives**

- `Res<ButtonInput<KeyCode>>`: `pressed` (held), `just_pressed`/`just_released` (edges).
- `Time`: `delta_secs()`, `elapsed_secs()`; frame-rate-independent motion (`speed * dt`).
- Timers (`Timer`, `TimerMode::Repeating`) for cooldowns.
- Separating **reading input** from **acting on it** via an intent component.

**Resources**

- Examples: `examples/input/keyboard_input.rs`, `examples/time/timers.rs`.

**Exercise:** `u27_drive.rs`. A tank you can drive: W/S for throttle, A/D to rotate, Space to fire a placeholder bullet (a sprite with a velocity component, despawned after 2 s). Structure it as:

```
#[derive(Component, Default)]
struct DriveIntent { throttle: f32, turn: f32, fire: bool }

fn read_keyboard(keys: Res<ButtonInput<KeyCode>>, mut q: Query<&mut DriveIntent, With<PlayerControlled>>) { /* write intent */ }
fn apply_drive(time: Res<Time>, mut q: Query<(&DriveIntent, &mut Transform, &TankStats)>) { /* move */ }
fn fire_weapons(/* reads DriveIntent, Weapon cooldown Timer, Commands */) { }
```

Then cap the frame rate artificially (for example, sleep 40 ms in a system for one run) and confirm the tank covers the same distance per second.

**Success criteria:** movement speed is the same at 144 FPS and 25 FPS; holding Space fires at the cooldown rate, not the frame rate.

**Why it works this way** Multiplying by `delta_secs()` converts "units per second" into "units this frame," so speed doesn't depend on frame rate. But it's only an approximation: with variable steps, anything involving acceleration, collisions, or feedback behaves slightly differently at different frame rates, which is fine for a cursor and not fine for physics (next unit). The `DriveIntent` split is the most important structural idea in this unit. The input system writes _what the controller wants_; the movement system reads intents _regardless of where they came from_. In Unit 3.5, a bot brain will write the same component, and the movement code won't change at all.

**Checkpoints**

1. How do "key is down" and "key was just pressed" differ, and which one should firing use?
2. Why does multiplying by delta time give frame-rate-independent movement, and where does that approximation break down?
3. What does the `DriveIntent` component buy you later?

---

## Unit 2.8: Fixed Timestep and the Physics Boundary

**Effort:** L · **Prerequisites:** 2.7, 1.14

**Objectives**

- `FixedUpdate` and `Time<Fixed>` (Bevy's default fixed rate is 64 Hz; change it with `Time::<Fixed>::from_hz(...)`).
- The accumulator model: `FixedUpdate` runs zero or more times per frame.
- Inside fixed systems, `Res<Time>` reports the fixed step.
- The **input-in-fixed-update pitfall** and its fix.
- Interpolation: why rendering can look jittery when simulation and frame rates differ, and how it's smoothed.

**Resources**

- _Fix Your Timestep!_ (Fiedler), reread now, especially the "free the physics" and interpolation sections.
- Example: `examples/movement/physics_in_fixed_timestep.rs` (read the whole comment header; it explains exactly this unit's topic).
- `Time<Fixed>` docs on docs.rs.

**Exercise:** `u28_fixed.rs`.

1. Add a `FixedUpdate` system and an `Update` system that each count their runs; display both counts (log once per second). Set the fixed rate to 20 Hz and then to 200 Hz, and observe how many fixed runs happen per frame.
2. Move `apply_drive` from Unit 2.7 into `FixedUpdate`. Keep `read_keyboard` in `Update`. Explain why.
3. **Provoke the pitfall:** put a `just_pressed(Space)` check directly inside a `FixedUpdate` system, set the fixed rate to 10 Hz, and tap Space quickly. Some taps are missed or (at high fixed rates) counted twice. Fix it by latching the press into `DriveIntent` in `Update` and clearing it in `FixedUpdate` after it's consumed.
4. At 10 Hz fixed, notice the stutter in motion; read how the example linked above interpolates between the previous and current fixed state for rendering.

**Success criteria:** the tank drives identically at any frame rate; no missed shots at a 10 Hz fixed rate; you can explain why the simulation and rendering rates are decoupled.

**Why it works this way** Physics solvers assume a constant step: with a big step, fast objects pass through thin walls, springs overshoot, and energy appears from nowhere; with a varying step, the same inputs produce different outcomes on different machines. A fixed timestep advances the simulation in constant increments, running as many as real time demands, exactly like Unit 1.14's accumulator, and this is how Bevy's `RunFixedMainLoop` drives `FixedUpdate` and its sibling schedules (`FixedPreUpdate`, `FixedPostUpdate`...). Input, however, is updated once per _frame_, so edge-triggered checks inside a schedule that runs 0 or 2 times per frame will miss or double-count edges. The standard fix is to capture input into state in `Update` and consume it in the fixed schedule. Rendering happens once per frame, typically between two simulation states, so for smoothness the renderer interpolates. Avian provides transform interpolation for exactly this, which you'll switch on in Unit 3.2.

Here's the timing picture the capstone will use:

```
every frame (Update):     read input → write intents     render, HUD, gizmos
                               │                               ▲
every fixed step (64 Hz):      ▼                               │
  FixedUpdate:      consequences → sense → think → act         │
  FixedPostUpdate:  Avian physics step ────────────────────────┘ (writes Transform)
```

**Checkpoints**

1. Why is a variable timestep dangerous for physics?
2. Why might `FixedUpdate` run twice in one frame and zero times in the next?
3. Why is `just_pressed` unreliable inside `FixedUpdate`, and what's the fix?
4. Does a fixed physics step mean every system should run in `FixedUpdate`? Which shouldn't?

**Phase 2 exit gate.** You can explain `Entity`, `Component`, `Resource`, `Query` (data and filters), `Commands` (and sync points), `Message` vs observer event, `Schedule`, `SystemSet`, `Plugin`, `States`, `Transform` vs `GlobalTransform`, `Time` vs `Time<Fixed>`. Given any system signature, you can describe its data access and whether it conflicts with another.

---

# PHASE 3: Capstone, a Programmable 2D Tank Combat Arena

**Phase goal:** build the game in milestones that each add _one_ major conceptual layer and each end in something runnable. At no point should you be learning a new Rust concept, a new Bevy concept, a new physics concept, and a new gameplay system all at once.

### The architecture you are building toward

```
                 ┌──────────────────────── FixedUpdate (64 Hz), chained sets ─────────────────────────┐
                 │                                                                                     │
  Avian messages │  Consequences          Sense                 Think                  Act            │
 (from last step)│  ─────────────         ─────────             ─────────              ─────────      │
 CollisionStart ─┼─▶ projectile hits ──▶  build Sensors  ──▶   brain.tick() ──▶       validate Action  │
                 │   DamageDealt msg      snapshot per tank     (dyn BotBrain)         → velocities   │
                 │   apply damage         (reads Position,      → ControlIntent        → turret angle  │
                 │   LastHit, deaths      raycasts, radar)                             → spawn shells  │
                 │   round-over check                                                                  │
                 └─────────────────────────────────────────────────────────────────────────────────────┘
                                   │
                 FixedPostUpdate:  Avian physics step (solver, collisions, writes Position/Rotation → Transform)
                                   │
                 Update (per frame): keyboard → ControlIntent (manual tanks only), HUD, gizmos, interpolation
```

The single most important principle: **the brain does not know Bevy exists.** A bot never receives a `Query`, `Commands`, `World`, `Entity`, `Transform`, or any Avian type. It receives an owned `Sensors` value and returns an `Action`. Everything else is the simulation's job.

### Project layout

Start a new workspace (or evolve `arena/` from Phase 1):

```
tank_arena/
  Cargo.toml                    # workspace; profile settings from Phase 2
  crates/
    arena_core/                 # from Phase 1, evolved: Sensors, Action, BotBrain, brains, math helpers
    arena_game/                 # Bevy + Avian binary
      src/
        main.rs                 # App assembly only
        config.rs               # ArenaConfig, TankStats (resources)
        arena.rs                # ArenaPlugin: walls, camera, floor
        tank.rs                 # TankPlugin: components, spawning, drive
        weapons.rs              # WeaponsPlugin: firing, projectiles
        combat.rs               # CombatPlugin: hits, damage, deaths
        sensors.rs              # SensorPlugin
        brains.rs               # BrainPlugin: BotController, think system
        round.rs                # RoundPlugin: states, spawning roster, winner
        debug.rs                # gizmos, physics debug toggle
```

---

## Unit 3.1: Skeleton, Arena, and Round States

**Effort:** M · **Prerequisites:** Phase 2 complete

**Objectives**

- Set up the two-crate workspace with `arena_game` depending on `arena_core` by path.
- Plugins per feature; an `ArenaConfig` resource; a `RoundState` state machine.
- A visible arena (floor, four wall sprites) and a camera that frames it at any window size.

**Resources**

- Your Unit 2.5 code; examples `examples/2d/2d_shapes.rs` and `examples/window/window_settings.rs`.
- Camera scaling: the `Projection`/`OrthographicProjection` docs (look for scaling modes that fit a fixed world area into the window).

**Exercise**

- `ArenaConfig { width: 1200.0, height: 800.0, wall_thickness: 20.0, max_round_ticks: 64 * 180 }` as a resource inserted in `main`.
- `ArenaPlugin` spawns a floor sprite and four wall sprites on `Startup`; the camera uses a scaling mode that keeps the whole arena visible when the window is resized.
- `RoundPlugin` defines `RoundState { Setup, Running, Finished }`; `Setup` immediately moves to `Running`; pressing `R` in `Finished` returns to `Setup`.
- `main.rs` does nothing but assemble plugins.

**Success criteria:** the window shows a framed arena at any size; `main.rs` is under 30 lines; state transitions log on `OnEnter`.

**Why it works this way** Starting with the nouns of the domain (arena, round, tank) as plugins gives every later feature an obvious home and keeps `main` as a readable table of contents. Plugins also make it cheap to build _different apps from the same parts_, which you'll exploit in Unit 3.10 for a headless tournament runner that uses every plugin except rendering and HUD.

**Checkpoints**

1. Which state in the game is global (resources, states) and which belongs to a tank (components)?
2. Why keep `main.rs` to plugin assembly only?
3. What would you need to change to run the same simulation without a window?

---

## Unit 3.2: One Manually Driven Tank (No Physics Yet)

**Effort:** M · **Prerequisites:** 3.1, 2.6–2.8

**Objectives**

- Define the tank's component set, using **required components** so a tank can't be spawned half-built.
- Drive it through the intent component in `FixedUpdate`, with a turret as a child entity.
- Keep simulation state (turret angle) separate from presentation (turret sprite).

**Exercise**

```
#[derive(Component)]
#[require(Health, ControlIntent, TurretAngle, Weapon)]
pub struct Tank;

#[derive(Component, Default, Clone, Copy)]
pub struct ControlIntent { pub throttle: f32, pub turn: f32, pub turret_turn: f32, pub fire: bool }

#[derive(Component, Default)] pub struct TurretAngle(pub f32);   // radians, relative to body
#[derive(Component)] pub struct PlayerControlled;
#[derive(Component)] pub struct TurretVisual;                    // marker on the child sprite
```

(Give `Health` and `Weapon` sensible `Default`s.) Systems: `read_keyboard` (`Update`; WASD drive, Q/E turret, Space fire, latched), `drive_kinematic` (`FixedUpdate`; moves `Transform` directly with a speed and turn rate from a `TankStats` component), `turn_turret` (`FixedUpdate`; updates `TurretAngle` at a limited rate), `sync_turret_visual` (`Update`; copies `TurretAngle` onto the child sprite's local rotation). Clamp the tank inside the arena by hand for now.

**Success criteria:** you can drive around and aim; removing `PlayerControlled` makes the tank sit still without any other code change; the turret sprite never holds authoritative state.

**Why it works this way** This milestone isolates ECS, input, and rendering from physics. When you add Avian in the next unit, any new problems come from the physics boundary, not from basic Bevy mechanics. Required components (`#[require(...)]`) make Bevy insert defaults for any listed component you didn't provide, so every `Tank` is guaranteed to have an intent, a turret angle, and a weapon. That's an invariant enforced by construction rather than by remembering. Keeping `TurretAngle` on the tank (simulation) and making the child sprite a _view_ of it means a headless simulation doesn't need the sprite at all.

**Checkpoints**

1. What does `#[require(...)]` guarantee, and why is that better than documenting "remember to add X"?
2. Which system owns player input, and which owns movement? Why separate them?
3. Why is the turret angle a component on the tank and not just the child sprite's rotation?

---

## Unit 3.3: Integrate Avian2D: Bodies, Colliders, Walls

**Effort:** L · **Prerequisites:** 3.2

**Objectives**

- Add `PhysicsPlugins`, set zero gravity for a top-down world, set a length unit suited to pixel coordinates.
- Make the tank a `RigidBody::Dynamic` with a `Collider`; make walls `RigidBody::Static`.
- Drive the tank by setting **velocities with an acceleration limit**, not by writing `Transform`.
- Understand Avian's `Position`/`Rotation` vs Bevy's `Transform`, and enable transform interpolation.
- Collision layers; the physics debug renderer.

**Resources**

- Avian2D crate docs (docs.rs/avian2d/0.7): read "Getting started", then the `dynamics::rigid_body` module docs (RigidBody types, velocity, damping, mass properties) and the `schedule` module docs (where physics runs).
- Avian 2D examples: the "move marbles"/"one-way platform"/"kinematic character" examples show velocity control and collision layers.

**Exercise**

```
app.add_plugins(PhysicsPlugins::default().with_length_unit(20.0))
   .insert_resource(Gravity(Vec2::ZERO));
// debug overlay while developing:
// app.add_plugins(PhysicsDebugPlugin::default());

#[derive(PhysicsLayer, Default)]
pub enum GameLayer { #[default] Default, Tank, Wall, Projectile }

// on the tank:
(RigidBody::Dynamic,
 Collider::rectangle(32.0, 40.0),
 LinearDamping(1.5),
 AngularDamping(4.0),
 CollisionLayers::new(GameLayer::Tank, [GameLayer::Tank, GameLayer::Wall, GameLayer::Projectile]),
 TransformInterpolation)
// on each wall:
(RigidBody::Static, Collider::rectangle(w, h),
 CollisionLayers::new(GameLayer::Wall, [GameLayer::Tank, GameLayer::Projectile]))
```

Replace `drive_kinematic` with:

```
fn drive(time: Res<Time>,
         mut q: Query<(&ControlIntent, &TankStats, &Rotation, &mut LinearVelocity, &mut AngularVelocity)>) {
    let dt = time.delta_secs(); // fixed step, because this runs in FixedUpdate
    for (intent, stats, rot, mut lin, mut ang) in &mut q {
        let forward = *rot * Vec2::Y;
        let target = forward * intent.throttle * stats.max_speed;
        // move current velocity toward target, limited by acceleration
        lin.0 += (target - lin.0).clamp_length_max(stats.accel * dt);
        ang.0 = intent.turn * stats.turn_rate;
    }
}
```

Remove your manual clamping. Drive into walls at an angle. Spawn a second, uncontrolled tank and ram it. Toggle the debug renderer on and off.

**Success criteria:** the tank stops at walls without clamping code; ramming pushes the other tank, which then coasts to a stop; motion is smooth at any frame rate; no system writes the tank's `Transform` any more.

**Why it works this way** Before physics, the architecture was `input → Transform`. Now it's `input → velocity → solver → Position/Rotation → Transform`. **The physics engine is authoritative for physical state.** If you keep writing `Transform` on a dynamic body you're teleporting it past the solver every tick, and collisions will jitter or tunnel. Avian stores physics state in its own components (`Position`, `Rotation`) and synchronises with `Transform` around each step, so gameplay code should read and write `Position`, `Rotation`, and the velocity components.

The drive code deliberately steers the _whole_ velocity toward "forward × throttle × max speed," which also bleeds off sideways velocity: that's what treads do (tanks don't drift like hovercraft). Collisions still work because the solver applies contact impulses during its step; your code then nudges velocity back toward the intended value at a limited rate. That limit is what keeps the tank feeling heavy rather than snapping instantly. `with_length_unit(20.0)` tells Avian roughly how big a "unit" object is in your coordinates; internal tolerances (like how much overlap counts as contact or when bodies may sleep) scale with it, which matters when your world is measured in pixels rather than metres. Collision layers answer "who can touch whom" before any expensive narrow-phase work happens. Avian runs the step in `FixedPostUpdate` by default, after your `FixedUpdate` systems, so the intent you apply this tick takes effect in this tick's step. `TransformInterpolation` smooths rendering between fixed steps, as discussed in Unit 2.8.

**Checkpoints**

1. Why shouldn't you set a dynamic body's `Transform` every tick?
2. What's the conceptual difference between a collider and a rigid body?
3. Why does the drive code limit the _change_ in velocity rather than setting it directly?
4. Why does the length unit matter in a pixel-scale world?

---

## Unit 3.4: Physics-Driven Projectiles

**Effort:** L · **Prerequisites:** 3.3

**Objectives**

- A `Weapon` component with cooldown enforced by the game, not the controller.
- Spawn projectiles from the muzzle with an initial velocity; let physics carry them.
- Prevent **tunnelling** of fast projectiles with continuous collision detection.
- Record **ownership** (who fired) on each projectile; give projectiles a lifetime.

**Resources**

- Avian docs: the `dynamics::ccd` module (speculative collision vs swept CCD), the `Sensor` component, `CollisionLayers`.

**Exercise**

```
#[derive(Component)]
pub struct Weapon { pub cooldown: Timer, pub muzzle_speed: f32, pub damage: f32 }

#[derive(Component)]
pub struct Projectile { pub owner: Entity, pub damage: f32 }

#[derive(Component)]
pub struct Lifetime(pub Timer);
```

`fire_weapons` (in `FixedUpdate`, after `drive`) ticks every weapon's cooldown; when `intent.fire` is set and the cooldown has finished, it computes the turret's world direction (`Vec2::from_angle(turret.0).rotate(forward)`), places the muzzle just outside the tank's collider, and spawns:

```
(Projectile { owner: tank_entity, damage: weapon.damage },
 Lifetime(Timer::from_seconds(3.0, TimerMode::Once)),
 Sprite::from_color(Color::srgb(1.0, 0.9, 0.3), Vec2::splat(6.0)),
 Transform::from_translation(muzzle.extend(2.0)),
 RigidBody::Dynamic,
 Collider::circle(3.0),
 LinearVelocity(dir * weapon.muzzle_speed),
 SweptCcd::default(),
 CollisionLayers::new(GameLayer::Projectile, [GameLayer::Tank, GameLayer::Wall]))
```

Clear the fire intent after consuming it. Despawn projectiles when their `Lifetime` finishes. Then experiment: set `muzzle_speed` to 3000, remove `SweptCcd`, and fire at a wall from close range; watch shells pass through it. Put `SweptCcd` back. Try firing while driving fast in the direction of fire: should shells inherit the tank's velocity? Pick one answer and write down why.

**Success criteria:** shells leave the barrel, travel straight, bounce off or stop at walls, never tunnel at 3000 units/s, and expire; holding fire respects the cooldown.

**Why it works this way** A projectile is a good first physics object because its behaviour is fully determined by initial conditions: you set position and velocity once and the solver does the rest. This contrasts with the tank, whose velocity you adjust every tick. **Tunnelling** happens because discrete simulation only checks overlaps at step boundaries: at 3000 units/s and 64 Hz a shell moves about 47 units per step, far more than a 20-unit wall. Swept CCD sweeps the shape along its path between steps and catches the crossing. It costs more, so it goes only on small, fast bodies. The `Projectile` shells in this course are real dynamic bodies, so they'll nudge a tank slightly on impact (their mass is small); if you'd rather have hits with no physical push, give shells a `Sensor` component so they report overlaps without generating contact forces. That's a legitimate design choice worth trying. Ownership lives on the projectile because by the time it hits something, the tank that fired it may have moved, fired again, or been destroyed.

Whether shells inherit tank velocity is a _game-design_ decision: inheriting is more physically faithful; not inheriting makes aiming simpler for bots. RoboCode doesn't inherit. Whichever you choose, write it down, because bots in Unit 3.11 will assume it.

**Checkpoints**

1. What determines a projectile's trajectory after launch in this top-down world?
2. Why does fast, small, discrete-step motion tunnel, and how does swept CCD fix it?
3. Why does the game, not the controller, own the weapon cooldown?
4. Why is the owner stored on the projectile rather than looked up at impact time?

---

## Unit 3.5: The `BotBrain` Contract, Version 2 (Pure Rust)

**Effort:** L · **Prerequisites:** 3.4, 1.16

**Objectives**

- Evolve the Phase 1 contract into the capstone's final `Sensors`/`Action` API in `arena_core`.
- Choose conventions (angles, units, IDs) deliberately and document them.
- Write brains and tests with no engine at all.

**Exercise:** in `arena_core/src/bot.rs`:

```
/// Conventions: distances in world units; angles in radians, counter-clockwise positive,
/// wrapped to (-PI, PI]. "Bearing" is relative to the tank's BODY heading.
/// Forward is the body's +Y axis. One tick = one fixed simulation step.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BotId(pub u32);

#[derive(Debug, Clone, Default)]
pub struct Sensors {
    pub tick: u64,
    pub dt: f32,
    pub me: SelfState,
    pub radar: Vec<RadarContact>,        // enemies inside the turret-aligned radar cone
    pub proximity: Vec<ProximityPing>,   // anything within a short 360° range: no identity
    pub walls: WallProbe,                // ray distances to the nearest wall
    pub last_hit: Option<HitReport>,
    pub arena: ArenaInfo,
}

#[derive(Debug, Clone, Default)]
pub struct SelfState {
    pub id: BotId, pub health: f32, pub max_health: f32,
    pub position: (f32, f32), pub heading: f32, pub speed: f32,
    pub turret: f32,              // relative to body
    pub gun_ready_in: f32,        // seconds until the weapon can fire
    pub shell_speed: f32,         // needed for aiming maths
}

#[derive(Debug, Clone, Copy)]
pub struct RadarContact { pub id: BotId, pub distance: f32, pub bearing: f32,
                          pub heading: f32, pub speed: f32, pub health: f32 }
#[derive(Debug, Clone, Copy)]
pub struct ProximityPing { pub distance: f32, pub bearing: f32 }
#[derive(Debug, Clone, Copy, Default)]
pub struct WallProbe { pub ahead: f32, pub behind: f32, pub left: f32, pub right: f32 }
#[derive(Debug, Clone, Copy)]
pub struct HitReport { pub by: BotId, pub bearing: f32, pub ticks_ago: u32 }
#[derive(Debug, Clone, Copy, Default)]
pub struct ArenaInfo { pub width: f32, pub height: f32 }

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Action { pub throttle: f32, pub turn: f32, pub turret_turn: f32, pub fire: bool }

pub trait BotBrain: Send + Sync {
    fn name(&self) -> &str;
    fn on_round_start(&mut self, _info: &RoundInfo) {}
    fn tick(&mut self, sensors: &Sensors) -> Action;
}

#[derive(Debug, Clone, Copy)]
pub struct RoundInfo { pub seed: u64, pub bot_count: u32 }

impl Action {
    /// The simulation calls this on every action. Brains cannot opt out.
    pub fn sanitized(self) -> Action { /* clamp to [-1,1]; non-finite -> 0.0 */ }
}
pub fn wrap_angle(a: f32) -> f32 { /* to (-PI, PI] */ }
```

(Make `Default` work for `BotId` and anything else that needs it.) Port your Phase 1 brains; write `SittingDuck` (does nothing) and `Spinner` (drives in a circle, sweeps turret, fires when anything is on radar). Write table-driven tests: enemy dead ahead → fires; enemy at bearing +90° → turns left; `last_hit` from behind → throttle forward; `NaN` everywhere → `sanitized()` produces zeros.

**Success criteria:** `cargo test -p arena_core` runs in well under a second and covers every brain; `arena_core` still has no Bevy dependency.

**Why it works this way** This is the most consequential design decision in the project, so it's worth spelling out the choices:

- **Owned snapshot, not references.** Brains hold no borrows into the world, so the ECS can store and call them freely, run them in parallel, record their inputs for replays, and (in Unit 4.2) hand the same data to a script.
- **Relative bearings, not world coordinates for enemies.** They're what a turret needs, they make brain logic rotation-invariant, and they're what a real sensor would report. Own position is exposed (think of it as GPS) because avoiding walls is otherwise tedious.
- **`BotId`, not `Entity`.** `Entity` is an engine type; exposing it would couple bots to Bevy and leak an implementation detail (entity IDs are recycled with generations). `BotId` is stable for the whole round and meaningful in stats.
- **Radar vs proximity.** Radar is precise but narrow and aligned with the turret, so scanning is a strategic cost (aiming elsewhere means not seeing). Proximity is 360° but anonymous and short, a "something's close" warning. The trade-off creates interesting behaviour.
- **What's deliberately missing:** enemy brain state, enemies outside sensor range, the full list of bots, anything about projectiles in flight. Every omission is a design lever; adding "incoming shell warning" later is a balancing decision, not a bug fix.
- **`Send + Sync` supertraits** so any `Box<dyn BotBrain>` can live inside a component (Unit 1.8).
- **Default method `on_round_start`** so simple brains don't need boilerplate, while stateful brains can reset and seed RNGs.

**Checkpoints**

1. Why should `Sensors` be an owned snapshot rather than a collection of ECS references?
2. Why expose `BotId` instead of `Entity`?
3. What would become hard to test if `BotBrain::tick` took `&mut World`?
4. Name two pieces of information deliberately _not_ exposed, and what exposing each would change.

---

## Unit 3.6: Brains Inside the ECS, and the Action Adapter

**Effort:** L · **Prerequisites:** 3.5, 2.3

**Objectives**

- Store a `Box<dyn BotBrain>` in a component; call it from a system.
- Build a _minimal_ sensor snapshot (own state only, for now) in a `Sense` set; tick brains in `Think`; apply actions in `Act`.
- Make keyboard control and bot control two interchangeable writers of `ControlIntent`.
- Contain misbehaving brains (panics, slow ticks).

**Exercise**

```
#[derive(Component)]
pub struct BotController { pub brain: Box<dyn BotBrain> }  // BotBrain: Send + Sync, so this is a valid component

#[derive(Component, Default)]
pub struct SensorSnapshot(pub Sensors);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimSet { Consequences, Sense, Think, Act }

app.configure_sets(FixedUpdate,
    (SimSet::Consequences, SimSet::Sense, SimSet::Think, SimSet::Act)
        .chain()
        .run_if(in_state(RoundState::Running)));

fn think(mut q: Query<(&SensorSnapshot, &mut BotController, &mut ControlIntent)>) {
    for (snap, mut ctl, mut intent) in &mut q {
        let action = ctl.brain.tick(&snap.0).sanitized();
        *intent = ControlIntent::from(action);
    }
}
```

`build_sensors` (in `Sense`) fills `SelfState` from `Position`, `Rotation`, `LinearVelocity`, `Health`, `TurretAngle`, `Weapon`. `drive`, `turn_turret`, and `fire_weapons` move into `Act`. Spawn one tank with `BotController { brain: Box::new(Spinner::default()) }` and one with `PlayerControlled`. Then:

1. Replace the `for` loop in `think` with `q.par_iter_mut().for_each(...)`. It compiles. Explain why this is safe.
2. Wrap the `tick` call in `std::panic::catch_unwind(AssertUnwindSafe(|| ...))`; write a brain that panics on tick 100; on a caught panic, insert a `Faulted` marker component and have that tank go idle. Log the brain's name.
3. Time each `tick` with `std::time::Instant`; log a warning if any brain takes more than 1 ms.

**Success criteria:** the bot tank drives and fires by itself while you drive the other; a panicking brain disables only its own tank; `drive`/`fire_weapons` contain no code that knows whether a tank is human- or bot-controlled.

**Why it works this way** This is where Phase 1's trait objects meet Phase 2's ECS. The component stores the brain; the **system** owns orchestration and decides when `tick()` is called. The brain is just data with behaviour. `ControlIntent` is the seam that makes controllers interchangeable: movement and weapons read intents without caring whether a keyboard or an algorithm wrote them (Unit 2.7's design paying off).

`par_iter_mut` is safe because each iteration gets exclusive access to one entity's `BotController` and `ControlIntent` and only shared access to its own `SensorSnapshot`. No brain can see another brain, and `BotBrain: Send` means it may be moved to a worker thread. That guarantee came from the trait bound you wrote in Unit 1.8, and the compiler enforced it.

`catch_unwind` turns a panic in one brain into a recoverable error for that tank rather than a crash of the whole game. It works only with the default `panic = "unwind"` strategy; if a profile sets `panic = "abort"`, it can't help. It also can't stop a brain that loops forever or allocates wildly. A compiled-in brain is trusted native code; timing it can _detect_ slowness but can't _preempt_ it. Unit 4.2 revisits this properly.

**Checkpoints**

1. Why is `BotBrain` a trait stored in a component rather than being a component itself?
2. Why does `par_iter_mut` over brains compile, and which Phase 1 bound makes it legal?
3. What can `catch_unwind` protect against, and what can't it?
4. Why is the `Action` sanitised in the simulation rather than trusted from the brain?

---

## Unit 3.7: Collisions, Damage, Death, and `last_hit`

**Effort:** L · **Prerequisites:** 3.6, 2.4

**Objectives**

- Consume Avian's collision messages; turn _physics facts_ into _gameplay facts_.
- A `DamageDealt` message, a damage system, a death system, and `LastHit` state feeding `Sensors::last_hit`.
- Handle duplicate and same-step collision reports robustly.

**Resources**

- Avian docs: `collision::collision_events` module (the `CollisionStart`/`CollisionEnd` messages and the `CollisionEventsEnabled` component that opts an entity in to generating them).

**Exercise:** add `CollisionEventsEnabled` to projectiles. Then, in `SimSet::Consequences`, chained in this order:

```
#[derive(Message)]
pub struct DamageDealt { pub target: Entity, pub source: BotId, pub amount: f32, pub from_dir: Vec2 }
#[derive(Message)]
pub struct TankDestroyed { pub victim: BotId, pub by: Option<BotId> }

fn projectile_hits(
    mut collisions: MessageReader<CollisionStart>,
    projectiles: Query<(&Projectile, &LinearVelocity)>,
    tanks: Query<&BotId, With<Tank>>,
    mut damage: MessageWriter<DamageDealt>,
    mut commands: Commands,
    mut resolved: Local<Vec<Entity>>,   // shells already handled in THIS batch
) {
    resolved.clear();
    for c in collisions.read() {
        // Which side is the projectile? (Colliders are on the bodies themselves in this project,
        // so collider entities are body entities. Avian also reports the bodies explicitly.)
        let (shell, other) = if projectiles.contains(c.collider1) { (c.collider1, c.collider2) }
                             else if projectiles.contains(c.collider2) { (c.collider2, c.collider1) }
                             else { continue };
        if resolved.contains(&shell) { continue; }  // same shell reported twice this step
        let Ok((proj, vel)) = projectiles.get(shell) else { continue };
        if let (Ok(target_id), Ok(owner_id)) = (tanks.get(other), tanks.get(proj.owner)) {
            if other != proj.owner {
                damage.write(DamageDealt { target: other, source: *owner_id,
                                           amount: proj.damage, from_dir: vel.0.normalize_or_zero() });
            }
        }
        resolved.push(shell);
        commands.entity(shell).try_despawn();
    }
}
```

(Check field names against the Avian 0.7 docs; the shape is what matters.) Then `apply_damage` (reads `DamageDealt`; lowers `Health`; sets `LastHit { by, from_dir, tick }`), `destroy_dead` (entities whose health is ≤ 0: write `TankDestroyed`, despawn), and extend `build_sensors` to fill `last_hit` with a bearing computed from `from_dir` relative to the victim's heading.

Two issues to notice: the owner might already be despawned when its shell lands (then `tanks.get(proj.owner)` fails; decide whether that damage still counts and how it's attributed); and a shell can report contacts with two things in the same step, which is why the `resolved` list and `try_despawn` are there. (Before reading on, try the obvious alternative: inserting a `Spent` marker component and filtering the query with `Without<Spent>`. Work out why that _doesn't_ prevent double damage within one batch.)

**Success criteria:** shooting a tank lowers its health and sets its `last_hit` to the shooter's `BotId`; tanks at zero health disappear; shells never damage their owner; no warnings about despawning missing entities; no double damage from one shell.

**Why it works this way** **Collision is a physics fact; damage is a gameplay fact.** The message layer keeps them apart. The collision system translates "shell A touched body B" into "bot 3 dealt 10 damage to tank B from this direction," and knows nothing about health. The damage system knows nothing about physics. That separation means you can later add mines, ramming damage, or area effects that produce `DamageDealt` without being collisions, and add armour or shields without touching collision code. Collision messages from Avian's step in `FixedPostUpdate` are read in the next tick's `Consequences` set, so damage resolves one fixed step (about 16 ms) after contact. It's consistent and invisible, and it's a good example of choosing a clear, deterministic ordering over "instant" reactions. Because commands are deferred, a marker component inserted during this system is invisible to this system's own queries until the next sync point, so a `Without<Spent>` filter can't stop the second report in the same batch. The system needs its own memory for the duration of the loop (the `Local` list), which is the Unit 2.4 "the bullet still exists" lesson in a place where it actually bites. The `chain()` on the sets means Bevy inserts a sync point so later sets see the despawns applied.

**Checkpoints**

1. Why should the collision system not contain the damage logic?
2. Why does damage apply one fixed step after the physical contact, and why is that acceptable?
3. Why can't a `Spent` marker component, inserted with `Commands`, prevent a shell from dealing damage twice in the same step?
4. Why is `last_hit` exposed as a bearing and a `BotId` rather than as raw collision data?

---

## Unit 3.8: Many Bots, Round Lifecycle, and a Winner

**Effort:** M · **Prerequisites:** 3.7, 2.5

**Objectives**

- Spawn N bots from a roster at spread-out positions on `OnEnter(Running)`.
- Detect the end of the round (one or zero survivors, or time limit) and record a result.
- Clean up everything between rounds and restart.

**Exercise**

- A `Roster` resource: `Vec<(String, fn() -> Box<dyn BotBrain>)>` (a _factory_ per entry, since each round needs fresh brains). Spawn positions evenly on a circle inside the arena, facing the centre; colours by index; `BotId` by index.
- Call `on_round_start` for each brain with a `RoundInfo`.
- A `SimTick` resource incremented at the start of `Consequences`.
- `check_round_over` (last in `Consequences`): count surviving tanks; if ≤ 1, or `SimTick` ≥ `max_round_ticks`, write a `RoundResult { winner: Option<BotId>, ticks, survivors }` resource and set `NextState(RoundState::Finished)`. On `Finished`, log the winner and restart after 3 seconds.
- Mark every round entity (tanks, shells) as despawned on exiting `Running` (state-scoped entities).

**Success criteria:** four bots with at least two different brains fight to a declared winner (or a draw on timeout), and the next round starts clean, repeatedly, for 20+ rounds with a stable entity count.

**Why it works this way** Round state is global and singular, so it's a state plus a resource, not data duplicated on every tank. Victory detection is a separate system from damage because they answer different questions: "how much health does this tank have?" versus "is the round over?" Keeping them apart means changing win conditions (teams, king-of-the-hill, time limits) touches one system. Factories rather than brain instances in the roster are an ownership decision: a `Box<dyn BotBrain>` is _moved into_ a tank's component when spawned and dropped when the tank is despawned, so every round needs new ones. Because `check_round_over` runs after `destroy_dead` in a chained set, the automatic sync point means the despawns are already applied when it counts survivors.

**Checkpoints**

1. Why is the roster a list of factory functions rather than brain instances?
2. Why is the round result a resource?
3. What would break if `check_round_over` ran _before_ `destroy_dead`'s commands were applied?

---

## Unit 3.9: Real Sensors: Radar Cone, Proximity, and Wall Rays

**Effort:** L · **Prerequisites:** 3.8, 1.10

**Objectives**

- Fill the full `Sensors` struct each tick from ECS state and spatial queries.
- Design the sensor system's queries so no borrow conflicts arise.
- Visualise every sensor with gizmos.

**Resources**

- Avian docs: `spatial_query` module (`SpatialQuery` system parameter, `cast_ray`, `SpatialQueryFilter` with layer masks).
- Bevy `Vec2` docs (glam): `angle_to`, `from_angle`, `rotate`, `length`, `normalize_or_zero`.

**Exercise**

```
fn build_sensors(
    tick: Res<SimTick>, arena: Res<ArenaConfig>, time: Res<Time>, spatial: SpatialQuery,
    everyone: Query<(&BotId, &Position, &Rotation, &LinearVelocity, &Health), With<Tank>>,
    mut me: Query<(&BotId, &Position, &Rotation, &LinearVelocity, &Health,
                   &TurretAngle, &Weapon, &LastHit, &SensorConfig, &mut SensorSnapshot)>,
) { /* ... */ }
```

For each tank in `me`: compute `forward = rot * Vec2::Y` and the turret's world direction; iterate `everyone` (skipping itself by `BotId`); for each other tank compute `delta`, `distance`, `bearing = wrap_angle(forward.angle_to(delta))`; add a `RadarContact` if within radar range and within the cone half-angle of the turret direction; add a `ProximityPing` if within proximity range. Cast four rays (ahead, behind, left, right) filtered to `GameLayer::Wall` for `WallProbe`. Draw everything with gizmos in an `Update` debug system (radar cone as an arc, contacts as lines, wall rays as short lines). Reuse the contact vector's allocation across ticks (`clear()` then push) rather than creating a new `Vec` each time.

Then rewrite `Spinner` into `Tracker`: sweep the turret until something is on radar, then keep the turret on the contact (turret_turn proportional to bearing error), drive toward it, fire when bearing error is under 3°, and back away when `walls.ahead` is small.

**Success criteria:** `Tracker` visibly finds, follows, and shoots other bots; a bot with nothing on radar behaves differently from one that sees an enemy; the gizmo overlay matches what the bot "knows."

**Why it works this way** The two queries both _read_ position, rotation, velocity, and health, and only `me` _writes_, and what it writes (`SensorSnapshot`) isn't in `everyone` at all. Since shared reads never conflict, there's no B0001 and no `ParamSet`: the data layout, not a clever workaround, removes the conflict. This is the toy-ECS lesson from Unit 1.15 at full scale. If you'd tried to store sensor results in a component that `everyone` also reads, you'd have had to snapshot first.

The all-pairs loop is O(n²). For 4–16 bots that's at most 256 cheap checks per tick, far below anything measurable. If you ever have hundreds of agents you'd switch to a spatial query (Avian's shape intersection queries use its broad-phase acceleration structures). Knowing _when not to optimise_ is part of the lesson. The sensor system is a pure function of world state, which keeps bots from cheating: they can only know what this function puts in the snapshot.

**Checkpoints**

1. Why don't `everyone` and `me` conflict even though they read the same components?
2. How do you stop a tank from detecting itself?
3. What's the cost of the O(n²) loop at your bot counts, and when would you change it?
4. Why is radar aligned to the turret a _strategic_ choice rather than just a simplification?

---

## Unit 3.10: Timing, Determinism, and Headless Simulation

**Effort:** L · **Prerequisites:** 3.9, 2.8, 1.11

**Objectives**

- Write down the complete per-tick schedule and justify every ordering constraint.
- Make matches **reproducible**: same seed + same roster + same config ⇒ same result, on the same machine and build.
- Run the simulation **headless** and faster than real time, driven from a test.

**Resources**

- Avian docs: the `schedule` module (`PhysicsSchedule`, `PhysicsSystems` sets) and the `enhanced-determinism` feature description.
- Bevy docs: `TimeUpdateStrategy`, `MinimalPlugins`, `ScheduleRunnerPlugin`.
- Crates: `rand` with `rand_chacha` (`ChaCha8Rng::seed_from_u64`).

**Exercise**

1. **Schedule diagram.** Write `SCHEDULE.md` listing every system, its set, and why it's there (use the diagram at the start of Phase 3 as a template). For each `.after()`/`.chain()`, write the reason in one line.
2. **Hunt nondeterminism.** Add a `MatchSeed` resource holding a seeded `ChaCha8Rng`; derive spawn jitter and each brain's `RoundInfo::seed` from it. Audit for: any `HashMap` iterated where order affects results (switch to `BTreeMap`, or sort); any randomness from `thread_rng()`; any logic depending on the order in which several systems wrote the same message type; any use of wall-clock time inside the simulation.
3. **Headless runner.** In `arena_game`, add a `SimPlugins` group containing every simulation plugin but no rendering, HUD, or gizmos. Write:

```
pub fn run_headless_match(seed: u64, roster: Roster, cfg: ArenaConfig) -> RoundResult {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin /* + whatever else Avian needs */))
       .add_plugins(SimPlugins)
       .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 64.0)))
       .insert_resource(seed_resource(seed)).insert_resource(roster).insert_resource(cfg);
    loop {
        app.update();
        if let Some(r) = app.world().get_resource::<RoundResult>() { return r.clone(); }
    }
}
```

(If Avian or Bevy panics about a missing plugin or resource, the message names it; add it. That's normal for headless setups and worth learning to read.) Then write an integration test: run the same seed twice and `assert_eq!` the results (winner, ticks, per-bot damage); run a different seed and see the result change.

**Success criteria:** identical seeds give identical results in repeated test runs; a 3-minute (simulated) round runs headless in a few seconds; `SCHEDULE.md` explains every ordering.

**Why it works this way** Reproducibility is what turns a simulation into something you can debug and measure: a bug that happens "sometimes" becomes a seed you can replay, and a strategy comparison becomes an experiment rather than an anecdote. Nondeterminism creeps in through a small set of doors: variable timesteps (closed by `FixedUpdate` plus a manual time strategy headless), unseeded randomness (closed by one seeded RNG), `HashMap`'s randomised iteration (from Unit 1.11; closed by ordered maps or sorting), and order-dependent consumption of messages written by unordered systems (closed by explicit ordering or by making consumers order-independent, e.g. summing damage). Floating-point results can also differ _across_ CPUs, compilers, or build settings; Avian's `enhanced-determinism` feature tackles cross-platform differences in its own maths, but full cross-machine lockstep determinism is a much bigger topic (and a reason some networked games choose Rapier or fixed-point maths). This course aims for same-machine, same-build reproducibility, which is enough for testing and tournaments.

`TimeUpdateStrategy::ManualDuration` makes each `app.update()` advance time by exactly one fixed step, so `FixedUpdate` and physics run exactly once per update regardless of how fast the machine is. That's what lets a headless match run many times faster than real time.

**Checkpoints**

1. Which state must be computed before physics runs, and which after?
2. List four sources of nondeterminism and how you closed each.
3. Why should randomness be an explicit, seeded dependency rather than calls scattered through systems?
4. Why is reproducibility valuable even for a single-player, offline game?

---

## Unit 3.11: Bot Personalities and Strategy

**Effort:** L (open-ended) · **Prerequisites:** 3.10, 1.5

**Objectives**

- Write at least three genuinely different strategies, each an internal enum state machine.
- Implement **predictive aiming** (leading a moving target).
- Keep all strategy in `arena_core`, tested without Bevy.

**Resources**

- RoboWiki (the RoboCode community wiki) for classic strategy ideas: search for "RoboWiki linear targeting", "wave surfing", "radar lock". Concepts transfer directly even though the API differs.
- _Game Programming Patterns_, "State" (again, now for AI).

**Exercise.** Suggested personalities (pick at least three):

- **Tracker** (from 3.9): lock on, approach, shoot.
- **Sniper:** keep distance near a preferred range, fire only when the predicted hit chance is high, using linear lead: with the target's relative position **p**, velocity **v**, and shell speed _s_, solve |**p** + **v**t| = *s*t for the smallest positive _t_, i.e. (**v**·**v** − *s*²)t² + 2(**p**·**v**)t + **p**·**p** = 0, then aim at **p** + **v**t. Reconstruct **p** and **v** in your own frame from the radar contact's distance, bearing, heading, and speed.
- **Dodger:** move perpendicular to the last known threat, reverse direction randomly (seeded RNG) or on hit.
- **Wall-hugger:** patrol along walls with the turret facing inward (the classic RoboCode "Walls" bot).
- **Opportunist:** target the weakest visible enemy; retreat when own health is low.

Each keeps a small memory, e.g. `enum Mode { Search, Track { id: BotId, lost_for: u32 }, Evade { ticks: u32 } }` plus a `BTreeMap<BotId, LastSeen>` (ordered, for determinism). Unit-test the aiming maths separately (a stationary target, a target moving perpendicular, a target moving away faster than the shell, where there's no solution).

**Success criteria:** watching a match, you can tell the personalities apart without reading their names; the Sniper hits a straight-line mover far more often than the Tracker does.

**Why it works this way** The point of the capstone isn't that bots compile; it's that the architecture makes interesting algorithmic behaviour cheap to express and test. Each brain is a small state machine over sensor snapshots, which is exactly the enum-and-match style from Unit 1.5. Differences between personalities live entirely in strategy code; the simulation engine stays identical for all of them. Predictive aiming is a nice example of the value of the `Sensors` design: because contacts include heading and speed, a brain can reason about the future without any access to the engine.

**Checkpoints**

1. Which behaviours belong in strategy code, and which must stay in the simulation?
2. Why does predictive aiming need the contact's heading and speed, not just its position?
3. When does the lead equation have no positive solution, and what should the brain do then?

---

## Unit 3.12 (Stretch within the Capstone): Tournaments, Instrumentation, and Balancing

**Effort:** L · **Prerequisites:** 3.11, 3.10

**Objectives**

- Build a headless tournament runner; collect per-bot statistics.
- Tell real strategy differences from noise.
- Tune game parameters from data, not intuition.

**Exercise**

- Add stats collection systems (reading existing messages, so combat code doesn't change): shots fired, shots hit, damage dealt and taken, ticks survived, kills, placement.
- A binary `arena_tournament` (in `arena_game/src/bin/`) that runs round-robin 1v1s _and_ 4-bot free-for-alls, 50+ seeds per pairing, **swapping spawn positions** for fairness, printing a table and writing a CSV. Run the batches in parallel across CPU cores (one headless `App` per thread, e.g. with `std::thread::scope`); each app is independent, so this is straightforward.
- Report each win rate with an approximate 95% interval: _p_ ± 1.96·√(_p_(1−*p*)/_n_). If two bots' intervals overlap heavily, you haven't shown one is better.
- Move balance knobs (`TankStats`, `Weapon`, sensor ranges) into a `balance.toml` loaded at startup (`serde` + `toml`, or your Unit 1.6 parser). Find one parameter change that flips which personality dominates, and explain why in terms of the strategies.

**Success criteria:** a full tournament of your bots completes in minutes; you can make and defend a data-backed claim like "Sniper beats Tracker in 1v1 (71% ± 6%) but loses in free-for-all because it's slow to acquire targets."

**Why it works this way** Watching a handful of matches produces anecdotes; hundreds of seeded headless matches produce evidence. This is the simulation-as-experiment mindset behind the management and tactics sims that motivated the project. Position swapping and many seeds remove spawn luck; the confidence interval tells you how many matches you need before believing a difference. Stats are gathered by _additional readers_ of the same messages, which demonstrates the payoff of message-based decoupling: a whole new feature with zero changes to combat code. Running independent `App`s on separate threads works because nothing is shared between them; that's the Send/Sync story from Phase 1 at the level of whole simulations.

**Checkpoints**

1. Why swap spawn positions between matches?
2. With 20 matches and a 60% win rate, is a bot clearly better than a 50% bot? Roughly how many matches would you need?
3. Why is it significant that stats collection required no changes to combat systems?

---

## Unit 3.13: Capstone Completion Gate and Retrospective

**Effort:** S · **Prerequisites:** 3.1–3.11 (3.12 recommended)

**The capstone is complete when a round with at least four bots running at least three different brain implementations does all of the following with no human input:** begins; bots move under physics; bots scan and acquire targets on radar; bots fire; shells travel physically and collide; hits cause damage; `last_hit` is reported to victims; tanks are eliminated; a winner (or a timeout draw) is declared; the next round starts cleanly.

**Architecture checklist**

- Brains depend only on `arena_core`; no Bevy or Avian types in any brain.
- All brain output passes through `Action::sanitized()` and game-enforced limits (cooldown, turn rates).
- Global state is in resources and states; per-tank state in components; cross-system gameplay signals are messages.
- The physics engine alone writes tank and shell positions.
- Every ordering constraint is written down with its reason.
- The same seed reproduces the same match headless.

**Retrospective exercise:** without looking at code, write one paragraph each explaining: why `BotBrain` is a trait and why it's stored as `Box<dyn BotBrain>`; why it requires `Send + Sync`; why sensors are owned snapshots; why the physics engine rather than the bot owns motion; why collision and damage are separate; why the simulation runs on a fixed step; why forty bots don't need forty copies of the gameplay pipeline; and why a scripting language could replace a Rust brain without redesigning the simulation. If each paragraph comes easily, you've learned much more than "how to make a Bevy tank game."

---

# PHASE 4: Optional Stretch Modules

**Everything in this phase is optional.** Start it only once the Unit 3.13 gate passes. The modules are independent of one another.

---

## Unit 4.1 (Optional): Game Feel, HUD, and Visual Polish

**Effort:** M–L · **Prerequisites:** 3.13

**Objectives**

- Make a match readable without the console: health bars, a round timer, a scoreboard, a winner banner.
- Add feedback (hit flashes, muzzle flash, small camera shake, explosion fade) **without touching the simulation**.
- Replace rectangles with sprites.

**Resources**

- Bevy UI examples: `examples/ui/` (text, nodes/flexbox layout), `examples/2d/sprite_sheet.rs`, `examples/2d/sprite_flipping.rs`.
- Free CC0 art: Kenney's top-down tank packs (kenney.nl, "Top-down Tanks Redux").
- Optional: `bevy_egui` for a developer panel (sliders for balance values, bot selection); check its Bevy 0.19-compatible version.

**Exercise:** a `PresentationPlugin` that only _reads_ simulation state and messages (`Health`, `RoundResult`, `SimTick`, `DamageDealt`, `TankDestroyed`) to drive visuals: world-space health bars as child sprites scaled by health fraction; a UI node for the timer and per-bot scores; a banner on `OnEnter(Finished)`; a brief colour flash on the damaged tank; particles or a fading circle on destruction. Add a key to toggle sensor gizmos per bot.

**Success criteria:** the headless tournament from 3.12 still builds and gives _identical_ results with and without `PresentationPlugin` in the app, which proves the presentation layer is strictly downstream of the simulation.

**Why it works this way** Presentation is a _consumer_ of simulation state, never a producer. Keeping that direction strict means you can iterate freely on visuals without risking gameplay bugs, and a headless build is literally the same simulation minus one plugin. Effects triggered from messages (`DamageDealt` → flash) are the observer/message decoupling from Phase 2 applied to "juice."

**Checkpoints**

1. How does the identical-results test prove the presentation layer is non-invasive?
2. Why are health bars better driven by `Health` components than by `DamageDealt` messages, and flashes the other way round?

---

## Unit 4.2 (Optional): Runtime-Scripted Bots with Rhai, and a Real Lesson in Sandboxing

**Effort:** XL · **Prerequisites:** 3.13 (3.10 and 3.12 strongly recommended)

This module is framed as **retrofitting scripting onto a working game.** If the `Sensors → Action` boundary was designed well, the rest of the game shouldn't change at all: a scripted bot is just another `BotBrain`.

### Why Rhai (and when to override that)

**Rhai** is an embedded scripting language written in pure Rust. It has Rust-like syntax, no C toolchain or FFI layer, straightforward registration of Rust types and functions, and first-class configurable safety limits. With its `sync` feature, the engine and compiled scripts are `Send + Sync`, which you need in order to put them in components (Unit 1.8 again). It's interpreted and slower than Lua, but a few bots ticking at 64 Hz need microseconds per tick, so speed doesn't matter here.

**Lua** via `mlua` is the reasonable alternative: faster (especially LuaJIT or Luau), a huge ecosystem, and familiar to modders. Luau in particular has a well-developed sandboxing story. The costs are a C dependency and a foreign runtime with its own memory model. Choose Lua if you expect others to write lots of bots and want a widely known language; choose Rhai (the default here) for simplicity and a pure-Rust build. **WebAssembly** (e.g. `wasmtime` with fuel metering) is the strongest isolation option and lets bot authors write in Rust itself; it's outlined in 4.3 as a further step.

### Threat model first (the security teaching moment)

Before writing code, write down **who writes the scripts** and **what they might do**. There are three distinct situations:

| Who writes bots           | Realistic risk                                                                   | What's sufficient                                                                                                                            |
| ------------------------- | -------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| You only                  | Bugs: infinite loops, huge arrays, NaN outputs                                   | Rhai limits + output validation                                                                                                              |
| Friends / a local club    | Bugs plus deliberate cheating (reading things they shouldn't, stalling the game) | The above + a strictly minimal API + per-tick budgets + a fault policy                                                                       |
| Strangers on the internet | Deliberate attacks on the host                                                   | A language-level sandbox is **not** enough on its own; add process or OS isolation (separate process, container, seccomp) or use WebAssembly |

Then enumerate threats and the control for each:

- **CPU exhaustion** (`loop {}`): `engine.set_max_operations(n)` bounds the work per call; an `on_progress` callback can also enforce a wall-clock deadline. On violation, the call returns an error and that bot forfeits the tick.
- **Memory exhaustion** (building huge strings, arrays, or maps): `set_max_string_size`, `set_max_array_size`, `set_max_map_size`; plus `set_max_call_levels` and `set_max_expr_depths` against stack exhaustion via deep recursion or deeply nested expressions. Some limits are unlimited by default. **Set every one explicitly.**
- **Capability leakage:** Rhai has no file, network, or process access unless the host registers functions that provide it. So the rule is simple: register only pure getters on `Sensors` and pure maths helpers. Never register anything that touches the `World`, `Commands`, the filesystem, or the clock. Disable `eval` so scripts can't construct code at runtime (`engine.disable_symbol("eval")`).
- **Information leakage / cheating:** the script sees exactly the `Sensors` snapshot, which is the same information boundary compiled bots have. This is where Unit 3.5's "what's deliberately missing" list becomes a security property.
- **Host corruption via outputs:** scripts return data that the host converts into an `Action` and runs through `sanitized()`; type mismatches or missing keys produce a default action, never a panic.
- **Supply chain:** the host depends on crates you didn't write. Run `cargo audit` (RustSec advisory database) and consider `cargo deny`. Avian 0.7's own changelog includes migrating off a dependency flagged by a RustSec advisory, which is a real example of why this matters.

### Exercise

1. Add `rhai = { version = "1", features = ["sync"] }` to `arena_game` (or a new `arena_script` crate so `arena_core` stays dependency-free).
2. Implement `struct RhaiBrain { engine: Arc<Engine>, ast: AST, memory: rhai::Map, name: String }` implementing `BotBrain`. `tick` converts `Sensors` into a Rhai map (or registers `Sensors` as a custom type with getters), calls the script's `tick` function with the sensors and the bot's `memory` map, and converts the returned map into an `Action`. Script functions in Rhai can't see script-level global variables (by design), so persistent memory has to be passed in and returned explicitly. That's another small sandboxing property worth noticing.

   ```
   // bots/coward.rhai  (the host passes Sensors as a flattened map: s.health, s.turret, s.radar[...])
   fn tick(s, mem) {
       let a = #{ throttle: 1.0, turn: 0.0, turret_turn: 0.5, fire: false };
       if s.health < 30.0 { a.turn = 1.0; a.throttle = -1.0; }
       else if s.radar.len() > 0 {
           let c = s.radar[0];
           a.turret_turn = c.bearing - s.turret;
           a.fire = abs(c.bearing - s.turret) < 0.05;
       }
       #{ action: a, mem: mem }
   }
   ```

3. Configure the engine with every limit above; create it once and share it (`Arc<Engine>`).
4. **Hot reload:** poll each script file's modification time every second (or use the `notify` crate); on change, recompile; if compilation fails, log the error and **keep running the previous AST**.
5. **Fault policy:** if a script errors (limit exceeded, runtime error, bad return shape) three ticks in a row, mark the bot `Faulted` and idle it. Surface the error text in the HUD/log.
6. **Red-team your own sandbox:** write `bots/evil_*.rhai` scripts that try an infinite loop, unbounded recursion, a 100 MB string, returning `throttle: "fast"`, returning NaN, and calling `eval`. Each must be contained without crashing or stalling the game. Add them to an automated test.

**Success criteria:** the simulation cannot tell whether an `Action` came from Rust or Rhai; editing a script changes behaviour within a second without restarting; every evil script is contained; a Rhai bot can take part in the 3.12 tournament.

**Why it works this way** A stable domain interface lets the implementation technology behind it change: nothing in physics, combat, sensors, or rounds knows scripts exist. The security design works in layers, each covering what the previous one can't: the _API surface_ decides what a script can even ask for (capabilities); _resource limits_ bound what it can consume; _output validation_ protects the host from what it returns; _fault policy_ keeps one bad bot from degrading the match; and _process isolation_ is the backstop when the author may be hostile, because an interpreter bug or a limit you forgot becomes a host compromise without it. This "defence in depth, starting from a written threat model" is the same way you'd reason about a public web endpoint behind Nginx: the proxy, the app's validation, rate limits, and container isolation each cover a different failure.

Note the contrast with Unit 3.6: compiled Rust brains get none of these protections. They're trusted code. Scripting is what makes it _possible_ to accept code you don't trust, and only because the host mediates everything.

**Checkpoints**

1. Why is "embedded scripting language" not the same thing as "secure sandbox"?
2. Which Rhai limits address CPU, memory, and stack exhaustion respectively?
3. Why is keeping the previous AST on a failed reload the right behaviour?
4. For bots submitted by strangers, what would you add beyond Rhai's limits, and why?

---

## Unit 4.3 (Optional): Doors to Your Other Interests

These are pointers, not full units. Each builds directly on something the capstone already has.

- **Tournament and ladder system.** Persist every headless match to **SQLite** (`rusqlite` for simple synchronous use, or `sqlx` if you want async and compile-time-checked queries). A natural schema: `bots(id, name, version, source_hash)`, `matches(id, seed, config_hash, started_at)`, `participants(match_id, bot_id, placement, damage_dealt, damage_taken, shots, hits, ticks_alive)`. Add Elo or Glicko-2 ratings computed from `participants`. Because matches are reproducible from `(seed, roster versions, config)`, you can store those instead of recordings and **replay any match on demand**. A small Go or Python web front end over the same SQLite file would fit your existing stack nicely.
- **Stat tracking and replays.** Record each tick's `Sensors` and `Action` per bot (behind a feature flag) to debug a brain offline, or feed recorded `Sensors` back into a modified brain to see what it _would_ have done.
- **Management / tycoon layer.** Treat a round as one "event" inside a larger game loop: between rounds, a `Career` state spends earnings on parts (armour, engines, radar range) that modify `TankStats` and `SensorConfig`; bots are "drivers" with contracts; repair costs create economic pressure. Bevy `States` (maybe `SubStates`) model the career flow, and the arena becomes the execution engine underneath a management sim, much as a football manager game runs a match engine.
- **Coach/tactics layer.** Give a team of bots shared radar (a team-level resource written by all, read by all) and let a "coach" brain assign roles each round. This is a classic multi-agent coordination problem.
- **Construction and physics.** Avian supports joints (revolute joints with motors, prismatic joints), so tanks could be _assembled_ from parts: a hull, a separately simulated turret body, tracks. That's the first step toward the KSP-style "build it and see if it survives" loop.
- **WebAssembly bots.** Compile bots written in Rust to WASM and run them in `wasmtime` with fuel metering and memory limits: strong isolation, near-native speed, and bot authors can use real Rust. This is the right tool if the ladder ever accepts outside submissions.
- **Upgrading Bevy.** When Bevy 0.20 and the matching Avian release are out, upgrading the finished capstone using the official migration guide is an excellent consolidation exercise: you'll re-encounter every API you used, in context. Do it on a branch, one crate at a time, with the headless determinism test as your safety net.
- **Networking (caution).** Multiplayer or remote bots are where your self-described weak areas (security and networking) would concentrate. If you go there, start with the threat-model habit from 4.2 and read up on lockstep vs server-authoritative architectures _before_ writing code.

---

# Appendices

## Appendix A: Compiler and Runtime Error Field Guide

| Code                                      | Plain meaning                                                              | First thing to check                                                                                  |
| ----------------------------------------- | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| E0382                                     | Use of a moved value                                                       | Where did it move? Borrow instead, or restructure so the move happens last                            |
| E0499                                     | Two `&mut` to the same thing alive at once                                 | Split the data (fields, `split_at_mut`) or do a two-phase update                                      |
| E0502                                     | `&mut` while a `&` is still alive (or vice versa)                          | Is the shared borrow used after the mutation? Reorder or copy the needed value out first              |
| E0505                                     | Moving out while borrowed                                                  | The borrow must end before the move                                                                   |
| E0506                                     | Assigning to something that's borrowed                                     | Same as E0505, for assignment                                                                         |
| E0597 / E0716                             | Borrowed value doesn't live long enough / temporary dropped while borrowed | Store the value in a named variable that outlives the borrow, or return owned data                    |
| E0106                                     | Missing lifetime specifier                                                 | Which input does the output borrow from? Say so with `'a`                                             |
| E0277                                     | Trait bound not satisfied                                                  | Read which trait and which type; often `Send`/`Sync`, `Component`, `Copy`, or a missing derive        |
| E0038                                     | Trait isn't dyn-compatible                                                 | Generic methods or `Self`-returning methods; move them to a separate trait or add `where Self: Sized` |
| E0308                                     | Mismatched types                                                           | Often a missing `&`, a `;` turning a value into `()`, or `f32` vs `f64`                               |
| Bevy B0001                                | Query conflict within one system (panics at startup)                       | Add `Without<..>` filters or use `ParamSet`                                                           |
| Bevy B0002                                | `Res<T>` and `ResMut<T>` of the same resource in one system                | Keep only `ResMut<T>`                                                                                 |
| Missing resource panic                    | A system asked for `Res<T>` that was never inserted                        | Add the plugin that provides it, `init_resource`, or use `Option<Res<T>>`                             |
| "system param invalid" / `single()` error | Zero or several entities matched a single-entity query                     | Make the system return `Result` and use `?`, or rethink the assumption                                |

When you meet a borrow error: **name the value, its owner, every live borrow, and where each borrow is last used, before changing any code.** Resist `.clone()` until you can say why the borrow is a problem.

## Appendix B: Version-Drift Checklist (use when resuming after months away)

1. `rustup update`; note the new stable version.
2. Check crates.io for the latest Bevy and Avian; read the Avian compatibility table.
3. **If you're mid-course, don't upgrade.** `Cargo.lock` keeps you on the versions the course was written for.
4. If you do upgrade (after the capstone): read _every_ Bevy migration guide between your version and the target in order, then Avian's; upgrade one version step at a time; run the headless determinism test after each step.
5. When a tutorial disagrees with this document, check which Bevy version it targets before assuming either is wrong.

## Appendix C: Go / C++ / Python → Rust and Bevy Map

| You know                          | Rust / Bevy equivalent                             | Where the analogy breaks                                                          |
| --------------------------------- | -------------------------------------------------- | --------------------------------------------------------------------------------- |
| GC (Go, Python)                   | Ownership + `Drop`                                 | Checked at compile time; no pauses; you must structure data so ownership is clear |
| C++ RAII, `std::move`             | Ownership, moves by default                        | Moved-from values are unusable, and the compiler checks it                        |
| C++ `const T&` / `T&`             | `&T` / `&mut T`                                    | Aliasing rules are enforced; no dangling references in safe code                  |
| Go interface                      | `dyn Trait` (fat pointer)                          | Implemented explicitly; generics offer static dispatch too                        |
| C++ templates                     | Generics + trait bounds                            | Bounds are checked at the definition site                                         |
| Go `if err != nil { return err }` | `?` on `Result`                                    | Errors are typed; can't use the value on the error path                           |
| Python exceptions                 | `Result` / `panic!`                                | Recoverable errors are values; panics are for bugs                                |
| Go goroutines + `-race`           | Threads + `Send`/`Sync`                            | Data races are compile errors in safe Rust                                        |
| Go package                        | Crate (compilation) + modules (namespacing)        | Modules don't have to follow directories one-to-one                               |
| `go.mod` / `go.sum`               | `Cargo.toml` / `Cargo.lock`                        | Features are a Cargo-specific concept                                             |
| OOP game object with methods      | Entity + components + systems                      | Behaviour lives in systems, not on objects                                        |
| Observer pattern / event bus      | Bevy messages (buffered) and observers (immediate) | Messages are pulled per system, per tick                                          |
| Web middleware input validation   | `Action::sanitized()`, script limits               | Same principle: validate at the trust boundary                                    |

## Appendix D: Mastery Gates at a Glance

- **Before Bevy (end of Phase 1):** ownership, borrowing, lifetimes (reading level), `Option`/`Result`, traits, generics vs `dyn`, `Send`/`Sync`, closures, iterators, collections, modules and workspaces, and why the toy ECS avoids borrow conflicts.
- **Before physics (end of Phase 2):** Entity, Component, Resource, Query (data and filters), Commands and sync points, messages vs observers, schedules, sets, plugins, states, `Transform` vs `GlobalTransform`, `Time` vs `Time<Fixed>`; and the ability to read any system signature as a data-access declaration.
- **Before AI (Unit 3.4 done):** rigid body vs collider, dynamic vs static, velocity vs force vs impulse, damping, CCD and tunnelling, collision layers, the physics schedule, why physics owns position.
- **Before scripting (Phase 3 done):** the full `Sensors → BotBrain → Action → adapter → physics → messages → consequences` loop, why the bot can't manipulate ECS state, and what makes a match reproducible.

The real curriculum is the progression of questions you can answer unaided: first _"why did this value move?"_, then _"generic or trait object?"_, then _"what data does this system touch, and what can run in parallel?"_, then _"who is authoritative for this physical state?"_, and finally _"is this a simulation concern or a decision concern, and does the bot actually need to know this?"_
