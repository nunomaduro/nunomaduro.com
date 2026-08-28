#!/usr/bin/env node
'use strict';

/**
 * Updates the About page (presentation/templates/about.html) from live data:
 *   - the per-platform follower counts on each social link, and
 *   - the hero stat cards: total open-source downloads (summed from Packagist)
 *     and total followers across all platforms. Hero values are placeholders
 *     located by `data-stat` attributes and rewritten in place; if a source is
 *     unavailable the card is left exactly as-is.
 *
 * Usage:
 *   node scripts/update-social.js          # fetch + update the template
 *   node scripts/update-social.js --dry    # fetch + print, but do not write
 *
 * Each account is located in the HTML by its (unique) link href. If that link
 * already shows a count, the number is updated in place; if it doesn't, a count
 * <span> is inserted (matching the existing markup). When a fetch returns no
 * number the link is left exactly as-is, so the page never breaks and accounts
 * with no public count simply stay as plain links.
 *
 * Every platform is read live. Four of them have no public, documented API for
 * follower counts and are fetched the way a browser or a crawler would be
 * served instead (see the fetchers below): X via the web app's own guest-token
 * GraphQL call, Threads via the crawler-facing og:description, LinkedIn via the
 * logged-out profile page, Kick via its channel API. Those are the fragile
 * ones, so each falls back to a manual `X_FOLLOWERS` / `THREADS_FOLLOWERS` /
 * `LINKEDIN_FOLLOWERS` / `KICK_FOLLOWERS` env var, and then to a terminal
 * prompt — type a value (e.g. 66k, 1.2k, 1500) or press enter to skip and leave
 * the link exactly as-is. GitHub's API is rate-limited when anonymous; set
 * `GITHUB_TOKEN` to raise the limit.
 */

const fs = require('fs');
const path = require('path');
const readline = require('readline');

// Ask the user a question on the terminal and resolve to their trimmed answer.
// In a non-interactive run (CI, piped input) this resolves to '' immediately so
// the script never hangs waiting on stdin.
function ask(question) {
  if (!process.stdin.isTTY) return Promise.resolve('');
  const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
  return new Promise((resolve) =>
    rl.question(question, (answer) => {
      rl.close();
      resolve(answer.trim());
    }),
  );
}

const ABOUT_HTML = path.join(__dirname, '../presentation/templates/about.html');

// Markup used for an inserted count span — mirrors the existing count spans.
const COUNT_CLASS =
  'text-base sm:text-sm font-semibold tabular-nums text-zinc-500 group-hover:text-amber-400';

const UA =
  'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 ' +
  '(KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36';

// Browser-like headers: several platforms (e.g. Instagram) serve a bare JS
// shell to plain fetch() but the real, server-rendered page to a "browser".
const BROWSER_HEADERS = {
  'User-Agent': UA,
  Accept:
    'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8',
  'Accept-Language': 'en-US,en;q=0.9',
  'Sec-Fetch-Dest': 'document',
  'Sec-Fetch-Mode': 'navigate',
  'Sec-Fetch-Site': 'none',
  'Upgrade-Insecure-Requests': '1',
};

async function getText(url, headers = {}) {
  const res = await fetch(url, {
    headers: { ...BROWSER_HEADERS, ...headers },
    redirect: 'follow',
  });
  if (!res.ok) throw new Error(`HTTP ${res.status} for ${url}`);
  return res.text();
}

// Parse an abbreviated count like "28.7K" / "1.2M" / "4,040" into a number.
function parseAbbrev(str) {
  const m = String(str).replace(/,/g, '').match(/([\d.]+)\s*([KkMm]?)/);
  if (!m) return null;
  const mult = { k: 1e3, m: 1e6 }[m[2].toLowerCase()] || 1;
  return Math.round(parseFloat(m[1]) * mult);
}

// --- Per-platform fetchers (return a raw count, or null if unavailable) ---

async function youtubeSubs(handle) {
  // The public channel page exposes "<n> subscribers" once consent is set.
  // Channels with a hidden/zero count render nothing, so we return null.
  const html = await getText(`https://youtube.com/@${handle}`, {
    Cookie: 'CONSENT=YES+1',
  });

  // A channel page also embeds cards for *other* channels (links, features),
  // each carrying its own "<n> subscribers" label — so take the count that
  // sits next to this exact handle rather than the first one on the page.
  const own = html.match(
    new RegExp(`@${escapeRegex(handle)}(?![\\w.-])[^0-9]{0,16}([\\d.,]+[KM]?)\\s+subscribers`, 'i'),
  );
  if (own) return parseAbbrev(own[1]);

  // Some (usually smaller) channels render the header without the handle. Fall
  // back to the page's count only when it's unambiguous — i.e. every
  // "<n> subscribers" on the page shows the same number.
  const all = [...html.matchAll(/([\d.,]+[KM]?)\s+subscribers/gi)].map((m) => m[1]);
  const unique = [...new Set(all)];
  return unique.length === 1 ? parseAbbrev(unique[0]) : null;
}

