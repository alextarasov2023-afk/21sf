#!/usr/bin/env node
// Rasterizes the icon assets in tools/icons into mixins/example_mod/icons.rs.
// SVG paths are flattened and filled at 4x, then box-filtered down, so the
// in-game atlas holds small anti-aliased white silhouettes (the logo keeps
// its own colours). Run: node tools/gen_icons.mjs
import fs from 'node:fs';
import path from 'node:path';
import zlib from 'node:zlib';

const HERE = path.dirname(new URL(import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'));
const SRC = path.join(HERE, 'icons');
const OUT = path.join(HERE, '..', 'mixins', 'example_mod', 'icons.rs');
const SUBSAMPLE = 4;

const ICONS = [
  { name: 'logo', file: 'P-logo.svg', size: 96, color: null },
  { name: 'sword', file: 'sword_battle_icon_206195.svg', size: 64, color: '#fff' },
  { name: 'running', file: '3289577-fast-run-running_107103.svg', size: 64, color: '#fff' },
  // The photo icon's subpaths are nested cut-outs (circle and mountain), so
  // even-odd gives the frame holes without relying on their winding order.
  { name: 'camera', file: 'iconfinder-picture-image-photo-camera-photography-4593188_122257.svg', size: 64, color: '#fff', rule: 'evenodd' },
  { name: 'person', file: '3289576-individual-man-people-person_107097.svg', size: 64, color: '#fff' },
  { name: 'cog', file: 'cog.svg', size: 64, color: '#fff' },
  { name: 'folder', file: 'folder-open.svg', size: 64, color: '#fff' },
  { name: 'dots', file: 'three-dots-more-indicator_icon-icons.com_72518.png', size: 64, color: '#fff' },
  { name: 'search', file: 'instrument_lupa_on5ekftxwvjk_64.png', size: 64, color: '#fff' },
];

// ---- crc32 / PNG writing ----
const CRC = new Int32Array(256);
for (let n = 0; n < 256; n++) { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; CRC[n] = c; }
function crc32(buf) { let c = -1; for (const b of buf) c = CRC[(c ^ b) & 0xff] ^ (c >>> 8); return (c ^ -1) >>> 0; }
function chunk(type, data) {
  const len = Buffer.alloc(4); len.writeUInt32BE(data.length);
  const body = Buffer.concat([Buffer.from(type, 'ascii'), data]);
  const crc = Buffer.alloc(4); crc.writeUInt32BE(crc32(body));
  return Buffer.concat([len, body, crc]);
}
function encodePng(w, h, rgba) {
  const raw = Buffer.alloc((w * 4 + 1) * h);
  for (let y = 0; y < h; y++) {
    raw[y * (w * 4 + 1)] = 0;
    rgba.copy(raw, y * (w * 4 + 1) + 1, y * w * 4, (y + 1) * w * 4);
  }
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(w, 0); ihdr.writeUInt32BE(h, 4);
  ihdr[8] = 8; ihdr[9] = 6; ihdr[10] = 0; ihdr[11] = 0; ihdr[12] = 0;
  return Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    chunk('IHDR', ihdr),
    chunk('IDAT', zlib.deflateSync(raw, { level: 9 })),
    chunk('IEND', Buffer.alloc(0)),
  ]);
}

