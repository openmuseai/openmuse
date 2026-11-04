import createModule from './ksp_product.mjs';

const decoder = new TextDecoder('utf-8', { fatal: true });

class KspProductBridge {
  constructor(module) {
    this.module = module;
    if (module._ksp_product_abi_version() !== 2 ||
        module._ksp_product_edit_abi_version() !== 6) {
      throw new Error('Web product ABI version mismatch');
    }
  }

  get memory() {
    return new DataView(this.module.HEAPU8.buffer);
  }

  allocate(size) {
    const pointer = this.module._malloc(Math.max(size, 1));
    if (!pointer) throw new Error('Web product memory allocation failed');
    return pointer;
  }

  copyIn(bytes) {
    const pointer = this.allocate(bytes.length);
    this.module.HEAPU8.set(bytes, pointer);
    return pointer;
  }

  readBytes(pointer, length) {
    if (!pointer || !length || length > 512 * 1024 * 1024) {
      throw new Error('Web product returned invalid PDF bytes');
    }
    return this.module.HEAPU8.slice(pointer, pointer + length);
  }

  check(code, operation) {
    if (code < 0) throw new Error(`${operation}: product error ${code}`);
    return code;
  }

  pageCount(bytes) {
    const input = this.copyIn(bytes);
    try {
      return this.check(
        this.module._ksp_product_page_count(input, BigInt(bytes.length)),
        '读取页数',
      );
    } finally {
      this.module._free(input);
    }
  }

  compose(sources) {
    if (!sources.length) throw new Error('没有待组合的 PDF');
    const selection = this.allocate(sources.length * 24);
    const output = this.allocate(16);
    const error = this.allocate(256);
    const inputs = [];
    try {
      this.module.HEAPU8.fill(0, output, output + 16);
      this.module.HEAPU8.fill(0, error, error + 256);
      sources.forEach((source, index) => {
        if (!source.bytes.length || !source.pageIndices.length) {
          throw new Error('PDF 字节或页序为空');
        }
        const pdf = this.copyIn(source.bytes);
        const pages = this.allocate(source.pageIndices.length * 4);
        inputs.push(pdf, pages);
        let view = this.memory;
        source.pageIndices.forEach((page, offset) => {
          view.setInt32(pages + offset * 4, page, true);
        });
        view = this.memory;
        const base = selection + index * 24;
        view.setUint32(base, pdf, true);
        view.setBigUint64(base + 8, BigInt(source.bytes.length), true);
        view.setUint32(base + 16, pages, true);
        view.setUint32(base + 20, source.pageIndices.length, true);
      });
      const code = this.module._ksp_product_compose_pages(
        selection, sources.length, output, error, 256,
      );
      if (code !== 0) {
        const end = this.module.HEAPU8.indexOf(0, error);
        const message = decoder.decode(this.module.HEAPU8.slice(error, end));
        throw new Error(`页面组合失败：${message} (${code})`);
      }
      const view = this.memory;
      return this.readBytes(
        view.getUint32(output, true),
        Number(view.getBigUint64(output + 8, true)),
      );
    } finally {
      this.module._ksp_product_free_bytes(output);
      for (const pointer of inputs) this.module._free(pointer);
      for (const pointer of [error, output, selection]) this.module._free(pointer);
    }
  }

  composeFlat(pdfBytes, pageIndices) {
    return this.compose(pdfBytes.map((bytes, index) => ({
      bytes,
      pageIndices: pageIndices[index],
    })));
  }

  open(bytes) {
    const input = this.copyIn(bytes);
    const error = this.allocate(4);
    try {
      this.memory.setInt32(error, 0, true);
      const session = this.module._ksp_product_edit_open_bytes(
        input, BigInt(bytes.length), error,
      );
      if (!session) {
        throw new Error(`打开 PDF 失败：${this.memory.getInt32(error, true)}`);
      }
      return session;
    } finally {
      this.module._free(error);
      this.module._free(input);
    }
  }

  close(session) {
    if (session) this.module._ksp_product_edit_close(session);
  }

  editPageCount(session) {
    return this.check(this.module._ksp_product_edit_page_count(session), '读取页数');
  }

  loadPage(session, pageIndex) {
    this.check(this.module._ksp_product_edit_load_page(session, pageIndex), '加载编辑页');
  }

