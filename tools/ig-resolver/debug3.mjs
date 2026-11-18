const UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124 Safari/537.36";
const APP_ID = "936619743392459";
const id = process.argv[2] || "DVK5Ex0DeNc";

const EMBED_HEADERS = {
  Accept: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
  "Accept-Language": "en-GB,en;q=0.9",
  "Cache-Control": "max-age=0",
  Dnt: "1",
  Priority: "u=0, i",
  "Sec-Ch-Ua": '"Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99"',
  "Sec-Ch-Ua-Mobile": "?0",
  "Sec-Ch-Ua-Platform": '"macOS"',
  "Sec-Fetch-Dest": "document",
  "Sec-Fetch-Mode": "navigate",
  "Sec-Fetch-Site": "none",
  "Sec-Fetch-User": "?1",
  "Upgrade-Insecure-Requests": "1",
  "User-Agent": UA,
};

function objFromEntries(name, data) {
  const m = new RegExp(`\\["${name}",.*?,(\\{.*?\\}),\\d+\\]`).exec(data);
  try { return m ? JSON.parse(m[1]) : null; } catch { return null; }
}

const r = await fetch(`https://www.instagram.com/p/${id}/`, { headers: EMBED_HEADERS });
const html = await r.text();
const jar = (r.headers.getSetCookie?.() || []).map((c) => c.split(";")[0]).filter(Boolean);
const sd = objFromEntries("SiteData", html);
const pol = objFromEntries("PolarisSiteData", html);
const web = objFromEntries("DGWWebConfig", html);
const push = objFromEntries("InstagramWebPushInfo", html);
const lsd = (objFromEntries("LSD", html) || {}).token;
const csrf = (objFromEntries("InstagramSecurityConfig", html) || {}).csrf_token;
jar.push(`csrftoken=${csrf}`, `ig_did=${pol?.device_id}`, "wd=1280x720", "dpr=2", `mid=${pol?.machine_id}`, "ig_nrcb=1");
const cookie = jar.filter(Boolean).join("; ");
console.log("anon cookie:", cookie.slice(0, 120));

const body = new URLSearchParams({
  __d: "www", __a: "1", __s: "::" + Math.random().toString(36).slice(2, 8),
  __hs: sd?.haste_session || "x", __req: "b", __ccg: "EXCELLENT",
  __rev: push?.rollout_hash || "1019933358", __hsi: sd?.hsi || "1",
  __user: "0", av: "0", dpr: "2", lsd,
  fb_api_caller_class: "RelayModern",
  fb_api_req_friendly_name: "PolarisPostActionLoadPostQueryQuery",
  variables: JSON.stringify({ shortcode: id, fetch_tagged_user_count: null, hoisted_comment_id: null, hoisted_reply_id: null }),
  server_timestamps: "true", doc_id: "8845758582119845",
}).toString();

const res = await fetch("https://www.instagram.com/graphql/query", {
  method: "POST",
  headers: {
    ...EMBED_HEADERS,
    "x-ig-app-id": web?.appId || APP_ID,
    "X-FB-LSD": lsd,
    "X-CSRFToken": csrf || "",
    "X-Bloks-Version-Id": (objFromEntries("WebBloksVersioningID", html) || {}).versioningID,
    "x-asbd-id": "129477",
    cookie,
    "content-type": "application/x-www-form-urlencoded",
    "X-FB-Friendly-Name": "PolarisPostActionLoadPostQueryQuery",
    "Sec-Fetch-Site": "same-origin",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Dest": "empty",
  },
  body,
});
console.log("gql status", res.status);
const txt = await res.text();
console.log("len", txt.length, "head", txt.slice(0, 160));
const json = JSON.parse(txt);
const media = json?.data?.xdt_shortcode_media;
if (media) {
  console.log("OK is_video", media.is_video);
  console.log("display", (media.display_url || "").slice(0, 70));
  console.log("video", (media.video_url || "none").slice(0, 70));
  console.log("carousel", media.edge_sidecar_to_children?.edges?.length || 0);
} else {
  console.log("NO MEDIA", txt.slice(0, 120));
}