// ---- tiny PNG reader (8-bit RGBA/RGB, no interlace) ----
function decodePng(buf) {
  let at = 8, w = 0, h = 0, color = 0, idat = [];
  while (at < buf.length) {
    const len = buf.readUInt32BE(at);
    const type = buf.toString('ascii', at + 4, at + 8);
    const data = buf.subarray(at + 8, at + 8 + len);
    if (type === 'IHDR') { w = data.readUInt32BE(0); h = data.readUInt32BE(4); color = data[9]; }
    if (type === 'IDAT') idat.push(data);
    at += 12 + len;
  }
  const channels = color === 6 ? 4 : color === 2 ? 3 : (() => { throw new Error('unsupported PNG colour type ' + color); })();
  const raw = zlib.inflateSync(Buffer.concat(idat));
  const stride = w * channels;
  const out = Buffer.alloc(w * h * 4);
  let prev = Buffer.alloc(stride);
  for (let y = 0; y < h; y++) {
    const filter = raw[y * (stride + 1)];
    const line = Buffer.from(raw.subarray(y * (stride + 1) + 1, (y + 1) * (stride + 1)));
    for (let i = 0; i < stride; i++) {
      const a = i >= channels ? line[i - channels] : 0;
      const b = prev[i];
      const c = i >= channels ? prev[i - channels] : 0;
      let v = line[i];
      if (filter === 1) v += a;
      else if (filter === 2) v += b;
      else if (filter === 3) v += (a + b) >> 1;
      else if (filter === 4) { const p = a + b - c, pa = Math.abs(p - a), pb = Math.abs(p - b), pc = Math.abs(p - c); v += pa <= pb && pa <= pc ? a : pb <= pc ? b : c; }
      line[i] = v & 0xff;
    }
    for (let x = 0; x < w; x++) {
      const s = x * channels, d = (y * w + x) * 4;
      if (channels === 4) { out[d] = line[s]; out[d + 1] = line[s + 1]; out[d + 2] = line[s + 2]; out[d + 3] = line[s + 3]; }
      else { out[d] = line[s]; out[d + 1] = line[s + 1]; out[d + 2] = line[s + 2]; out[d + 3] = 255; }
    }
    prev = line;
  }
  return { w, h, rgba: out };
}