async function tiktokFollowers(handle) {
  const html = await getText(`https://tiktok.com/@${handle}`);
  const m = html.match(/"followerCount":(\d+)/);
  return m ? parseInt(m[1], 10) : null;
}

async function githubFollowers(user) {
  const headers = { Accept: 'application/vnd.github+json' };
  if (process.env.GITHUB_TOKEN) headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
  const json = JSON.parse(await getText(`https://api.github.com/users/${user}`, headers));
  return typeof json.followers === 'number' ? json.followers : null;
}

async function mastodonFollowers(instance, user) {
  // Mastodon exposes a clean public API for any account on the instance.
  const url = `https://${instance}/api/v1/accounts/lookup?acct=${user}`;
  const json = JSON.parse(await getText(url, { Accept: 'application/json' }));
  return typeof json.followers_count === 'number' ? json.followers_count : null;
}

async function kickFollowers(handle) {
  // Kick's channel API is public but fronted by Cloudflare, which sometimes
  // 403s server-side requests — hence the `KICK_FOLLOWERS` fallback. Note it
  // returns followers_count as a string ("86") about as often as a number.
  const json = JSON.parse(
    await getText(`https://kick.com/api/v2/channels/${handle}`, { Accept: 'application/json' }),
  );
  const count = parseInt(json.followers_count, 10);
  return Number.isFinite(count) ? count : null;
}

// X's web app talks to its own GraphQL API, and it does so before you log in —
// so the same anonymous handshake works here: activate a guest token with the
// bundle's (public, hard-coded) bearer token, then ask for the profile. The
// query id is baked into that bundle and rotates on deploys, so we try the ones
// we know of in turn and let the caller fall back to `X_FOLLOWERS`.
const X_BEARER =
  'AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D' +
  '1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA';

const X_QUERY_IDS = [
  'sLVLhk0bGj3MVFEKTdax1w',
  'G3KGOASz96M-Qu0nwmGXNg',
  'qW5u-DAuXpMEG0zA1F7UGQ',
  'NimuplG1OB7Fd2btCLdBOw',
  'k5XapwcSikNsEsILW5FvgA',
];

// UserByScreenName rejects the call unless every feature flag it knows about is
// present, so send the full set the web app sends.
const X_FEATURES = {
  hidden_profile_subscriptions_enabled: true,
  rweb_tipjar_consumption_enabled: true,
  responsive_web_graphql_exclude_directive_enabled: true,
  verified_phone_label_enabled: false,
  subscriptions_verification_info_is_identity_verified_enabled: true,
  subscriptions_verification_info_verified_since_enabled: true,
  highlights_tweets_tab_ui_enabled: true,
  responsive_web_twitter_article_notes_tab_enabled: true,
  subscriptions_feature_can_gift_premium: true,
  creator_subscriptions_tweet_preview_api_enabled: true,
  responsive_web_graphql_skip_user_profile_image_extensions_enabled: false,
  responsive_web_graphql_timeline_navigation_enabled: true,
};

async function xFollowers(handle) {
  const activate = await fetch('https://api.x.com/1.1/guest/activate.json', {
    method: 'POST',
    headers: { Authorization: `Bearer ${X_BEARER}`, 'User-Agent': UA },
  });
  if (!activate.ok) throw new Error(`HTTP ${activate.status} for guest/activate.json`);
  const { guest_token: guestToken } = JSON.parse(await activate.text());

  const query =
    `variables=${encodeURIComponent(JSON.stringify({ screen_name: handle }))}` +
    `&features=${encodeURIComponent(JSON.stringify(X_FEATURES))}`;

  for (const id of X_QUERY_IDS) {
    const res = await fetch(`https://api.x.com/graphql/${id}/UserByScreenName?${query}`, {
      headers: {
        Authorization: `Bearer ${X_BEARER}`,
        'x-guest-token': guestToken,
        'User-Agent': UA,
      },
    });
    if (!res.ok) continue; // a rotated-out query id — try the next one
    const m = (await res.text()).match(/"followers_count":(\d+)/);
    if (m) return parseInt(m[1], 10);
  }
  return null;
}

