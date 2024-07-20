// .wrangler/tmp/bundle-uOoR5y/checked-fetch.js
var urls = /* @__PURE__ */ new Set();
function checkURL(request, init) {
  const url = request instanceof URL ? request : new URL(
    (typeof request === "string" ? new Request(request, init) : request).url
  );
  if (url.port && url.port !== "443" && url.protocol === "https:") {
    if (!urls.has(url.toString())) {
      urls.add(url.toString());
      console.warn(
        `WARNING: known issue with \`fetch()\` requests to custom HTTPS ports in published Workers:
 - ${url.toString()} - the custom port will be ignored when the Worker is published using the \`wrangler deploy\` command.
`
      );
    }
  }
}
globalThis.fetch = new Proxy(globalThis.fetch, {
  apply(target, thisArg, argArray) {
    const [request, init] = argArray;
    checkURL(request, init);
    return Reflect.apply(target, thisArg, argArray);
  }
});

// build/worker/shim.mjs
import U from "./e09f30a44210704e944ffbfe2d277fdf9702e876-index.wasm";
import Yt from "./e09f30a44210704e944ffbfe2d277fdf9702e876-index.wasm";
import { WorkerEntrypoint as Pt } from "cloudflare:workers";
var D = Object.defineProperty;
var F = (e, t) => {
  for (var n in t)
    D(e, n, { get: t[n], enumerable: true });
};
var g = {};
F(g, { IntoUnderlyingByteSource: () => q, IntoUnderlyingSink: () => T, IntoUnderlyingSource: () => C, MinifyConfig: () => S, PipeOptions: () => L, PolishConfig: () => Y, QueuingStrategy: () => v, R2Range: () => R, ReadableStreamGetReaderOptions: () => W, RequestRedirect: () => Z, __wbg_buffer_4e79326814bdd393: () => Rt, __wbg_buffer_55ba7a6b1b92e2ac: () => xt, __wbg_byobRequest_08c18cee35def1f4: () => Tt, __wbg_byteLength_5299848ed3264181: () => St, __wbg_byteOffset_b69b0a07afccce19: () => Wt, __wbg_call_587b30eea3e09332: () => dt, __wbg_cause_52959bcad93f9e0f: () => ft, __wbg_cf_703652f0d2c5b8d1: () => et, __wbg_close_da7e6fb9d9851e5a: () => Lt, __wbg_close_e9110ca16e2567db: () => It, __wbg_enqueue_d71a1a518e21f5c3: () => $t, __wbg_error_a7e23606158b68b9: () => Ft, __wbg_headers_1eff4f53324496e6: () => tt, __wbg_instanceof_Error_fac23a8832b241da: () => ct, __wbg_length_0aab7ffd65ad19ed: () => ht, __wbg_log_dc06ec929fc95a20: () => rt, __wbg_method_e15eb9cf1c32cdbb: () => G, __wbg_new_143b41b4342650bb: () => ot, __wbg_new_2b55e405e4af4986: () => wt, __wbg_new_2b6fea4ea03b1b95: () => Ut, __wbg_new_87297f22973157c8: () => vt, __wbg_newwithbyteoffsetandlength_88d1d8be5df94b9b: () => mt, __wbg_newwithlength_89eeca401d8918c2: () => gt, __wbg_newwithoptbuffersourceandinit_6c49960a834dd7cf: () => pt, __wbg_newwithoptreadablestreamandinit_d238e5b972c7b57f: () => at, __wbg_newwithoptstrandinit_ff70839f3334d3aa: () => bt, __wbg_resolve_ae38ad63c43ff98b: () => Ot, __wbg_respond_8fadc5f5c9d95422: () => qt, __wbg_set_07da13cc24b69217: () => lt, __wbg_set_3698e3ca519b3c3c: () => jt, __wbg_set_76353df4722f4954: () => st, __wbg_then_8df675b8bb5d5e3c: () => At, __wbg_toString_506566b763774a16: () => ut, __wbg_url_3325e0ef088003ca: () => Q, __wbg_view_231340b0dd8a2484: () => Ct, __wbindgen_cb_drop: () => Mt, __wbindgen_closure_wrapper994: () => Nt, __wbindgen_debug_string: () => kt, __wbindgen_memory: () => yt, __wbindgen_number_new: () => zt, __wbindgen_object_clone_ref: () => Dt, __wbindgen_object_drop_ref: () => _t, __wbindgen_string_get: () => it, __wbindgen_string_new: () => nt, __wbindgen_throw: () => Et, fetch: () => I, getMemory: () => N });
var z = new WebAssembly.Instance(U, { "./index_bg.js": g });
var r = z.exports;
function N() {
  return r.memory;
}
var P = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
var $ = new P("utf-8", { ignoreBOM: true, fatal: true });
$.decode();
var m = null;
function k() {
  return (m === null || m.byteLength === 0) && (m = new Uint8Array(r.memory.buffer)), m;
}
function w(e, t) {
  return e = e >>> 0, $.decode(k().subarray(e, e + t));
}
var d = new Array(128).fill(void 0);
d.push(void 0, null, true, false);
var x = d.length;
function i(e) {
  x === d.length && d.push(d.length + 1);
  let t = x;
  return x = d[t], d[t] = e, t;
}
function o(e) {
  return d[e];
}
function H(e) {
  e < 132 || (d[e] = x, x = e);
}
function a(e) {
  let t = o(e);
  return H(e), t;
}
var h = 0;
var X = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
var E = new X("utf-8");
var B = typeof E.encodeInto == "function" ? function(e, t) {
  return E.encodeInto(e, t);
} : function(e, t) {
  let n = E.encode(e);
  return t.set(n), { read: e.length, written: n.length };
};
function O(e, t, n) {
  if (n === void 0) {
    let f = E.encode(e), y = t(f.length) >>> 0;
    return k().subarray(y, y + f.length).set(f), h = f.length, y;
  }
  let _ = e.length, s = t(_) >>> 0, u = k(), c = 0;
  for (; c < _; c++) {
    let f = e.charCodeAt(c);
    if (f > 127)
      break;
    u[s + c] = f;
  }
  if (c !== _) {
    c !== 0 && (e = e.slice(c)), s = n(s, _, _ = c + e.length * 3) >>> 0;
    let f = k().subarray(s + c, s + _), y = B(e, f);
    c += y.written;
  }
  return h = c, s;
}
function p(e) {
  return e == null;
}
var j = null;
function b() {
  return (j === null || j.byteLength === 0) && (j = new Int32Array(r.memory.buffer)), j;
}
function A(e) {
  let t = typeof e;
  if (t == "number" || t == "boolean" || e == null)
    return `${e}`;
  if (t == "string")
    return `"${e}"`;
  if (t == "symbol") {
    let s = e.description;
    return s == null ? "Symbol" : `Symbol(${s})`;
  }
  if (t == "function") {
    let s = e.name;
    return typeof s == "string" && s.length > 0 ? `Function(${s})` : "Function";
  }
  if (Array.isArray(e)) {
    let s = e.length, u = "[";
    s > 0 && (u += A(e[0]));
    for (let c = 1; c < s; c++)
      u += ", " + A(e[c]);
    return u += "]", u;
  }
  let n = /\[object ([^\]]+)\]/.exec(toString.call(e)), _;
  if (n.length > 1)
    _ = n[1];
  else
    return toString.call(e);
  if (_ == "Object")
    try {
      return "Object(" + JSON.stringify(e) + ")";
    } catch {
      return "Object";
    }
  return e instanceof Error ? `${e.name}: ${e.message}
${e.stack}` : _;
}
function J(e, t, n, _) {
  let s = { a: e, b: t, cnt: 1, dtor: n }, u = (...c) => {
    s.cnt++;
    let f = s.a;
    s.a = 0;
    try {
      return _(f, s.b, ...c);
    } finally {
      --s.cnt === 0 ? r.__wbindgen_export_2.get(s.dtor)(f, s.b) : s.a = f;
    }
  };
  return u.original = s, u;
}
function V(e, t, n) {
  r.__wbindgen_export_3(e, t, i(n));
}
function I(e, t, n) {
  let _ = r.fetch(i(e), i(t), i(n));
  return a(_);
}
function l(e, t) {
  try {
    return e.apply(this, t);
  } catch (n) {
    r.__wbindgen_export_4(i(n));
  }
}
function K(e, t, n, _) {
  r.__wbindgen_export_5(e, t, i(n), i(_));
}
var Y = Object.freeze({ Off: 0, 0: "Off", Lossy: 1, 1: "Lossy", Lossless: 2, 2: "Lossless" });
var Z = Object.freeze({ Error: 0, 0: "Error", Follow: 1, 1: "Follow", Manual: 2, 2: "Manual" });
var q = class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingbytesource_free(t);
  }
  get type() {
    let t, n;
    try {
      let u = r.__wbindgen_add_to_stack_pointer(-16);
      r.intounderlyingbytesource_type(u, this.__wbg_ptr);
      var _ = b()[u / 4 + 0], s = b()[u / 4 + 1];
      return t = _, n = s, w(_, s);
    } finally {
      r.__wbindgen_add_to_stack_pointer(16), r.__wbindgen_export_6(t, n);
    }
  }
  get autoAllocateChunkSize() {
    return r.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  start(t) {
    r.intounderlyingbytesource_start(this.__wbg_ptr, i(t));
  }
  pull(t) {
    let n = r.intounderlyingbytesource_pull(this.__wbg_ptr, i(t));
    return a(n);
  }
  cancel() {
    let t = this.__destroy_into_raw();
    r.intounderlyingbytesource_cancel(t);
  }
};
var T = class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingsink_free(t);
  }
  write(t) {
    let n = r.intounderlyingsink_write(this.__wbg_ptr, i(t));
    return a(n);
  }
  close() {
    let t = this.__destroy_into_raw(), n = r.intounderlyingsink_close(t);
    return a(n);
  }
  abort(t) {
    let n = this.__destroy_into_raw(), _ = r.intounderlyingsink_abort(n, i(t));
    return a(_);
  }
};
var C = class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingsource_free(t);
  }
  pull(t) {
    let n = r.intounderlyingsource_pull(this.__wbg_ptr, i(t));
    return a(n);
  }
  cancel() {
    let t = this.__destroy_into_raw();
    r.intounderlyingsource_cancel(t);
  }
};
var S = class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_minifyconfig_free(t);
  }
  get js() {
    return r.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set js(t) {
    r.__wbg_set_minifyconfig_js(this.__wbg_ptr, t);
  }
  get html() {
    return r.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  set html(t) {
    r.__wbg_set_minifyconfig_html(this.__wbg_ptr, t);
  }
  get css() {
    return r.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  set css(t) {
    r.__wbg_set_minifyconfig_css(this.__wbg_ptr, t);
  }
};
var L = class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_pipeoptions_free(t);
  }
  get preventClose() {
    return r.pipeoptions_preventClose(this.__wbg_ptr) !== 0;
  }
  get preventCancel() {
    return r.pipeoptions_preventCancel(this.__wbg_ptr) !== 0;
  }
  get preventAbort() {
    return r.pipeoptions_preventAbort(this.__wbg_ptr) !== 0;
  }
  get signal() {
    let t = r.pipeoptions_signal(this.__wbg_ptr);
    return a(t);
  }
};
var v = class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_queuingstrategy_free(t);
  }
  get highWaterMark() {
    return r.queuingstrategy_highWaterMark(this.__wbg_ptr);
  }
};
var R = class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_r2range_free(t);
  }
  get offset() {
    try {
      let _ = r.__wbindgen_add_to_stack_pointer(-16);
      r.__wbg_get_r2range_offset(_, this.__wbg_ptr);
      var t = b()[_ / 4 + 0], n = b()[_ / 4 + 1];
      return t === 0 ? void 0 : n >>> 0;
    } finally {
      r.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set offset(t) {
    r.__wbg_set_r2range_offset(this.__wbg_ptr, !p(t), p(t) ? 0 : t);
  }
  get length() {
    try {
      let _ = r.__wbindgen_add_to_stack_pointer(-16);
      r.__wbg_get_r2range_length(_, this.__wbg_ptr);
      var t = b()[_ / 4 + 0], n = b()[_ / 4 + 1];
      return t === 0 ? void 0 : n >>> 0;
    } finally {
      r.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set length(t) {
    r.__wbg_set_r2range_length(this.__wbg_ptr, !p(t), p(t) ? 0 : t);
  }
  get suffix() {
    try {
      let _ = r.__wbindgen_add_to_stack_pointer(-16);
      r.__wbg_get_r2range_suffix(_, this.__wbg_ptr);
      var t = b()[_ / 4 + 0], n = b()[_ / 4 + 1];
      return t === 0 ? void 0 : n >>> 0;
    } finally {
      r.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set suffix(t) {
    r.__wbg_set_r2range_suffix(this.__wbg_ptr, !p(t), p(t) ? 0 : t);
  }
};
var W = class {
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_readablestreamgetreaderoptions_free(t);
  }
  get mode() {
    let t = r.readablestreamgetreaderoptions_mode(this.__wbg_ptr);
    return a(t);
  }
};
function G(e, t) {
  let n = o(t).method, _ = O(n, r.__wbindgen_export_0, r.__wbindgen_export_1), s = h;
  b()[e / 4 + 1] = s, b()[e / 4 + 0] = _;
}
function Q(e, t) {
  let n = o(t).url, _ = O(n, r.__wbindgen_export_0, r.__wbindgen_export_1), s = h;
  b()[e / 4 + 1] = s, b()[e / 4 + 0] = _;
}
function tt(e) {
  let t = o(e).headers;
  return i(t);
}
function et(e) {
  let t = o(e).cf;
  return p(t) ? 0 : i(t);
}
function nt(e, t) {
  let n = w(e, t);
  return i(n);
}
function rt(e) {
  console.log(o(e));
}
function _t(e) {
  a(e);
}
function ot() {
  return l(function() {
    let e = new Headers();
    return i(e);
  }, arguments);
}
function st() {
  return l(function(e, t, n, _, s) {
    o(e).set(w(t, n), w(_, s));
  }, arguments);
}
function it(e, t) {
  let n = o(t), _ = typeof n == "string" ? n : void 0;
  var s = p(_) ? 0 : O(_, r.__wbindgen_export_0, r.__wbindgen_export_1), u = h;
  b()[e / 4 + 1] = u, b()[e / 4 + 0] = s;
}
function ct(e) {
  let t;
  try {
    t = o(e) instanceof Error;
  } catch {
    t = false;
  }
  return t;
}
function ut(e) {
  let t = o(e).toString();
  return i(t);
}
function ft(e) {
  let t = o(e).cause;
  return i(t);
}
function bt() {
  return l(function(e, t, n) {
    let _ = new Response(e === 0 ? void 0 : w(e, t), o(n));
    return i(_);
  }, arguments);
}
function gt(e) {
  let t = new Uint8Array(e >>> 0);
  return i(t);
}
function pt() {
  return l(function(e, t) {
    let n = new Response(o(e), o(t));
    return i(n);
  }, arguments);
}
function at() {
  return l(function(e, t) {
    let n = new Response(o(e), o(t));
    return i(n);
  }, arguments);
}
function dt() {
  return l(function(e, t, n) {
    let _ = o(e).call(o(t), o(n));
    return i(_);
  }, arguments);
}
function wt(e, t) {
  try {
    var n = { a: e, b: t }, _ = (u, c) => {
      let f = n.a;
      n.a = 0;
      try {
        return K(f, n.b, u, c);
      } finally {
        n.a = f;
      }
    };
    let s = new Promise(_);
    return i(s);
  } finally {
    n.a = n.b = 0;
  }
}
function lt() {
  return l(function(e, t, n) {
    return Reflect.set(o(e), o(t), o(n));
  }, arguments);
}
function ht(e) {
  return o(e).length;
}
function yt() {
  let e = r.memory;
  return i(e);
}
function xt(e) {
  let t = o(e).buffer;
  return i(t);
}
function mt(e, t, n) {
  let _ = new Uint8Array(o(e), t >>> 0, n >>> 0);
  return i(_);
}
function jt(e, t, n) {
  o(e).set(o(t), n >>> 0);
}
function kt(e, t) {
  let n = A(o(t)), _ = O(n, r.__wbindgen_export_0, r.__wbindgen_export_1), s = h;
  b()[e / 4 + 1] = s, b()[e / 4 + 0] = _;
}
function Et(e, t) {
  throw new Error(w(e, t));
}
function Ot(e) {
  let t = Promise.resolve(o(e));
  return i(t);
}
function Mt(e) {
  let t = a(e).original;
  return t.cnt-- == 1 ? (t.a = 0, true) : false;
}
function At(e, t) {
  let n = o(e).then(o(t));
  return i(n);
}
function qt(e, t) {
  o(e).respond(t >>> 0);
}
function Tt(e) {
  let t = o(e).byobRequest;
  return p(t) ? 0 : i(t);
}
function Ct(e) {
  let t = o(e).view;
  return p(t) ? 0 : i(t);
}
function St(e) {
  return o(e).byteLength;
}
function Lt(e) {
  o(e).close();
}
function vt(e, t) {
  let n = new Error(w(e, t));
  return i(n);
}
function Rt(e) {
  let t = o(e).buffer;
  return i(t);
}
function Wt(e) {
  return o(e).byteOffset;
}
function It(e) {
  o(e).close();
}
function $t(e, t) {
  o(e).enqueue(o(t));
}
function Dt(e) {
  let t = o(e);
  return i(t);
}
function Ft(e) {
  console.error(o(e));
}
function Ut() {
  let e = new Object();
  return i(e);
}
function zt(e) {
  return i(e);
}
function Nt(e, t, n) {
  let _ = J(e, t, 88, V);
  return i(_);
}
var M = class extends Pt {
  async fetch(t) {
    return await I(t, this.env, this.ctx);
  }
  async queue(t) {
    return await (void 0)(t, this.env, this.ctx);
  }
  async scheduled(t) {
    return await (void 0)(t, this.env, this.ctx);
  }
};
var Ht = ["IntoUnderlyingByteSource", "IntoUnderlyingSink", "IntoUnderlyingSource", "MinifyConfig", "PolishConfig", "R2Range", "RequestRedirect", "fetch", "queue", "scheduled", "getMemory"];
Object.keys(g).map((e) => {
  Ht.includes(e) | e.startsWith("__") || (M.prototype[e] = g[e]);
});
var Gt = M;

// ../../../.npm/_npx/32026684e21afda6/node_modules/wrangler/templates/middleware/middleware-ensure-req-body-drained.ts
var drainBody = async (request, env, _ctx, middlewareCtx) => {
  try {
    return await middlewareCtx.next(request, env);
  } finally {
    try {
      if (request.body !== null && !request.bodyUsed) {
        const reader = request.body.getReader();
        while (!(await reader.read()).done) {
        }
      }
    } catch (e) {
      console.error("Failed to drain the unused request body.", e);
    }
  }
};
var middleware_ensure_req_body_drained_default = drainBody;

// ../../../.npm/_npx/32026684e21afda6/node_modules/wrangler/templates/middleware/middleware-miniflare3-json-error.ts
function reduceError(e) {
  return {
    name: e?.name,
    message: e?.message ?? String(e),
    stack: e?.stack,
    cause: e?.cause === void 0 ? void 0 : reduceError(e.cause)
  };
}
var jsonError = async (request, env, _ctx, middlewareCtx) => {
  try {
    return await middlewareCtx.next(request, env);
  } catch (e) {
    const error = reduceError(e);
    return Response.json(error, {
      status: 500,
      headers: { "MF-Experimental-Error-Stack": "true" }
    });
  }
};
var middleware_miniflare3_json_error_default = jsonError;

// .wrangler/tmp/bundle-uOoR5y/middleware-insertion-facade.js
var __INTERNAL_WRANGLER_MIDDLEWARE__ = [
  middleware_ensure_req_body_drained_default,
  middleware_miniflare3_json_error_default
];
var middleware_insertion_facade_default = Gt;

// ../../../.npm/_npx/32026684e21afda6/node_modules/wrangler/templates/middleware/common.ts
var __facade_middleware__ = [];
function __facade_register__(...args) {
  __facade_middleware__.push(...args.flat());
}
function __facade_invokeChain__(request, env, ctx, dispatch, middlewareChain) {
  const [head, ...tail] = middlewareChain;
  const middlewareCtx = {
    dispatch,
    next(newRequest, newEnv) {
      return __facade_invokeChain__(newRequest, newEnv, ctx, dispatch, tail);
    }
  };
  return head(request, env, ctx, middlewareCtx);
}
function __facade_invoke__(request, env, ctx, dispatch, finalMiddleware) {
  return __facade_invokeChain__(request, env, ctx, dispatch, [
    ...__facade_middleware__,
    finalMiddleware
  ]);
}

// .wrangler/tmp/bundle-uOoR5y/middleware-loader.entry.ts
var __Facade_ScheduledController__ = class {
  constructor(scheduledTime, cron, noRetry) {
    this.scheduledTime = scheduledTime;
    this.cron = cron;
    this.#noRetry = noRetry;
  }
  #noRetry;
  noRetry() {
    if (!(this instanceof __Facade_ScheduledController__)) {
      throw new TypeError("Illegal invocation");
    }
    this.#noRetry();
  }
};
function wrapExportedHandler(worker) {
  if (__INTERNAL_WRANGLER_MIDDLEWARE__ === void 0 || __INTERNAL_WRANGLER_MIDDLEWARE__.length === 0) {
    return worker;
  }
  for (const middleware of __INTERNAL_WRANGLER_MIDDLEWARE__) {
    __facade_register__(middleware);
  }
  const fetchDispatcher = function(request, env, ctx) {
    if (worker.fetch === void 0) {
      throw new Error("Handler does not export a fetch() function.");
    }
    return worker.fetch(request, env, ctx);
  };
  return {
    ...worker,
    fetch(request, env, ctx) {
      const dispatcher = function(type, init) {
        if (type === "scheduled" && worker.scheduled !== void 0) {
          const controller = new __Facade_ScheduledController__(
            Date.now(),
            init.cron ?? "",
            () => {
            }
          );
          return worker.scheduled(controller, env, ctx);
        }
      };
      return __facade_invoke__(request, env, ctx, dispatcher, fetchDispatcher);
    }
  };
}
function wrapWorkerEntrypoint(klass) {
  if (__INTERNAL_WRANGLER_MIDDLEWARE__ === void 0 || __INTERNAL_WRANGLER_MIDDLEWARE__.length === 0) {
    return klass;
  }
  for (const middleware of __INTERNAL_WRANGLER_MIDDLEWARE__) {
    __facade_register__(middleware);
  }
  return class extends klass {
    #fetchDispatcher = (request, env, ctx) => {
      this.env = env;
      this.ctx = ctx;
      if (super.fetch === void 0) {
        throw new Error("Entrypoint class does not define a fetch() function.");
      }
      return super.fetch(request);
    };
    #dispatcher = (type, init) => {
      if (type === "scheduled" && super.scheduled !== void 0) {
        const controller = new __Facade_ScheduledController__(
          Date.now(),
          init.cron ?? "",
          () => {
          }
        );
        return super.scheduled(controller);
      }
    };
    fetch(request) {
      return __facade_invoke__(
        request,
        this.env,
        this.ctx,
        this.#dispatcher,
        this.#fetchDispatcher
      );
    }
  };
}
var WRAPPED_ENTRY;
if (typeof middleware_insertion_facade_default === "object") {
  WRAPPED_ENTRY = wrapExportedHandler(middleware_insertion_facade_default);
} else if (typeof middleware_insertion_facade_default === "function") {
  WRAPPED_ENTRY = wrapWorkerEntrypoint(middleware_insertion_facade_default);
}
var middleware_loader_entry_default = WRAPPED_ENTRY;
export {
  q as IntoUnderlyingByteSource,
  T as IntoUnderlyingSink,
  C as IntoUnderlyingSource,
  S as MinifyConfig,
  L as PipeOptions,
  Y as PolishConfig,
  v as QueuingStrategy,
  R as R2Range,
  W as ReadableStreamGetReaderOptions,
  Z as RequestRedirect,
  __INTERNAL_WRANGLER_MIDDLEWARE__,
  Rt as __wbg_buffer_4e79326814bdd393,
  xt as __wbg_buffer_55ba7a6b1b92e2ac,
  Tt as __wbg_byobRequest_08c18cee35def1f4,
  St as __wbg_byteLength_5299848ed3264181,
  Wt as __wbg_byteOffset_b69b0a07afccce19,
  dt as __wbg_call_587b30eea3e09332,
  ft as __wbg_cause_52959bcad93f9e0f,
  et as __wbg_cf_703652f0d2c5b8d1,
  Lt as __wbg_close_da7e6fb9d9851e5a,
  It as __wbg_close_e9110ca16e2567db,
  $t as __wbg_enqueue_d71a1a518e21f5c3,
  Ft as __wbg_error_a7e23606158b68b9,
  tt as __wbg_headers_1eff4f53324496e6,
  ct as __wbg_instanceof_Error_fac23a8832b241da,
  ht as __wbg_length_0aab7ffd65ad19ed,
  rt as __wbg_log_dc06ec929fc95a20,
  G as __wbg_method_e15eb9cf1c32cdbb,
  ot as __wbg_new_143b41b4342650bb,
  wt as __wbg_new_2b55e405e4af4986,
  Ut as __wbg_new_2b6fea4ea03b1b95,
  vt as __wbg_new_87297f22973157c8,
  mt as __wbg_newwithbyteoffsetandlength_88d1d8be5df94b9b,
  gt as __wbg_newwithlength_89eeca401d8918c2,
  pt as __wbg_newwithoptbuffersourceandinit_6c49960a834dd7cf,
  at as __wbg_newwithoptreadablestreamandinit_d238e5b972c7b57f,
  bt as __wbg_newwithoptstrandinit_ff70839f3334d3aa,
  Ot as __wbg_resolve_ae38ad63c43ff98b,
  qt as __wbg_respond_8fadc5f5c9d95422,
  lt as __wbg_set_07da13cc24b69217,
  jt as __wbg_set_3698e3ca519b3c3c,
  st as __wbg_set_76353df4722f4954,
  At as __wbg_then_8df675b8bb5d5e3c,
  ut as __wbg_toString_506566b763774a16,
  Q as __wbg_url_3325e0ef088003ca,
  Ct as __wbg_view_231340b0dd8a2484,
  Mt as __wbindgen_cb_drop,
  Nt as __wbindgen_closure_wrapper994,
  kt as __wbindgen_debug_string,
  yt as __wbindgen_memory,
  zt as __wbindgen_number_new,
  Dt as __wbindgen_object_clone_ref,
  _t as __wbindgen_object_drop_ref,
  it as __wbindgen_string_get,
  nt as __wbindgen_string_new,
  Et as __wbindgen_throw,
  middleware_loader_entry_default as default,
  I as fetch,
  N as getMemory,
  Yt as wasmModule
};
//# sourceMappingURL=shim.js.map
