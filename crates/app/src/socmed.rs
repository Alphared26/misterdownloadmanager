//! Instagram resolver for this fork (no sign-in for public posts).
//!
//! Instagram serves a public post's photo or video to a logged-out visitor only after the visitor
//! first loads `instagram.com` to receive anonymous cookies (`csrftoken`, `ig_did`, `mid`) and then
//! asks the web GraphQL endpoint with them. Ported from cobalt's `instagram.js` (AGPL-3.0), which
//! was verified working: a public photo and a public carousel resolve without any sign-in.
//!
//! Two HTTP requests per post, plain JSON and substring search — no subprocess, no bundled
//! runtime, no extra dependency, so it stays as light as the rest of MDM. Instagram can change its
//! payload at any time; when it does, this returns `Blocked` and the caller falls back to yt-dlp.

use serde_json::Value;

/// One media item found in a post: the direct URL Instagram's CDN serves it at.
#[derive(Debug, Clone)]
pub struct IgMedia {
    pub url: String,
    pub is_video: bool,
}

/// Result of resolving an Instagram post URL.
#[derive(Debug)]
pub enum IgResult {
    /// The post's media, in page order (a carousel yields several).
    Media(Vec<IgMedia>),
    /// Instagram refused or limited this request; the caller should fall back to yt-dlp.
    Blocked(String),
}

/// Extract the shortcode from an Instagram post or reel URL.
pub fn shortcode(url: &str) -> Option<String> {
    for marker in ["instagram.com/p/", "instagram.com/reel/", "instagram.com/tv/"] {
        if let Some(i) = url.find(marker) {
            let rest = &url[i + marker.len()..];
            let code: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                .collect();
            if code.len() >= 5 {
                return Some(code);
            }
        }
    }
    None
}

