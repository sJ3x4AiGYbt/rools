# DirScope

DirScope is an Active Directory auditing utility designed to identify accounts, groups, permissions, trust relationships, Kerberos delegations, and potential security misconfigurations.

```
dirscope <command> [global options] [specific options]
```

The tool is used with the following syntax, requiring a **specific command** (only one must be used), and optionally combining **global and/or specific options** to customize the execution:

```bash
dirscope users -d lab.local -u admin -p Passw0rd --json --enabled
```

| option                  | description                   | default                        |
| ----------------------- | ----------------------------- | ------------------------------ |
| `--help`                | Display help                  |                                |
| `-d, --domain <fqdn>`   | Domain to audit               | Current workstation domain     |
| `-u, --username <user>` | Username for authentication   | Current user                   |
| `-p, --password <pass>` | Password                      | Prompt if omitted              |
| `--dc <ip>`             | Targeted Domain Controller    | Auto-detect DC                 |
| `--kerberos`            | Force Kerberos authentication | NTLM/Kerberos auto-negociation |
| `--ldap-only`           | Skip SMB, RPC, WinRM checks   | Mixed protocols                |
| `--timeout <sec>`       | Maximum scan duration         | `no-timeout`                   |
| `--json`                | Output in JSON format         |                                |
| `--yaml`                | Output in YAML format         |                                |
| `--csv`                 | Output in CSV format          |                                |
| `--export <file>`       | Export results to a file      | `dirscope-<cmd>-<date>.txt`    |
| `--zip`                 | Compress exported file        |                                |

### `users`

Lists all domain user objects from Active Directory. If no filter: lists all AD users with standard attributes.

| option       | description                          |
| ------------ | ------------------------------------ |
| `--enabled`  | Show only enabled accounts           |
| `--disabled` | Show only disabled accounts          |
| `--locked`   | List locked accounts                 |
| `--expired`  | Show accounts with expired passwords |
| `--spn`      | Show users with an SPN               |

### `weakcreds`

Detects insecure account configurations affecting authentication. If no option: runs all vulnerability checks.

| option                    | description                                       |
| ------------------------- | ------------------------------------------------- |
| `--asrep`                 | List accounts without Kerberos pre-authentication |
| `--kerberoastable`        | List accounts with exposed SPNs                   |
| `--des`                   | List accounts allowing legacy DES encryption      |
| `--password-never-expire` | Accounts whose password never expires             |

### `groups`

Enumerates security and distribution groups in the domain. If no option: lists all groups (without listing members).

| option         | description                                      |
| -------------- | ------------------------------------------------ |
| `--members`    | Include group members                            |
| `--privileged` | Show only sensitive groups (Domain Admins, etc.) |
| `--nested`     | Resolve nested groups                            |

### `delegation`

Identifies unconstrained, constrained and RBCD delegation scenarios. If no option: performs a full delegation assessment.

| option             | description                                       |
| ------------------ | ------------------------------------------------- |
| `--unconstrained`  | List *unconstrained* delegation                   |
| `--constrained`    | List *constrained* delegation                     |
| `--resource-based` | List RBCD (Resource-Based Constrained Delegation) |
| `--spn-only`       | Show only hosts with relevant SPNs                |

### `trust`

Enumerates inter-domain trust configurations. If no option: shows all trusts with basic details.

| option            | description                       |
| ----------------- | --------------------------------- |
| `--external`      | Show only external trusts         |
| `--forest`        | Display inter-forest trusts       |
| `--direction`     | Show direction (Inbound/Outbound) |
| `--sid-filtering` | Check if SID Filtering is enabled |

### `acl`

Reviews sensitive permissions and risky delegation of rights in AD. If no option: audits core high-value AD objects (Domain Root, DCs, Admins).

| option                      | description                                            |
| --------------------------- | ------------------------------------------------------ |
| `--object <dn>`             | Target a specific object (full DN)                     |
| `--privileged-only`         | Show only sensitive ACLs (GenericAll, WriteDACL, etc.) |
| `--export <file>`           | Export in JSON/YAML format                             |
| `--effective-rights <user>` | Calculate effective permissions for a specific user    |

### `schema`

Lists structural definitions used by Active Directory. If no option: shows both schema classes and attributes count.

| option         | description            |
| -------------- | ---------------------- |
| `--attributes` | List schema attributes |
| `--classes`    | List object classes    |