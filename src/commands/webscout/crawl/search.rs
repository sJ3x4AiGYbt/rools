use crate::commands::webscout::http;
use std::collections::HashSet;

const PATTERNS: &[(&str, &str)] = &[
  ("admin", "/admin"),
  ("admin", "/admin/login"),
  ("admin", "/administrator"),
  ("admin", "/wp-admin"),
  ("admin", "/dashboard"),
  ("admin", "/manager"),
  ("admin", "/controlpanel"),
  ("admin", "/cp"),
  ("auth", "/login"),
  ("auth", "/signin"),
  ("auth", "/logout"),
  ("auth", "/register"),
  ("auth", "/signup"),
  ("auth", "/forgot-password"),
  ("auth", "/reset-password"),
  ("api", "/api"),
  ("api", "/api/v1"),
  ("api", "/api/v2"),
  ("api", "/graphql"),
  ("api", "/rest"),
  ("api", "/swagger"),
  ("api", "/swagger-ui"),
  ("api", "/swagger-ui.html"),
  ("api", "/openapi.json"),
  ("api", "/api-docs"),
  ("config", "/.env"),
  ("config", "/.env.local"),
  ("config", "/.env.production"),
  ("config", "/config.php"),
  ("config", "/config.yml"),
  ("config", "/config.json"),
  ("config", "/settings.php"),
  ("config", "/web.config"),
  ("config", "/phpinfo.php"),
  ("vcs", "/.git"),
  ("vcs", "/.git/config"),
  ("vcs", "/.gitignore"),
  ("vcs", "/.svn"),
  ("vcs", "/.hg"),
  ("backup", "/backup"),
  ("backup", "/backup.zip"),
  ("backup", "/backup.sql"),
  ("backup", "/db.sql"),
  ("backup", "/dump.sql"),
  ("backup", "/old"),
  ("info", "/robots.txt"),
  ("info", "/sitemap.xml"),
  ("info", "/security.txt"),
  ("info", "/.well-known/security.txt"),
  ("info", "/crossdomain.xml"),
  ("info", "/humans.txt"),
  ("server", "/server-status"),
  ("server", "/server-info"),
  ("server", "/.htaccess"),
  ("server", "/nginx.conf"),
  ("server", "/web.config"),
  ("files", "/uploads"),
  ("files", "/files"),
  ("files", "/static"),
  ("files", "/assets"),
  ("files", "/media"),
];

#[derive(Debug, Clone)]
pub struct SearchHit {
  pub url: String,
  pub category: String,
  pub status: u16,
  pub size: usize,
}

pub fn run(origin: &str, client: &http::HttpClient, visited: &HashSet<String>) -> Vec<SearchHit> {
  let soft404_size = client.get(origin).ok().map(|r| r.body.len());

  let mut hits = Vec::new();
  let mut probed: HashSet<String> = HashSet::new();

  for (category, path) in PATTERNS {
    let url = format!("{}{}", origin.trim_end_matches('/'), path);

    if visited.contains(&url) || probed.contains(&url) {
      continue;
    }
    probed.insert(url.clone());

    match client.get(&url) {
      Err(_) => continue,
      Ok(res) => {
        if res.status == 404 {
          continue;
        }

        let size = res.body.len();
        let is_html = res.content_type.contains("text/html");

        if is_html
          && let Some(root_size) = soft404_size
          && size == root_size
        {
          continue;
        }

        hits.push(SearchHit {
          url,
          category: category.to_string(),
          status: res.status,
          size,
        });
      }
    }
  }

  hits
}
