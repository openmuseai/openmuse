/// #if MOBILE
import "./assets/scss/mobile.scss";
import {initKeyboardToolbar} from "./mobile/util/keyboardToolbar";
import {initMobileBars, bindMobileBarsScroll} from "./mobile/util/mobileBars";
import {initEditorName} from "./mobile/util/initFramework";
import {closeModel} from "./mobile/util/closePanel";
import {handleTouchStart, handleTouchMove, handleTouchEnd, handleTouchCancel, handleTouchContextMenu, handleTouchFocusOut, handleTouchSelectionChange, handleTouchEditing} from "./mobile/util/touch";
/// #else
import "./assets/scss/base.scss";
/// #endif
import {Protyle} from "./protyle";
import {Constants} from "./constants";
import {systemConfig} from "./config/systemConfig";
import {Menus} from "./menus";
import {fetchSyncPost} from "./util/fetch";
import {getLocalStorage, applyPlatformClasses} from "./protyle/util/compatibility";
import {addScriptSync} from "./protyle/util/addScript";
import {loadLanguages} from "./boot/loadLanguages";
import {initAssets, loadAssets, setInlineStyle} from "./util/assets";
import {correctHotkey} from "./boot/globalEvent/commonHotkey";
import {initMessage} from "./dialog/message";
import {getEditorSaveFailureVersion, flushPendingEditorSaves} from "./protyle/util/editorSave";
import {addBaseURL} from "./util/pathName";


const send = (method: string, data: unknown = {}) => {
    (window as any).OpenMuseResource?.postMessage(JSON.stringify({method, data}));
};
let editor: Protyle;
let writable = false;
let changed = false;
let hasEdited = false;
let mutationFailed = false;
const pendingMutations = new Set<Promise<void>>();
const failureVersion = getEditorSaveFailureVersion();
const refreshTitle = () => {
    const input = document.getElementById("toolbarName") as HTMLInputElement;
    if (input) {
        const empty = !input.value.trim();
        document.getElementById("mobileTopBar").classList.toggle("fn__none", empty);
        if (empty) document.body.style.setProperty("--mobile-topbar-height", "0px");
        else document.body.style.removeProperty("--mobile-topbar-height");
    }
    document.querySelectorAll<HTMLElement>(".protyle-title").forEach(element => {
        const title = element.querySelector<HTMLElement>(".protyle-title__input");
        element.classList.toggle("openmuse-empty-title", !!title && !title.textContent.trim());
    });
};
const api = async (url: string, data = {}) => {
    const response = await fetchSyncPost(url, data);
    if (response.code !== 0) throw new Error(response.msg || url);
    return response.data;
};