// Threads serves plain fetch() a bare JS shell, but it serves crawlers a
// server-rendered page whose og:description opens with "<n> Followers".
async function threadsFollowers(handle) {
  const html = await getText(`https://www.threads.com/@${handle}`, {
    'User-Agent': 'Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)',
  });
  const og = html.match(/<meta[^>]+property="og:description"[^>]+content="([^"]*)"/i);
  const m = (og ? og[1] : '').match(/([\d.,]+[KM]?)\s+Followers/i);
  return m ? parseAbbrev(m[1]) : null;
}

// LinkedIn only shows the count to signed-in visitors *in the app*, but the
// logged-out public profile still renders it. The page also carries "N
// followers" for each suggested person in the sidebar, so match the profile's
// own line — the one paired with the connections count.
async function linkedinFollowers(slug) {
  const html = await getText(`https://www.linkedin.com/in/${slug}/`);
  const m =
    html.match(/([\d.,]+[KM]?)\s*followers\s*<\/span>\s*<span>[^<]*connections/i) ||
    html.match(/not-first-middot[\s\S]{0,200}?([\d.,]+[KM]?)\s*followers/i);
  return m ? parseAbbrev(m[1]) : null;
}

/**
 * Read a count from a platform that may well refuse to answer, falling back to
 * a manual `<PLATFORM>_FOLLOWERS` env var. Returning null (rather than
 * throwing) lets the caller prompt for the number and, failing that, leave the
 * link untouched.
 */
async function withOverride(label, envVar, fetcher) {
  try {
    const count = await fetcher();
    if (count != null) return count;
    console.warn(`⚠  ${label.padEnd(12)} no count in the response — trying ${envVar}`);
  } catch (err) {
    console.warn(`⚠  ${label.padEnd(12)} live fetch failed (${err.message}) — trying ${envVar}`);
  }
  return process.env[envVar] ? parseAbbrev(process.env[envVar]) : null;
}

// Each account: the (unique) href that locates it in the HTML + a fetcher.
const ACCOUNTS = [
  {
    label: 'x',
    href: 'https://twitter.com/enunomaduro',
    fetch: () => withOverride('x', 'X_FOLLOWERS', () => xFollowers('enunomaduro')),
  },
  {
    label: 'linkedin',
    href: 'https://www.linkedin.com/in/nunomaduro/',
    fetch: () =>
      withOverride('linkedin', 'LINKEDIN_FOLLOWERS', () => linkedinFollowers('nunomaduro')),
  },
  {
    label: 'youtube',
    href: 'https://youtube.com/nunomaduro?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro'),
  },
  {
    label: 'youtube·extra',
    href: 'https://youtube.com/@nunomaduro_extra?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro_extra'),
  },
  {
    label: 'youtube·extra2',
    href: 'https://youtube.com/@nunomaduro_extra2?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro_extra2'),
  },
  {
    label: 'youtube·vods',
    href: 'https://youtube.com/@nunomaduro_vods?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro_vods'),
  },
  {
    label: 'tiktok',
    href: 'https://tiktok.com/@enunomaduro',
    fetch: () => tiktokFollowers('enunomaduro'),
  },
  {
    label: 'tiktok·extra',
    href: 'https://tiktok.com/@nunomaduro_extra',
    fetch: () => tiktokFollowers('nunomaduro_extra'),
  },
  {
    label: 'instagram',
    href: 'https://instagram.com/enunomaduro',
    async fetch() {
      const html = await getText('https://instagram.com/enunomaduro/');
      const a = html.match(/"edge_followed_by":\{"count":(\d+)\}/);
      if (a) return parseInt(a[1], 10);
      const b = html.match(/([\d.,]+)\s+Followers/i);
      return b ? parseAbbrev(b[1]) : null;
    },
  },
  {
    label: 'instagram·extra',
    href: 'https://instagram.com/nunomaduro_extra',
    async fetch() {
      const html = await getText('https://instagram.com/nunomaduro_extra/');
      const a = html.match(/"edge_followed_by":\{"count":(\d+)\}/);
      if (a) return parseInt(a[1], 10);
      const b = html.match(/([\d.,]+)\s+Followers/i);
      return b ? parseAbbrev(b[1]) : null;
    },
  },
  {
    label: 'twitch',
    href: 'https://twitch.tv/nunomaduro',
    async fetch() {
      const json = await getText('https://api.ivr.fi/v2/twitch/user?login=nunomaduro');
      const m = json.match(/"followers":(\d+)/);
      return m ? parseInt(m[1], 10) : null;
    },
  },
  {
    label: 'github',
    href: 'https://github.com/nunomaduro',
    fetch: () => githubFollowers('nunomaduro'),
  },
  {
    label: 'kick',
    href: 'https://kick.com/nunomaduro',
    fetch: () => withOverride('kick', 'KICK_FOLLOWERS', () => kickFollowers('nunomaduro')),
  },
  {
    label: 'mastodon',
    href: 'https://mastodon.social/@nunomaduro',
    fetch: () => mastodonFollowers('mastodon.social', 'nunomaduro'),
  },
  {
    label: 'threads',
    href: 'https://threads.com/@enunomaduro',
    fetch: () =>
      withOverride('threads', 'THREADS_FOLLOWERS', () => threadsFollowers('enunomaduro')),
  },
];

