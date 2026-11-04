/** Plugin-owned browser/native boundary; never a Muse Host command. */
export async function saveClientExport(path: string): Promise<{
    status: "success" | "canceled" | "error"; name?: string; message?: string;
} | undefined> {
    const bridge = (window as any).SiYuanClient;
    if (!bridge) return undefined;
    const url = new URL(path, location.origin);
    if (url.origin !== location.origin || !/^\/(export|assets)\//.test(url.pathname)) {
        throw Error("Export must belong to the local notebook");
    }
    const response = await fetch(url, {credentials: "same-origin"});
    if (!response.ok) throw Error("Export is unavailable");
    const blob = await response.blob();
    if (blob.size > 64 * 1024 * 1024) throw Error("Export exceeds 64 MiB");
    const encoded = await new Promise<string>((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(String(reader.result).split(",")[1]);
        reader.onerror = reject;
        reader.readAsDataURL(blob);
    });
    const id = crypto.randomUUID();
    return new Promise((resolve) => {
        const handler = (event: Event) => {
            const detail = (event as CustomEvent).detail;
            if (detail?.id !== id) return;
            window.removeEventListener("muse-client-response", handler);
            resolve(detail.result);
        };
        window.addEventListener("muse-client-response", handler);
        bridge.postMessage(JSON.stringify({id, method: "saveExport", name: decodeURIComponent(url.pathname.split("/").pop()!), blob: encoded}));
    });
}