// 只挂载文档编辑器；应用导航、账户、工作区与标签由 OpenMuse 提供。
const boot = async () => {
    // Original startup establishes /assets resolution before Protyle inserts
    // images. A standalone surface lives below /stage/build/resource*/.
    addBaseURL();
    const params = new URLSearchParams(location.search);
    const id = params.get("id");
    const notebook = params.get("notebook");
    const canEdit = params.get("editable") !== "false";
    if (!/^\d{14}-[a-z0-9]{7}$/.test(id || "") || !/^\d{14}-[a-z0-9]{7}$/.test(notebook || "")) {
        throw new Error("Invalid resource document");
    }
    const app = {plugins: [], appId: Constants.SIYUAN_APPID} as any;
    (window as any).siyuan = {
        zIndex: 10, isReady: false, notebooks: [], reqIds: {}, backStack: [],
        layout: {}, dialogs: [], blockPanels: [], closedTabs: [], ctrlIsPressed: false,
        altIsPressed: false, shiftIsPressed: false,
    };
    /// #if MOBILE
    (window.siyuan as any).mobile = {size: {}, docks: {}};
    /// #endif
    const result = await api("/api/system/getConf");
    window.siyuan.config = systemConfig(result.conf, () => structuredClone(Constants.SIYUAN_EMPTY_LAYOUT));
    window.siyuan.config.readonly = !canEdit;
    // Opening a resource in view mode uses SiYuan's own default-read-only
    // policy. Its lock button then performs the original temporary unlock;
    // it must not rely on broadcasting an unchanged persisted document attr.
    window.siyuan.config.editor.readOnly = true;
    window.siyuan.config.appearance.mode = params.get("theme") === "dark" ? 1 : 0;
    Object.assign(window.siyuan.config.appearance, {
        modeOS: false, themeLight: "daylight", themeDark: "midnight",
        icon: "litheness", themeJS: false,
    });
    if (params.get("language")) window.siyuan.config.appearance.lang = params.get("language");
    window.siyuan.user = null;
    window.siyuan.isPublish = false;
    applyPlatformClasses();
    correctHotkey(app);
    await addScriptSync(`${Constants.PROTYLE_CDN}/js/lute/lute.min.js?v=${Constants.SIYUAN_VERSION}`, "protyleLuteScript");
    await addScriptSync(`${Constants.PROTYLE_CDN}/js/protyle-html.js?v=${Constants.SIYUAN_VERSION}`, "protyleWcHtmlScript");
    await new Promise<void>((resolve, reject) => getLocalStorage(resolve, reject));
    await loadLanguages(window.siyuan.config.appearance.lang, Constants.SIYUAN_VERSION, languages => {
        window.siyuan.languages = languages;
    }, error => { throw error; });
    window.siyuan.notebooks = (await api("/api/notebook/lsNotebooks")).notebooks.filter((box: {id: string}) => box.id === notebook);
    window.siyuan.emojis = await api("/api/system/getEmojiConf");
    window.siyuan.menus = new Menus(app);
    initMessage();
    await loadAssets(window.siyuan.config.appearance);
    const themeStyle = document.getElementById("themeDefaultStyle") as HTMLLinkElement;
    if (!themeStyle) throw new Error("Resource editor theme is unavailable");
    if (!themeStyle.sheet) await new Promise<void>((resolve, reject) => {
        const timeout = window.setTimeout(() => reject(new Error("Resource editor theme timed out")), 30000);
        themeStyle.addEventListener("load", () => { clearTimeout(timeout); resolve(); }, {once: true});
        themeStyle.addEventListener("error", () => { clearTimeout(timeout); reject(new Error("Resource editor theme failed to load")); }, {once: true});
    });
    initAssets(true);
    await setInlineStyle();
    /// #if MOBILE
    const saveMobileTitle = initEditorName();
    initKeyboardToolbar();
    initMobileBars();
    document.getElementById("modelClose").addEventListener("click", closeModel);
    document.addEventListener("touchstart", handleTouchStart, false);
    document.addEventListener("touchmove", handleTouchMove, {passive: false});
    document.addEventListener("touchend", handleTouchEnd, false);
    document.addEventListener("touchcancel", handleTouchCancel, false);
    document.addEventListener("contextmenu", handleTouchContextMenu, true);
    document.addEventListener("focusout", handleTouchFocusOut, true);
    document.addEventListener("selectionchange", handleTouchSelectionChange, true);
    document.addEventListener("beforeinput", handleTouchEditing, true);
    document.addEventListener("paste", handleTouchEditing, true);
    /// #endif
    const mount = (block: string, root: string) => {
    let action: TProtyleAction[] = [Constants.CB_GET_ALL];
    /// #if MOBILE
    action = [Constants.CB_GET_HL, Constants.CB_GET_SETID];
    /// #endif
    editor = new Protyle(app, document.getElementById("editor") || document.getElementById("resource-editor"), {
        blockId: block, rootId: root, notebookId: notebook, databaseAttr: true,
        action,
        render: {title: true, background: true, breadcrumb: true, gutter: true, scroll: true,
            /// #if MOBILE
            titleShowTop: true,
            /// #endif
        },
        /// #if MOBILE
        typewriterMode: true,
        /// #endif
        after: instance => {
            editor = instance;
            (window as any).openMuseResourceEditor = instance;
            /// #if MOBILE
            window.siyuan.mobile.editor = instance;
            document.getElementById("toolbarName").classList.remove("fn__hidden");
            document.getElementById("toolbarNameReadonly").classList.remove("fn__hidden");
            bindMobileBarsScroll(instance.protyle.contentElement);
            /// #endif
            refreshTitle();
            instance.disable();
            new MutationObserver(() => {
                const editing = !instance.protyle.disabled;
                if (editing === writable) return;
                writable = editing;
                hasEdited ||= writable;
                send("editing", {editing});
            }).observe(instance.protyle.wysiwyg.element, {attributes: true, attributeFilter: ["data-readonly"]});
            window.siyuan.isReady = true;
            document.body.removeAttribute("aria-busy");
            document.documentElement.dataset.resourceReady = "true";
            send("ready");
        },
    }, {pluginExtensions: false});
    (window as any).openMuseResourceEditor = editor;
    /// #if MOBILE
    window.siyuan.mobile.editor = editor;
    /// #endif
    };
    mount(id, id);
    // 捕获数据库等非文本操作；失败事务由 flushPendingEditorSaves 拒绝保存。
    const originalFetch = window.fetch.bind(window);
    window.fetch = async (...args) => {
        const url = typeof args[0] === "string" ? args[0] : args[0] instanceof Request ? args[0].url : args[0].toString();
        const response = originalFetch(...args);
        if (hasEdited && /\/api\/(transactions|attr\/set|filetree\/(create|rename|move|remove|duplicate)|asset\/upload|av\/(set|add|remove|insert|update|sort|duplicate))/.test(url)) {
            changed = true;
            send("dirty");
            const confirmed = response.then(async (result: Response) => {
                const value = await result.clone().json();
                if (!result.ok || value.code !== 0) throw new Error(value.msg || url);
            });
            pendingMutations.add(confirmed);
            void confirmed.catch(() => { mutationFailed = true; }).finally(() => pendingMutations.delete(confirmed));
        }
        return response;
    };
    document.addEventListener("input", () => {
        if (writable) { changed = true; send("dirty"); }
    }, true);
    document.addEventListener("keydown", event => {
        window.siyuan.ctrlIsPressed = event.ctrlKey || event.metaKey;
        window.siyuan.shiftIsPressed = event.shiftKey;
        window.siyuan.altIsPressed = event.altKey;
        if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "s") {
            event.preventDefault(); send("save");
        }
    });
    const flushEditor = async (preserveEditing = false) => {
        if (!preserveEditing) {
            writable = false;
            (document.activeElement as HTMLElement)?.blur();
        }
        /// #if MOBILE
        await saveMobileTitle();
        /// #endif
        editor.protyle.title?.flushPendingInput();
        await editor.protyle.wysiwyg.flushPendingInput();
        if (!preserveEditing) editor.disable();
        await flushPendingEditorSaves(failureVersion);
        while (pendingMutations.size) await Promise.all(pendingMutations);
        if (mutationFailed) throw new Error("A resource mutation was not confirmed");
    };
    let navigating = false;
    (window as any).openMuseResource = {
        refreshTitle,
        openLink(uri: string, previewable = false) {
            // Native image/PDF previews remain inside the original editor.
            // External documents leave through the plugin's resource boundary.
            if (previewable || /^siyuan:\/\//i.test(uri)) return false;
            send("openLink", {uri});
            return true;
        },
        async openBlock(block: string) {
            if (navigating || !/^\d{14}-[a-z0-9]{7}$/.test(block)) return;
            navigating = true;
            try {
                const info = await api("/api/block/getBlockInfo", {id: block, notebook});
                if (info.box !== notebook) { send("referenceUnavailable"); return; }
                await flushEditor();
                editor.destroy();
                mount(block, info.rootID);
            } catch (error) { send("referenceUnavailable", {message: String(error)}); }
            finally { navigating = false; }
        },
        setEditing(value: boolean) {
            writable = value === true && canEdit;
            hasEdited ||= writable;
            if (writable) editor.enable(); else editor.disable();
        },
        async flush(requestId: string, preserveEditing = false) {
            try {
                await flushEditor(preserveEditing);
                send("flushed", {requestId, changed});
            } catch (error) { send("flushError", {requestId, message: String(error)}); }
        },
    };
};
void boot().catch(error => {
    document.body.removeAttribute("aria-busy");
    send("error", {message: String(error)});
});
