// MCP Apps Views can have an opaque sandbox origin. GenOffice's UI preferences
// use Web Storage; keep those preferences inside this View when it is isolated.
for (const name of ['localStorage', 'sessionStorage'] as const) {
  try { window[name].getItem('__probe') }
  catch {
    const values = new Map<string,string>()
    Object.defineProperty(window,name,{value:{
      get length(){return values.size},key:(i:number)=>Array.from(values.keys())[i]??null,
      getItem:(key:string)=>values.get(String(key))??null,
      setItem:(key:string,value:string)=>{values.set(String(key),String(value))},
      removeItem:(key:string)=>{values.delete(String(key))},clear:()=>values.clear(),
    }})
  }
}

// WebKit's about:blank document may omit randomUUID although getRandomValues
// is available. UUID generation stays local and uses cryptographic randomness.
if (!globalThis.crypto.randomUUID) {
  Object.defineProperty(globalThis.crypto, 'randomUUID', {value: () => {
    const bytes = crypto.getRandomValues(new Uint8Array(16))
    bytes[6] = (bytes[6] & 15) | 64
    bytes[8] = (bytes[8] & 63) | 128
    const hex = Array.from(bytes, b => b.toString(16).padStart(2, '0')).join('')
    return `${hex.slice(0,8)}-${hex.slice(8,12)}-${hex.slice(12,16)}-${hex.slice(16,20)}-${hex.slice(20)}`
  }})
}
