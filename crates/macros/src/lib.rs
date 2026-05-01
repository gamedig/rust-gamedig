// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *   ____                      ____  _         __  __                            *
// *  / ___| __ _ _ __ ___   ___|  _ \(_) __ _  |  \/  | __ _  ___ _ __ ___  ___   *
// * | |  _ / _` | '_ ` _ \ / _ \ | | | |/ _` | | |\/| |/ _` |/ __| '__/ _ \/ __|  *
// * | |_| | (_| | | | | | |  __/ |_| | | (_| | | |  | | (_| | (__| | | (_) \__ \  *
// *  \____|\__,_|_| |_| |_|\___|____/|_|\__, | |_|  |_|\__,_|\___|_|  \___/|___/  *
// *                                     |___/                                     *
// *                           Copyright (c) 2022 - 2026                           *
// *                      GameDig Organization & Contributors                      *
// *                                                                               *
// *                        Licensed under the MIT License                         *
// *         See the LICENSE file in the project root for more information         *
// *                                                                               *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
mod wrap;

#[proc_macro]
pub fn wrap(input: proc_macro::TokenStream) -> proc_macro::TokenStream { wrap::expand(input) }
