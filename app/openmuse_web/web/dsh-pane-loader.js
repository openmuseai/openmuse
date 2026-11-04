let modulePromise

function paneModule() {
  return modulePromise ??= import('./dsh-pane.js?v=2f1fac19d9c20bd4')
}

globalThis.openMuseDshMount = async (container, path) => {
  const module = await paneModule()
  return module.mount(container, path)
}

globalThis.openMuseDshUnmount = async (container) => {
  if (modulePromise) return (await modulePromise).unmount(container)
}
