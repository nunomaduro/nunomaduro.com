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
 * Some platforms have no free way to read follower counts (X / Twitter,
 * Threads) or sit behind Cloudflare (Kick, which 403s server-side requests).
 * For any account it can't fetch, the script prompts for the number on the
 * terminal — type a value (e.g. 66k, 1.2k, 1500) or press enter to skip and
 * leave the link as-is. The matching `X_FOLLOWERS` / `THREADS_FOLLOWERS` /
 * `KICK_FOLLOWERS` env vars still work as a non-interactive fallback (CI,
 * piped input). GitHub's API is rate-limited when anonymous; set `GITHUB_TOKEN`
 * to raise the limit.
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
  const m = html.match(/([\d.,]+[KM]?)\s+subscribers/i);
  return m ? parseAbbrev(m[1]) : null;
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
  // Kick's public channel API often sits behind Cloudflare and 403s server-side
  // requests; fall back to a manual `KICK_FOLLOWERS` override when it does.
  try {
    const json = await getText(`https://kick.com/api/v2/channels/${handle}`, {
      Accept: 'application/json',
    });
    const m = json.match(/"followers_?count":(\d+)/i);
    if (m) return parseInt(m[1], 10);
  } catch {
    // fall through to the manual override below
  }
  return process.env.KICK_FOLLOWERS ? parseAbbrev(process.env.KICK_FOLLOWERS) : null;
}

// Each account: the (unique) href that locates it in the HTML + a fetcher.
const ACCOUNTS = [
  {
    label: 'x',
    href: 'https://twitter.com/enunomaduro',
    // No free API for X follower counts — allow a manual override.
    fetch: async () => (process.env.X_FOLLOWERS ? parseAbbrev(process.env.X_FOLLOWERS) : null),
  },
  {
    label: 'youtube',
    href: 'https://youtube.com/nunomaduro?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro'),
  },
  {
    label: 'youtube·2',
    href: 'https://youtube.com/@nunomaduro2?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro2'),
  },
  {
    label: 'youtube·3',
    href: 'https://youtube.com/@nunomaduro3?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro3'),
  },
  {
    label: 'youtube·4',
    href: 'https://youtube.com/@nunomaduro4?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro4'),
  },
  {
    label: 'tiktok',
    href: 'https://tiktok.com/@enunomaduro',
    fetch: () => tiktokFollowers('enunomaduro'),
  },
  {
    label: 'tiktok·clips',
    href: 'https://tiktok.com/@nunomaduroclips',
    fetch: () => tiktokFollowers('nunomaduroclips'),
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
    fetch: () => kickFollowers('nunomaduro'),
  },
  {
    label: 'mastodon',
    href: 'https://mastodon.social/@nunomaduro',
    fetch: () => mastodonFollowers('mastodon.social', 'nunomaduro'),
  },
  {
    label: 'threads',
    href: 'https://threads.com/@enunomaduro',
    // No free API for Threads follower counts — allow a manual override.
    fetch: async () => (process.env.THREADS_FOLLOWERS ? parseAbbrev(process.env.THREADS_FOLLOWERS) : null),
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
