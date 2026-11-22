import 'dart:async';
import 'dart:convert';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mcp_app_runtime/openmuse_mcp_app_runtime.dart';

void main(){
  const html='<html>reviewed app</html>';
  Map<String,dynamic> definition()=>{'id':'example.app','title':'Example','version':'1.0','html':html,'uiSha256':sha256.convert(utf8.encode(html)).toString(),'uiResourceUri':'ui://example/main'};
  test('paired UI must match digest and cannot select an arbitrary protocol',(){
    expect(()=>RemoteMcpApp.paired(definition:{...definition(),'html':'changed'},scope:()=> 'account',request:(_,__)async=>{}),throwsFormatException);
    expect(()=>RemoteMcpApp.paired(definition:{...definition(),'uiResourceUri':'https://evil.invalid/'},scope:()=> 'account',request:(_,__)async=>{}),throwsFormatException);
  });
  test('paired calls are scoped and cannot bypass the agent visibility boundary',()async{
    var account='first';
    final app=RemoteMcpApp.paired(definition:definition(),scope:()=>account,request:(_,__)async=>{});
    await app.request('tools/list',{});
    await expectLater(app.request('arbitrary',{}),throwsStateError);
    await expectLater(app.request('tools/list',{},agent:true),throwsStateError);
    account='second';await expectLater(app.request('tools/list',{}),throwsStateError);
    account='first';await app.closeSession();await expectLater(app.request('tools/list',{}),throwsStateError);
  });
  test('revoking an in-flight result rejects it rather than exposing stale data',()async{
    final result=Completer<Map<String,Object?>>();
    final app=RemoteMcpApp.paired(definition:definition(),scope:()=> 'account',request:(_,__)=>result.future);
    final pending=app.request('tools/list',{});await app.closeSession();
    final rejected=expectLater(pending,throwsStateError);result.complete({'private':'old result'});await rejected;
  });
}
