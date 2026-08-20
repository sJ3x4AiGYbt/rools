# Contributing

## Table of Contents

- [Getting Started](#getting-started)
- [Branch Strategy](#branch-strategy)
- [Commit Convention](#commit-convention)
- [Code Style](#code-style)
- [Before Pushing](#before-pushing)
- [Release Process](#release-process)

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- `cargo fmt` and `cargo clippy` available (included with Rust)

### Setup

```bash
git clone https://github.com/<org>/<repo>.git
cd <repo>
cargo build
cargo test
```

---

## Branch Strategy

| Branch | Purpose |
|---|---|
| `main` | Stable, production-ready code |
| `staging` | Pre-production integration branch |
| `feature/<name>` | New features |
| `bugfix/<name>` | Bug fixes |
| `release/<version>` | Release preparation |

All changes must go through a pull request. Direct pushes to `main` are not allowed.

---

## Commit Convention

Commits follow this format:

```
<type>: <short description>

<optional body explaining why and what this changes>
```

**Rules:**
- Use the **imperative, present tense**: `add` not `added`, `fix` not `fixed`
- **No capital letter** at the beginning
- **No period** at the end
- Keep the subject line under 72 characters

**Types:**

| Type | When to use |
|---|---|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation only |
| `style` | Formatting, no logic change |
| `test` | Adding or updating tests |
| `chore` | Tooling, CI, dependencies |

**Example:**

```
feat: add user authentication via JWT

Implements login and token refresh endpoints.
Tokens expire after 1 hour and are rotated on each refresh.
```

---

## Code Style

### Naming

| Element | Convention | Example |
|---|---|---|
| Functions, methods, variables | `snake_case` | `parse_input` |
| Types, enums | `UpperCamelCase` | `TokenKind` |
| Constants | `UPPER_SNAKE_CASE` | `MAX_RETRY_COUNT` |
| Error constants | `UPPER_SNAKE_CASE` | `ERR_INVALID_PORT` |
| File names | `snake_case.rs` | `hello_world.rs` |

### Indentation

Use only spaces, 2 spaces per level.

### Braces

Opening brace on the same line as the declaration, preceded by a space

### Parentheses

Don't use in `if`, `while`, `for` conditions:

```rust
// ✅
if x > 0 {

// ❌
if (x > 0) {
```

### Spaces around operators

Except postfix and unary:

```rust
age += 1;
result = !flag;
size += -2 + 3 * (a + b);
```

### Formatting strings

Use the captured variable syntax consistently across the project:

```rust
// ✅ Preferred
println!("{x}");

// ❌ Avoid
println!("{}", x);
```

### Imports

Always import the **parent module**, never the function directly:

```rust
// ✅
use crate::front_of_house::hosting;
hosting::add_to_waitlist();

// ❌
use crate::front_of_house::hosting::add_to_waitlist;
add_to_waitlist();
```

Always use **absolute paths** from `crate::`. Do not use `self::` or `super::`.

**Exception — constants:** import constants directly rather than through their parent
module. Rust's captured-identifier format syntax (`format_args!("{FIELD_URL}")`, see
*Formatting strings* below) requires a bare identifier in scope; a qualified path like
`constants::FIELD_URL` cannot be used there. Since most constants exist to be
interpolated into output strings, keep them as direct imports:

```rust
// ✅
use crate::commands::webscout::common::constants::FIELD_URL;
format_args!("{FIELD_URL}");

// ❌ does not compile
use crate::commands::webscout::common::constants;
format_args!("{constants::FIELD_URL}");
```

**Exception — traits:** import traits directly rather than through their parent module.
Calling a trait method on a concrete type (e.g. `monitor.run()`) requires the trait
itself — not just its module — to be in scope, or the call fails to resolve:

```rust
// ✅
use crate::commands::sysviz::common::config::SystemMonitor;
monitor.run(&config); // resolves via the trait

// ❌ does not compile: `run` not found on `LinuxMonitor`
use crate::commands::sysviz::common::config;
monitor.run(&config);
```

### Comments

Use block comments only. Never use `//`. Doxygen format is recommended for public items:

```rust
/**
 * @brief Brief description of what this does
 * @param param_name What this parameter represents
 * @return What this returns
 */
```

### Integer Overflow

Always handle potential integer overflow explicitly. Use the appropriate method depending on the desired behavior:

| Method | Behavior |
|---|---|
| `wrapping_add`, `wrapping_mul`, … | Wrap on overflow |
| `checked_add`, `checked_mul`, … | Return `None` on overflow |
| `overflowing_add`, … | Return value + overflow flag |
| `saturating_add`, … | Clamp to min/max on overflow |

---

## Before Pushing

Run both commands and make sure they pass with **zero warnings**:

```bash
cargo fmt
cargo clippy -- -D warnings
```

A pull request that fails either check will not be reviewed.

---

## Release Process

Releases are built and distributed via **GitHub Releases**, supporting macOS, Linux, and Windows.

1. Create a `release/<version>` branch from `staging`
2. Update the version in `Cargo.toml`
3. Open a pull request to `main`
4. Once merged, tag the commit:
   ```bash
   git tag -a v<version> -m "release: v<version>"
   git push origin v<version>
   ```
5. The CI pipeline will automatically build binaries for all platforms and publish the GitHub Release

Binaries will be available for direct download on the [Releases page](../../releases).