/** Session memory for Web Tx (device token + document focus). Desktop UDS does not use this. */

export interface DeviceAuth {
  readonly token: string;
  readonly deviceId: string;
}

export interface DocumentFocus {
  readonly workspaceId: string;
  readonly viewId: string;
}

let lastDeviceAuth: DeviceAuth | undefined;
let lastDocumentFocus: DocumentFocus | undefined;

export const getLastDeviceAuth = (): DeviceAuth | undefined => lastDeviceAuth;
export const getLastDocumentFocus = (): DocumentFocus | undefined => lastDocumentFocus;

export const setLastDeviceAuth = (value: DeviceAuth | undefined): void => {
  lastDeviceAuth = value;
};

export const setLastDocumentFocus = (value: DocumentFocus | undefined): void => {
  lastDocumentFocus = value;
};

export const resetSession = (): void => {
  lastDeviceAuth = undefined;
  lastDocumentFocus = undefined;
};
