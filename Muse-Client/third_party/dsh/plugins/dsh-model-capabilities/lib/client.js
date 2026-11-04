/*!
 * dsh-model-capabilities — client half.
 *
 * Lazy-CJS factory bundle served by the DSH web client module system.
 * Registers one extension cell into the official Models settings card seat
 * (`settings.models.provider-card`, keyed by the `llm-pi-ai` settings
 * namespace), giving every pi-ai provider card:
 *
 *   - per model: accepted modalities (input) and thinking-intensity tiers
 *     (reasoningEfforts: unset / false / standard preset / custom wire values)
 *   - provider level: default reasoning effort, default modalities,
 *     gateway compat switches (supportsDeveloperRole, supportsReasoningEffort,
 *     maxTokensField, thinkingFormat)
 *   - provider request headers: a generic editor writing `providers.<route>.headers`
 *     (names lowercased; user-agent reserved). A value containing `{{session}}`
 *     is substituted per request at the wire layer with the current DSH
 *     session's SHA-256 token — the opencode Go session-affinity mechanism;
 *     the Host face also auto-stamps the header on matching hosts when the
 *     user set none
 *
 * UI: built on the official @deepseek-ai/dsh-client-ui-primitives atoms
 * (Button / Pill / Input / Menu / DisclosureRow / StateDot) and official
 * `--dsw-*` tokens only — no hardcoded colors, so light/dark themes follow
 * the app. Custom CSS is a single dedupe-guarded sheet (data-plugin-css).
 *
 * Data owner: the pi-ai adapter's `llm-pi-ai` settings namespace. This browser
 * half talks to the same-origin Host bridge of this bundle — GET/POST
 * /model-capabilities — which reads through ctx.settings and writes through
 * settings.mutate() with the view revision, so conflicts are refused and every
 * write is validated by the pi-ai config schema (assertServiceable) before it
 * reaches settings.yaml. Conflicts and schema rejections surface verbatim.
 */
