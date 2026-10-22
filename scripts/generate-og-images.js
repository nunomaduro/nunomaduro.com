const fs = require('fs');
const path = require('path');
const { createCanvas, loadImage, registerFont } = require('canvas');

const ASSETS_DIR = path.join(__dirname, 'assets');
const POSTS_DIR = path.join(__dirname, '../content/posts');
const OUTPUT_DIR = path.join(__dirname, '../public/og');

const WIDTH = 1200;
const HEIGHT = 630;
const CARD = { x: 32, y: 32, width: WIDTH - 64, height: HEIGHT - 64, radius: 40 };
const PADDING = 80;
const TEXT_WIDTH = 640;

const COLORS = {
  canvas: '#f2f2f4',
  card: '#ffffff',
  text: '#18181b',
  amber: '#f59e0b',
};

const PAGES = [
  { id: 'about', title: 'open source, speaker, and content creator' },
  { id: 'posts', title: 'posts' },
  { id: 'talks', title: 'talks' },
  { id: 'sponsorships', title: 'sponsorships' },
  { id: 'not-found', title: 'page not found' },
];

registerFont(path.join(ASSETS_DIR, 'Geist-Bold.ttf'), { family: 'Geist', weight: 'bold' });

function frontMatter(file) {
  const content = fs.readFileSync(file, 'utf-8');
  const match = content.match(/^---\n([\s\S]*?)\n---/);

  if (!match) {
    throw new Error(`the post ${file} has no front matter`);
  }

  const properties = {};

  for (const line of match[1].split('\n')) {
    const separator = line.indexOf(':');

    if (separator !== -1) {
      properties[line.slice(0, separator).trim()] = line.slice(separator + 1).trim();
    }
  }

  for (const key of ['title']) {
    if (!properties[key]) {
      throw new Error(`the post ${file} has no \`${key}\` in its front matter`);
    }
  }

  return properties;
}

function wrap(ctx, text, maxWidth) {
  const lines = [];
  let line = '';

  for (const word of text.split(' ')) {
    const candidate = line ? `${line} ${word}` : word;

    if (ctx.measureText(candidate).width > maxWidth && line) {
      lines.push(line);
      line = word;
    } else {
      line = candidate;
    }
  }

  lines.push(line);

  return lines;
}

function truncate(ctx, lines, maxLines, maxWidth) {
  if (lines.length <= maxLines) {
    return lines;
  }

  const kept = lines.slice(0, maxLines);
  let last = kept[maxLines - 1];

  while (ctx.measureText(`${last}…`).width > maxWidth) {
    last = last.slice(0, last.lastIndexOf(' '));
  }

  kept[maxLines - 1] = `${last}…`;

  return kept;
}

function fitTitle(ctx, title) {
  for (const size of [88, 76, 64, 56, 48]) {
    ctx.font = `bold ${size}px Geist`;
    const lines = wrap(ctx, title, TEXT_WIDTH);

    if (lines.length <= 3) {
      return { size, lines };
    }
  }

  ctx.font = 'bold 48px Geist';

  return { size: 48, lines: truncate(ctx, wrap(ctx, title, TEXT_WIDTH), 3, TEXT_WIDTH) };
}

function roundedRect(ctx, { x, y, width, height, radius }) {
  ctx.beginPath();
  ctx.roundRect(x, y, width, height, radius);
}

function drawPhoto(ctx, photo) {
  const size = 470;
  const x = CARD.x + CARD.width - size + 30;
  const y = CARD.y + CARD.height - size + 40;

  ctx.save();
  roundedRect(ctx, CARD);
  ctx.clip();
  ctx.drawImage(photo, x, y, size, size);

  const fade = ctx.createLinearGradient(x, 0, x + size * 0.45, 0);
  fade.addColorStop(0, COLORS.card);
  fade.addColorStop(1, 'rgba(255, 255, 255, 0)');
  ctx.fillStyle = fade;
  ctx.fillRect(x, y, size * 0.45, size);
  ctx.restore();
}

function drawWordmark(ctx) {
  ctx.font = 'bold 34px Geist';
  ctx.fillStyle = COLORS.text;
  ctx.fillText('nunomaduro', CARD.x + PADDING, CARD.y + 96);

  const width = ctx.measureText('nunomaduro').width;
  ctx.fillStyle = COLORS.amber;
  ctx.fillText('.', CARD.x + PADDING + width, CARD.y + 96);
}

function render(title, photo, output) {
  const canvas = createCanvas(WIDTH, HEIGHT);
  const ctx = canvas.getContext('2d');

  ctx.fillStyle = COLORS.canvas;
  ctx.fillRect(0, 0, WIDTH, HEIGHT);

  ctx.fillStyle = COLORS.card;
  roundedRect(ctx, CARD);
  ctx.fill();

  drawPhoto(ctx, photo);
  drawWordmark(ctx);

  const { size, lines } = fitTitle(ctx, title);
  const lineHeight = Math.round(size * 1.08);
  const top = CARD.y + 140;
  let y = top + (CARD.y + CARD.height - 60 - top - lines.length * lineHeight) / 2 + size * 0.85;

  ctx.fillStyle = COLORS.text;

  for (const line of lines) {
    ctx.fillText(line, CARD.x + PADDING, y);
    y += lineHeight;
  }

  fs.mkdirSync(path.dirname(output), { recursive: true });
  fs.writeFileSync(output, canvas.toBuffer('image/png'));
  console.log(`generated ${path.relative(process.cwd(), output)}`);
}

async function main() {
  const photo = await loadImage(path.join(ASSETS_DIR, 'profile.png'));

  for (const page of PAGES) {
    render(page.title, photo, path.join(OUTPUT_DIR, `${page.id}.png`));
  }

  const posts = fs.existsSync(POSTS_DIR)
    ? fs.readdirSync(POSTS_DIR).filter((file) => file.endsWith('.md'))
    : [];

  for (const file of posts) {
    const post = frontMatter(path.join(POSTS_DIR, file));
    const slug = path.basename(file, '.md');

    render(post.title, photo, path.join(OUTPUT_DIR, 'posts', `${slug}.png`));
  }
}

main();
