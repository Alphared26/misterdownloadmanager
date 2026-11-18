// Standalone IG resolver — no npm deps, uses Node's built-in fetch.
// Ported from cobalt's instagram.js (AGPL-3.0), trimmed to public posts + reels.
const UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124 Safari/537.36";
const MOBILE_UA = "Instagram 275.0.0.27.98 Android (33/13; 280dpi; 720x1423; Xiaomi; Redmi 7; onclite; qcom; en_US; 458229237)";
const APP_ID = "936619743392459";

function objFromEntries(name, data) {
  const m = new RegExp(`\\["${name}",.*?,(\\{.*?\\}),\\d+\\]`).exec(data);
  try { return m ? JSON.parse(m[1]) : null; } catch { return null; }
}

// Step 1: load instagram.com once to collect the anonymous cookies IG hands to anyone.
async function gatherAnon(id) {
  const res = await fetch(`https://www.instagram.com/p/${id}/`, {
    headers: { "user-agent": UA, "accept-language": "en-GB,en;q=0.9", "sec-fetch-mode": "navigate", "upgrade-insecure-requests": "1" },
  });
  const html = await res.text();
  const setc = res.headers.getSetCookie?.() || [];
  const jar = setc.map((c) => c.split(";")[0]).filter(Boolean);
  const sd = objFromEntries("SiteData", html);
  const pol = objFromEntries("PolarisSiteData", html);
  const web = objFromEntries("DGWWebConfig", html);
  const push = objFromEntries("InstagramWebPushInfo", html);
  const lsd = (objFromEntries("LSD", html) || {}).token || "x";
  const csrf = (objFromEntries("InstagramSecurityConfig", html) || {}).csrf_token;
  jar.push(`csrftoken=${csrf}`, `ig_did=${pol?.device_id}`, "wd=1280x720", "dpr=2", `mid=${pol?.machine_id}`, "ig_nrcb=1");
  return { cookie: jar.filter(Boolean).join("; "), csrf, lsd, web, push, sd, html };
}

// Step 2: GraphQL query with the anonymous session — works for photos, videos and reels.
async function gql(id, s) {
  const body = new URLSearchParams({
    __d: "www", __a: "1", __s: "::" + Math.random().toString(36).slice(2, 8),
    __hs: s.sd?.haste_session || "x", __req: "b", __ccg: "EXCELLENT",
    __rev: s.push?.rollout_hash || "1019933358", __hsi: s.sd?.hsi || "1",
    __user: "0", av: "0", dpr: "2", lsd: s.lsd,
    fb_api_caller_class: "RelayModern",
    fb_api_req_friendly_name: "PolarisPostActionLoadPostQueryQuery",
    variables: JSON.stringify({ shortcode: id, fetch_tagged_user_count: null, hoisted_comment_id: null, hoisted_reply_id: null }),
    server_timestamps: "true", doc_id: "8845758582119845",
  }).toString();
  const res = await fetch("https://www.instagram.com/graphql/query", {
    method: "POST",
    headers: {
      "user-agent": UA, "content-type": "application/x-www-form-urlencoded",
      "x-ig-app-id": s.web?.appId || APP_ID, "X-CSRFToken": s.csrf || "",
      "X-FB-LSD": s.lsd, "x-asbd-id": "129477", cookie: s.cookie,
      "X-FB-Friendly-Name": "PolarisPostActionLoadPostQueryQuery",
    },
    body,
  });
  const json = await res.json().catch(() => null);
  return json?.data?.xdt_shortcode_media || null;
}

// Walk a carousel and collect every image / video.
function collect(node, out) {
  if (!node) return;
  if (node.video_url) out.push({ type: "video", url: node.video_url, thumb: node.display_url });
  else if (node.display_url) out.push({ type: "photo", url: node.display_url });
  for (const e of node.edge_sidecar_to_children?.edges || []) collect(e.node, out);
}

async function resolve(url) {
  const m = /instagram\.com\/(?:[\w.]+\/)?(p|reel|tv)\/([\w-]{5,})/.exec(url);
  if (!m) return { error: "bukan tautan IG post/reel" };
  const id = m[2];
  const s = await gatherAnon(id);
  const media = await gql(id, s);
  if (!media) return { error: "Gagal mengambil media (Instagram mungkin membatasi akses)" };
  const out = [];
  collect(media, out);
  if (!out.length) return { error: "Tidak ada media ditemukan" };
  return {
    is_video: !!media.is_video,
    caption: media.edge_media_to_caption?.edges?.[0]?.node?.text || "",
    media: out,
  };
}

const port = process.env.PORT || 12711;
const http = require("http");
http.createServer(async (req, res) => {
  res.setHeader("Access-Control-Allow-Origin", "*");
  const u = new URL(req.url, "http://x");
  const target = u.searchParams.get("url");
  res.setHeader("Content-Type", "application/json; charset=utf-8");
  if (!target) { res.end(JSON.stringify({ error: "parameter url kosong" })); return; }
  try {
    res.end(JSON.stringify(await resolve(target)));
  } catch (e) {
    res.end(JSON.stringify({ error: String(e && e.message || e) }));
  }
}).listen(port, "127.0.0.1", () => console.log(`IG resolver on 127.0.0.1:${port}`));
