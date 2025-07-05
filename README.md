# splitguard for windows

A Rust program that routes traffic from selected applications through WireGuard VPN, while leaving all other traffic untouched.

## How it works

- You define which apps should use the VPN in `apps.txt` (e.g. `firefox.exe`, `discord.exe`).
- The program starts WireGuard and a tun2socks process.
- tun2socks exposes a local SOCKS5 proxy on `127.0.0.1:<port>`.
- The program monitors running processes and logs when a matching application is active.

To ensure traffic goes through the VPN, the selected applications must be configured to use the local SOCKS5 proxy.

## Setup

1. Install WireGuard and tun2socks.
2. Build the project:

```

cargo build --release

```

3. Create a `config.toml` file:

```

wg\_config\_path = "wg0.conf"
tun2socks\_path = "C:\Tools\tun2socks.exe"
socks5\_port = 1080
check\_interval = 3
app\_list = "apps.txt"

```

4. Create `apps.txt` and list executable names (one per line):

```

firefox.exe
discord.exe

```

5. Make sure each listed application is configured to use the proxy `127.0.0.1:1080`.

## Notes

- The program does not intercept or redirect traffic by force.
- It only monitors the selected applications and sets up the local VPN tunnel.
- Applications must support SOCKS5 proxying or be configured manually to use it.

License: MIT

