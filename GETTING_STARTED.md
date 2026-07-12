# Getting Started (Beginner Guide)

This guide assumes zero prior experience with Rust, the command line, or Git. It walks you through getting `entra-access-graph-engine` (the `eagraph` tool) running from scratch, step by step.

If you get stuck, check the [Troubleshooting](#troubleshooting) table at the bottom of your platform's section.

There is no installer and no graphical interface: this is a command-line tool. You will type commands into a terminal window and read the output there.

---

## Windows

### 1. Open a terminal

Right-click the **Start** button and choose **Terminal** (or **Windows PowerShell** on older versions of Windows). A dark window with a blinking cursor will open: that's your terminal.

### 2. Check if Rust is already installed

Type the following and press Enter:

```powershell
rustc --version
cargo --version
```

- If you see version numbers (e.g. `rustc 1.78.0`), Rust is installed. Skip to step 4.
- If you see something like `'rustc' is not recognized as an internal or external command`, Rust is not installed yet (or not on your system PATH). Continue to step 3.

### 3. Install Rust

Go to [https://rustup.rs](https://rustup.rs) in your browser. Download `rustup-init.exe` and run it. Accept the default options in the installer. When it finishes, **close and reopen your terminal** (this is important, see Troubleshooting) and repeat step 2 to confirm `rustc --version` now works.

### 4. Get the code

The easiest way, no Git required:

1. Open [https://github.com/9t29zhmwdh-coder/entra-access-graph-engine](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine) in your browser.
2. Click the green **Code** button, then click **Download ZIP**.
3. Extract the ZIP file somewhere you'll remember, e.g. `Documents\entra-access-graph-engine`.

Alternative, if you already have Git installed:

```powershell
git clone https://github.com/9t29zhmwdh-coder/entra-access-graph-engine.git
```

### 5. Open the project folder in the terminal

Navigate into the folder you extracted (adjust the path to where you put it):

```powershell
cd Documents\entra-access-graph-engine
```

### 6. Build the tool

```powershell
cargo build --release
```

This downloads dependencies and compiles the tool. It can take a few minutes the first time. You'll see a lot of text scroll by, that's normal.

### 7. Run it

Try the demo mode first: it needs no Azure account or credentials at all, so you can see real output immediately:

```powershell
.\target\release\eagraph.exe scan --dry-run --format html --output report
```

<!-- TODO: Screenshot of the terminal output after a successful --dry-run scan -->

This uses a built-in mock (fake) data set and writes a report file (`report.html`) in the current folder. Open `report.html` in your web browser afterward to see an interactive graph.

Once you have a real Azure App Registration with the right permissions (see the main [README.md](README.md#requirements)), you can run a live scan against your own tenant:

```powershell
$env:AZURE_TENANT_ID="your-tenant-id"
$env:AZURE_CLIENT_ID="your-client-id"
$env:AZURE_CLIENT_SECRET="your-client-secret"
.\target\release\eagraph.exe scan --format html --output report --min-risk high
```

### Troubleshooting

| Problem | Likely Cause | Fix |
|---|---|---|
| `'rustc' is not recognized...` even after installing Rust | Terminal was opened before Rust was added to PATH | Close the terminal window completely and open a new one |
| `cargo build` fails with linker errors (e.g. `link.exe not found`) | Missing C++ Build Tools | Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (select "Desktop development with C++" during install), then retry |
| Live scan fails to connect / times out reaching Microsoft/Azure | Corporate proxy or firewall blocking outbound HTTPS to `graph.microsoft.com` / `login.microsoftonline.com` | Check with your network admin, or try from a network without a proxy; `--dry-run` still works fully offline |
| `cargo build --release` seems to hang on "Updating crates.io index" | Slow or blocked internet connection | Wait, or check firewall/proxy settings for `crates.io` and `github.com` |

---

## Linux

### 1. Open a terminal

The exact steps depend on your desktop environment (GNOME, KDE, XFCE, etc.). Usually you can search for "Terminal" in your application menu, or use the keyboard shortcut `Ctrl+Alt+T` on many distributions.

### 2. Check if Rust is already installed

```bash
rustc --version
cargo --version
```

- If you see version numbers, Rust is installed: skip to step 4.
- If you see `command not found`, Rust isn't installed yet, or it isn't on your PATH. Continue to step 3.

### 3. Install Rust

Go to [https://rustup.rs](https://rustup.rs) and run the curl one-liner shown there, for example:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompts (the default option is fine). When it's done, **close and reopen your terminal** so the new PATH settings take effect, then repeat step 2.

### 4. Get the code

Easiest, no Git knowledge required:

1. Open [https://github.com/9t29zhmwdh-coder/entra-access-graph-engine](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine) in your browser.
2. Click the green **Code** button, then **Download ZIP**.
3. Extract it, e.g. with your file manager, or via terminal:
   ```bash
   unzip entra-access-graph-engine-main.zip
   ```

Alternative, if Git is installed:

```bash
git clone https://github.com/9t29zhmwdh-coder/entra-access-graph-engine.git
```

### 5. Open the project folder

```bash
cd entra-access-graph-engine
```

### 6. Build the tool

```bash
cargo build --release
```

The first build downloads dependencies and compiles everything; this can take a few minutes.

### 7. Run it

Try the demo mode first, no Azure credentials needed:

```bash
./target/release/eagraph scan --dry-run --format html --output report
```

This produces `report.html` using built-in mock data. Open it in a web browser to see the interactive graph.

For a real scan against your Azure tenant (see [README.md](README.md#requirements) for the required App Registration permissions):

```bash
export AZURE_TENANT_ID=your-tenant-id
export AZURE_CLIENT_ID=your-client-id
export AZURE_CLIENT_SECRET=your-client-secret
./target/release/eagraph scan --format html --output report --min-risk high
```

### Troubleshooting

| Problem | Likely Cause | Fix |
|---|---|---|
| `rustc: command not found` even after installing | Terminal session was opened before Rust was added to `~/.cargo/env` / PATH | Close the terminal completely and open a new one (or run `source "$HOME/.cargo/env"`) |
| `cargo build` fails with linker errors (`cc` not found or similar) | Missing a C linker / build essentials | Install your distro's build tools, e.g. `sudo apt install build-essential` (Debian/Ubuntu) or the equivalent for your distro |
| Live scan can't reach Microsoft/Azure endpoints | Corporate proxy or firewall blocking `graph.microsoft.com` / `login.microsoftonline.com` | Check proxy/firewall settings with your network admin; `--dry-run` works fully offline as a fallback |
| Permission denied when running `./target/release/eagraph` | Execute bit not set (rare after `cargo build`) | Run `chmod +x target/release/eagraph` |

---

## macOS

### 1. Open a terminal

Press `Cmd+Space` to open Spotlight, type `Terminal`, and press Enter.

### 2. Check if Rust is already installed

```bash
rustc --version
cargo --version
```

- Version numbers shown → Rust is installed, skip to step 4.
- `command not found` → Rust isn't installed or not on PATH. Continue to step 3.

### 3. Install Rust

Go to [https://rustup.rs](https://rustup.rs) and run the curl one-liner shown there:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Accept the default installation option. Afterward, **close and reopen Terminal** so your shell picks up the new PATH, then repeat step 2 to confirm.

### 4. Get the code

Easiest, no Git knowledge required:

1. Open [https://github.com/9t29zhmwdh-coder/entra-access-graph-engine](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine) in your browser.
2. Click the green **Code** button, then **Download ZIP**.
3. Double-click the downloaded ZIP file in Finder to extract it.

Alternative, if Git is installed (macOS often ships it, or via Xcode Command Line Tools):

```bash
git clone https://github.com/9t29zhmwdh-coder/entra-access-graph-engine.git
```

### 5. Open the project folder

```bash
cd entra-access-graph-engine
```

### 6. Build the tool

```bash
cargo build --release
```

This compiles the project; the first run takes a few minutes while dependencies download.

### 7. Run it

Start with the demo mode, no Azure credentials required:

```bash
./target/release/eagraph scan --dry-run --format html --output report
```

This writes `report.html` using built-in mock data; open it in your browser to explore the interactive graph.

For a live scan (see [README.md](README.md#requirements) for the required Azure App Registration permissions):

```bash
export AZURE_TENANT_ID=your-tenant-id
export AZURE_CLIENT_ID=your-client-id
export AZURE_CLIENT_SECRET=your-client-secret
./target/release/eagraph scan --format html --output report --min-risk high
```

### Troubleshooting

| Problem | Likely Cause | Fix |
|---|---|---|
| `rustc: command not found` even after installing | Terminal session predates the PATH change | Close and reopen Terminal (or run `source "$HOME/.cargo/env"`) |
| `cargo build` fails with linker/compiler errors | Missing Xcode Command Line Tools | Run `xcode-select --install` and follow the prompts, then retry |
| Live scan can't reach Microsoft/Azure endpoints | Corporate proxy, VPN, or firewall blocking `graph.microsoft.com` / `login.microsoftonline.com` | Check with your network admin or try a different network; `--dry-run` works fully offline as a fallback |
| Terminal says "cannot be opened because the developer cannot be verified" | Gatekeeper blocking an unsigned binary you built yourself | This is expected for locally built binaries; right-click the file → Open, or allow it in System Settings → Privacy & Security |

---

## Next Steps

Once the demo (`--dry-run`) works, see the main [README.md](README.md) for:
- Full feature list and risk level definitions
- How to set up an Azure App Registration ([docs/azure_integration.md](docs/azure_integration.md))
- Output format details (JSON, GraphML, HTML)
