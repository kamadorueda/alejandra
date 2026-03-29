# Alejandra: Vision & Strategy

## The Opportunity

With RFC 166 adopted and nixfmt crowned as the official Nix formatter,
the Nix formatting landscape has consolidated.
nixpkgs-fmt is archived. Alejandra is the only serious alternative standing.

This is not a threat — it is an opportunity.

**nixfmt is the formatter developers are required to use.
Alejandra should be the formatter developers choose to use.**

Every language ecosystem with an official formatter
also has a thriving alternative that developers love more:
Prettier vs Biome in JavaScript,
Black vs Ruff in Python,
gofmt vs gofumpt in Go.
The alternative wins by being faster, more opinionated, or more ergonomic.
Alejandra can be all three.

---

## Current State Assessment

### What We Have

- **Performance**: Rust-based, formats all of Nixpkgs in 14 seconds (4 threads).
  nixfmt is Haskell-based and processes one file at a time.
- **Style loyalty**: Users who try nixfmt come back.
  In [#404](https://github.com/kamadorueda/alejandra/issues/404),
  multiple users reported preferring Alejandra after testing RFC 166:
  *"After trying nixfmt-rfc-style, I have decided to keep using alejandra
  in my personal projects"* (6 thumbs up).
  *"I don't think I'm willing to use an RFC166 compliant formatter
  on any of my projects"* (6 thumbs up).
- **Web playground**: The only Nix formatter you can try in your browser,
  powered by WASM. This is a unique onboarding advantage.
- **Editor coverage**: VSCode, Neovim, Vim, GNU Emacs, Doom Emacs, pre-commit.
- **Public domain**: The Unlicense — maximum freedom for adoption.
- **Active development**: v4.0.0 shipped April 2025 with pipe operators,
  configurable indentation, and doc comment support.
  Web frontend actively being developed.

### What Users Are Asking For

An analysis of all 63 open issues and 98 closed issues reveals clear patterns:

| Category | Open Issues | Signal |
|---|---|---|
| Formatting/style fixes | 15+ | Core product quality |
| Skip-formatting directives | 3 (#292, #418, #463) | Most-requested missing feature |
| Comment handling bugs | 7+ | Reliability pain point |
| Configurability | 5+ (#298, #352, #387, #415, #360) | Growing demand |
| Line length / expression breaking | 8+ | Style completeness |
| Multiline string handling | 8+ | Correctness concern |
| Performance / stability | 4+ | Production readiness |
| Distribution / platform support | 6+ (#430, #449, #450) | Adoption barriers |

### What We Don't Have (Yet)

1. **No skip-formatting directives** (`# alejandra: off/on`).
   This is the single most impactful missing feature.
   Every mature formatter has this. Black, Prettier, rustfmt — all of them.
   Users need escape hatches for tables, ASCII art, and workarounds for bugs.
   ([#292](https://github.com/kamadorueda/alejandra/issues/292),
   [#418](https://github.com/kamadorueda/alejandra/issues/418),
   [#463](https://github.com/kamadorueda/alejandra/issues/463))

2. **Idempotency gaps**.
   Some files require multiple formatting passes to stabilize
   ([#250](https://github.com/kamadorueda/alejandra/issues/250),
   [#408](https://github.com/kamadorueda/alejandra/issues/408)).
   This erodes trust.

3. **Windows support** ([#430](https://github.com/kamadorueda/alejandra/issues/430)).

4. **crates.io publishing** ([#449](https://github.com/kamadorueda/alejandra/issues/449)).
   Rust developers expect `cargo install alejandra` to work.

5. **Limited configurability**.
   v4.0.0 opened the door with indentation options.
   Users want more: spacing in containers ([#360](https://github.com/kamadorueda/alejandra/issues/360)),
   line length limits, attribute sorting ([#276](https://github.com/kamadorueda/alejandra/issues/276)).

---

## Strategic Positioning

### Identity: "The Formatter Developers Love"

Rename the tagline from "The Uncompromising Nix Code Formatter"
to something that communicates choice and quality:

> **Alejandra — The Nix formatter that gets out of your way.**

"Uncompromising" was powerful when the market had no standard.
Now that nixfmt *is* the standard, "uncompromising" sounds like rigidity.
The new positioning should communicate:
- I chose this because it's better for *my* workflow
- It respects my preferences while still being opinionated
- It's fast and stays out of my way

### The Two-Audience Strategy

**Audience 1: Personal/team projects** (primary growth market)
- Developers who use Nix for their own configs, flakes, and projects
- They don't need RFC 166 compliance
- They want: speed, beautiful output, good editor integration, configurability
- Message: *"Your Nix code, your style. Just faster and cleaner."*

**Audience 2: Nixpkgs contributors** (secondary, compatibility market)
- They must use nixfmt for Nixpkgs PRs
- But they use Alejandra for everything else
- Potential: offer an RFC 166 compatibility mode so they only need one tool
- Message: *"One formatter, every context."*

### Differentiation Matrix

| Capability | nixfmt | Alejandra | Advantage |
|---|---|---|---|
| Speed | Slow (Haskell, single-file) | Fast (Rust, multi-threaded) | **Alejandra** |
| Style | RFC 166 (committee-designed) | Community-evolved, principled | **Alejandra** (subjective, but validated by user retention) |
| Web playground | No | Yes (WASM) | **Alejandra** |
| Configurability | Minimal | Growing (indentation, more planned) | **Alejandra** |
| Official status | Yes (required for Nixpkgs) | No | nixfmt |
| Broken code handling | Yes (partial parsing) | No | nixfmt |
| Skip-formatting | No | Planned | **Alejandra** (once shipped) |
| Clean diffs | Moderate | Strong (trailing commas, vertical layout) | **Alejandra** |
| Library/API | Limited | Rust crate + WASM | **Alejandra** |

---

## Unique Value Proposition

Alejandra's moat is built on four pillars:

### 1. Speed as a Feature

Formatting should be invisible. At 14 seconds for all of Nixpkgs
(vs minutes for nixfmt), Alejandra is fast enough for
save-on-format in editors, CI pipelines, and pre-commit hooks
without anyone noticing. Speed is not a nice-to-have —
it's what makes "transparent formatting" possible.

### 2. Style That Developers Prefer

The evidence is in the issues. Users try nixfmt and come back.
Alejandra's trailing commas, vertical layouts, and clean diff philosophy
produce code that is easier to read and modify.
The style guide (STYLE.md) makes principled arguments —
not "this is the standard" but "this is *better* and here's why."

### 3. The Web Playground

No other Nix formatter lets you try it before installing.
The WASM-powered playground is an acquisition funnel:
curiosity → try it → love the output → install it.
This is how Prettier won JavaScript.

### 4. Embeddability

The Rust library crate and WASM bindings make Alejandra
embeddable in other tools: editors, CI systems, web apps,
language servers, and AI coding assistants.
nixfmt's Haskell implementation makes embedding much harder.

---

## Roadmap: Prioritized Initiatives

### Tier 1: Ship or Lose Users (next release)

These are blocking adoption or eroding trust.

**1.1 Skip-Formatting Directives**
- Implement `# alejandra: off` / `# alejandra: on` comment directives
- Also support `# fmt: off` / `# fmt: on` for familiarity (Black, Prettier convention)
- This is the #1 most-requested feature across issues
  ([#292](https://github.com/kamadorueda/alejandra/issues/292),
  [#418](https://github.com/kamadorueda/alejandra/issues/418),
  [#463](https://github.com/kamadorueda/alejandra/issues/463))
- Unlocks: workaround for bugs, hand-formatted tables, ASCII art in comments

**1.2 Idempotency Guarantee**
- Formatting the same file twice must always produce identical output
- Add CI check: format → format again → assert no diff
- Fixes [#250](https://github.com/kamadorueda/alejandra/issues/250),
  [#408](https://github.com/kamadorueda/alejandra/issues/408)

**1.3 Comment Handling Fixes**
- Fix extra newlines in inherit comments ([#407](https://github.com/kamadorueda/alejandra/issues/407))
- Fix missing indentation in function body comments ([#383](https://github.com/kamadorueda/alejandra/issues/383))
- Fix multiple argument comments ([#375](https://github.com/kamadorueda/alejandra/issues/375))
- Comments are where formatting bugs are most *visible* to users

**1.4 Multiline String Correctness**
- Fix content modification in multiline strings ([#409](https://github.com/kamadorueda/alejandra/issues/409))
- Fix string escape handling ([#442](https://github.com/kamadorueda/alejandra/issues/442))
- Any formatter that changes the *meaning* of code will lose trust permanently

### Tier 2: Grow the Market (v5.0 cycle)

**2.1 Expanded Configuration**
- Graduate indentation config from experimental to stable
- Add configurable options for:
  - Spacing in containers: `[x]` vs `[ x ]` ([#360](https://github.com/kamadorueda/alejandra/issues/360))
  - Max line width ([#349](https://github.com/kamadorueda/alejandra/issues/349))
- Philosophy: opinionated defaults, escape hatches for teams.
  Not "configure everything" but "configure the things people actually fight about."

**2.2 RFC 166 Compatibility Mode**
- Optional mode that produces nixfmt-compatible output
- Allows Nixpkgs contributors to use one tool for everything
- Reduces the "two formatters" friction that causes people to just pick nixfmt
- Implementation: a preset in `alejandra.toml` (`style = "rfc166"`)
- This is strategically important even if few use it —
  it removes the objection "but I also contribute to Nixpkgs"

**2.3 Distribution Expansion**
- Publish to crates.io ([#449](https://github.com/kamadorueda/alejandra/issues/449))
- Windows support ([#430](https://github.com/kamadorueda/alejandra/issues/430))
- macOS binary fixes ([#470](https://github.com/kamadorueda/alejandra/issues/470))
- .gitignore support ([#428](https://github.com/kamadorueda/alejandra/issues/428))

**2.4 Attribute Sorting**
- Optional alphabetical sorting of set attributes
  ([#276](https://github.com/kamadorueda/alejandra/issues/276))
- This is a power feature that nixfmt doesn't offer
- Should be opt-in via config

### Tier 3: Expand the Vision (long-term)

**3.1 Alejandra as a Platform**
- Stable Rust API for embedding in other tools
- Language server integration (format-on-type, range formatting)
- AI assistant integration (format Nix output from LLMs)
- The WASM build already enables browser-based tooling

**3.2 Diff-Aware Formatting**
- Format only changed lines (useful for incremental adoption in large codebases)
- `alejandra --diff HEAD` to format only your changes
- Solves the "I can't adopt a formatter because the initial diff is too large" problem

**3.3 Migration Tooling**
- `alejandra --from nixfmt` to convert from nixfmt style
- `alejandra --from nixpkgs-fmt` for legacy migrations
- Lower the switching cost to zero

**3.4 Community Style Presets**
- Ship named presets: `default`, `compact`, `rfc166`, `nixpkgs-classic`
- Let communities define and share their preferred style
- `alejandra.toml`: `preset = "compact"`

---

## Community & Adoption Strategy

### 1. Make Switching Effortless

- One-command migration from nixfmt: `alejandra --migrate .`
- Side-by-side diff tool (the web playground already supports this)
- Document exactly how Alejandra differs from nixfmt and why

### 2. Win the "First Five Minutes"

- The web playground is the #1 acquisition tool. Invest in it.
- Add a "nixfmt vs Alejandra" comparison mode to the playground
- Show the same input formatted by both tools, side by side
- Let the output speak for itself

### 3. Be Where Developers Are

- Publish to crates.io (Rust developers expect this)
- Homebrew is done — ensure it stays current
- Ensure Nixpkgs package tracks releases promptly
- GitHub Action for CI (`uses: kamadorueda/alejandra-action@v4`)

### 4. Build Social Proof

- Add a "Who uses Alejandra" section to the README
- Badges: `[![code style: alejandra](badge-url)](repo-url)` (already exists)
- Encourage projects to add the badge
- Collect testimonials from the issue tracker (there are many)

### 5. Engage, Don't Dictate

- The #95 comma debate showed that top-down style decisions create friction
- For new configurable options, use GitHub Discussions or polls
- Be transparent about trade-offs in STYLE.md (already good — keep doing this)
- Acknowledge RFC 166 as a valid choice, position Alejandra as the *better* choice

### 6. Reduce Contributor Friction

- Clear CONTRIBUTING.md (already exists)
- Label issues with `good first issue` for newcomers
- The 63 open issues are a goldmine of contribution opportunities
- Mentorship: respond to PRs quickly, guide contributors

---

## Metrics of Success

How we'll know the strategy is working:

| Metric | Current | 6-Month Target | 12-Month Target |
|---|---|---|---|
| GitHub stars | ~700+ | 1,000 | 1,500 |
| Open issues resolved | 63 open | < 40 open | < 25 open |
| crates.io downloads | 0 | 500/month | 2,000/month |
| Web playground monthly visits | Unknown | Track & grow | 2x baseline |
| Nixpkgs package installs | Unknown | Track | Growing quarter over quarter |
| Time to format Nixpkgs | 14s (4 threads) | 10s | 8s |

---

## Summary: The Path to Most-Loved

1. **Don't chase nixfmt.** Don't adopt RFC 166 as default. Own the alternative position.
2. **Fix trust issues first.** Idempotency, multiline strings, comment handling.
3. **Ship skip-formatting.** It's the unlock for everything else.
4. **Expand config thoughtfully.** Not "configure everything" — just the things people fight about.
5. **Offer RFC 166 as an option.** Remove the objection without losing identity.
6. **Invest in the playground.** It's the best acquisition funnel in the Nix ecosystem.
7. **Be faster, be embeddable, be everywhere.** Speed and portability are permanent advantages.

Alejandra doesn't need to be the official formatter.
It needs to be the one developers install anyway.
