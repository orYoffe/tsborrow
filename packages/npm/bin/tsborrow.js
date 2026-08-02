#!/usr/bin/env node

import { createHash } from "node:crypto";
import { createWriteStream, existsSync, readFileSync, renameSync, rmSync } from "node:fs";
import { chmod, mkdir, readFile } from "node:fs/promises";
import { get } from "node:https";
import { homedir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const packageRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const { version } = JSON.parse(await readFile(join(packageRoot, "package.json"), "utf8"));
const repository = "https://github.com/orYoffe/tsborrow";

const target = {
  "linux-x64": "tsborrow-linux-x64",
  "darwin-x64": "tsborrow-darwin-x64",
  "darwin-arm64": "tsborrow-darwin-arm64",
  "win32-x64": "tsborrow-win32-x64.exe"
}[`${process.platform}-${process.arch}`];

if (!target && !process.env.TSBORROW_BINARY) {
  console.error(`tsborrow: unsupported platform ${process.platform}-${process.arch}`);
  process.exit(2);
}

const cacheRoot = process.env.TSBORROW_CACHE_DIR || join(homedir(), ".cache", "tsborrow");
const binary = process.env.TSBORROW_BINARY || join(cacheRoot, version, target);

if (!process.env.TSBORROW_BINARY && !existsSync(binary)) {
  await install(binary, target, version);
}

const child = spawnSync(binary, process.argv.slice(2), { stdio: "inherit" });
if (child.error) {
  console.error(`tsborrow: ${child.error.message}`);
  process.exit(2);
}
process.exit(child.status ?? 2);

async function install(destination, asset, releaseVersion) {
  await mkdir(dirname(destination), { recursive: true });
  const base = `${repository}/releases/download/v${releaseVersion}`;
  const temporary = `${destination}.${process.pid}.tmp`;
  try {
    await download(`${base}/${asset}`, temporary);
    const checksumFile = `${temporary}.sha256`;
    await download(`${base}/${asset}.sha256`, checksumFile);
    const expected = readFileSync(checksumFile, "utf8").trim().split(/\s+/)[0].toLowerCase();
    const actual = createHash("sha256").update(readFileSync(temporary)).digest("hex");
    if (actual !== expected) {
      throw new Error(`checksum mismatch for ${asset}`);
    }
    renameSync(temporary, destination);
    if (process.platform !== "win32") await chmod(destination, 0o755);
    rmSync(checksumFile, { force: true });
  } catch (error) {
    rmSync(temporary, { force: true });
    rmSync(`${temporary}.sha256`, { force: true });
    throw error;
  }
}

function download(url, destination, redirects = 0) {
  if (redirects > 5) return Promise.reject(new Error("too many download redirects"));
  return new Promise((resolve, reject) => {
    get(url, { headers: { "user-agent": `tsborrow-npm/${version}` } }, (response) => {
      if (response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
        response.resume();
        download(response.headers.location, destination, redirects + 1).then(resolve, reject);
        return;
      }
      if (response.statusCode !== 200) {
        response.resume();
        reject(new Error(`download failed with HTTP ${response.statusCode}`));
        return;
      }
      const file = createWriteStream(destination, { mode: 0o755 });
      response.pipe(file);
      file.on("finish", () => file.close(resolve));
      file.on("error", reject);
    }).on("error", reject);
  });
}