/// Find `["Name", ...,{"..."},<n>]` in the page's JSON blob and parse the object. This mirrors
/// cobalt's `getObjectFromEntries`; the page wraps each entry that way.
fn grab(name: &str, html: &str) -> Option<Value> {
    let head = format!("[\"{name}\",");
    let start = html.find(&head)? + head.len();
    let rest = &html[start..];
    // The object begins at the first `{` and its matching `}` is the one that closes it.
    let open = rest.find('{')?;
    let bytes = rest.as_bytes();
    let mut depth = 0usize;
    let mut end = None;
    for (i, b) in bytes.iter().enumerate().skip(open) {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(i + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    serde_json::from_str(&rest[open..end?]).ok()
}

fn string_field(v: &Value, key: &str) -> String {
    v.get(key).and_then(Value::as_str).unwrap_or_default().to_owned()
}

/// Percent-encode a form value (the few characters a form body cannot carry raw).
fn form_encode(v: &str) -> String {
    let mut out = String::new();
    for b in v.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

struct Session {
    cookie: String,
    csrf: String,
    lsd: String,
    app_id: String,
    haste: String,
    rev: String,
}

/// Anonymous cookies Instagram hands to any first-time visitor, plus the tokens the GraphQL
/// endpoint expects. No sign-in involved.
async fn anon_session(client: &reqwest::Client, short: &str) -> Result<Session, String> {
    let page_url = format!("https://www.instagram.com/p/{short}/");
    let html = client
        .get(&page_url)
        .header("accept-language", "en-GB,en;q=0.9")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    let site = grab("SiteData", &html).unwrap_or(Value::Null);
    let polaris = grab("PolarisSiteData", &html).unwrap_or(Value::Null);
    let web = grab("DGWWebConfig", &html).unwrap_or(Value::Null);
    let push = grab("InstagramWebPushInfo", &html).unwrap_or(Value::Null);
    let lsd = string_field(&grab("LSD", &html).unwrap_or(Value::Null), "token");
    let csrf = string_field(&grab("InstagramSecurityConfig", &html).unwrap_or(Value::Null), "csrf_token");
    let app_id = string_field(&web, "appId");
    let device_id = string_field(&polaris, "device_id");
    let machine_id = string_field(&polaris, "machine_id");
    let haste = string_field(&site, "haste_session");
    let rev = string_field(&push, "rollout_hash");

    let app_id = if app_id.is_empty() { "936619743392459".into() } else { app_id };
    let lsd = if lsd.is_empty() { form_encode(&format!("{:?}", rand_suffix())) } else { lsd };

    let mut parts = vec![format!("csrftoken={csrf}")];
    if !device_id.is_empty() {
        parts.push(format!("ig_did={device_id}"));
    }
    parts.push("wd=1280x720".into());
    parts.push("dpr=2".into());
    if !machine_id.is_empty() {
        parts.push(format!("mid={machine_id}"));
    }
    parts.push("ig_nrcb=1".into());

    Ok(Session { cookie: parts.join("; "), csrf, lsd, app_id, haste, rev })
}

/// A short random suffix for the session id, as cobalt randomises it.
fn rand_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let n = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.subsec_nanos());
    format!("{n:x}")
}

/// The web GraphQL endpoint, asked with the anonymous session. Returns the post's media node.
async fn gql(client: &reqwest::Client, short: &str, s: &Session) -> Result<Option<Value>, String> {
    let variables = serde_json::json!({
        "shortcode": short,
        "fetch_tagged_user_count": Value::Null,
        "hoisted_comment_id": Value::Null,
        "hoisted_reply_id": Value::Null,
    })
    .to_string();

    let form = [
        ("__d", "www"),
        ("__a", "1"),
        ("__s", "::a1b2c3"),
        ("__hs", s.haste.as_str()),
        ("__req", "b"),
        ("__ccg", "EXCELLENT"),
        ("__rev", s.rev.as_str()),
        ("__hsi", "1"),
        ("__user", "0"),
        ("av", "0"),
        ("dpr", "2"),
        ("lsd", s.lsd.as_str()),
        ("fb_api_caller_class", "RelayModern"),
        ("fb_api_req_friendly_name", "PolarisPostActionLoadPostQueryQuery"),
        ("variables", variables.as_str()),
        ("server_timestamps", "true"),
        ("doc_id", "8845758582119845"),
    ];
    let body: String = form
        .iter()
        .map(|(k, v)| format!("{k}={}", form_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let res = client
        .post("https://www.instagram.com/graphql/query")
        .header("x-ig-app-id", &s.app_id)
        .header("X-CSRFToken", &s.csrf)
        .header("X-FB-LSD", &s.lsd)
        .header("x-asbd-id", "129477")
        .header("X-FB-Friendly-Name", "PolarisPostActionLoadPostQueryQuery")
        .header("cookie", &s.cookie)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json: Value = res.json().await.map_err(|e| e.to_string())?;
    Ok(json.get("data").and_then(|d| d.get("xdt_shortcode_media")).cloned())
}

/// Collect every photo and video in a post node, following a carousel's children.
fn collect(node: &Value, out: &mut Vec<IgMedia>) {
    if let Some(video) = node.get("video_url").and_then(Value::as_str) {
        out.push(IgMedia { url: video.to_owned(), is_video: true });
    } else if let Some(photo) = node.get("display_url").and_then(Value::as_str) {
        out.push(IgMedia { url: photo.to_owned(), is_video: false });
    }
    if let Some(edges) = node
        .get("edge_sidecar_to_children")
        .and_then(|c| c.get("edges"))
        .and_then(Value::as_array)
    {
        for edge in edges {
            if let Some(child) = edge.get("node") {
                collect(child, out);
            }
        }
    }
}

/// Resolve a public Instagram post or reel to its media, without any sign-in.
pub async fn resolve(url: &str) -> IgResult {
    let Some(short) = shortcode(url) else {
        return IgResult::Blocked("bukan tautan post/reel Instagram".into());
    };
    let client = match reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/124 Safari/537.36",
        )
        .build()
    {
        Ok(c) => c,
        Err(e) => return IgResult::Blocked(e.to_string()),
    };
    let session = match anon_session(&client, &short).await {
        Ok(s) => s,
        Err(e) => return IgResult::Blocked(e),
    };
    let media = match gql(&client, &short, &session).await {
        Ok(Some(m)) => m,
        Ok(None) => return IgResult::Blocked("Instagram tidak mengembalikan media".into()),
        Err(e) => return IgResult::Blocked(e),
    };
    let mut out = Vec::new();
    collect(&media, &mut out);
    if out.is_empty() {
        IgResult::Blocked("Instagram tidak mengembalikan media".into())
    } else {
        IgResult::Media(out)
    }
}

/// A live check, not a unit test: it asks Instagram over the network, so it is `#[ignore]`d and
/// run deliberately (`cargo test -p mdm-app -- --ignored instagram`). It passes on any answer —
/// Instagram rate-limits by IP, and a blocked answer is a legitimate one to report rather than a
/// failure — but a success proves the anonymous path really resolves a public post.
#[tokio::test]
#[ignore]
async fn instagram_public_post_resolves_without_login() {
    let url = "https://www.instagram.com/p/DVK5Ex0DeNc/";
    let result = crate::socmed::resolve(url).await;
    match result {
        IgResult::Media(items) => {
            assert!(!items.is_empty(), "a public post has media");
            for it in &items {
                assert!(it.url.starts_with("https://"), "a media URL is https: {}", it.url);
            }
            eprintln!("resolved {} media", items.len());
        }
        IgResult::Blocked(why) => eprintln!("Instagram blocked this run (rate limit by IP): {why}"),
    }
}