  domains(session) {
    const count = this.check(this.module._ksp_product_edit_domain_count(session), '读取编辑域');
    const box = this.allocate(16);
    try {
      return Array.from({ length: count }, (_, index) => {
        this.check(this.module._ksp_product_edit_domain_rect(
          session, index, box, box + 4, box + 8, box + 12,
        ), '读取编辑域坐标');
        const view = this.memory;
        return {
          index,
          left: view.getFloat32(box, true),
          bottom: view.getFloat32(box + 4, true),
          right: view.getFloat32(box + 8, true),
          top: view.getFloat32(box + 12, true),
        };
      });
    } finally {
      this.module._free(box);
    }
  }

  readUtf16(call) {
    const length = this.check(call(0, 0), '读取文字');
    if (!length) return '';
    const output = this.allocate(length * 2);
    try {
      const copied = this.check(call(output, length), '读取文字');
      const view = this.memory;
      const parts = [];
      for (let offset = 0; offset < copied; offset += 8192) {
        const chars = [];
        for (let index = offset; index < Math.min(copied, offset + 8192); index++) {
          chars.push(view.getUint16(output + index * 2, true));
        }
        parts.push(String.fromCharCode(...chars));
      }
      return parts.join('');
    } finally {
      this.module._free(output);
    }
  }

  domainText(session, index) {
    return this.readUtf16((output, capacity) =>
      this.module._ksp_product_edit_domain_text(session, index, output, capacity));
  }

  focusedText(session) {
    return this.readUtf16((output, capacity) =>
      this.module._ksp_product_edit_focused_text(session, output, capacity));
  }

  readFocus(session) {
    const values = this.allocate(24);
    try {
      const text = this.focusedText(session);
      this.check(this.module._ksp_product_edit_selection(session, values, values + 4), '读取选区');
      const caretCode = this.module._ksp_product_edit_caret(
        session, values + 8, values + 12, values + 16, values + 20,
      );
      const view = this.memory;
      return {
        text,
        start: view.getInt32(values, true),
        end: view.getInt32(values + 4, true),
        caret: caretCode < 0 ? null : {
          bottomX: view.getFloat32(values + 8, true),
          bottomY: view.getFloat32(values + 12, true),
          topX: view.getFloat32(values + 16, true),
          topY: view.getFloat32(values + 20, true),
        },
      };
    } finally {
      this.module._free(values);
    }
  }

  focusAt(session, x, y) {
    const hit = this.check(this.module._ksp_product_edit_focus_at(session, x, y), '定位文字');
    if (!hit) throw new Error('点击位置没有可编辑文字');
    return this.readFocus(session);
  }

  setSelection(session, start, end) {
    this.check(this.module._ksp_product_edit_set_selection(session, start, end), '移动选区');
    return this.readFocus(session);
  }

  withUtf16(text, action) {
    const pointer = this.allocate(text.length * 2);
    try {
      const view = this.memory;
      for (let index = 0; index < text.length; index++) {
        view.setUint16(pointer + index * 2, text.charCodeAt(index), true);
      }
      return action(text.length ? pointer : 0, text.length);
    } finally {
      this.module._free(pointer);
    }
  }

  replaceDomain(session, index, text) {
    this.withUtf16(text, (pointer, length) => this.check(
      this.module._ksp_product_edit_replace_domain(session, index, pointer, length),
      '替换编辑域',
    ));
  }

  replaceRange(session, start, end, text) {
    this.check(this.module._ksp_product_edit_set_selection(session, start, end), '设置选区');
    this.withUtf16(text, (pointer, length) => this.check(
      this.module._ksp_product_edit_replace_selection(session, pointer, length),
      '替换选区',
    ));
    return this.readFocus(session);
  }

  undo(session) {
    this.check(this.module._ksp_product_edit_undo(session), '撤销');
  }

  redo(session) {
    this.check(this.module._ksp_product_edit_redo(session), '重做');
  }

  canUndo(session) {
    return !!session && this.module._ksp_product_edit_can_undo(session) > 0;
  }

  canRedo(session) {
    return !!session && this.module._ksp_product_edit_can_redo(session) > 0;
  }

  pageSize(session, pageIndex) {
    const values = this.allocate(8);
    try {
      this.check(this.module._ksp_product_edit_page_size(
        session, pageIndex, values, values + 4,
      ), '读取页面尺寸');
      const view = this.memory;
      return { width: view.getFloat32(values, true), height: view.getFloat32(values + 4, true) };
    } finally {
      this.module._free(values);
    }
  }

