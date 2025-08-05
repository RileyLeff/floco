# Floco Library Redesign: Technical Overview

## 1. Current Architecture

### 1.1 Core Design
Floco is a Rust library that creates constrained newtypes - types that guarantee their values meet user-defined predicates. At its heart:

- **`Floco<T, C>`**: A wrapper struct containing a value `T` and a phantom marker `C`
- **`Constrained<T>` trait**: Defines validation logic for a given constraint marker
- **`constrained_type!` macro**: Generates all the boilerplate for a new constrained type

### 1.2 Dual-Mode Validation System
The library currently operates in two modes:

**Default Mode (Compile-Time Validation):**
- Requires nightly Rust (`const_trait_impl`, `const_default` features)
- Validates default values at compile time
- Uses `const` trait implementations
- Fails to compile if defaults are invalid

**Runtime Mode (via `runtime-defaults` feature):**
- Works on stable Rust
- Validates defaults at first use
- Required for non-const types like `uom::Quantity`
- Panics at runtime if defaults are invalid

### 1.3 Current Implementation
```rust
// The trait uses conditional const based on feature
#[cfg_attr(not(feature = "runtime-defaults"), const_trait)]
pub trait Constrained<T>: Sized
where
    T: PartialOrd + Debug + Copy,  // Note: requires Copy
{
    fn is_valid(value: T) -> bool;
    fn emit_error(value: T) -> Self::Error;
}

// The macro generates different code based on cfg
#[cfg(not(feature = "runtime-defaults"))]
$crate::__floco_const_impl! {
    $crate::Constrained<$InnerType> for [<$TypeName Constraint>] {
        fn is_valid($val_id: $InnerType) -> bool { $validator }
        // ...
    }
}
```

## 2. Problems with Current Design

### 2.1 Macro Hygiene Issue (Critical)
**The Problem**: When a macro uses `#[cfg(feature = "...")]`, it checks for that feature in the crate where the macro is *expanded*, not where it's *defined*.

**Example**:
```rust
// User's Cargo.toml
floco = { git = "...", features = ["runtime-defaults"] }

// User's code
constrained_type! { /* ... */ }  // This checks for "runtime-defaults" in USER's crate!
```

**Result**: Users get confusing errors about missing features in their own crate, even though they correctly enabled the feature on floco.

### 2.2 Default Behavior Requires Nightly
- New users can't just `cargo add floco` and start using it
- Most of the ecosystem uses stable Rust
- Creates friction for adoption

### 2.3 Copy Trait Requirement Too Restrictive
- Limits usage to primitive numeric types
- Can't create constrained `String`, `Vec<T>`, or other heap-allocated types
- Prevents use with many domain types

### 2.4 Runtime Panics in Library Code
- `Default::default()` can panic in runtime mode
- Generally considered bad practice for libraries
- No way for users to handle validation failures gracefully

## 3. Proposed Solution

### 3.1 Feature Flag Restructuring

**Invert the defaults**:
```toml
[features]
default = []  # Runtime validation (stable Rust)
const-validation = []  # Opt-in compile-time validation (nightly)
```

**Benefits**:
- Works on stable Rust by default
- `uom` compatibility out of the box
- Nightly users can opt into stronger guarantees
- Aligns with Rust ecosystem practices

### 3.2 Fix Macro Hygiene

**Current approach** (broken):
```rust
// Checks feature in user's crate
#[cfg(feature = "runtime-defaults")]
impl Constrained<T> for Marker { ... }
```

**New approach**:
```rust
// In floco's lib.rs - build different modules based on features
#[cfg(feature = "const-validation")]
mod const_impl {
    // Implementation with const traits
}

#[cfg(not(feature = "const-validation"))]
mod runtime_impl {
    // Implementation without const traits
}

// Export the appropriate implementation
#[cfg(feature = "const-validation")]
pub use const_impl::*;

#[cfg(not(feature = "const-validation"))]
pub use runtime_impl::*;
```

Now the macro doesn't need any `cfg` attributes - it just uses whatever implementation is available.

### 3.3 Copy → Clone Migration

**Change all bounds**:
```rust
// Before
T: PartialOrd + Debug + Copy

// After
T: PartialOrd + Debug + Clone
```

**Update method signatures**:
```rust
// Before
pub fn get(&self) -> T { self.0 }

// After
pub fn get(&self) -> &T { &self.0 }
pub fn into_inner(self) -> T { self.0 }
```

**Update arithmetic operations**:
```rust
// For Copy types, clone() is optimized to memcpy
let result = self.0.clone().add(other.0.clone());
```

### 3.4 Panic Behavior (Defer)
For now, keep the current panic behavior in runtime mode. Future improvements could include:
- `TryDefault` trait for fallible defaults
- Build-time validation scripts
- Only allowing provably valid defaults

But these can be addressed in a future release.

## 4. Migration Path

### 4.1 For Floco Development
1. Ask any clarifying questions you deem necessary before starting
2. Use the search tool to gather context if you're unsure of the latest version of something. For example, thiserror 2.0 supports the long-awaited, now-stabilized error-in-core on stable rust.
3. Update feature flags and module structure
4. Change Copy to Clone throughout
5. Update all tests
6. Test with real projects using `uom`

### 4.2 For Users
Since we're inverting defaults, this is a **breaking change**:

**Current users on nightly** (using default):
- No change needed, everything continues working

**Current users on stable** (using `runtime-defaults`):
```toml
# Before
floco = { version = "0.2", features = ["runtime-defaults"] }

# After  
floco = { version = "0.3" }  # runtime is now default!
```

**Current users wanting const validation**:
```toml
# After
floco = { version = "0.3", features = ["const-validation"] }
```

## 5. Benefits of This Approach

1. **Broader Compatibility**: Works on stable Rust by default
2. **Better Ergonomics**: No confusing feature flag errors
3. **More Flexible**: Supports String, Vec, and other Clone types
4. **Cleaner Architecture**: Feature detection happens in floco, not user code
5. **Ecosystem Friendly**: Follows Rust best practices for optional nightly features

This redesign maintains all current functionality while fixing fundamental architectural issues and making the library more accessible to the broader Rust community.