// ---- SVG parsing (paths and circles only, enough for these assets) ----
function parseSvg(text) {
  const view = /viewBox="([^"]+)"/.exec(text);
  const [vx, vy, vw, vh] = (view ? view[1] : '0 0 100 100').trim().split(/[\s,]+/).map(Number);
  const shapes = [];
  const shapeRe = /<(path|circle)\b([^>]*?)\/?>/g;
  let m;
  while ((m = shapeRe.exec(text))) {
    const attrs = m[2];
    const get = (k) => { const r = new RegExp(k + '="([^"]*)"').exec(attrs); return r ? r[1] : null; };
    const fill = get('fill');
    if (fill === 'none') continue;
    const rule = get('fill-rule') === 'evenodd' ? 'evenodd' : 'nonzero';
    const color = fill || '#000';
    if (m[1] === 'circle') {
      shapes.push({ kind: 'circle', cx: +get('cx'), cy: +get('cy'), r: +get('r'), color, rule });
    } else {
      shapes.push({ kind: 'path', d: get('d') || '', color, rule });
    }
  }
  return { vx, vy, vw, vh, shapes };
}
function tokenize(d) {
  const re = /([MmZzLlHhVvCcSsQqTtAa])|(-?\d*\.?\d+(?:e[-+]?\d+)?)/g;
  const out = [];
  let m;
  while ((m = re.exec(d))) out.push(m[1] || parseFloat(m[2]));
  return out;
}
function flattenPath(d, sx, sy) {
  const t = tokenize(d);
  const subpaths = [];
  let poly = [], x = 0, y = 0, startX = 0, startY = 0, i = 0, cmd = null;
  let lastCtrl = null, lastQ = null;
  const pt = (px, py) => [px * sx, py * sy];
  const push = (px, py) => { poly.push(pt(px, py)); };
  const cubic = (x1, y1, x2, y2, x3, y3) => {
    for (let s = 1; s <= 24; s++) {
      const u = s / 24, v = 1 - u;
      push(v * v * v * x + 3 * v * v * u * x1 + 3 * v * u * u * x2 + u * u * u * x3,
           v * v * v * y + 3 * v * v * u * y1 + 3 * v * u * u * y2 + u * u * u * y3);
    }
  };
  const quad = (x1, y1, x2, y2) => {
    for (let s = 1; s <= 20; s++) {
      const u = s / 20, v = 1 - u;
      push(v * v * x + 2 * v * u * x1 + u * u * x2, v * v * y + 2 * v * u * y1 + u * u * y2);
    }
  };
  const arc = (rx, ry, rot, large, sweep, x2, y2) => {
    rx = Math.abs(rx); ry = Math.abs(ry);
    if (!rx || !ry) { push(x2, y2); x = x2; y = y2; return; }
    const phi = (rot * Math.PI) / 180, cos = Math.cos(phi), sin = Math.sin(phi);
    const dx = (x - x2) / 2, dy = (y - y2) / 2;
    const x1p = cos * dx + sin * dy, y1p = -sin * dx + cos * dy;
    const lambda = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
    if (lambda > 1) { const s = Math.sqrt(lambda); rx *= s; ry *= s; }
    const sign = large === sweep ? -1 : 1;
    const num = rx * rx * ry * ry - rx * rx * y1p * y1p - ry * ry * x1p * x1p;
    const den = rx * rx * y1p * y1p + ry * ry * x1p * x1p;
    const coef = sign * Math.sqrt(Math.max(0, num / den));
    const cxp = (coef * rx * y1p) / ry, cyp = (-coef * ry * x1p) / rx;
    const cx = cos * cxp - sin * cyp + (x + x2) / 2, cy = sin * cxp + cos * cyp + (y + y2) / 2;
    const angle = (ux, uy, vx, vy) => {
      const dot = ux * vx + uy * vy, len = Math.hypot(ux, uy) * Math.hypot(vx, vy);
      const a = Math.acos(Math.max(-1, Math.min(1, dot / len)));
      return ux * vy - uy * vx < 0 ? -a : a;
    };
    const theta = angle(1, 0, (x1p - cxp) / rx, (y1p - cyp) / ry);
    let delta = angle((x1p - cxp) / rx, (y1p - cyp) / ry, (-x1p - cxp) / rx, (-y1p - cyp) / ry);
    if (!sweep && delta > 0) delta -= 2 * Math.PI;
    if (sweep && delta < 0) delta += 2 * Math.PI;
    const steps = Math.max(8, Math.ceil(Math.abs(delta) / 0.1));
    for (let s = 1; s <= steps; s++) {
      const th = theta + (delta * s) / steps;
      push(cx + rx * Math.cos(th) * cos - ry * Math.sin(th) * sin,
           cy + rx * Math.cos(th) * sin + ry * Math.sin(th) * cos);
    }
    x = x2; y = y2;
  };
  const args = (n, rel) => {
    const a = [];
    for (let k = 0; k < n; k++) a.push(t[i++]);
    if (rel) return a;
    return a;
  };
  const num = () => t[i++];
  const flush = () => { if (poly.length > 1) subpaths.push(poly); poly = []; };
  while (i < t.length) {
    if (typeof t[i] === 'string') { cmd = t[i++]; } else if (cmd === null) { i++; continue; }
    const rel = cmd === cmd.toLowerCase();
    const C = cmd.toUpperCase();
    if (C === 'M') {
      flush();
      const ax = num(), ay = num();
      x = rel ? x + ax : ax; y = rel ? y + ay : ay;
      startX = x; startY = y; push(x, y);
      cmd = rel ? 'l' : 'L';
    } else if (C === 'L') { const ax = num(), ay = num(); x = rel ? x + ax : ax; y = rel ? y + ay : ay; push(x, y); }
    else if (C === 'H') { const ax = num(); x = rel ? x + ax : ax; push(x, y); }
    else if (C === 'V') { const ay = num(); y = rel ? y + ay : ay; push(x, y); }
    else if (C === 'C') {
      const [x1, y1, x2, y2, x3, y3] = args(6);
      const ax1 = rel ? x + x1 : x1, ay1 = rel ? y + y1 : y1;
      const ax2 = rel ? x + x2 : x2, ay2 = rel ? y + y2 : y2;
      const ax3 = rel ? x + x3 : x3, ay3 = rel ? y + y3 : y3;
      cubic(ax1, ay1, ax2, ay2, ax3, ay3); lastCtrl = [ax2, ay2]; lastQ = null; x = ax3; y = ay3;
    } else if (C === 'S') {
      const [x2, y2, x3, y3] = args(4);
      const ax2 = rel ? x + x2 : x2, ay2 = rel ? y + y2 : y2;
      const ax3 = rel ? x + x3 : x3, ay3 = rel ? y + y3 : y3;
      const [cx1, cy1] = lastCtrl ? [2 * x - lastCtrl[0], 2 * y - lastCtrl[1]] : [x, y];
      cubic(cx1, cy1, ax2, ay2, ax3, ay3); lastCtrl = [ax2, ay2]; lastQ = null; x = ax3; y = ay3;
    } else if (C === 'Q') {
      const [x1, y1, x2, y2] = args(4);
      const ax1 = rel ? x + x1 : x1, ay1 = rel ? y + y1 : y1;
      const ax2 = rel ? x + x2 : x2, ay2 = rel ? y + y2 : y2;
      quad(ax1, ay1, ax2, ay2); lastQ = [ax1, ay1]; lastCtrl = null; x = ax2; y = ay2;
    } else if (C === 'T') {
      const [x2, y2] = args(2);
      const ax2 = rel ? x + x2 : x2, ay2 = rel ? y + y2 : y2;
      const [qx, qy] = lastQ ? [2 * x - lastQ[0], 2 * y - lastQ[1]] : [x, y];
      quad(qx, qy, ax2, ay2); lastQ = [qx, qy]; lastCtrl = null; x = ax2; y = ay2;
    } else if (C === 'A') {
      const [rx, ry, rot, large, sweep, x2, y2] = args(7);
      arc(rx, ry, rot, large, sweep, rel ? x + x2 : x2, rel ? y + y2 : y2);
      lastCtrl = lastQ = null;
    } else if (C === 'Z') {
      if (poly.length) { push(startX, startY); flush(); }
      x = startX; y = startY; lastCtrl = lastQ = null;
    } else { i++; }
  }
  flush();
  return subpaths;
}

