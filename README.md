# Turso compilation failure

`cargo check --target wasm32-unknown-unknown`

```
   Compiling turso_core v0.4.0-pre.17 (https://github.com/tursodatabase/turso#fcfcc045)
error[E0283]: type annotations needed
   --> /Users/me/.cargo/git/checkouts/turso-455557cd4a2364c7/fcfcc04/core/mvcc/database/checkpoint_state_machine.rs:208:66
    |
208 |                     .is_none_or(|txid_max_old| b <= txid_max_old.into())
    |                                                  --              ^^^^
    |                                                  |
    |                                                  type must be known at this point
    |
    = note: multiple `impl`s satisfying `u64: PartialOrd<_>` found in the following crates: `candid`, `core`:
            - impl PartialOrd for u64;
            - impl PartialOrd<candid::types::number::Int> for u64;
            - impl PartialOrd<candid::types::number::Nat> for u64;
help: try using a fully qualified path to specify the expected types
    |
208 -                     .is_none_or(|txid_max_old| b <= txid_max_old.into())
208 +                     .is_none_or(|txid_max_old| b <= <std::num::NonZero<u64> as Into<T>>::into(txid_max_old))
    |

error[E0283]: type annotations needed
   --> /Users/me/.cargo/git/checkouts/turso-455557cd4a2364c7/fcfcc04/core/mvcc/database/checkpoint_state_machine.rs:218:67
    |
218 |                     .is_some_and(|txid_max_old| e <= txid_max_old.into())
    |                                                   --              ^^^^
    |                                                   |
    |                                                   type must be known at this point
    |
    = note: multiple `impl`s satisfying `u64: PartialOrd<_>` found in the following crates: `candid`, `core`:
            - impl PartialOrd for u64;
            - impl PartialOrd<candid::types::number::Int> for u64;
            - impl PartialOrd<candid::types::number::Nat> for u64;
help: try using a fully qualified path to specify the expected types
    |
218 -                     .is_some_and(|txid_max_old| e <= txid_max_old.into())
218 +                     .is_some_and(|txid_max_old| e <= <std::num::NonZero<u64> as Into<T>>::into(txid_max_old))
    |

error[E0283]: type annotations needed
   --> /Users/me/.cargo/git/checkouts/turso-455557cd4a2364c7/fcfcc04/core/mvcc/database/checkpoint_state_machine.rs:234:90
    |
234 |                     .is_none_or(|txid_max_old| begin_ts.is_some_and(|b| b > txid_max_old.into()));
    |                                                                           -              ^^^^
    |                                                                           |
    |                                                                           type must be known at this point
    |
    = note: multiple `impl`s satisfying `u64: PartialOrd<_>` found in the following crates: `candid`, `core`:
            - impl PartialOrd for u64;
            - impl PartialOrd<candid::types::number::Int> for u64;
            - impl PartialOrd<candid::types::number::Nat> for u64;
help: try using a fully qualified path to specify the expected types
    |
234 -                     .is_none_or(|txid_max_old| begin_ts.is_some_and(|b| b > txid_max_old.into()));
234 +                     .is_none_or(|txid_max_old| begin_ts.is_some_and(|b| b > <std::num::NonZero<u64> as Into<T>>::into(txid_max_old)));
    |

For more information about this error, try `rustc --explain E0283`.
error: could not compile `turso_core` (lib) due to 3 previous errors
```
