"use strict";

//! Calculate the percentage of games from node that we support
// Expects node-gamedig checkout out in git root /node-gamedig
// Expects the generic example to output a list of game IDs when no arguments are provided

import process from "node:process";
import { closeSync, openSync, writeSync } from "node:fs";
import { spawnSync } from "node:child_process";

function setOutput(key, value) {
    const file = openSync(process.env.GITHUB_OUTPUT, "a");
    writeSync(file, `${key}=${value}\n`);
    closeSync(file);
}

// Get node IDs

import { games } from "../../../node-gamedig/lib/games.js";

const node_ids = new Set(Object.keys(games));
const node_total = node_ids.size;

// Get rust IDs

const command = spawnSync("cargo", ["run", "-p", "gamedig", "--example", "generic"]);

if (command.status !== 0) {
    console.error(command.stderr.toString("utf8"));
    process.exit(1);
}

const rust_ids_pretty = command.stdout.toString("utf8");
const rust_ids = new Set(rust_ids_pretty.split("\n").map(line => line.split("\t")[0]).filter(id => id.length > 0));

// Detect missing node IDs

for (const id of rust_ids) {
    if (node_ids.delete(id)) {
        rust_ids.delete(id);
    }
}

console.log("Node remains", node_ids);
console.log("Rust remains", rust_ids);

const percent = 1 - (node_ids.size / node_total);

// Output percent to 2 decimal places
setOutput("percent", Math.round(percent*10000)/100);