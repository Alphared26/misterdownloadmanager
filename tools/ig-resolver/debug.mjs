const UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124 Safari/537.36";
const id = "DVK5Ex0DeNc";
const r = await fetch(`https://www.instagram.com/p/${id}/`, { headers: { "user-agent": UA, "accept-language": "en-GB,en;q=0.9" } });
const html = await r.text();
const get = (name) => {
  const m = new RegExp(`\\["${name}",.*?,(\\{.*?\\}),\\d+\\]`).exec(html);
  return m ? m[1].slice(0, 120) : null;
};
for (const n of ["SiteData", "PolarisSiteData", "DGWWebConfig", "InstagramWebPushInfo", "LSD", "InstagramSecurityConfig"]) {
  console.log(n, "=>", get(n));
}
// Look for the media payload directly
for (const k of ['"shortcode_media"', '"xdt_shortcode_media"', 'display_url', 'video_url', '"is_video"']) {
  const i = html.indexOf(k);
  console.log(k, i);
}
