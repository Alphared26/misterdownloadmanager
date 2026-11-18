const UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124 Safari/537.36";
const APP_ID = "936619743392459";
const id = process.argv[2] || "DVK5Ex0DeNc";

function objFromEntries(name, data) {
  const m = new RegExp(`\\["${name}",.*?,(\\{.*?\\}),\\d+\\]`).exec(data);
  try { return m ? JSON.parse(m[1]) : null; } catch (e) { console.log("parse fail", name, e.message); return null; }
}

const r = await fetch(`https://www.instagram.com/p/${id}/`, { headers: { "user-agent": UA, "accept-language": "en-GB,en;q=0.9" } });
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
console.log("cookie:", cookie.slice(0, 160));

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
    "user-agent": UA, "content-type": "application/x-www-form-urlencoded",
    "x-ig-app-id": web?.appId || APP_ID, "X-CSRFToken": csrf || "",
    "X-FB-LSD": lsd, "x-asbd-id": "129477", cookie,
    "X-FB-Friendly-Name": "PolarisPostActionLoadPostQueryQuery",
  },
  body,
});
console.log("gql status", res.status);
const txt = await res.text();
console.log("gql len", txt.length, "head", txt.slice(0, 200));
const json = JSON.parse(txt);
const media = json?.data?.xdt_shortcode_media;
console.log("has media", !!media, "is_video", media?.is_video, "err", json?.message || "");
if (media) {
  console.log("display", media.display_url?.slice(0, 80));
  console.log("video", media.video_url?.slice(0, 80));
  console.log("children", media.edge_sidecar_to_children?.edges?.length);
}