// ---- rasterization ----
function rasterize(parsed, size, color) {
  const W = size * SUBSAMPLE, H = size * SUBSAMPLE;
  const sx = W / parsed.vw, sy = H / parsed.vh;
  const acc = new Float32Array(W * H * 4); // straight alpha over
  for (const shape of parsed.shapes) {
    let polys;
    if (shape.kind === 'circle') {
      polys = [[]];
      const n = 160;
      for (let k = 0; k < n; k++) {
        const a = (k / n) * Math.PI * 2;
        polys[0].push([(shape.cx + Math.cos(a) * shape.r - parsed.vx) * sx, (shape.cy + Math.sin(a) * shape.r - parsed.vy) * sy]);
      }
    } else {
      polys = flattenPath(shape.d, sx, sy);
    }
    const tint = color || shape.color;
    const [r, g, b] = tint === '#fff' || tint === '#FFFFFF' ? [255, 255, 255] : hex(tint);
    // All subpaths of one shape fill together: even-odd and non-zero count
    // winding across them, which is how the camera's cut-outs work.
    const cover = new Float32Array(W * H);
    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
    const edges = [];
    for (const poly of polys) {
      for (const [px, py] of poly) { minX = Math.min(minX, px); minY = Math.min(minY, py); maxX = Math.max(maxX, px); maxY = Math.max(maxY, py); }
      for (let k = 0; k < poly.length; k++) {
        const [ax, ay] = poly[k], [bx, by] = poly[(k + 1) % poly.length];
        if (ay === by) continue;
        edges.push(ay < by ? [ax, ay, bx, by, 1] : [bx, by, ax, ay, -1]);
      }
    }
    const x0 = Math.max(0, Math.floor(minX)), x1 = Math.min(W - 1, Math.ceil(maxX));
    const y0 = Math.max(0, Math.floor(minY)), y1 = Math.min(H - 1, Math.ceil(maxY));
    for (let py = y0; py <= y1; py++) {
      for (let px = x0; px <= x1; px++) {
        let hits = 0;
        for (let sy2 = 0; sy2 < 2; sy2++) {
          for (let sx2 = 0; sx2 < 2; sx2++) {
            const x = px + (sx2 + 0.5) / 2, y = py + (sy2 + 0.5) / 2;
            let wind = 0;
            for (const [ax, ay, bx, by, dir] of edges) {
              if (ay > y === by > y) continue;
              const xi = ax + ((y - ay) / (by - ay)) * (bx - ax);
              if (xi > x) wind += dir;
            }
            const inside = shape.rule === 'evenodd' ? Math.abs(wind % 2) === 1 : wind !== 0;
            if (inside) hits++;
          }
        }
        const c = hits / 4;
        if (c > 0) cover[py * W + px] = c;
      }
    }
    for (let p = 0; p < W * H; p++) {
      const a = cover[p];
      if (!a) continue;
      const d = p * 4;
      const da = acc[d + 3], oa = a + da * (1 - a);
      acc[d] = (r * a + acc[d] * da * (1 - a)) / oa;
      acc[d + 1] = (g * a + acc[d + 1] * da * (1 - a)) / oa;
      acc[d + 2] = (b * a + acc[d + 2] * da * (1 - a)) / oa;
      acc[d + 3] = oa;
    }
  }
  // Box filter down to size.
  const out = Buffer.alloc(size * size * 4);
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      let r = 0, g = 0, b = 0, a = 0;
      for (let sy2 = 0; sy2 < SUBSAMPLE; sy2++) {
        for (let sx2 = 0; sx2 < SUBSAMPLE; sx2++) {
          const d = ((y * SUBSAMPLE + sy2) * W + x * SUBSAMPLE + sx2) * 4;
          const aa = acc[d + 3];
          r += acc[d] * aa; g += acc[d + 1] * aa; b += acc[d + 2] * aa; a += aa;
        }
      }
      const n = SUBSAMPLE * SUBSAMPLE, d = (y * size + x) * 4;
      out[d] = a > 0 ? Math.round(r / a) : 255;
      out[d + 1] = a > 0 ? Math.round(g / a) : 255;
      out[d + 2] = a > 0 ? Math.round(b / a) : 255;
      out[d + 3] = Math.round((a / n) * 255);
    }
  }
  return out;
}
function hex(c) {
  const v = parseInt(c.replace('#', ''), 16);
  return [(v >> 16) & 255, (v >> 8) & 255, v & 255];
}