window.__ModuleLoader__.load({
  id: 'dsh-model-capabilities',
  factory: (require) => {
    var module = { exports: {} };
    var exports = module.exports;

    const React = require('react');
    const primitives = require('@deepseek-ai/dsh-client-ui-primitives');
    const e = React.createElement;
    const { Button, Pill, Input, Menu, DisclosureRow, StateDot } = primitives;
    const IconChevronDown = primitives.IconChevronDownOutline14 ?? primitives.IconChevronDownOutline16;
    const IconThink = primitives.IconThinkOutline16 ?? primitives.IconSettingsOutline16;

    /* ── styles ──────────────────────────────────────────────────────────────
     * Official `--dsw-*` tokens only; density follows the settings panel
     * (14px titles, 12px tertiary labels, 13px controls, hairline sections).
     * Injected once per tag id — the guard keeps HMR/reloads from stacking. */
    const CSS_TAG = 'dsh-model-capabilities/client.css';
    const CSS = [
      '.mc{display:flex;flex-direction:column;gap:12px;margin-top:12px;padding-top:14px;border-top:1px solid var(--dsw-alias-border-l2)}',
      '.mc-title{display:flex;align-items:center;gap:8px;font-size:14px;line-height:22px;color:var(--dsw-alias-label-primary)}',
      '.mc-route{font-family:var(--ds-font-family-code,ui-monospace,Menlo,Consolas,monospace);font-size:12px;color:var(--dsw-alias-label-tertiary)}',
      '.mc-section{display:flex;flex-direction:column;gap:8px}',
      '.mc-sectionTitle{font-size:12px;line-height:18px;color:var(--dsw-alias-label-tertiary)}',
      '.mc-row{display:flex;align-items:center;gap:10px;min-height:28px;flex-wrap:wrap}',
      '.mc-label{flex:none;width:96px;font-size:12px;line-height:18px;color:var(--dsw-alias-label-secondary)}',
      '.mc-seg{display:inline-flex;gap:6px;flex-wrap:wrap}',
      '.mc-selectLabel{max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}',
      '.mc-effortRow{display:flex;align-items:center;gap:6px}',
      '.mc-effortInput{width:150px}',
      '.mc-effortInput input{width:100%;min-width:0;box-sizing:border-box}',
      '.mc-headerName{width:190px}',
      '.mc-headerValue{flex:1;min-width:140px}',
      '.mc-headerName input,.mc-headerValue input{width:100%;min-width:0;box-sizing:border-box}',
      '.mc-status{display:inline-flex;align-items:center;gap:6px;font-size:12px;line-height:18px;color:var(--dsw-alias-label-secondary)}',
      '.mc-statusError{color:var(--dsw-alias-state-error-primary)}',
      '.mc-actions{display:flex;align-items:center;gap:10px;flex-wrap:wrap}',
      '.mc-note{font-size:12px;line-height:18px;color:var(--dsw-alias-label-tertiary)}',
      '.mc-modelTitle{font-family:var(--ds-font-family-code,ui-monospace,Menlo,Consolas,monospace);font-size:12px;color:var(--dsw-alias-label-primary)}',
      '.mc-modelSummary{display:inline-flex;gap:6px;flex-wrap:wrap}',
      '.mc-modelBody{display:flex;flex-direction:column;gap:10px;padding:2px 0 4px}',
      '.mc-grid{display:flex;flex-wrap:wrap;gap:8px 14px}',
    ].join('\n');

    function mountStyles() {
      if (typeof document === 'undefined') return null;
      const selector = 'style[data-plugin-css="' + CSS_TAG + '"]';
      const existing = document.querySelector(selector);
      if (existing !== null) return existing;
      const tag = document.createElement('style');
      tag.dataset.plugin = 'dsh-model-capabilities';
      tag.dataset.pluginCss = CSS_TAG;
      tag.textContent = CSS;
      document.head.appendChild(tag);
      return tag;
    }

    /* ── domain data (bridge contract unchanged) ── */
    const LEVELS = ['off', 'minimal', 'low', 'medium', 'high', 'xhigh', 'max'];
    const LEVEL_LABEL = { off: '关', minimal: '极低', low: '低', medium: '中', high: '高', xhigh: '超高', max: '最高' };
    const FORMATS = ['openai', 'deepseek', 'openrouter', 'together', 'baseten', 'zai', 'qwen', 'chat-template', 'qwen-chat-template', 'string-thinking', 'ant-ling'];
    const PRESET = { off: null, low: 'low', medium: 'medium', high: 'high', max: 'max' };
    const HEADER_NAME_RE = /^[a-z0-9!#$%&'*+\-.^_`|~]+$/;

    function isOpencodeRoute(providerId, baseURL) {
      return /opencode/i.test(providerId ?? '') || /opencode\.ai/i.test(baseURL ?? '');
    }
    function headerRowsOf(dict) {
      const rows = Object.entries(dict ?? {}).map(([name, value]) => ({ name, value: typeof value === 'string' ? value : '' }));
      rows.sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
      return rows;
    }
    /** First validation error across the header rows, or null. Fully empty rows are ignored. */
    function headerRowsError(rows) {
      const seen = new Set();
      for (const row of rows) {
        const name = row.name.trim().toLowerCase();
        const value = row.value.trim();
        if (name === '' && value === '') continue;
        if (name === '') return '存在缺少名称的请求头';
        if (!HEADER_NAME_RE.test(name)) return `请求头名不合法：${name}`;
        if (name === 'user-agent') return 'user-agent 由 DSH 归属头占用，写入会被忽略';
        if (value === '') return `请求头 ${name} 缺少值`;
        if (value.length > 512) return `请求头 ${name} 的值超过 512 字符`;
        if (seen.has(name)) return `请求头重复：${name}`;
        seen.add(name);
      }
      return null;
    }
    /** The desired final header dict; fully empty rows are dropped. */
    function headersPayload(rows) {
      const dict = {};
      for (const row of rows) {
        const name = row.name.trim().toLowerCase();
        if (name === '') continue;
        dict[name] = row.value.trim();
      }
      return dict;
    }

    function effortsOf(m) {
      const efforts = {};
      for (const level of LEVELS) efforts[level] = { on: false, value: '' };
      if (m.reasoningEfforts === false) return { efforts, kind: 'false' };
      if (m.reasoningEfforts && typeof m.reasoningEfforts === 'object') {
        let preset = true;
        for (const level of LEVELS) {
          const value = m.reasoningEfforts[level];
          if (value === undefined) continue;
          efforts[level] = { on: true, value: value === null ? '' : String(value) };
          if (!(level in PRESET) || String(PRESET[level] ?? '') !== String(value ?? '')) preset = false;
        }
        return { efforts, kind: preset ? 'preset' : 'custom' };
      }
      return { efforts, kind: 'unset' };
    }
    function normalizeModel(m) {
      const { efforts, kind } = effortsOf(m);
      return { id: String(m.id ?? ''), input: Array.isArray(m.input) ? m.input.slice() : [], efforts, kind };
    }
    function loadView(revision, profile) {
      const compat = profile.compat ?? {};
      return {
        revision,
        hasModelsList: profile.hasModelsList === true || Array.isArray(profile.models),
        reasoning: typeof profile.reasoning === 'string' ? profile.reasoning : 'unset',
        defaultInput: Array.isArray(profile.defaultInput) ? profile.defaultInput.slice() : [],
        compat: {
          supportsDeveloperRole: typeof compat.supportsDeveloperRole === 'boolean' ? compat.supportsDeveloperRole : 'unset',
          supportsReasoningEffort: typeof compat.supportsReasoningEffort === 'boolean' ? compat.supportsReasoningEffort : 'unset',
          maxTokensField: typeof compat.maxTokensField === 'string' ? compat.maxTokensField : 'unset',
          thinkingFormat: typeof compat.thinkingFormat === 'string' ? compat.thinkingFormat : 'unset',
        },
        baseURL: typeof profile.baseURL === 'string' ? profile.baseURL : null,
        sessionAffinity: profile.sessionAffinity === 'per-session' ? 'per-session' : null,
        headersTouched: false,
        headerRows: headerRowsOf(profile.headers),
        models: (Array.isArray(profile.models) ? profile.models : []).map((m) => normalizeModel(m)),
      };
    }
    function effortValid(snapModel) {
      return LEVELS.filter((level) => snapModel.efforts[level].on && level !== 'off').every((level) => snapModel.efforts[level].value.trim() !== '');
    }
    function effortPayload(snapModel) {
      if (snapModel.kind === 'unset') return null;
      if (snapModel.kind === 'false') return false;
      if (snapModel.kind === 'preset') {
        const out = {};
        for (const [level, value] of Object.entries(PRESET)) out[level] = value;
        return out;
      }
      const dict = {};
      for (const level of LEVELS) {
        if (!snapModel.efforts[level].on) continue;
        const value = snapModel.efforts[level].value.trim();
        dict[level] = level === 'off' ? (value === '' ? null : value) : value;
      }
      return dict;
    }
    function modalityOf(model) {
      return model.input.includes('image') ? 'both' : model.input.includes('text') ? 'text' : 'inherit';
    }
    const MODALITY_LABEL = { inherit: '继承', text: '仅文本', both: '文本+图像' };
    const KIND_LABEL = { unset: '未设置', false: '禁用', preset: '标准档位', custom: '自定义' };

    /* ── official atoms ── */

    /** Dropdown select built on the official Menu (portal escapes card overflow). */
    function Select({ value, options, disabled, onChange }) {
      const [open, setOpen] = React.useState(false);
      const current = options.find((option) => option.value === value) ?? options[0];
      return e(Menu, {
        open,
        onClose: () => setOpen(false),
        portal: true,
        compact: true,
        dense: true,
        selectedId: String(value),
        items: options.map((option) => ({ id: String(option.value), label: option.label })),
        onSelect: (id) => {
          setOpen(false);
          const next = options.find((option) => String(option.value) === id);
          if (next !== undefined) onChange(next.value);
        },
        anchor: e(Button, { variant: 'outline', size: 'sm', disabled, onClick: () => setOpen((v) => !v) },
          e('span', { className: 'mc-selectLabel' }, current === undefined ? '' : current.label),
          IconChevronDown === undefined ? null : e(IconChevronDown, { size: 14 })),
      });
    }

    /** Segmented choice rendered as official Pills. */
    function Segmented({ value, options, onChange }) {
      return e('span', { className: 'mc-seg' },
        options.map((option) => e(Pill, {
          key: String(option.value),
          active: option.value === value,
          onClick: () => onChange(option.value),
        }, option.label)));
    }

    /** One status line: official StateDot + text. */
    function Status({ state, text, error }) {
      return e('span', { className: 'mc-status' + (error ? ' mc-statusError' : '') },
        e(StateDot, { state, size: 10 }),
        text);
    }

    /* ── main cell ── */

    function ModelCapabilities(props) {
      const providerId = props?.provider?.provider;
      const configured = props?.configured !== false;
      const [snap, setSnap] = React.useState(undefined);
      const [busy, setBusy] = React.useState(false);
      const [status, setStatus] = React.useState(undefined); // { state, text, error }
      const [openModels, setOpenModels] = React.useState(() => new Set());
      const [openCompat, setOpenCompat] = React.useState(false);
      const [openHeaders, setOpenHeaders] = React.useState(false);

      const load = async () => {
        if (providerId === undefined) return;
        setSnap(undefined);
        setStatus(undefined);
        try {
          const response = await fetch(`/model-capabilities?provider=${encodeURIComponent(providerId)}`, { credentials: 'same-origin' });
          const json = await response.json().catch(() => null);
          if (!json || json.ok !== true) {
            setStatus({ state: 'error', text: String((json && json.error) || 'load-failed'), error: true });
            return;
          }
          const profile = {
            hasModelsList: json.hasModelsList === true,
            reasoning: json.reasoning,
            defaultInput: json.defaultInput,
            compat: json.compat,
            baseURL: json.baseURL,
            headers: json.headers,
            sessionAffinity: json.sessionAffinity,
            models: json.models,
          };
          setSnap(loadView(Number(json.revision) || 0, profile));
        } catch (error) {
          setStatus({ state: 'error', text: String(error), error: true });
        }
      };
      React.useEffect(() => { void load(); }, [providerId]);

      const patchModel = (index, fn) => setSnap((current) => current === undefined ? current : ({ ...current, models: current.models.map((model, at) => at === index ? fn(model) : model) }));
      const setEffortValue = (index, level, value) => patchModel(index, (model) => ({ ...model, efforts: { ...model.efforts, [level]: { ...model.efforts[level], value } } }));
      const toggleEffort = (index, level) => patchModel(index, (model) => ({ ...model, efforts: { ...model.efforts, [level]: { ...model.efforts[level], on: !model.efforts[level].on, value: !model.efforts[level].on && level !== 'off' && model.efforts[level].value === '' ? level : model.efforts[level].value } } }));
      const toggleModel = (index) => setOpenModels((current) => {
        const next = new Set(current);
        if (!next.delete(index)) next.add(index);
        return next;
      });

      /* header edits — every touch marks the dict dirty so apply sends it */
      const patchHeaders = (fn) => setSnap((current) => current === undefined ? current : ({ ...current, headersTouched: true, headerRows: fn(current.headerRows) }));
      const editHeaderRow = (index, patch) => patchHeaders((rows) => rows.map((row, at) => at === index ? { ...row, ...patch } : row));
      const removeHeaderRow = (index) => patchHeaders((rows) => rows.filter((row, at) => at !== index));
      const addHeaderRow = () => patchHeaders((rows) => [...rows, { name: '', value: '' }]);

      const apply = async () => {
        if (snap === undefined) return;
        setBusy(true);
        setStatus({ state: 'ongoing', text: '保存中…' });
        try {
          const compat = snap.compat;
          const compatPatch = {};
          for (const key of ['supportsDeveloperRole', 'supportsReasoningEffort']) {
            if (compat[key] === true || compat[key] === false) compatPatch[key] = compat[key];
            else compatPatch[key] = 'unset';
          }
          for (const key of ['maxTokensField', 'thinkingFormat']) {
            if (typeof compat[key] === 'string' && compat[key] !== 'unset') compatPatch[key] = compat[key];
            else compatPatch[key] = 'unset';
          }
          const response = await fetch('/model-capabilities', {
            method: 'POST',
            credentials: 'same-origin',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({
              provider: providerId,
              revision: snap.revision,
              reasoning: snap.reasoning === 'unset' ? null : snap.reasoning,
              defaultInput: snap.defaultInput.length === 0 ? null : snap.defaultInput,
              // Full-replace semantics on the Host; undefined (dropped by
              // JSON.stringify) leaves the stored dict untouched.
              headers: snap.headersTouched ? headersPayload(snap.headerRows) : undefined,
              compatPatch,
              models: snap.models.map((model) => ({
                id: model.id,
                input: model.input.length === 0 ? null : model.input,
                reasoningEfforts: effortPayload(model),
              })),
            }),
          });
          const json = await response.json().catch(() => null);
          if (json && json.ok === true) {
            // Refresh first (load clears status), then surface success so the
            // write result stays visible instead of being wiped by the reload.
            await load();
            setStatus({ state: 'done', text: '已写入 llm-pi-ai · settings.yaml' });
          } else if (json && json.error === 'conflict') {
            // Refresh the view revision so a plain retry can succeed; keep the
            // conflict copy visible.
            await load();
            setStatus({ state: 'error', text: 'conflict — 设置已被其他编辑修改（revision 过期），视图已刷新，可直接重试', error: true });
          } else {
            setStatus({ state: 'error', text: String((json && json.error) || 'write-failed'), error: true });
          }
        } catch (error) {
          setStatus({ state: 'error', text: String(error), error: true });
        } finally {
          setBusy(false);
        }
      };

      /* ── render ── */
      const parts = [];
      parts.push(e('div', { key: 'head', className: 'mc-title' },
        IconThink === undefined ? null : e(IconThink, { size: 16 }),
        '模型能力',
        e('span', { className: 'mc-route' }, providerId ?? ''),
        configured ? null : e(Pill, null, '草稿 — 官方卡片保存后生效')));

      if (snap === undefined) {
        parts.push(e('div', { key: 'loading', className: 'mc-actions' },
          e(Status, { state: 'ongoing', text: status === undefined ? '加载中…' : '' }),
          status !== undefined && status.text !== '' ? e('span', { key: 'e', className: 'mc-status mc-statusError' }, status.text) : null,
          e(Button, { key: 'r', variant: 'outline', size: 'sm', onClick: () => { void load(); } }, '重试')));
        return e('div', { className: 'mc' }, ...parts);
      }
      if (!snap.hasModelsList) {
        parts.push(e('p', { key: 'no-models', className: 'mc-note' },
          '该提供方使用内置目录模型，无模型清单可配置。先在官方编辑器中自定义模型目录，再回到这里设置能力。'));
        return e('div', { className: 'mc' }, ...parts);
      }

      /* header rows + derived state */
      const opencodeDetected = isOpencodeRoute(providerId, snap.baseURL);
      const headerError = headerRowsError(snap.headerRows);

      /* provider-level */
      parts.push(e('div', { key: 'provider', className: 'mc-section' },
        e('div', { className: 'mc-sectionTitle' }, '提供方默认值'),
        e('div', { className: 'mc-row' },
          e('span', { className: 'mc-label' }, '默认思考强度'),
          e(Select, {
            value: snap.reasoning,
            options: [{ value: 'unset', label: '未设置' }].concat(LEVELS.map((level) => ({ value: level, label: `${LEVEL_LABEL[level]} (${level})` }))),
            onChange: (value) => setSnap((current) => current === undefined ? current : ({ ...current, reasoning: value })),
          })),
        e('div', { className: 'mc-row' },
          e('span', { className: 'mc-label' }, '默认模态'),
          e(Segmented, {
            value: snap.defaultInput.includes('image') ? 'both' : snap.defaultInput.includes('text') ? 'text' : 'none',
            options: [{ value: 'none', label: '未设置（默认文本）' }, { value: 'text', label: '仅文本' }, { value: 'both', label: '文本+图像' }],
            onChange: (value) => setSnap((current) => current === undefined ? current : ({ ...current, defaultInput: value === 'both' ? ['text', 'image'] : value === 'text' ? ['text'] : [] })),
          }))));

      /* compat (collapsed by default) */
      const compat = snap.compat;
      parts.push(e('div', { key: 'compat' },
        e(DisclosureRow, {
          title: '兼容设置 · 网关 400 修复',
          open: openCompat,
          expandable: true,
          expandOnRowClick: true,
          keepContentWhenOpen: true,
          onToggle: () => setOpenCompat((value) => !value),
          collapsedContent: e('span', { className: 'mc-note' }, 'developer 角色 / reasoning_effort / 输出上限字段 / 思考格式'),
          children: e('div', { className: 'mc-modelBody' },
            e('div', { className: 'mc-row' },
              e('span', { className: 'mc-label' }, 'developer 角色'),
              e(Select, {
                value: String(compat.supportsDeveloperRole),
                options: [{ value: 'unset', label: '未设置' }, { value: 'true', label: '支持' }, { value: 'false', label: '不支持（用 system）' }],
                onChange: (value) => setSnap((current) => current === undefined ? current : ({ ...current, compat: { ...current.compat, supportsDeveloperRole: value === 'unset' ? 'unset' : value === 'true' } })),
              }),
              e('span', { className: 'mc-label' }, 'reasoning_effort'),
              e(Select, {
                value: String(compat.supportsReasoningEffort),
                options: [{ value: 'unset', label: '未设置' }, { value: 'true', label: '支持' }, { value: 'false', label: '不支持' }],
                onChange: (value) => setSnap((current) => current === undefined ? current : ({ ...current, compat: { ...current.compat, supportsReasoningEffort: value === 'unset' ? 'unset' : value === 'true' } })),
              })),
            e('div', { className: 'mc-row' },
              e('span', { className: 'mc-label' }, '输出上限字段'),
              e(Select, {
                value: compat.maxTokensField,
                options: [{ value: 'unset', label: '未设置' }, { value: 'max_completion_tokens', label: 'max_completion_tokens' }, { value: 'max_tokens', label: 'max_tokens' }],
                onChange: (value) => setSnap((current) => current === undefined ? current : ({ ...current, compat: { ...current.compat, maxTokensField: value } })),
              }),
              e('span', { className: 'mc-label' }, '思考格式'),
              e(Select, {
                value: compat.thinkingFormat,
                options: [{ value: 'unset', label: '未设置' }].concat(FORMATS.map((format) => ({ value: format, label: format }))),
                onChange: (value) => setSnap((current) => current === undefined ? current : ({ ...current, compat: { ...current.compat, thinkingFormat: value } })),
              }))),
        })));

      /* request headers (collapsed by default) — the opencode Go session-affinity row first */
      parts.push(e('div', { key: 'headers' },
        e(DisclosureRow, {
          title: '请求头',
          open: openHeaders,
          expandable: true,
          expandOnRowClick: true,
          keepContentWhenOpen: true,
          onToggle: () => setOpenHeaders((value) => !value),
          collapsedContent: e('span', { className: 'mc-note' },
            snap.headerRows.length === 0 ? '未设置' : `已设置 ${snap.headerRows.length} 个（${snap.headerRows.map((row) => row.name || '?').join(', ')}）`),
          children: e('div', { className: 'mc-modelBody' },
            snap.sessionAffinity === 'per-session' ? e('div', { className: 'mc-note' },
              'opencode 官方要求（opencode.ai/docs/go）：Go 套餐请求应携带 x-opencode-session（会话亲和路由、优化 prompt caching）。本网关已匹配：不自建该头时，插件在 wire 层自动注入——每个 DSH 会话一个 SHA-256 稳定标识；也可在下方自建 x-opencode-session 行，值填 {{session}} 即由 DSH 每次请求自动填入该会话标识（推荐），填固定值则原样发送——自建值优先于自动注入。DSH 已发送真实归属 User-Agent，无需伪装 opencode 客户端。')
              : opencodeDetected ? e('div', { className: 'mc-note' },
                'opencode 官方要求（opencode.ai/docs/go）：Go 套餐请求应携带 x-opencode-session。本网关未命中默认匹配（opencode.ai）：在下方自建 x-opencode-session 行，值填 {{session}}——DSH 会在每次请求时自动填入当前会话的 SHA-256 稳定标识（填固定值则原样发送）；也可在 profile 层按行 config 扩展 hosts 启用自动注入。DSH 已发送真实归属 User-Agent，无需伪装 opencode 客户端。')
              : null,
            snap.headerRows.map((row, index) => e('div', { className: 'mc-row', key: `hdr-${index}` },
              e(Input, { className: 'mc-headerName', value: row.name, placeholder: 'x-custom-header', onChange: (event) => editHeaderRow(index, { name: event.target.value }) }),
              e(Input, { className: 'mc-headerValue', value: row.value, placeholder: '值；{{session}} = 会话 SHA-256', onChange: (event) => editHeaderRow(index, { value: event.target.value }) }),
              e(Button, { variant: 'outline', size: 'sm', onClick: () => removeHeaderRow(index) }, '删除'))),
            e('div', { className: 'mc-row' },
              e(Button, { variant: 'outline', size: 'sm', onClick: addHeaderRow }, '添加请求头'),
              e('span', { className: 'mc-note' }, '按需手动添加；值支持 {{session}} 占位——每次请求自动替换为当前 DSH 会话的 SHA-256 标识；名称写入时统一为小写；user-agent 不可设置（DSH 归属头占用）'))),
        })));

      /* models */
      parts.push(e('div', { key: 'models', className: 'mc-section' },
        e('div', { className: 'mc-sectionTitle' }, `模型（${snap.models.length}）— 逐行覆盖提供方默认值`),
        snap.models.map((model, index) => e(DisclosureRow, {
          key: `model-${index}`,
          title: model.id,
          open: openModels.has(index),
          expandable: true,
          expandOnRowClick: true,
          keepContentWhenOpen: true,
          onToggle: () => toggleModel(index),
          titleClassName: 'mc-modelTitle',
          collapsedContent: e('span', { className: 'mc-modelSummary' },
            e(Pill, null, MODALITY_LABEL[modalityOf(model)]),
            e(Pill, null, KIND_LABEL[model.kind] ?? model.kind)),
          children: e('div', { className: 'mc-modelBody' },
            e('div', { className: 'mc-row' },
              e('span', { className: 'mc-label' }, '模态'),
              e(Segmented, {
                value: modalityOf(model),
                options: [{ value: 'inherit', label: '继承' }, { value: 'text', label: '仅文本' }, { value: 'both', label: '文本+图像' }],
                onChange: (value) => patchModel(index, (m) => ({ ...m, input: value === 'both' ? ['text', 'image'] : value === 'text' ? ['text'] : [] })),
              })),
            e('div', { className: 'mc-row' },
              e('span', { className: 'mc-label' }, '思考强度'),
              e(Segmented, {
                value: model.kind,
                options: [{ value: 'unset', label: '未设置' }, { value: 'false', label: '禁用' }, { value: 'preset', label: '标准档位' }, { value: 'custom', label: '自定义' }],
                onChange: (value) => patchModel(index, (m) => ({ ...m, kind: value })),
              })),
            model.kind === 'custom' ? e('div', { className: 'mc-grid' },
              LEVELS.map((level) => e('span', { className: 'mc-effortRow', key: level },
                e(Pill, {
                  active: model.efforts[level].on,
                  onClick: () => toggleEffort(index, level),
                }, LEVEL_LABEL[level]),
                e(Input, {
                  className: 'mc-effortInput',
                  value: model.efforts[level].value,
                  placeholder: level === 'off' ? '留空=不发送' : '线上值, 如 ultra',
                  disabled: !model.efforts[level].on,
                  onChange: (event) => setEffortValue(index, level, event.target.value),
                })),
              )) : null),
        }))));

      const invalid = snap.models.some((model) => model.kind === 'custom' && !effortValid(model)) || headerError !== null;
      parts.push(e('div', { key: 'actions', className: 'mc-actions' },
        e(Button, {
          variant: 'primary',
          size: 'sm',
          disabled: busy || invalid || snap.models.length === 0 || snap.models.some((model) => model.id.trim() === ''),
          onClick: () => { void apply(); },
        }, busy ? '保存中…' : '应用能力配置'),
        invalid ? e('span', { className: 'mc-status mc-statusError' }, headerError !== null ? headerError : '自定义档位有勾选但线上值为空') : null,
        status !== undefined ? e(Status, { state: status.state, text: status.text, error: status.error }) : null));

      return e('div', { className: 'mc' }, ...parts);
    }

    const inject = ['slots'];

    function apply(ctx) {
      const style = mountStyles();
      ctx.effect(() => {
        return () => {
          if (style !== null && style.parentNode !== null) style.remove();
        };
      });
      ctx.slots.inject('settings.models.provider-card', () => ctx.slots.register(
        { name: 'settings.models.provider-card', key: 'llm-pi-ai' },
        (props) => e(ModelCapabilities, props),
      ));
    }

    exports.inject = inject;
    exports.apply = apply;
    return module.exports;
  },
});
