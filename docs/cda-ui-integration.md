# CDA Web UI Integration

## Decision Record

**Decision:** Integrate [cda-ui](https://github.com/theswiftfox/cda-ui) into the CDA binary.

### Why cda-ui is worth shipping

| Capability | Value |
|---|---|
| ECU network browser | Tree view of all connected ECUs with variant identification and state |
| Data / DIDs | Read and write ECU data identifiers |
| Faults / DTCs | View and clear diagnostic trouble codes with status bits |
| Modes | Read and set ECU diagnostic modes |
| Configurations | Inspect ECU configuration parameters |
| Locks | Create and release ECU resource locks |
| Generic UDS | Send raw hex UDS frames directly |
| Auth scripts | Built-in JS editor for multi-step authentication flows (e.g. ZenZefi cert exchange) |

The UI covers the full SOVD v15 API surface, requires zero extra infrastructure (no separate web server), and ships under Apache-2.0 — same as CDA.

---

## Building with the UI

### Prerequisites

- Rust ≥ 1.88.0
- Node.js ≥ 18
- The `cda-ui` submodule initialised:

```bash
git submodule update --init cda-ui
```

### Build

```bash
cargo build --release -p opensovd-cda --features ui
```

The first build runs `npm ci` and `npm run build` automatically inside `cda-ui/`.
Subsequent builds only re-run npm if `src/`, `index.html`, `package-lock.json`, or `vite.config.ts` change.

---

## Testing the UI

### Quick smoke test (no ECU required)

```bash
./target/release/opensovd-cda --features ui   # or debug build

# HTML served correctly
curl -s http://localhost:20002/ui | grep '<title>'

# JS asset loads (replace hash with your build output)
curl -I http://localhost:20002/ui/assets/index-*.js
```

Open `http://localhost:20002/ui` in a browser — you should see the CDA login panel.

---

## Using the UI with a Real ECU

### 1. Configure `opensovd-cda.toml`

```toml
[database]
path = "/path/to/mdd/files"   # directory containing one or more .mdd files

[doip]
tester_address = "192.168.1.100"   # IP of the host network interface on the DoIP VLAN
tester_subnet  = "255.255.0.0"
gateway_port   = 13400             # standard DoIP port

[server]
address = "0.0.0.0"
port    = 20002
```

MDD files are generated from ODX using [odx-converter](https://github.com/eclipse-opensovd/odx-converter).

### 2. Run CDA

```bash
./target/release/opensovd-cda
```

### 3. Open the UI

Navigate to `http://localhost:20002/ui`.

### 4. Log in

The UI shows a login panel. The default credentials are:

| Field | Value |
|---|---|
| Client ID | any string (e.g. `test_client`) |
| Client Secret | `secret` |

Clicking **Login** calls `POST /authorize`, obtains a JWT, and attaches it as `Authorization: Bearer <token>`
to every subsequent API request.
Use **Skip** to proceed without a token (only works if CDA is configured without the `auth` feature).

### 5. Navigate ECUs

Once connected the left panel shows a tree of ECUs discovered via DoIP. Click an ECU to open its detail tabs.
