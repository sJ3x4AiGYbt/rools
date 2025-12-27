# DirScope

DirScope is a local Active Directory inspection tool designed to enumerate directory objects, identify insecure configurations, and highlight common privilege escalation paths.

```bash
dirscope <command> [options]
```

* If the machine is not domain-joined → `--help`
* Only **one command** may be used per run
* Commands may be combined with **global options** and **command-specific options** to refine the analysis scope

## Global options

| option            | description                                                     |
| ----------------- | --------------------------------------------------------------- |
| `--help`          | Display help and usage examples                                 |
| `--timeout <sec>` | Maximum command duration (default: `no-timeout`)                |
| `--ldap-only`     | Restrict analysis to LDAP queries only (passive, non-intrusive) |
| `--json`          | Output results in JSON format                                   |
| `--yaml`          | Output results in YAML format                                   |
| `--csv`           | Output results in CSV format                                    |
| `--export <file>` | Export results to a file (default: `dirscope-<cmd>-<date>.txt`) |
| `--zip`           | Compress exported output                                        |

## Command: `users`

Enumerates user accounts in the current Active Directory domain.

* If no option is specified → list all domain users with standard attributes

| option                    | description                                    |
| ------------------------- | ---------------------------------------------- |
| `--enabled`               | Show only enabled accounts                     |
| `--disabled`              | Show only disabled accounts                    |
| `--locked`                | List locked user accounts                      |
| `--expired`               | Show accounts with expired passwords           |
| `--spn`                   | Show users with a registered Service Principal |
| `--asrep`                 | Accounts without Kerberos pre-authentication   |
| `--kerberoastable`        | Accounts with exposed Service Principal Names  |
| `--des`                   | Accounts allowing legacy DES encryption        |
| `--password-never-expire` | Accounts whose password never expires          |

## Command: `groups`

Enumerates Active Directory groups and their relationships.

* If no option is specified → list all groups without resolving members

| option         | description                                  |
| -------------- | -------------------------------------------- |
| `--members`    | Include group members                        |
| `--privileged` | Show only high-privilege or sensitive groups |
| `--nested`     | Resolve nested group memberships             |

## Command: `delegation`

Analyzes Kerberos delegation configurations that may allow lateral movement.

* If no option is specified → perform a full delegation assessment

| option             | description                                           |
| ------------------ | ----------------------------------------------------- |
| `--unconstrained`  | Identify unconstrained delegation                     |
| `--constrained`    | Identify constrained delegation                       |
| `--resource-based` | Identify Resource-Based Constrained Delegation (RBCD) |
| `--spn-only`       | Show only objects with relevant SPNs                  |

## Command: `trust`

Enumerates inter-domain trust relationships.

* If no option is specified → list all detected trusts with basic properties

| option            | description                                  |
| ----------------- | -------------------------------------------- |
| `--external`      | Show only external trusts                    |
| `--forest`        | Show forest-level trusts                     |
| `--direction`     | Display trust direction (inbound / outbound) |
| `--sid-filtering` | Check whether SID filtering is enabled       |

## Command: `acl`

Reviews Access Control Lists on sensitive Active Directory objects.

* If no option is specified → audit core high-value objects (Domain root, DCs, Admin groups)

| option                      | description                                           |
| --------------------------- | ----------------------------------------------------- |
| `--object <dn>`             | Target a specific object using its distinguished name |
| `--privileged-only`         | Show only sensitive or high-risk permissions          |
| `--effective-rights <user>` | Calculate effective permissions for a specific user   |

## Notes

* When `--ldap-only` is enabled, checks requiring RPC, SMB, or Win32 APIs are skipped.
* Results depend on the privileges of the current user context.
* Output format options (`--json`, `--yaml`, `--csv`) are mutually exclusive.

## Example

```bash
dirscope users --asrep --kerberoastable --json
```