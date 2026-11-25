import 'dart:convert';

/// Escaping is specific to the host-owned template location. User documents
/// never pass through this helper or through a text replacement pass.
String htmlMessageText(String value) => const HtmlEscape().convert(value);

String htmlMessageScript(Map<String, Object?> namespace) {
  final encoded = jsonEncode(namespace)
      .replaceAll('<', r'\u003c')
      .replaceAll('>', r'\u003e')
      .replaceAll('&', r'\u0026')
      .replaceAll('\u2028', r'\u2028')
      .replaceAll('\u2029', r'\u2029');
  return '<script>window.openMuseMessageData=$encoded;</script><script>$htmlMessageRuntime</script>';
}

const htmlMessageRuntime = r'''
(() => {
  let data = window.openMuseMessageData;
  function validate(nodes, args) {
    for (const node of nodes) {
      if (['arg', 'plural', 'select'].includes(node[0])) {
        if (!Object.hasOwn(args, node[1])) throw new Error('Missing message argument');
        const value = args[node[1]];
        if ((node[0] === 'plural' || node[2] === 'number') && (typeof value !== 'number' || !Number.isFinite(value))) throw new Error('Expected finite number');
        if (node[0] === 'select' && value == null) throw new Error('Null select argument');
      }
      if (node[0] === 'plural' || node[0] === 'select') Object.values(node[2]).forEach(arm => validate(arm, args));
    }
  }
  function render(nodes, args, count) {
    let out = '';
    const number = value => new Intl.NumberFormat(data.locale).format(value);
    for (const node of nodes) {
      if (node[0] === 'text') out += node[1];
      else if (node[0] === 'arg') out += node[2] === 'number' ? number(args[node[1]]) : String(args[node[1]]);
      else if (node[0] === 'hash') out += count === undefined ? '#' : number(count);
      else {
        const value = args[node[1]], arms = node[2];
        const key = node[0] === 'plural'
          ? (Object.hasOwn(arms, '=' + value) ? '=' + value : new Intl.PluralRules(data.locale).select(value))
          : String(value);
        out += render(arms[key] || arms.other, args, node[0] === 'plural' ? value : count);
      }
    }
    return out;
  }
  const api = window.openMuseMessages = {
    format(id, args = {}) { const nodes = data.messages[id]; if (!nodes) throw new Error('Unknown message'); validate(nodes, args); return render(nodes, args); },
    set(element, id, args = {}) {
      element.dataset.message = id;
      element.dataset.messageArgs = JSON.stringify(args);
      element.textContent = api.format(id, args);
    },
    update(next) {
      if (next.locale === data.locale && next.revision === data.revision) return;
      data = next;
      document.documentElement.lang = data.locale;
      document.documentElement.dir = data.direction;
      document.querySelectorAll('[data-message]').forEach(el => {
        el.textContent = api.format(el.dataset.message, JSON.parse(el.dataset.messageArgs || '{}'));
      });
      document.querySelectorAll('[data-date]').forEach(el => { el.textContent = new Date(el.dataset.date).toLocaleString(data.locale); });
      document.querySelectorAll('[data-aria-message]').forEach(el => { el.setAttribute('aria-label', api.format(el.dataset.ariaMessage)); });
      document.querySelectorAll('[data-placeholder-message]').forEach(el => {
        el.placeholder = api.format(el.dataset.placeholderMessage);
      });
    },
    bind(fetchData) {
      let pending = false;
      const timer = setInterval(async () => {
        if (pending || document.hidden) return;
        pending = true;
        try { api.update(await fetchData()); } catch (_) {} finally { pending = false; }
      }, 1500);
      window.addEventListener('pagehide', () => clearInterval(timer), {once: true});
    },
  };
})();
''';
