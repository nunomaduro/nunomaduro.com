#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const { createCanvas, loadImage } = require('canvas');

const CHANNEL_ID = 'UCO_hYZF2gb_CyG5sA7ArlGg';
const SUPPORT_HTML = path.join(__dirname, '../presentation/templates/support.html');
const PUBLIC_DIR = path.join(__dirname, '../public');
const TEMPLATES_DIR = path.join(__dirname, '../presentation/templates');
const TOP_VIDEOS = 10;
const WINDOW_DAYS = 365;
const THUMBNAIL_WIDTH = 640;
const THUMBNAIL_HEIGHT = 360;

const HEADERS = {
  'User-Agent':
    'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 ' +
    '(KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
  'Accept-Language': 'en-US,en;q=0.9',
  Cookie: 'CONSENT=YES+1',
};

const dryRun = process.argv.includes('--dry');

async function fetchText(url) {
  const response = await fetch(url, { headers: HEADERS, redirect: 'follow' });
  if (!response.ok) throw new Error(`HTTP ${response.status} for ${url}`);
  return response.text();
}

function decodeEntities(text) {
  return text
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&amp;/g, '&');
}

function escapeHtml(text) {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

async function recentUploadIds() {
  const feed = await fetchText(`https://www.youtube.com/feeds/videos.xml?channel_id=${CHANNEL_ID}`);
  return [...feed.matchAll(/<link rel="alternate" href="https:\/\/www\.youtube\.com\/watch\?v=([\w-]{11})"/g)].map(
    (match) => match[1],
  );
}

async function readVideo(id) {
  const html = await fetchText(`https://www.youtube.com/watch?v=${id}`);
  const views = html.match(/"viewCount":"(\d+)"/);
  const publishedAt = html.match(/"publishDate":"([^"]+)"/);
  const title = html.match(/<meta name="title" content="([^"]*)"/);
  if (!views || !publishedAt || !title) {
    throw new Error(`the page of the video ${id} has no view count, publish date or title`);
  }

  return {
    id,
    views: parseInt(views[1], 10),
    publishedAt: new Date(publishedAt[1]),
    title: decodeEntities(title[1]),
    isLivestream: /"isLiveContent":true/.test(html),
  };
}

function topVideosList(html) {
  const heading = html.indexOf('Top 10 most-watched videos</h3>');
  if (heading === -1) throw new Error('support.html has no "Top 10 most-watched videos" heading');
  const listStart = html.indexOf('\n', html.indexOf('<ol role="list"', heading)) + 1;
  const listEnd = html.indexOf('            </ol>', listStart);

  return { start: listStart, end: listEnd };
}

function shownTitles(listHtml) {
  const titles = new Map();
  for (const item of listHtml.matchAll(/watch\?v=([\w-]{11})"[\s\S]*?group-hover:text-amber-700">([^<]*)<\/span>/g)) {
    titles.set(item[1], item[2]);
  }

  return titles;
}

function renderItem(video, title) {
  const label = video.isLivestream
    ? '\n                            <span class="absolute bottom-2 left-2 rounded-md bg-zinc-900/85 px-1.5 py-0.5 text-xs font-semibold text-white">Livestream replay</span>'
    : '';

  return `                <li>
                    <a href="https://www.youtube.com/watch?v=${video.id}" class="group flex flex-col">
                        <span class="relative block">
                            <img src="/top-video.${video.id}.jpg" alt="Thumbnail of ${title}" width="${THUMBNAIL_WIDTH}" height="${THUMBNAIL_HEIGHT}" loading="lazy" class="aspect-video w-full rounded-2xl bg-canvas object-cover">${label}
                        </span>
                        <span class="mt-2.5 line-clamp-2 text-[15px] leading-snug font-semibold text-zinc-900 group-hover:text-amber-700">${title}</span>
                        <span class="mt-1 text-[13px] text-zinc-600 tabular-nums">${video.views.toLocaleString('en-US')} views</span>
                    </a>
                </li>
`;
}

