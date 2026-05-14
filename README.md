# War3StatsObserver

Rust bindings for the Warcraft 3 Stats Observer API memory map.
Great for a streaming or build order overlay.

The entry point is [`ObserverHandle`], which opens the shared memory map and
dereferences to [`ObserverData`]:

```rust
use warcraft3_stats_observer::ObserverHandle;

let od = match ObserverHandle::new() {
    Ok(od) => od,
    Err(e) => {
        eprintln!("Error opening observer API. Is Warcraft3 running? Error: {e:?}");
        // handle error
        return;
    }
};
```

A refresh rate of 0 (measured in milliseconds) disables the API. The default
rate is 500ms when using `ObserverHandle::new()`, but you can change it via
`ObserverHandle::new_with_refresh_rate(...)` or `set_refresh_rate(...)`.

The structs are packed due to how the memory map is laid out. This makes it a
pain to work with certain fields because Rust does not allow a
reference to an unaligned field. To get around this, you can make a copy in
place by wrapping a field in `{}`:
```rust
println!("refresh rate: {} ms", { od.refresh_rate });
```

See the examples folder for more.