@bind_rebound:
    cargo build --package rebound-bind

@bind: bind_rebound

@clean:
    cargo clean
