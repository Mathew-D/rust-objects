🎮 Student Guide — Cloud Worker Setup (Web Editor Method)

👉 Click “Sign up with GitHub”

🌐 STEP 2 — Enable Workers access
Click Compute
Click Workers & Pages
If asked, create a workers.dev subdomain
(example: student-games)

You will get:

https://your-worker.student-games.workers.dev
🧱 STEP 3 — Create a Worker
Click “Create Application”
Choose "Start with Hello World!"
Paste the following Inside your Worker:

👉 You will see something like:

export default {
  async fetch(request, env) {
    return new Response("Hello World");
  }
};

🧠 Replace with your API code

Delete everything and paste:

// Cloudflare Worker: index.js
// Handles DB actions for Rust client via HTTP POST
// Supports: fetch, fetch_by_id, insert, update, update_by_column, delete

export default {
  async fetch(request, env) {

    const corsHeaders = {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'POST, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type',
    };

    // CORS preflight
    if (request.method === 'OPTIONS') {
      return new Response(null, { status: 204, headers: corsHeaders });
    }

    if (request.method !== 'POST') {
      return new Response('Method Not Allowed', { status: 405, headers: corsHeaders });
    }

    let data;
    try {
      data = await request.json();
    } catch {
      return new Response('Invalid JSON', { status: 400, headers: corsHeaders });
    }

    const { action, table, id, record, column, value } = data;

    if (!action || !table) {
      return new Response('Missing action or table', { status: 400, headers: corsHeaders });
    }

    // 🔒 Sanitize identifiers (table / column)
    let safeTable;
    try {
      safeTable = safe_identifier(table);
    } catch {
      return new Response('Invalid table name', { status: 400, headers: corsHeaders });
    }

    let safeColumn = null;
    if (column) {
      try {
        safeColumn = safe_identifier(column);
      } catch {
        return new Response('Invalid column name', { status: 400, headers: corsHeaders });
      }
    }

    let sql;
    let params = [];

    switch (action) {

      case 'fetch':
        sql = `SELECT * FROM ${safeTable}`;
        break;

      case 'fetch_by_id':
        sql = `SELECT * FROM ${safeTable} WHERE id = ?`;
        params = [wrap_param(id)];
        break;

      case 'insert':
        if (!record) {
          return new Response('Missing record', { status: 400, headers: corsHeaders });
        }

        const keys = Object.keys(record).filter(k => k !== 'id');
        const cols = keys.map(safe_identifier).join(', ');
        const placeholders = keys.map(() => '?').join(', ');

        sql = `INSERT INTO ${safeTable} (${cols}) VALUES (${placeholders})`;
        params = keys.map(k => wrap_param(record[k]));
        break;

      case 'update':
        if (!record || record.id == null) {
          return new Response('Missing record or id', { status: 400, headers: corsHeaders });
        }

        const updateKeys = Object.keys(record).filter(k => k !== 'id');
        const setClause = updateKeys.map(k => `${safe_identifier(k)} = ?`).join(', ');

        sql = `UPDATE ${safeTable} SET ${setClause} WHERE id = ?`;
        params = updateKeys.map(k => wrap_param(record[k]));
        params.push(wrap_param(record.id));
        break;

      case 'update_by_column':
        if (!id || !safeColumn) {
          return new Response('Missing id or column', { status: 400, headers: corsHeaders });
        }

        sql = `UPDATE ${safeTable} SET ${safeColumn} = ? WHERE id = ?`;
        params = [wrap_param(value), wrap_param(id)];
        break;

      case 'delete':
        if (!id) {
          return new Response('Missing id', { status: 400, headers: corsHeaders });
        }

        sql = `DELETE FROM ${safeTable} WHERE id = ?`;
        params = [wrap_param(id)];
        break;

      default:
        return new Response('Unknown action', { status: 400, headers: corsHeaders });
    }

    // 🔐 Env vars
    const TURSO_URL = env.TURSO_URL;
    const TURSO_AUTH_TOKEN = env.TURSO_AUTH_TOKEN;

    if (!TURSO_URL || !TURSO_AUTH_TOKEN) {
      return new Response('Missing DB credentials', { status: 500, headers: corsHeaders });
    }

    // 📡 Turso request
    const dbReq = [{
      steps: [{ stmt: { sql, args: params } }]
    }];

    const resp = await fetch(`${TURSO_URL}/v1/batch`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${TURSO_AUTH_TOKEN}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(dbReq)
    });

    if (!resp.ok) {
      const errText = await resp.text();
      return new Response(`DB error: ${errText}`, { status: 500, headers: corsHeaders });
    }

    let dbRes;
    try {
      dbRes = await resp.json();
    } catch {
      return new Response('Invalid DB response', { status: 500, headers: corsHeaders });
    }

    // 📦 Format response
    switch (action) {

      case 'fetch': {
        const step = dbRes.result?.step_results?.[0];

        if (!step?.cols || !step?.rows) {
          return new Response(JSON.stringify({ records: [] }), { status: 200, headers: corsHeaders });
        }

        const records = step.rows.map(row => {
          const obj = {};
          for (let i = 0; i < step.cols.length; i++) {
            const col = step.cols[i];
            let val = row[i]?.value ?? null;

            if (val !== null && typeof val === 'string' && col.decltype) {
              const decl = col.decltype.toUpperCase();

              if (decl.includes('INT')) val = parseInt(val, 10);
              else if (decl.includes('REAL') || decl.includes('FLOAT')) val = parseFloat(val);
            }

            obj[col.name] = val;
          }
          return obj;
        });

        return new Response(JSON.stringify({ records }), { status: 200, headers: corsHeaders });
      }

      case 'insert':
        return new Response(JSON.stringify({
         success: (dbRes.result?.step_results?.[0]?.rows_affected ?? 0) > 0
        }), { status: 200, headers: corsHeaders });

      case 'update':
      case 'update_by_column':
        return new Response(JSON.stringify({
          updated: dbRes.result?.step_results?.[0]?.rows_affected ?? 0
        }), { status: 200, headers: corsHeaders });

      case 'delete':
        return new Response(JSON.stringify({
          deleted: dbRes.result?.step_results?.[0]?.rows_affected ?? 0
        }), { status: 200, headers: corsHeaders });

      case 'fetch_by_id':
        return new Response(JSON.stringify({
          record: dbRes.result?.step_results?.[0]?.rows?.[0] ?? null
        }), { status: 200, headers: corsHeaders });

      default:
        return new Response(JSON.stringify({}), { status: 200, headers: corsHeaders });
    }
  }
};

// 🔒 Only allow safe SQL identifiers
function safe_identifier(name) {
  if (!/^[a-zA-Z0-9_]+$/.test(name)) {
    throw new Error("Invalid identifier");
  }
  return name;
}

// 🔧 Convert JS values to Turso format
function wrap_param(val) {
  if (val === null || val === undefined) return { type: 'null', value: null };

  if (typeof val === 'number') {
    if (Number.isInteger(val)) return { type: 'integer', value: String(val) };
    return { type: 'real', value: String(val) };
  }

  if (typeof val === 'boolean') {
    return { type: 'integer', value: val ? '1' : '0' };
  }

  return { type: 'text', value: String(val) };
}


Click Deploy

🔐 STEP 6 — Add database credentials

Inside Worker page:

Click Settings
Go to Variables
Click Add variable

Add:

TURSO_AUTH_TOKEN
TURSO_DATABASE_URL


🌍 STEP 7 — Get your API URL

Go back to:

👉 Worker overview page

Copy your URL:

https://your-worker.student-games.workers.dev

This is your backend API.

🎮 STEP 8 — Connect to your Rust game

In your database.rs file:

pub const WORKER_URL: &str = "https://YOUR-WORKER-HERE.workers.dev";

Now all database calls go through this Worker.