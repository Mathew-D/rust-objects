// LocalStorage plugin
// Exposes js_local_storage_set and js_local_storage_get for Rust FFI
let local_storage_result_buffer = "";

function js_local_storage_set(key_ptr, key_len, value_ptr, value_len) {
    const mem = wasm_memory.buffer;
    const decoder = new TextDecoder();
    const key = decoder.decode(new Uint8Array(mem, key_ptr, key_len));
    const value = decoder.decode(new Uint8Array(mem, value_ptr, value_len));
    localStorage.setItem(key, value);
}

function js_local_storage_get(key_ptr, key_len, out_ptr, out_len) {
    const mem = wasm_memory.buffer;
    const decoder = new TextDecoder();
    const encoder = new TextEncoder();
    const key = decoder.decode(new Uint8Array(mem, key_ptr, key_len));
    const value = localStorage.getItem(key);
    if (typeof value !== 'string') return 0;
    const bytes = encoder.encode(value);
    const len = Math.min(bytes.length, out_len);
    new Uint8Array(mem, out_ptr, len).set(bytes.subarray(0, len));
    return len;
}

function local_storage_register_plugin(importObject) {
    if (!importObject.env) importObject.env = {};
    importObject.env.js_local_storage_set = js_local_storage_set;
    importObject.env.js_local_storage_get = js_local_storage_get;
}

window.local_storage_register_plugin = local_storage_register_plugin;
miniquad_add_plugin({
    name: "local_storage",
    version: "0.1.0",
    register_plugin: local_storage_register_plugin
});

//Clipboard plugin
let clipboard_buffer = "";

function clipboard_register_plugin(importObject) {
    // ✅ COPY (put this back!)
    importObject.env.mq_copy_to_clipboard = function(ptr, len) {
        const text = new TextDecoder().decode(
            new Uint8Array(wasm_memory.buffer, ptr, len)
        );
        navigator.clipboard.writeText(text);
    };

    // ✅ REQUEST paste (async)
    importObject.env.mq_request_paste = async function() {
    if (navigator.userAgent.includes("Firefox")) { return }

    try {
        clipboard_buffer = await navigator.clipboard.readText();
    } catch {
        clipboard_buffer = "";
    }
};

    // ✅ LENGTH
    importObject.env.mq_get_paste_len = function() {
        return clipboard_buffer.length;
    };

    // ✅ FILL buffer (Rust allocates, JS writes)
    importObject.env.mq_fill_paste_buffer = function(ptr) {
        if (!clipboard_buffer) return;

        const enc = new TextEncoder();
        const bytes = enc.encode(clipboard_buffer);

        new Uint8Array(wasm_memory.buffer, ptr, bytes.length).set(bytes);
    };

    // ✅ CLEAR
    importObject.env.mq_clear_paste = function() {
        clipboard_buffer = "";
    };
}

window.clipboard_register_plugin = clipboard_register_plugin;
miniquad_add_plugin({
    name: "clipboard",
    version: "0.1.0",
    register_plugin: clipboard_register_plugin
});

// Database plugin
// Exposes mq_db_query for Rust to call via FFI

let db_query_result_buffer = "";

async function mq_db_query(ptr, len, url_ptr, url_len, token_ptr, token_len) {
    // WASM memory is expected to be available as wasm_memory
    try {
        const mem = wasm_memory.buffer;
        const decoder = new TextDecoder();
        const body = decoder.decode(new Uint8Array(mem, ptr, len));
        const url = decoder.decode(new Uint8Array(mem, url_ptr, url_len));
        const token = decoder.decode(new Uint8Array(mem, token_ptr, token_len));

        const resp = await fetch(url + "/v2/pipeline", {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${token}`,
                "Content-Type": "application/json"
            },
            body
        });
        db_query_result_buffer = await resp.text();
    
    } catch (e) {
        db_query_result_buffer = JSON.stringify({ error: "fetch_failed", message: e && e.message ? e.message : String(e) });
    }
}

function mq_db_query_result_len() {
    return db_query_result_buffer.length;
}

function mq_db_query_fill_result(ptr) {
    if (!db_query_result_buffer) return;
    const enc = new TextEncoder();
    const bytes = enc.encode(db_query_result_buffer);
    new Uint8Array(wasm_memory.buffer, ptr, bytes.length).set(bytes);
}

function mq_db_query_clear_result() {
    db_query_result_buffer = "";
}

function db_register_plugin(importObject) {
    if (!importObject.env) importObject.env = {};
    importObject.env.mq_db_query = mq_db_query;
    importObject.env.mq_db_query_result_len = mq_db_query_result_len;
    importObject.env.mq_db_query_fill_result = mq_db_query_fill_result;
    importObject.env.mq_db_query_clear_result = mq_db_query_clear_result;
}

window.db_register_plugin = window.db_register_plugin;
miniquad_add_plugin({
    name: "database",
    version: "0.1.0",
    register_plugin: db_register_plugin
});
