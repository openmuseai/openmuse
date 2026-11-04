import 'dart:io';

/// Numeric private transport addresses. Callers must still require an explicit
/// private-network allowance; this function never enables cleartext by itself.
/// IPv6 unique-local addresses include USB device-development tunnels.
bool isPrivateNativeTransportHost(String host) {
  final address=InternetAddress.tryParse(host);
  if(address==null)return false;
  final bytes=address.rawAddress;
  if(address.type==InternetAddressType.IPv6)return (bytes[0]&0xfe)==0xfc;
  return bytes[0]==10 || (bytes[0]==192 && bytes[1]==168) ||
    (bytes[0]==172 && bytes[1]>=16 && bytes[1]<=31);
}
