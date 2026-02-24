// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *    ____                      ____  _         _____ _____ ___    *
// *   / ___| __ _ _ __ ___   ___|  _ \(_) __ _  |  ___|  ___|_ _|   *
// *  | |  _ / _` | '_ ` _ \ / _ \ | | | |/ _` | | |_  | |_   | |    *
// *  | |_| | (_| | | | | | |  __/ |_| | | (_| | |  _| |  _|  | |    *
// *   \____|\__,_|_| |_| |_|\___|____/|_|\__, | |_|   |_|   |___|   *
// *                                      |___/                      *
// *                 Copyright (c) 2022 - 2026                       *
// *            GameDig Organization & Contributors                  *
// *                                                                 *
// *               Licensed under the MIT License                    *
// *  See the LICENSE file in the project root for more information  *
// *                                                                 *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
mod args;
mod player;
mod server;

use {args::Query, neon::prelude::*, server::Server};

fn query(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let query = Query::parse(&mut cx)?;

    let promise = cx
        .task(move || {
            gamedig::dict::Dict::query_by_game_id(&query.game_id, &query.addr, query.timeout)
                .map_err(|e| e.to_string())
        })
        .promise(|mut cx, result| {
            match result {
                Ok(server) => Server::to_js(&mut cx, &server),
                Err(msg) => cx.throw_error(msg),
            }
        });

    Ok(promise)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("query", query)?;
    Ok(())
}
