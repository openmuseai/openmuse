/** True unless the env var is an explicit off token. */
export const envFlagEnabled = (name: string, defaultOn = true): boolean => {
  const raw = process.env[name]?.trim().toLowerCase();
  if (raw === undefined || raw.length === 0) return defaultOn;
  return raw !== "0" && raw !== "false" && raw !== "off";
};

export const injectAtHead = (html: string, snippet: string): string => {
  const head = html.indexOf("<head>");
  if (head === -1) return `${snippet}${html}`;
  return `${html.slice(0, head + 6)}${snippet}${html.slice(head + 6)}`;
};

export const assertSafeScript = (body: string): void => {
  if (body.includes("<") || body.toLowerCase().includes("</script")) {
    throw new Error("injected script must not contain raw < or </script");
  }
};
