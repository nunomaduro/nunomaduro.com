const fs = require('fs');
const path = require('path');
const { createCanvas, loadImage } = require('canvas');

const PHOTO = path.join(__dirname, 'assets', 'profile.png');
const OUTPUT_DIR = path.join(__dirname, '../public');

const BACKGROUND = '#ffffff';
const RING = '#e4e4e7';
const PHOTO_SCALE = 0.92;

function avatar(photo, diameter) {
  const canvas = createCanvas(diameter, diameter);
  const ctx = canvas.getContext('2d');
  const radius = diameter / 2;
  const width = diameter * PHOTO_SCALE;

  ctx.beginPath();
  ctx.arc(radius, radius, radius, 0, Math.PI * 2);
  ctx.clip();
  ctx.fillStyle = BACKGROUND;
  ctx.fillRect(0, 0, diameter, diameter);
  ctx.imageSmoothingQuality = 'high';
  ctx.drawImage(photo, radius - width / 2, diameter - width, width, width);

  return canvas;
}

function tile(photo, size, diameter) {
  const canvas = createCanvas(size, size);
  const ctx = canvas.getContext('2d');
  const offset = (size - diameter) / 2;

  ctx.fillStyle = BACKGROUND;
  ctx.fillRect(0, 0, size, size);
  ctx.beginPath();
  ctx.arc(size / 2, size / 2, diameter / 2 + Math.max(1, size / 180), 0, Math.PI * 2);
  ctx.fillStyle = RING;
  ctx.fill();
  ctx.drawImage(avatar(photo, diameter), offset, offset);

  return canvas;
}

function ico(pngs) {
  const header = Buffer.alloc(6 + pngs.length * 16);
  header.writeUInt16LE(0, 0);
  header.writeUInt16LE(1, 2);
  header.writeUInt16LE(pngs.length, 4);

  let offset = header.length;

  pngs.forEach(({ size, buffer }, index) => {
    const entry = 6 + index * 16;
    header.writeUInt8(size, entry);
    header.writeUInt8(size, entry + 1);
    header.writeUInt16LE(1, entry + 4);
    header.writeUInt16LE(32, entry + 6);
    header.writeUInt32LE(buffer.length, entry + 8);
    header.writeUInt32LE(offset, entry + 12);
    offset += buffer.length;
  });

  return Buffer.concat([header, ...pngs.map(({ buffer }) => buffer)]);
}

function write(name, buffer) {
  fs.writeFileSync(path.join(OUTPUT_DIR, name), buffer);
  console.log(`generated public/${name}`);
}

async function main() {
  const photo = await loadImage(PHOTO);
  const favicon = (size) => avatar(photo, size).toBuffer('image/png');

  write('favicon-16x16.png', favicon(16));
  write('favicon-32x32.png', favicon(32));
  write('favicon.ico', ico([16, 32, 48].map((size) => ({ size, buffer: favicon(size) }))));
  write('apple-touch-icon.png', tile(photo, 180, 150).toBuffer('image/png'));
  write('android-icon-192x192.png', tile(photo, 192, 144).toBuffer('image/png'));
  write('android-icon-512x512.png', tile(photo, 512, 384).toBuffer('image/png'));
}

main();
