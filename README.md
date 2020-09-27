# integer-result-rs

🔢✅🚫

Using scalar types to indicate failure in Rust _is discouraged_, yet not uncommon in C.  When calling C functions from Rust, you have to check return values that indicate success or failure like you would in C.  This library adds methods to the primitive and non-zero integer types to ease the pain.

Now you can write this 🧼

```rust
use integer_result::Ext;

unsafe { some_c_function() }
  .ok_equal(0)
  .map_err(|val| YourRustyErrorType::from(val)) // or somethin' ..
```

Rather than this 🤢

```rust
let val = unsafe { some_c_function() };

if val == 0 {
  Ok(())
} else {
  Err(YourRustyErrorType::from(val))
}
```