  render(session, pageIndex, x, y, width, height, fullWidth, fullHeight, activePageOnly = false) {
    const pixels = width * height;
    if (width <= 0 || height <= 0 || pixels > 16 * 1024 * 1024) {
      throw new Error('渲染尺寸无效或超出限制');
    }
    const capacity = pixels * 4;
    const output = this.allocate(capacity);
    try {
      const written = activePageOnly
        ? this.module._ksp_product_edit_render_page(session, width, height, output, capacity)
        : x === 0 && y === 0 && width === fullWidth && height === fullHeight
          ? this.module._ksp_product_edit_render_page_at(session, pageIndex, width, height, output, capacity)
          : this.module._ksp_product_edit_render_page_region(
            session, pageIndex, x, y, width, height, fullWidth, fullHeight,
            output, capacity,
          );
      if (this.check(written, '渲染页面') !== capacity) {
        throw new Error('渲染字节长度异常');
      }
      return this.module.HEAPU8.slice(output, output + capacity);
    } finally {
      this.module._free(output);
    }
  }

  pageText(session, pageIndex) {
    const output = this.allocate(8);
    this.module.HEAPU8.fill(0, output, output + 8);
    try {
      this.check(this.module._ksp_product_edit_read_page_text(session, pageIndex, output), '读取页面文字');
      const view = this.memory;
      const pointer = view.getUint32(output, true);
      const count = view.getInt32(output + 4, true);
      if (count < 0 || count > 1000000 || (count && !pointer)) {
        throw new Error('页面文字结果无效');
      }
      const units = new Uint16Array(count);
      const boxes = new Float32Array(count * 4);
      for (let index = 0; index < count; index++) {
        const base = pointer + index * 20;
        units[index] = view.getUint32(base, true);
        for (let side = 0; side < 4; side++) {
          boxes[index * 4 + side] = view.getFloat32(base + 4 + side * 4, true);
        }
      }
      return { units, boxes };
    } finally {
      this.module._ksp_product_edit_free_page_text(output);
      this.module._free(output);
    }
  }

  pageLinks(session, pageIndex) {
    const output = this.allocate(16);
    this.module.HEAPU8.fill(0, output, output + 16);
    try {
      this.check(this.module._ksp_product_edit_read_page_links(session, pageIndex, output), '读取页面链接');
      const view = this.memory;
      const links = view.getUint32(output, true);
      const count = view.getInt32(output + 4, true);
      const uriData = view.getUint32(output + 8, true);
      const uriSize = view.getUint32(output + 12, true);
      if (count < 0 || count > 4096 || uriSize > 1024 * 1024 ||
          (count && !links) || (uriSize && !uriData)) {
        throw new Error('页面链接结果无效');
      }
      return Array.from({ length: count }, (_, index) => {
        const base = links + index * 32;
        const kind = view.getInt32(base + 16, true);
        const uriOffset = view.getUint32(base + 24, true);
        const uriLength = view.getUint32(base + 28, true);
        if (uriOffset > uriSize || uriLength > uriSize - uriOffset) {
          throw new Error('页面链接 URI 范围无效');
        }
        let uri = null;
        if (kind === 1) {
          try {
            uri = decoder.decode(this.module.HEAPU8.slice(
              uriData + uriOffset, uriData + uriOffset + uriLength,
            ));
          } catch (_) {
            uri = null;
          }
        }
        return {
          left: view.getFloat32(base, true),
          bottom: view.getFloat32(base + 4, true),
          right: view.getFloat32(base + 8, true),
          top: view.getFloat32(base + 12, true),
          kind: kind === 1 && uri === null ? 0 : kind,
          pageIndex: view.getInt32(base + 20, true),
          uri,
        };
      });
    } finally {
      this.module._ksp_product_edit_free_page_links(output);
      this.module._free(output);
    }
  }

  snapshot(session) {
    const output = this.allocate(16);
    this.module.HEAPU8.fill(0, output, output + 16);
    try {
      this.check(this.module._ksp_product_edit_export_bytes(session, output), '导出 PDF');
      const view = this.memory;
      return this.readBytes(
        view.getUint32(output, true),
        Number(view.getBigUint64(output + 8, true)),
      );
    } finally {
      this.module._ksp_product_free_bytes(output);
      this.module._free(output);
    }
  }
}

export async function createBridge() {
  const module = await createModule({
    locateFile: (file) => new URL(file, import.meta.url).href,
  });
  return new KspProductBridge(module);
}

globalThis.kspProductBridgeReady = createBridge();