function fromPng(file, size) {
  const { w, h, rgba } = decodePng(fs.readFileSync(file));
  const out = Buffer.alloc(size * size * 4);
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      let a = 0, n = 0;
      const x0 = Math.floor((x * w) / size), x1 = Math.max(x0 + 1, Math.floor(((x + 1) * w) / size));
      const y0 = Math.floor((y * h) / size), y1 = Math.max(y0 + 1, Math.floor(((y + 1) * h) / size));
      for (let sy = y0; sy < Math.min(y1, h); sy++) {
        for (let sx = x0; sx < Math.min(x1, w); sx++) {
          const d = (sy * w + sx) * 4;
          const alpha = rgba[d + 3] < 250 ? rgba[d + 3] : 255 - Math.round(0.299 * rgba[d] + 0.587 * rgba[d + 1] + 0.114 * rgba[d + 2]);
          a += alpha; n++;
        }
      }
      const d = (y * size + x) * 4;
      out[d] = 255; out[d + 1] = 255; out[d + 2] = 255;
      out[d + 3] = n ? Math.round(a / n) : 0;
    }
  }
  return out;
}

const entries = [];
for (const icon of ICONS) {
  const file = path.join(SRC, icon.file);
  const texture = icon.file.endsWith('.svg')
    ? (() => {
        const parsed = parseSvg(fs.readFileSync(file, 'utf8'));
        if (icon.rule) for (const shape of parsed.shapes) shape.rule = icon.rule;
        return rasterize(parsed, icon.size, icon.color);
      })()
    : fromPng(file, icon.size);
  fs.writeFileSync(path.join(SRC, icon.name + '.preview.png'), encodePng(icon.size, icon.size, texture));
  entries.push([icon.name, encodePng(icon.size, icon.size, texture).toString('base64'), icon.size]);
}

const source = [
  '// Generated by tools/gen_icons.mjs from the assets in tools/icons/ (SVG and',
  '// PNG originals). Binary PNGs keep the repository free of raw pixel arrays;',
  '// prism_ui decodes them once into the glyph atlas at run time.',
  '',
  'pub const ICONS: [(&str, u32, &str); ' + entries.length + '] = [',
  ...entries.map(([name, b64, size]) => `    ("${name}", ${size}, "${b64}"),`),
  '];',
  '',
  'pub fn decode_base64(text: &str) -> Vec<u8> {',
  '    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";',
  '    let mut out = Vec::with_capacity(text.len() / 4 * 3);',
  '    let mut bits = 0u32;',
  '    let mut count = 0u32;',
  '    for byte in text.bytes() {',
  '        let value = match byte {',
  '            b\'=\' => break,',
  '            _ => match TABLE.iter().position(|&c| c == byte) {',
  '                Some(index) => index as u32,',
  '                None => continue,',
  '            },',
  '        };',
  '        bits = (bits << 6) | value;',
  '        count += 6;',
  '        if count >= 8 {',
  '            count -= 8;',
  '            out.push((bits >> count) as u8);',
  '        }',
  '    }',
  '    out',
  '}',
  '',
].join('\n');
fs.writeFileSync(OUT, source);
console.log('wrote', OUT, fs.statSync(OUT).size, 'bytes');
