# Rust-Hydra

A **Hydra-like credential attack tool written in Rust**, designed for
**authorized security labs only** (TryHackMe, Hack The Box, local test environments).

⚠️ **DO NOT USE AGAINST SYSTEMS YOU DO NOT OWN OR HAVE EXPLICIT PERMISSION TO TEST.**

---

## ✨ Features

- HTTP form brute-force
- SSH authentication
- FTP authentication
- RDP (NLA / CredSSP)
- Streaming wordlists (constant RAM usage)
- Resume / restore support
- Progress bar + ETA
- Multi-threaded async engine
- Hydra-style workflow

---

## 🛠 Supported Protocols

| Protocol | Status |
|--------|--------|
| HTTP Forms | ✅ |
| SSH | ✅ |
| FTP | ✅ |
| RDP | ✅ |
| SMB | ❌ |
| WinRM | ❌ |

---

## 📦 Installation

```bash
git clone https://github.com/YOUR_USERNAME/rust-hydra.git
cd rust-hydra
cargo build --release

Usage
HTTP Form
./target/release/rust-hydra http http://10.10.10.10/login \
  -L users.txt \
  -P passwords.txt \
  --user-field username \
  --pass-field password \
  --fail "Invalid login"

SSH
./target/release/rust-hydra ssh 10.10.10.10 \
  -L users.txt \
  -P passwords.txt

RDP
./target/release/rust-hydra rdp 10.10.10.10 \
  -L users.txt \
  -P passwords.txt \
  -t 4

# SMB and WinRM on ToDo List.

Resume / Restore

Stop the tool with Ctrl+C.
Progress will be saved automatically.

Resume with:

./rust-hydra ... --resume

⚠️ Legal Disclaimer

This tool is intended for educational purposes and authorized testing only.
Unauthorized use against systems you do not own may be illegal.
