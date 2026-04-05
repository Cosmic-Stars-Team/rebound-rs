set shell := ["sh", "-cu"]
set windows-shell := ["pwsh", "-NoProfile", "-Command"]

@vendor *args:
    {{ if os_family() == "windows" { "$vendor_args = " + quote(args) + "; [string[]]$vendor_argv = if ($vendor_args) { @($vendor_args -split ' ') } else { @() }; if ($vendor_argv.Length -gt 0 -and $vendor_argv[0] -eq '--') { [string[]]$vendor_argv = if ($vendor_argv.Length -gt 1) { $vendor_argv[1..($vendor_argv.Length - 1)] } else { @() } }; $vendor_force = $false; $vendor_help = $false; $vendor_version = $null; for ($i = 0; $i -lt $vendor_argv.Length; $i++) { if ($vendor_argv[$i] -eq '--help') { $vendor_help = $true } elseif ($vendor_argv[$i] -eq '--force') { $vendor_force = $true } elseif ($vendor_argv[$i] -eq '--version') { if ($i + 1 -ge $vendor_argv.Length) { throw 'vendor: --version requires a value' }; $vendor_version = $vendor_argv[$i + 1]; $i++ } else { throw ('vendor: unsupported argument: ' + $vendor_argv[$i]) } }; if ($vendor_help) { & ./scripts/vendor.ps1 -Help } elseif ($vendor_force -and $vendor_version) { & ./scripts/vendor.ps1 -Force -Version $vendor_version } elseif ($vendor_force) { & ./scripts/vendor.ps1 -Force } elseif ($vendor_version) { & ./scripts/vendor.ps1 -Version $vendor_version } else { & ./scripts/vendor.ps1 }" } else { "set -- " + args + "; if [ \"$1\" = \"--\" ]; then shift; fi; sh \"scripts/vendor.sh\" \"$@\"" } }}

@bind_rebound: vendor
    cargo build --package rebound-bind

@bind: bind_rebound

@clean_rebound:
    {{ if os_family() == "windows" { "if (Test-Path -LiteralPath 'bind/rebound/c_src') { Remove-Item -LiteralPath 'bind/rebound/c_src' -Recurse -Force }; if (Test-Path -LiteralPath 'bind/rebound/src/bindings_gen.rs') { Remove-Item -LiteralPath 'bind/rebound/src/bindings_gen.rs' -Force }" } else { "rm -rvf bind/rebound/c_src bind/rebound/src/bindings_gen.rs" } }}

@clean: clean_rebound
    cargo clean
