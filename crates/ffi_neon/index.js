import { createRequire } from "module";

const r = createRequire(import.meta.url);
const native = r("./target/index.node");

/**
 * Query a game server.
 *
 * @param {string} gameId
 * @param {string} addr
 * @param {object | undefined} timeout
 * @returns {Promise<object>}
 */
export default function query(gameId, addr, timeout) {
  return native.query(gameId, addr, timeout);
}
