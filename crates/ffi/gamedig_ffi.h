/*
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 *    ____                      ____  _         _____ _____ ___    *
 *   / ___| __ _ _ __ ___   ___|  _ \(_) __ _  |  ___|  ___|_ _|   *
 *  | |  _ / _` | '_ ` _ \ / _ \ | | | |/ _` | | |_  | |_   | |    *
 *  | |_| | (_| | | | | | |  __/ |_| | | (_| | |  _| |  _|  | |    *
 *   \____|\__,_|_| |_| |_|\___|____/|_|\__, | |_|   |_|   |___|   *
 *                                      |___/                      *
 *                 Copyright (c) 2022 - 2026                       *
 *            GameDig Organization & Contributors                  *
 *                                                                 *
 *               Licensed under the MIT License                    *
 *  See the LICENSE file in the project root for more information  *
 *                                                                 *
 *     This header requires a C99 or later compatible compiler     *
 *                                                                 *
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 */

#ifndef GAMEDIG_FFI_H
#define GAMEDIG_FFI_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C"
{
#endif /* __cplusplus */

    /**
     * @brief TCP timeout configuration.
     *
     * All values are expressed in milliseconds. A value of `0` causes the
     * library to use its default timeout.
     */
    typedef struct gamedig_tcp_timeout_t
    {
        /** TCP connection timeout. */
        uint32_t connect_ms;

        /** TCP read timeout. */
        uint32_t read_ms;

        /** TCP write timeout. */
        uint32_t write_ms;
    } gamedig_tcp_timeout_t;

    /**
     * @brief UDP timeout configuration.
     *
     * All values are expressed in milliseconds. A value of `0` causes the
     * library to use its default timeout.
     */
    typedef struct gamedig_udp_timeout_t
    {
        /** UDP receive timeout. */
        uint32_t read_ms;

        /** UDP send timeout. */
        uint32_t write_ms;
    } gamedig_udp_timeout_t;

    /**
     * @brief HTTP timeout configuration.
     *
     * Value is expressed in milliseconds. A value of `0` causes the
     * library to use its default timeout.
     */
    typedef struct gamedig_http_timeout_t
    {
        /** Global HTTP request timeout. */
        uint32_t global_ms;
    } gamedig_http_timeout_t;

    /**
     * @brief Combined timeout configuration for all supported transports.
     *
     * Passing `NULL` where accepted causes all default timeouts to be used.
     */
    typedef struct gamedig_timeout_config_t
    {
        /** TCP timeout configuration. */
        gamedig_tcp_timeout_t tcp;

        /** UDP timeout configuration. */
        gamedig_udp_timeout_t udp;

        /** HTTP timeout configuration. */
        gamedig_http_timeout_t http;
    } gamedig_timeout_config_t;

    /**
     * @brief Tri state boolean value.
     *
     * Used when a protocol may not expose a particular boolean field.
     */
    typedef enum gamedig_opt_bool_t
    {
        GAMEDIG_BOOL_FALSE = 0,  /**< Explicitly false. */
        GAMEDIG_BOOL_TRUE = 1,   /**< Explicitly true. */
        GAMEDIG_BOOL_UNKNOWN = 2 /**< Not exposed by the protocol. */
    } gamedig_opt_bool_t;

    /**
     * @brief A player returned from a server query.
     *
     * Player objects are owned by the `gamedig_server_t` instance that
     * returned them.
     */
    typedef struct gamedig_player_t
    {
        /**
         * Player index within the returned list.
         * Not guaranteed to be stable between queries.
         */
        uint16_t id;

        /**
         * Player display name encoded as UTF-8 and NUL-terminated.
         * `NULL` if not provided by the protocol.
         */
        const char *name;
    } gamedig_player_t;

    /**
     * @brief List of connected players.
     *
     * If no players are connected, `data` is `NULL` and `len` is `0`.
     */
    typedef struct gamedig_player_list_t
    {
        /** Pointer to an array of players, or `NULL` if no players are connected. */
        const gamedig_player_t *data;

        /** Number of elements in the array. */
        size_t len;
    } gamedig_player_list_t;

    /**
     * @brief Server query result.
     *
     * All strings are UTF-8 and NUL-terminated. Optional strings are
     * represented as `NULL` pointers.
     *
     * Memory for all pointers contained in this structure is owned by
     * the library and must be released with `gamedig_ffi_free_server()`.
     */
    typedef struct gamedig_server_t
    {
        /** Server name. Guaranteed non-`NULL` on success. */
        const char *name;

        /** Server description, MOTD if available. Else `NULL`. */
        const char *description;

        /** Current map name if available. Else `NULL`. */
        const char *map;

        /** Current game mode if available. Else `NULL`. */
        const char *mode;

        /** Server version string if available. Else `NULL`. */
        const char *version;

        /** Anti cheat enabled status. */
        gamedig_opt_bool_t anti_cheat;

        /** Password requirement status. */
        gamedig_opt_bool_t has_password;

        /** Maximum reported player capacity. */
        uint16_t max_players;

        /** Current number of connected players. */
        uint16_t current_players;

        /** List of connected players if available. Else `NULL`. */
        const gamedig_player_list_t *players;
    } gamedig_server_t;

    /**
     * @brief Error message returned by the library.
     *
     * Dynamically allocated UTF-8 NUL terminated string.
     * Must be released with `gamedig_ffi_free_error()`.
     */
    typedef char *gamedig_error_t;

    /**
     * @brief Queries a server by game identifier.
     *
     * On success, returns `0` and initializes `out_srv`.
     * If `out_error` is not `NULL`, `*out_error` is set to `NULL`.
     *
     * On failure, returns non zero.
     * If `out_error` is not `NULL`, `*out_error` receives an allocated error message.
     *
     * @param game_id   UTF8 NUL terminated game identifier. Must not be `NULL`.
     * @param addr      UTF8 NUL terminated address string. Must not be `NULL`.
     * @param timeout   Optional timeout configuration. May be `NULL`.
     * @param out_srv   Output server structure. Must not be `NULL`.
     * @param out_error Optional output error message. May be `NULL`.
     *
     * @return `0` on success. Non zero on failure.
     */
    int gamedig_query(
        const char *game_id,
        const char *addr,
        const gamedig_timeout_config_t *timeout,
        gamedig_server_t *out_srv,
        gamedig_error_t *out_error);

    /**
     * @brief Releases resources owned by a server object.
     *
     * The structure itself is owned by the caller.
     * All internal allocations are released by this function.
     *
     * Safe to call with `NULL`.
     *
     * @param server Server object whose resources should be released.
     */
    void gamedig_ffi_free_server(gamedig_server_t *server);

    /**
     * @brief Releases an error message returned by the library.
     *
     * Safe to call with `NULL`.
     *
     * @param error Error string to release.
     */
    void gamedig_ffi_free_error(gamedig_error_t error);

#ifdef __cplusplus
} /* extern "C" */
#endif /* __cplusplus */

#endif /* GAMEDIG_FFI_H */