async function saveThumbnail(id) {
  const file = path.join(PUBLIC_DIR, `top-video.${id}.jpg`);
  if (fs.existsSync(file)) return false;

  let image;
  try {
    image = await loadImage(`https://i.ytimg.com/vi/${id}/maxresdefault.jpg`);
  } catch {
    image = await loadImage(`https://i.ytimg.com/vi/${id}/hqdefault.jpg`);
  }

  const canvas = createCanvas(THUMBNAIL_WIDTH, THUMBNAIL_HEIGHT);
  const scale = Math.max(THUMBNAIL_WIDTH / image.width, THUMBNAIL_HEIGHT / image.height);
  const width = image.width * scale;
  const height = image.height * scale;
  canvas
    .getContext('2d')
    .drawImage(image, (THUMBNAIL_WIDTH - width) / 2, (THUMBNAIL_HEIGHT - height) / 2, width, height);
  fs.writeFileSync(file, canvas.toBuffer('image/jpeg', { quality: 0.85 }));

  return true;
}

function referencedThumbnails() {
  const referenced = new Set();
  for (const file of fs.readdirSync(TEMPLATES_DIR, { recursive: true })) {
    if (!file.endsWith('.html')) continue;
    const html = fs.readFileSync(path.join(TEMPLATES_DIR, file), 'utf8');
    for (const match of html.matchAll(/top-video\.([\w-]{11})\.jpg/g)) referenced.add(match[1]);
  }

  return referenced;
}

function removeUnusedThumbnails() {
  const referenced = referencedThumbnails();
  for (const file of fs.readdirSync(PUBLIC_DIR)) {
    const match = file.match(/^top-video\.([\w-]{11})\.jpg$/);
    if (match && !referenced.has(match[1])) {
      fs.rmSync(path.join(PUBLIC_DIR, file));
      console.log(`−  removed public/${file}`);
    }
  }
}

function updateOtherViewCounts(html, videos) {
  return html.replace(
    /(href="https:\/\/www\.youtube\.com\/watch\?v=([\w-]{11})"[^>]*>[^<]*<\/a>(?:<\/td><td[^>]*>|<span[^>]*>))([\d,]+)(<)/g,
    (row, before, id, views, after) => {
      const video = videos.get(id);
      return video ? `${before}${video.views.toLocaleString('en-US')}${after}` : row;
    },
  );
}

function linkedVideoIds(html) {
  return [...html.matchAll(/href="https:\/\/www\.youtube\.com\/watch\?v=([\w-]{11})"[^>]*>[^<]*<\/a>(?:<\/td><td[^>]*>|<span[^>]*>)[\d,]+</g)].map(
    (match) => match[1],
  );
}

async function main() {
  const original = fs.readFileSync(SUPPORT_HTML, 'utf8');
  const list = topVideosList(original);
  const listHtml = original.slice(list.start, list.end);
  const titles = shownTitles(listHtml);

  const ids = new Set([...titles.keys(), ...(await recentUploadIds()), ...linkedVideoIds(original)]);
  const videos = new Map();
  for (const id of ids) videos.set(id, await readVideo(id));

  const since = Date.now() - WINDOW_DAYS * 24 * 60 * 60 * 1000;
  const top = [...videos.values()]
    .filter((video) => video.publishedAt.getTime() >= since)
    .sort((a, b) => b.views - a.views)
    .slice(0, TOP_VIDEOS);

  top.forEach((video, index) => {
    const state = titles.has(video.id) ? '=' : '+';
    console.log(`${state}  ${String(index + 1).padStart(2)}. ${video.views.toLocaleString('en-US').padStart(7)}  ${video.title}`);
  });
  for (const id of titles.keys()) {
    if (!top.some((video) => video.id === id)) console.log(`−      ${videos.get(id).title}`);
  }

  const items = top.map((video) => renderItem(video, titles.get(video.id) ?? escapeHtml(video.title))).join('');
  const html = updateOtherViewCounts(original.slice(0, list.start) + items + original.slice(list.end), videos);

  if (dryRun) {
    console.log('\n(dry run — no files written)');
    return;
  }

  for (const video of top) {
    if (await saveThumbnail(video.id)) console.log(`+  saved public/top-video.${video.id}.jpg`);
  }

  if (html === original) {
    console.log('\n✔ Everything already up to date.');
    return;
  }

  fs.writeFileSync(SUPPORT_HTML, html);
  removeUnusedThumbnails();
  console.log(`\n✔ Updated ${path.relative(process.cwd(), SUPPORT_HTML)}`);
}

main().catch((error) => {
  console.error('Failed:', error.message);
  process.exit(1);
});
