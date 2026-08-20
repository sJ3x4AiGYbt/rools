pub const BANNER_WEBSCOUT: &str = r#"

Yb        dP       8    .d88b.                   w
 Yb  db  dP  .d88b 88b. YPwww. .d8b .d8b. 8   8 w8ww
  YbdPYbdP   8.dP' 8  8     d8 8    8' .8 8b d8  8
   YP  YP    `Y88P 88P' `Y88P' `Y8P `Y8P' `Y8P8  Y8P
"#;

/* crawl.rs & test.rs */
pub const FIELD_ID: &str = "id";
pub const FIELD_NAME: &str = "name";
pub const FIELD_SOURCE: &str = "source";
pub const FIELD_URL: &str = "url";
pub const FIELD_TIME: &str = "time";
pub const FIELD_TYPE: &str = "type";
pub const FIELD_STATUS: &str = "status";
pub const FIELD_SIZE: &str = "size";
pub const FIELD_INDEX: &str = "index";
pub const FIELD_ROOT: &str = "root";
pub const FIELD_INPUT: &str = "input";
pub const FIELD_ERROR: &str = "error";
pub const FIELD_FOUND: &str = "found";
pub const FIELD_YES: &str = "yes";
pub const FIELD_NO: &str = "no";

pub const FIELD_DURATION_MS: &str = "duration_ms";
pub const FIELD_CONTENT_TYPE: &str = "content_type";
pub const FIELD_LINKS: &str = "links";
pub const FIELD_LINKS_FOUND: &str = "links_found";

pub const FIELD_TECH: &str = "tech";
pub const FIELD_TECHNOLOGIES: &str = "technologies";
pub const FIELD_CATEGORY: &str = "category";
pub const FIELD_VERSION: &str = "version";

pub const FIELD_FORMS: &str = "forms";
pub const FIELD_FORMS_FOUND: &str = "forms_found";
pub const FIELD_TXT_FORMS_FOUND: &str = "form(s) found";
pub const FIELD_ACTION: &str = "action";
pub const FIELD_METHOD: &str = "method";
pub const FIELD_FIELDS: &str = "fields";
pub const FIELD_REQUIRED: &str = "required";

pub const FIELD_RECURSION: &str = "recursion";
pub const FIELD_SEARCH: &str = "search";

pub const FIELD_TARGET: &str = "target";
pub const FIELD_SCAN: &str = "scan";
pub const FIELD_MODULE: &str = "module";
pub const FIELD_FINDINGS: &str = "findings";
pub const FIELD_PARAM: &str = "param";
pub const FIELD_PAYLOAD: &str = "payload";
pub const FIELD_EVIDENCE: &str = "evidence";

pub const TXT_TECH_NONE: &str = "no technologies detected";
pub const TXT_TECH_FOUND: &str = "technologies detected";
pub const TXT_FORMS_NONE: &str = "no forms found";
pub const TXT_FORMS_FOUND: &str = "form(s) detected";
pub const TXT_SEARCH_NONE: &str = "no paths found";
pub const TXT_SEARCH_FOUND: &str = "hit(s) detected";

pub const WARN_MISSING_SCHEME: &str = "[warn]    URL was missing a scheme, using: ";
pub const ERROR_INVALID_URL: &str = "[error]   invalid URL: ";
pub const ERROR_HTTP_CLIENT: &str = "[error]   failed to create HTTP client: ";
pub const WARN_HTTPS_FALLBACK: &str = "[warn]    https unreachable";
pub const WARN_HTTPS_FALLBACK_MID: &str = "retrying with";

/* test/test.rs */
pub const ERROR_TEST_INVALID_JSON: &str = "[error]   invalid --body JSON: ";
pub const EVIDENCE_NOT_VULNERABLE: &str = "no vulnerability detected for this payload";

/* test/sqli.rs */
pub const MODULE_SQLI: &str = "sql injection";
pub const EVIDENCE_SQLI_ERROR_PREFIX: &str = "possible error-based SQLi:";
pub const EVIDENCE_SQLI_BOOLEAN: &str = "response length differs between TRUE";
pub const EVIDENCE_SQLI_BOOLEAN_MID: &str = "and FALSE";
pub const EVIDENCE_SQLI_BOOLEAN_SUFFIX: &str = "conditions — possible boolean-based blind SQLi";
pub const EVIDENCE_SQLI_TIME_BASE: &str = "response took ";
pub const EVIDENCE_SQLI_TIME_MID: &str = "baseline";
pub const EVIDENCE_SQLI_TIME_SUFFIX: &str =
  "for a payload requesting a 5s delay — possible time-based blind SQLi";
pub const EVIDENCE_SQLI_TIME_NO_BASE: &str =
  "for a payload requesting a 5s delay (no baseline available) — possible time-based blind SQLi";

/* test/xss.rs */
pub const MODULE_XSS: &str = "xss reflected";

/* test/csrf.rs */
pub const MODULE_CSRF: &str = "csrf protection";
pub const EVIDENCE_CSRF_NO_TOKEN: &str = "POST form with ";
pub const EVIDENCE_CSRF_NO_TOKEN_SUFFIX: &str = "field(s) has no CSRF token and no CSRF header";
pub const EVIDENCE_CSRF_NO_SAMESITE: &str = "session cookie missing SameSite attribute: ";
pub const EVIDENCE_CSRF_NO_SECURE: &str =
  "session cookie missing Secure attribute on an HTTPS site: ";

/* test/fuzzing.rs */
pub const MODULE_FUZZ: &str = "fuzzing";
pub const EVIDENCE_FUZZ_PATH_TRAVERSAL: &str = "file content leaked: ";
pub const EVIDENCE_FUZZ_TEMPLATE: &str =
  "expression evaluated: '49' found in response (absent from baseline)";
pub const EVIDENCE_FUZZ_CMD_INJECTION: &str = "command output detected: '";
pub const EVIDENCE_FUZZ_NULL_BYTE: &str = "server error on null byte (HTTP ";
pub const EVIDENCE_FUZZ_OVERFLOW: &str = "server error on long input (HTTP ";
pub const EVIDENCE_FUZZ_BOUNDARY: &str = "server error on boundary value '";

/* test/transport.rs */
pub const MODULE_TRANSPORT: &str = "transport security";
pub const EVIDENCE_TRANSPORT_NO_HSTS: &str =
  "HTTPS response is missing the Strict-Transport-Security header";
pub const EVIDENCE_TRANSPORT_NO_REDIRECT: &str = "plain HTTP responds (HTTP ";
pub const EVIDENCE_TRANSPORT_NO_REDIRECT_SUFFIX: &str =
  "without redirecting to HTTPS — the site is reachable over an unencrypted channel";
pub const EVIDENCE_TRANSPORT_NO_HTTPS: &str = "target does not appear to support HTTPS at all — all traffic, including credentials, travels in clear text";

/* common/http.rs */
pub const ERROR_HTTP_CLIENT_BUILD: &str = "unable to create the HTTP client : ";
pub const ERROR_HTTP_REQUEST: &str = "request failed : ";
pub const ERROR_HTTP_READ_BODY: &str = "unable to read the body : ";
pub const HTTP_USER_AGENT: &str = "WebScout/1.0";

/* common/url.rs */
pub const ERROR_URL_INVALID: &str = "invalid URL after correction : ";
pub const ERROR_URL_NO_HOST: &str = "unable to extract the domain name";
pub const ERROR_URL_EMPTY_HOST: &str = "the URL does not contain a domain name";
