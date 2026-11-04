import 'package:test/test.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
void main(){
  test('numeric private IPv4 and IPv6 ULA share one native transport policy',(){
    for(final host in ['10.0.2.2','172.16.0.1','172.31.255.255','192.168.1.80','fc00::1','fdce:a8c8:f270::2']){
      expect(isPrivateNativeTransportHost(host),isTrue,reason:host);
    }
    for(final host in ['172.32.0.1','192.169.1.1','8.8.8.8','2001:4860:4860::8888','fe80::1','10.999.999.999','fc00::invalid','private.example']){
      expect(isPrivateNativeTransportHost(host),isFalse,reason:host);
    }
  });
}
