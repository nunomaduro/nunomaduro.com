#!/usr/bin/env node
'use strict';

/**
 * Updates the social media follower counts shown on the About page
 * (presentation/templates/about.html) by fetching live numbers from each
 * platform.
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
 * X / Twitter has no free way to read follower counts, so it is best-effort:
 * set `X_FOLLOWERS` in the environment to override it manually. GitHub's API is
 * rate-limited when anonymous; set `GITHUB_TOKEN` to raise the limit.
 */

const fs = require('fs');
const path = require('path');

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
  const html = await getText(`https://www.youtube.com/@${handle}`, {
    Cookie: 'CONSENT=YES+1',
  });
  const m = html.match(/([\d.,]+[KM]?)\s+subscribers/i);
  return m ? parseAbbrev(m[1]) : null;
}

async function tiktokFollowers(handle) {
  const html = await getText(`https://www.tiktok.com/@${handle}`);
  const m = html.match(/"followerCount":(\d+)/);
  return m ? parseInt(m[1], 10) : null;
}

async function githubFollowers(user) {
  const headers = { Accept: 'application/vnd.github+json' };
  if (process.env.GITHUB_TOKEN) headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
  const json = JSON.parse(await getText(`https://api.github.com/users/${user}`, headers));
  return typeof json.followers === 'number' ? json.followers : null;
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
    href: 'https://www.youtube.com/@nunomaduro2?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro2'),
  },
  {
    label: 'youtube·3',
    href: 'https://www.youtube.com/@nunomaduro3?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro3'),
  },
  {
    label: 'youtube·4',
    href: 'https://www.youtube.com/@nunomaduro4?sub_confirmation=1',
    fetch: () => youtubeSubs('nunomaduro4'),
  },
  {
    label: 'tiktok',
    href: 'https://www.tiktok.com/@enunomaduro',
    fetch: () => tiktokFollowers('enunomaduro'),
  },
  {
    label: 'tiktok·clips',
    href: 'https://www.tiktok.com/@nunomaduroclips',
    fetch: () => tiktokFollowers('nunomaduroclips'),
  },
  {
    label: 'instagram',
    href: 'https://instagram.com/enunomaduro',
    async fetch() {
      const html = await getText('https://www.instagram.com/enunomaduro/');
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

async function main() {
  const dryRun = process.argv.includes('--dry');
  let html = fs.readFileSync(ABOUT_HTML, 'utf8');
  let changed = 0;

  const results = await Promise.all(
    ACCOUNTS.map(async (acc) => {
      try {
        return { acc, count: await acc.fetch() };
      } catch (err) {
        return { acc, count: null, error: err.message };
      }
    }),
  );

  for (const { acc, count, error } of results) {
    const pad = acc.label.padEnd(12);

    if (count == null) {
      const reason = error ? `fetch failed: ${error}` : 'no public count';
      console.warn(`⚠  ${pad} skipped (${reason}) — left as-is`);
      continue;
    }

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

  if (dryRun) {
    console.log('\n(dry run — no files written)');
    return;
  }

  if (changed > 0 || reordered) {
    fs.writeFileSync(ABOUT_HTML, html);
    const what = [changed > 0 && `${changed} count(s)`, reordered && 'order'].filter(Boolean).join(' + ');
    console.log(`\n✔ Updated ${what} in ${path.relative(process.cwd(), ABOUT_HTML)}`);
    console.log('  Run `npm run build` to regenerate the site.');
  } else {
    console.log('\n✔ Everything already up to date.');
  }
}

main().catch((err) => {
  console.error('Failed:', err);
  process.exit(1);
});
