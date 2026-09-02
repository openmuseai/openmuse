export type DesktopCarrierKind = "unix_domain_socket" | "windows_named_pipe";

export interface DesktopCarrierEndpoint {
  readonly kind: DesktopCarrierKind;
  readonly address: string;
  /** How the Host obtains peer identity independently of the Bridge envelope. */
  readonly peerIdentitySource: "peer_credentials" | "named_pipe_client_token";
}

export interface DesktopCarrierConnection {
  readonly endpoint: DesktopCarrierEndpoint;
  readonly readable: AsyncIterable<Uint8Array>;
  write(frame: Uint8Array): Promise<void>;
  close(reason?: unknown): Promise<void>;
}

/** Platform carrier seam; UDS and named-pipe implementations must preserve identical framing. */
export interface DesktopCarrier {
  connect(endpoint: DesktopCarrierEndpoint, signal: AbortSignal): Promise<DesktopCarrierConnection>;
}

export const unixDomainSocketEndpoint = (address: string): DesktopCarrierEndpoint => {
  if (!address.startsWith("/") || address.includes("\0")) {
    throw new TypeError("UDS endpoint must be an absolute path without NUL bytes");
  }
  return Object.freeze({
    kind: "unix_domain_socket",
    address,
    peerIdentitySource: "peer_credentials"
  });
};

export const windowsNamedPipeEndpoint = (address: string): DesktopCarrierEndpoint => {
  if (!address.startsWith("\\\\.\\pipe\\") || address.includes("\0")) {
    throw new TypeError("named-pipe endpoint must use the \\\\.\\pipe\\ namespace");
  }
  return Object.freeze({
    kind: "windows_named_pipe",
    address,
    peerIdentitySource: "named_pipe_client_token"
  });
};
