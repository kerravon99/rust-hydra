# Rust-Hydra

## Project Summary

Rust-Hydra is a high-performance, Rust-based password testing tool inspired by Hydra. Designed for **educational and authorized lab environments**, it allows you to perform credential testing on HTTP, SSH, and FTP services safely. It features:

- Asynchronous, concurrent authentication attempts
- Modular protocol support (HTTP, SSH, FTP)
- Hydra-style CLI compatibility
- Automatic protocol detection (scheme, port, probe)
- Resume and restore capabilities
- Streaming wordlists with zero RAM growth
- Progress bars with ETA tracking
- Stop-on-success option

This project is meant for **CTFs, labs, and authorized penetration tests only**.

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
| SMB | ✅ |
| WinRM | ✅ |

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

Future Development Goals

Hydra .restore file compatibility

Per-protocol progress bars

Attempt rate limiting (requests/sec)

Output file logging (-o option)

Hydra-style HTTP POST form syntax support

Protocol autodetection fallback order

Additional protocols (SMB, RDP, MySQL)

Improved resume & checkpoint accuracy with streaming wordlists

Advanced concurrency and queue management

Legal & Ethical Disclaimer

Rust-Hydra is intended solely for educational purposes and authorized environments:

TryHackMe CTFs

Hack The Box labs

Your own systems

Explicitly authorized penetration tests

Do NOT use this tool on production systems, public websites, or networks without permission. Unauthorized access is illegal and unethical.

By using this software, you accept full responsibility for compliance with laws and regulations in your jurisdiction.

⚠️ Legal Disclaimer

This tool is intended for educational purposes and authorized testing only.
Unauthorized use against systems you do not own may be illegal.
