use std::collections::HashSet;

use crate::commands::webscout::common::http;

use crate::commands::webscout::common::constants::{
  EVIDENCE_FUZZ_BOUNDARY, EVIDENCE_FUZZ_CMD_INJECTION, EVIDENCE_FUZZ_NULL_BYTE,
  EVIDENCE_FUZZ_OVERFLOW, EVIDENCE_FUZZ_PATH_TRAVERSAL, EVIDENCE_FUZZ_TEMPLATE, MODULE_FUZZ,
};

use crate::commands::webscout::crawl::parser;

use crate::commands::webscout::test::probe;

struct Payload {
  value: &'static str,
  category: &'static str,
}

const PAYLOADS: &[Payload] = &[
  Payload {
    value: "../../../etc/passwd",
    category: "path-traversal",
  },
  Payload {
    value: "..%2F..%2F..%2Fetc%2Fpasswd",
    category: "path-traversal",
  },
  Payload {
    value: "....//....//etc/passwd",
    category: "path-traversal",
  },
  Payload {
    value: "{{7*7}}",
    category: "template-injection",
  },
  Payload {
    value: "${7*7}",
    category: "template-injection",
  },
  Payload {
    value: "<%= 7*7 %>",
    category: "template-injection",
  },
  Payload {
    value: "#{7*7}",
    category: "template-injection",
  },
  Payload {
    value: "; ls",
    category: "command-injection",
  },
  Payload {
    value: "| id",
    category: "command-injection",
  },
  Payload {
    value: "`id`",
    category: "command-injection",
  },
  Payload {
    value: "$(id)",
    category: "command-injection",
  },
  Payload {
    value: "value%00",
    category: "null-byte",
  },
  Payload {
    value: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    category: "overflow",
  },
  Payload {
    value: "-1",
    category: "boundary",
  },
  Payload {
    value: "0",
    category: "boundary",
  },
  Payload {
    value: "999999999999999999",
    category: "boundary",
  },
];

pub fn run(
  forms: &[parser::FormInfo],
  url: &str,
  client: &http::HttpClient,
  baseline_body: &str,
  json_body: Option<&serde_json::Value>,
) -> probe::ModuleReport {
  let mut findings = Vec::new();
  let mut tested = Vec::new();
  let mut seen: HashSet<(String, String, String)> = HashSet::new();

  for p in PAYLOADS {
    for result in probe::probe_url_params(client, url, p.value) {
      if let Some(f) = check(&result, p.category, baseline_body)
        && seen.insert((f.url.clone(), f.param.clone(), f.payload.clone()))
      {
        findings.push(f);
      }
      tested.push(probe::TestedItem {
        url: result.url.clone(),
        param: result.param.clone(),
        payload: result.payload.clone(),
      });
    }

    for form in forms {
      for result in probe::probe_form(client, form, p.value) {
        if let Some(f) = check(&result, p.category, baseline_body)
          && seen.insert((f.url.clone(), f.param.clone(), f.payload.clone()))
        {
          findings.push(f);
        }
        tested.push(probe::TestedItem {
          url: result.url.clone(),
          param: result.param.clone(),
          payload: result.payload.clone(),
        });
      }
    }

    if let Some(template) = json_body {
      for result in probe::probe_json_body(client, url, template, p.value) {
        if let Some(f) = check(&result, p.category, baseline_body)
          && seen.insert((f.url.clone(), f.param.clone(), f.payload.clone()))
        {
          findings.push(f);
        }
        tested.push(probe::TestedItem {
          url: result.url.clone(),
          param: result.param.clone(),
          payload: result.payload.clone(),
        });
      }
    }
  }

  probe::ModuleReport { findings, tested }
}

fn check(
  result: &probe::ProbeResult,
  category: &'static str,
  baseline_body: &str,
) -> Option<probe::Finding> {
  let body = &result.response.body;
  let status = result.response.status;

  let evidence = match category {
    "path-traversal" => {
      if body.contains("root:") || body.contains("/bin/bash") || body.contains("/bin/sh") {
        format!(
          "{}{}",
          EVIDENCE_FUZZ_PATH_TRAVERSAL,
          extract_line(body, "root:")
        )
      } else {
        return None;
      }
    }

    "template-injection" => {
      let scrubbed = body.replace(result.payload.as_str(), "");
      if scrubbed.contains("49") && !baseline_body.contains("49") {
        EVIDENCE_FUZZ_TEMPLATE.to_string()
      } else {
        return None;
      }
    }

    "command-injection" => {
      let signatures = ["uid=", "gid=", "total ", "drwxr", "-rwxr", "bin/sh"];
      let sig = signatures.iter().find(|s| body.contains(*s))?;
      format!("{EVIDENCE_FUZZ_CMD_INJECTION}{sig}'")
    }

    "null-byte" => {
      if status >= 500 {
        format!("{EVIDENCE_FUZZ_NULL_BYTE}{status})")
      } else {
        return None;
      }
    }

    "overflow" => {
      if status >= 500 {
        format!("{EVIDENCE_FUZZ_OVERFLOW}{status})")
      } else {
        return None;
      }
    }

    "boundary" if status >= 500 => {
      format!(
        "{}{}' (HTTP {})",
        EVIDENCE_FUZZ_BOUNDARY, result.payload, status
      )
    }

    _ => return None,
  };

  Some(probe::Finding {
    module: MODULE_FUZZ,
    url: result.url.clone(),
    param: result.param.clone(),
    payload: result.payload.clone(),
    evidence,
  })
}

fn extract_line(body: &str, marker: &str) -> String {
  body
    .lines()
    .find(|l| l.contains(marker))
    .unwrap_or(marker)
    .trim()
    .chars()
    .take(80)
    .collect()
}
