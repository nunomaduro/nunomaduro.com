const fs = require('fs');
const path = require('path');
const { createCanvas } = require('canvas');

const POSTS_DIR = path.join(__dirname, '../content/posts');
const OUTPUT_DIR = path.join(__dirname, '../public/og');

// Ensure output directory exists
if (!fs.existsSync(OUTPUT_DIR)) {
  fs.mkdirSync(OUTPUT_DIR, { recursive: true });
}

// Parse frontmatter from markdown file
function parseFrontmatter(content) {
  const match = content.match(/^---\n([\s\S]*?)\n---/);
  if (!match) return {};
  
  const frontmatter = {};
  match[1].split('\n').forEach(line => {
    const [key, ...valueParts] = line.split(':');
    if (key && valueParts.length) {
      frontmatter[key.trim()] = valueParts.join(':').trim();
    }
  });
  return frontmatter;
}

// Wrap text to fit within width
function wrapText(ctx, text, maxWidth) {
  const words = text.split(' ');
  const lines = [];
  let currentLine = '';

  for (const word of words) {
    const testLine = currentLine ? `${currentLine} ${word}` : word;
    const metrics = ctx.measureText(testLine);
    
    if (metrics.width > maxWidth && currentLine) {
      lines.push(currentLine);
      currentLine = word;
    } else {
      currentLine = testLine;
    }
  }
  
  if (currentLine) {
    lines.push(currentLine);
  }
  
  return lines;
}

// Generate OG image for a post
function generateOGImage(title, slug) {
  const width = 1200;
  const height = 630;
  
  const canvas = createCanvas(width, height);
  const ctx = canvas.getContext('2d');

  // Background - dark
  ctx.fillStyle = '#09090b';
  ctx.fillRect(0, 0, width, height);

  // Large purple gradient accent in corner
  const gradient = ctx.createRadialGradient(0, height, 0, 0, height, 600);
  gradient.addColorStop(0, 'rgba(139, 92, 246, 0.3)');
  gradient.addColorStop(1, 'rgba(139, 92, 246, 0)');
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, width, height);

  // Title text - BIG and bold
  ctx.fillStyle = '#fafafa';
  ctx.font = 'bold 72px sans-serif';
  
  const maxWidth = width - 160;
  const lines = wrapText(ctx, title, maxWidth);
  
  // Position title in upper portion
  const lineHeight = 90;
  let startY = 180;

  for (const line of lines) {
    ctx.fillText(line, 80, startY);
    startY += lineHeight;
  }

  // Author name at bottom left
  ctx.fillStyle = '#a1a1aa';
  ctx.font = 'bold 28px sans-serif';
  ctx.fillText('nunomaduro.com', 80, height - 60);

  // Save image
  const outputPath = path.join(OUTPUT_DIR, `${slug}.png`);
  const buffer = canvas.toBuffer('image/png');
  fs.writeFileSync(outputPath, buffer);
  
  console.log(`Generated: ${outputPath}`);
  return `/og/${slug}.png`;
}

// Process all posts
function processAllPosts() {
  const files = fs.readdirSync(POSTS_DIR).filter(f => f.endsWith('.md'));
  
  for (const file of files) {
    const content = fs.readFileSync(path.join(POSTS_DIR, file), 'utf-8');
    const frontmatter = parseFrontmatter(content);
    
    if (frontmatter.title && frontmatter.slug) {
      generateOGImage(frontmatter.title, frontmatter.slug);
    }
  }
  
  console.log(`\nGenerated ${files.length} OG images in ${OUTPUT_DIR}`);
}

processAllPosts();
