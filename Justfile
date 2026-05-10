@bind_rebound:
    cargo build --package rebound-bind

@bind: bind_rebound

@clean_rebound:
    {{ if os_family() == "windows" { "if (Test-Path -LiteralPath 'bind/c_src') { Remove-Item -LiteralPath 'bind/c_src' -Recurse -Force }; if (Test-Path -LiteralPath 'bind/src/bindings_gen.rs') { Remove-Item -LiteralPath 'bind/src/bindings_gen.rs' -Force }" } else { "rm -rvf bind/c_src bind/src/bindings_gen.rs" } }}

@clean: clean_rebound
    cargo clean
