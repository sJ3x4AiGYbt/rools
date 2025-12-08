# rools

Rust Offensive & Observability Lightweight Suite is a cross-platform (Windows & Linux) security and observability toolkit developed in **Rust**, providing fast, safe and modular assessment capabilities for system, memory, network, and web targets.

The suite bundles multiple focused command-line tools — each dedicated to a strategic domain of cybersecurity analysis:

1. **DirScope:** Audits Active Directory / LDAP environments for security risks
2. **SkyWave:** Detects vulnerable Wi-Fi networks and tests wireless intrusion resilience
3. **SentryProc:** Inspects running processes and detects suspicious behavior
4. **NetWatch:** Analyzes traffic, scans ports, detects anomalies in real time
5. **SysViz:** Explores system calls and execution integrity
6. **MalScan:** Scans suspicious binaries & artifacts for malicious patterns
7. **MemTrace:** Reads and searches target process memory safely
8. **WebScout:** Crawls and tests web applications for common vulnerabilities

### **build:**

> *Releases incoming*. Source build requires Rust 1.76+:

```bash
git clone https://github.com/<you>/rools.git
cd rools
cargo build --release
```

### **usage:**

All tools follow a unified syntax: `<tool> <command> [global options] [specific options]`