@vendor *args:
    sh "scripts/vendor.sh" {{ args }}

@bind_rebound: vendor
    cargo build --package rebound-bind

@bind: bind_rebound

@clean_rebound:
    rm -rvf bind/rebound/c_src
    rm -rvf bind/rebound/src/bindings_gen.rs

@clean: clean_rebound
    cargo clean
