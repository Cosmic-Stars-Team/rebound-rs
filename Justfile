@bind_rebound:
    cargo build --package rebound-bind

@bind: bind_rebound

@clean_rebound:
    {{ if os_family() == "windows" { "if (Test-Path -LiteralPath 'bind/rebound/c_src') { Remove-Item -LiteralPath 'bind/rebound/c_src' -Recurse -Force }; if (Test-Path -LiteralPath 'bind/rebound/src/bindings_gen.rs') { Remove-Item -LiteralPath 'bind/rebound/src/bindings_gen.rs' -Force }" } else { "rm -rvf bind/rebound/c_src bind/rebound/src/bindings_gen.rs" } }}

@clean: clean_rebound
    cargo clean