// Format a raw count the way the site does: whole "k" for >= 10k, one decimal
// below that, and the plain number under 1k (e.g. 66k, 29k, 9.2k, 3.1k, 3).
function formatCount(n) {
  if (n < 1000) return String(n);
  const k = n / 1000;
  if (k >= 10) return `${Math.round(k)}k`;
  const r = Math.round(k * 10) / 10;
  return `${Number.isInteger(r) ? r : r.toFixed(1)}k`;
}

function escapeRegex(s) {
  return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

/**
 * Set an account's count within its <a href="..."> block. Updates an existing
 * count span, or inserts one before </a> if none exists. Returns
 * { html, current, action } or null if the link isn't in the template.
 * `action` is 'unchanged' | 'update' | 'insert'.
 */
function setCount(html, href, value) {
  const reAnchor = new RegExp(`<a\\b[^>]*href="${escapeRegex(href)}"[\\s\\S]*?</a>`);
  const am = html.match(reAnchor);
  if (!am) return null;

  let anchor = am[0];
  let current = null;
  let action;

  const reSpan = /(<span class="[^"]*tabular-nums[^"]*">)([^<]*)(<\/span>)/;
  const sm = anchor.match(reSpan);

  if (sm) {
    current = sm[2];
    action = current === value ? 'unchanged' : 'update';
    anchor = anchor.replace(reSpan, `$1${value}$3`);
  } else {
    action = 'insert';
    anchor = anchor.replace(
      /\n([ \t]*)<\/a>$/,
      (_, indent) => `\n${indent}    <span class="${COUNT_CLASS}">${value}</span>\n${indent}</a>`,
    );
  }

  return { html: html.replace(reAnchor, anchor), current, action };
}

// The list of social links on the page, in source order.
const CONTAINER_RE = /(<div class="flex flex-col gap-3">\n)([\s\S]*?)(\n[ \t]*<\/div>)/;
const LINK_INDENT = ' '.repeat(16);

/**
 * Reorder the social links by displayed count, highest first. Links with no
 * count (e.g. @nunomaduro4) sort last; ties keep their original order.
 * Returns { html, order } where `order` is the new list of hrefs (or null if
 * the container/links weren't found).
 */
function sortByCount(html) {
  const m = html.match(CONTAINER_RE);
  if (!m) return { html, order: null };

  const [, open, body, close] = m;
  const anchors = body.match(/<a\b[\s\S]*?<\/a>/g);
  if (!anchors) return { html, order: null };

  const keyed = anchors.map((a, i) => {
    const span = a.match(/tabular-nums[^>]*>([^<]*)<\/span>/);
    const count = span ? parseAbbrev(span[1]) ?? -1 : -1;
    const href = (a.match(/href="([^"]*)"/) || [])[1] || '';
    return { a, count, href, i };
  });

  const sorted = [...keyed].sort((x, y) => y.count - x.count || x.i - y.i);
  const newBody = sorted.map((k) => LINK_INDENT + k.a).join('\n');

  return {
    html: html.replace(CONTAINER_RE, `${open}${newBody}${close}`),
    order: sorted.map((k) => k.href),
  };
}

// --- Aggregate stat cards (downloads hero, followers total) ---

// Format a large number compactly for the hero: 352M, 612M, 1.2B, 12.3B.
function formatBig(n) {
  if (n >= 1e9) {
    const b = n / 1e9;
    return `${b >= 10 ? Math.round(b * 10) / 10 : Math.round(b * 100) / 100}B`;
  }
  if (n >= 1e6) return `${Math.round(n / 1e6)}M`;
  if (n >= 1e3) return `${Math.round(n / 1e3)}k`;
  return String(n);
}

// Replace the inner text of a placeholder element located by a data attribute,
// e.g. <span data-stat="downloads">…</span>. Returns { html, found }.
function setStat(html, key, value) {
  const re = new RegExp(`(<(\\w+)[^>]*\\bdata-stat="${key}"[^>]*>)([\\s\\S]*?)(</\\2>)`);
  if (!re.test(html)) return { html, found: false };
  return { html: html.replace(re, (_m, open, _t, _inner, close) => `${open}${value}${close}`), found: true };
}

// Read the count currently shown for a social link (so a skipped account still
// contributes its existing value to the followers total).
function currentCount(html, href) {
  const reAnchor = new RegExp(`<a\\b[^>]*href="${escapeRegex(href)}"[\\s\\S]*?</a>`);
  const am = html.match(reAnchor);
  if (!am) return null;
  const sm = am[0].match(/tabular-nums[^>]*>([^<]*)<\/span>/);
  return sm ? parseAbbrev(sm[1]) : null;
}

// The open-source downloads hero sums every Packagist package under the vendors
// (orgs) below — each vendor is auto-expanded via Packagist's list.json, so new
// packages published under them are picked up automatically — plus the
// individually-listed laravel/* packages (the laravel vendor as a whole isn't
// owned here, so only the ones authored/co-authored by Nuno are counted). The
// trailing "+" on the hero means the exact figure doesn't need to be perfect.
const VENDORS = [
  'nunomaduro',       // personal vendor (collision, termwind, pail, phpinsights, …)
  'pestphp',          // Pest ecosystem
  'larastan',         // larastan/larastan
  'openai-php',       // client, laravel, symfony
  'laravel-zero',     // framework, foundation, …
  'pinkary-project',
  'web3-php',
  'narration',
  'gloss-php',
];

// laravel/* packages Nuno created or co-created (Packagist credits him as an
// author on each). Listed explicitly since the whole laravel vendor isn't his.
const LARAVEL_PACKAGES = [
  'laravel/serializable-closure',
  'laravel/pint',
  'laravel/pail',
  'laravel/vapor-ui',
  'laravel/pao',
  'laravel/cloud-cli',
];

// Packagist throttles bursts of requests with 429s; retry those (and transient
// 5xx) with a short backoff so a big vendor expansion doesn't drop counts.
async function getTextRetry(url, headers = {}, tries = 5) {
  for (let attempt = 1; ; attempt++) {
    try {
      return await getText(url, headers);
    } catch (err) {
      const status = Number((/HTTP (\d+)/.exec(err.message) || [])[1]) || 0;
      const retryable = status === 429 || status >= 500 || status === 0;
      if (!retryable || attempt >= tries) throw err;
      await new Promise((r) => setTimeout(r, 700 * attempt));
    }
  }
}

// Run an async fn over items with a fixed concurrency, preserving input order.
async function mapLimit(items, limit, fn) {
  const out = [];
  let i = 0;
  await Promise.all(
    Array.from({ length: Math.min(limit, items.length) }, async () => {
      while (i < items.length) {
        const idx = i++;
        out[idx] = await fn(items[idx], idx);
      }
    }),
  );
  return out;
}

async function packageDownloads(name) {
  const json = JSON.parse(await getTextRetry(`https://packagist.org/packages/${name}.json`, {
    Accept: 'application/json',
  }));
  const d = json?.package?.downloads?.total ?? json?.downloads?.total;
  return typeof d === 'number' ? d : null;
}

// Every package name published under a vendor (e.g. all of pestphp/*).
async function vendorPackages(vendor) {
  const json = JSON.parse(await getTextRetry(`https://packagist.org/packages/list.json?vendor=${vendor}`, {
    Accept: 'application/json',
  }));
  return Array.isArray(json.packageNames) ? json.packageNames : [];
}

async function packagistTotal() {
  // Expand each vendor to its package list, then merge with the explicit
  // laravel/* packages (de-duped, in case a name appears twice).
  const vendorLists = await mapLimit(VENDORS, 4, async (vendor) => {
    try {
      return await vendorPackages(vendor);
    } catch (err) {
      console.warn(`⚠  packagist   vendor ${vendor}: ${err.message}`);
      return [];
    }
  });

  const names = [...new Set([...vendorLists.flat(), ...LARAVEL_PACKAGES])];

  const counts = await mapLimit(names, 4, async (name) => {
    try {
      return await packageDownloads(name);
    } catch (err) {
      console.warn(`⚠  packagist   ${name}: ${err.message}`);
      return null;
    }
  });

  const got = counts.filter((n) => typeof n === 'number');
  if (!got.length) return null;
  console.log(`∑  packages    ${got.length}/${names.length} counted across ${VENDORS.length} vendors + ${LARAVEL_PACKAGES.length} laravel/*`);
  return got.reduce((a, b) => a + b, 0);
}

async function main() {
  const dryRun = process.argv.includes('--dry');
  const original = fs.readFileSync(ABOUT_HTML, 'utf8');
  let html = original;
  let changed = 0;

  // Kick off the slower downloads fetch up-front so it resolves while we
  // (possibly) prompt for the manual follower counts below.
  const downloadsP = packagistTotal().catch((e) => {
    console.warn(`⚠  downloads   ${e.message}`);
    return null;
  });

  const results = await Promise.all(
    ACCOUNTS.map(async (acc) => {
      try {
        return { acc, count: await acc.fetch() };
      } catch (err) {
        return { acc, count: null, error: err.message };
      }
    }),
  );

  let followersTotal = 0;
  for (const { acc, count: fetched, error } of results) {
    const pad = acc.label.padEnd(12);
    let count = fetched;

    // For accounts we couldn't fetch (e.g. X, Threads, a Cloudflare-blocked
    // Kick), ask for the number interactively. Blank skips and leaves as-is.
    if (count == null) {
      const reason = error ? `fetch failed: ${error}` : 'no public count';
      const answer = await ask(`?  ${pad} couldn't fetch (${reason}) — enter count (blank to skip): `);
      count = answer ? parseAbbrev(answer) : null;
      if (count == null) {
        // Still count the existing displayed value toward the followers total.
        const existing = currentCount(html, acc.href);
        if (existing != null) followersTotal += existing;
        console.warn(`⚠  ${pad} skipped — left as-is`);
        continue;
      }
    }

    followersTotal += count;
    const formatted = formatCount(count);
    const res = setCount(html, acc.href, formatted);
    if (!res) {
      console.warn(`⚠  ${pad} link not found in template`);
      continue;
    }

    html = res.html;
    if (res.action === 'unchanged') {
      console.log(`=  ${pad} ${formatted} (unchanged, raw ${count})`);
    } else if (res.action === 'insert') {
      console.log(`+  ${pad} ${formatted} (added, raw ${count})`);
      changed++;
    } else {
      console.log(`✎  ${pad} ${res.current} → ${formatted} (raw ${count})`);
      changed++;
    }
  }

  // Keep the links ordered by count, highest first.
  const beforeSort = html;
  const { html: sortedHtml, order } = sortByCount(html);
  html = sortedHtml;
  const reordered = sortedHtml !== beforeSort;
  if (order) {
    const labelFor = (h) => ACCOUNTS.find((a) => a.href === h)?.label ?? h;
    console.log(`\n↕  order: ${order.map(labelFor).join(' › ')}${reordered ? '' : ' (already sorted)'}`);
  }

  // --- Update the hero stat cards ---
  console.log('');

  if (followersTotal > 0) {
    html = setStat(html, 'followers', `${formatCount(followersTotal)}+`).html;
    console.log(`∑  followers   ${formatCount(followersTotal)}+ (raw ${followersTotal.toLocaleString()})`);
  }

  const downloads = await downloadsP;
  if (downloads != null) {
    html = setStat(html, 'downloads', `${formatBig(downloads)}+`).html;
    console.log(`∑  downloads   ${formatBig(downloads)}+ (raw ${downloads.toLocaleString()})`);
  } else {
    console.warn('⚠  downloads   unavailable — left as-is');
  }

  if (dryRun) {
    console.log('\n(dry run — no files written)');
    return;
  }

  if (html !== original) {
    fs.writeFileSync(ABOUT_HTML, html);
    console.log(`\n✔ Updated ${path.relative(process.cwd(), ABOUT_HTML)}`);
    console.log('  Run `npm run build` to regenerate the site.');
  } else {
    console.log('\n✔ Everything already up to date.');
  }
}

main().catch((err) => {
  console.error('Failed:', err);
  process.exit(1);
});
