let clipboard_buffer = "";

function register_plugin(importObject) {
    // ✅ COPY (put this back!)
    importObject.env.mq_copy_to_clipboard = function(ptr, len) {
        const text = new TextDecoder().decode(
            new Uint8Array(wasm_memory.buffer, ptr, len)
        );
        navigator.clipboard.writeText(text);
    };

    // ✅ REQUEST paste (async)
    importObject.env.mq_request_paste = async function() {
        try {
            clipboard_buffer = await navigator.clipboard.readText();
        } catch (e) {
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
// 👇 THIS is critical — expose globally
window.register_plugin = register_plugin;
miniquad_add_plugin({ register_plugin });
