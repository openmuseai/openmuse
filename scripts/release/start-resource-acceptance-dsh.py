#!/usr/bin/env python3
"""Start an isolated real DSH runtime for resource UI acceptance.

No account/model secrets are embedded in the application or test source.
The private runtime file is for the development lab only, never a release input.
"""
import argparse, http.cookiejar, json, os, pathlib, re, secrets, shutil, subprocess, time, urllib.request, uuid

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--model-input',type=pathlib.Path,required=True)
parser.add_argument('--state',type=pathlib.Path,required=True)
parser.add_argument('--model',help='Explicit model selected for this acceptance run')
args=parser.parse_args()
repo=pathlib.Path(__file__).resolve().parents[2]
args.state.mkdir(parents=True,exist_ok=True)
os.chmod(args.state,0o700)
model=json.loads(args.model_input.read_text())
if args.model:model['model']=args.model
closure=repo/'target/dsh-closure-020'
# The isolated runtime must use the same first-party client bridge as the App.
shutil.copy2(repo/'third_party/dsh/plugins/openmuse-dsh-bridge/lib/client.js', closure/'node_modules/openmuse-dsh-bridge/lib/client.js')
bridge=secrets.token_hex(32)
env=dict(os.environ,DSH_HOME=str(args.state/'dsh-home'),OPENMUSE_DSH_BRIDGE_TOKEN=bridge,
         OPENMUSE_RESOURCE_QA_API_KEY=model['apiKey'])
log=args.state/'dsh.log'
with log.open('w') as output:
 process=subprocess.Popen(['node',str(closure/'node_modules/@deepseek-ai/dsh/lib/bin.js'),'web',
   '--patch',str(closure/'node_modules/dsh-model-capabilities/openmuse.patch.yml'),
   '--host','127.0.0.1','--port','0','--no-open'],cwd=closure,env=env,
   stdout=output,stderr=subprocess.STDOUT,start_new_session=True)
(args.state/'dsh.pid').write_text(str(process.pid))
endpoint=None
for _ in range(180):
 match=re.search(r'dsh web: (http://127\.0\.0\.1:\d+/\?[^\s)]+)',log.read_text())
 if match: endpoint=match.group(1);break
 if process.poll() is not None:raise SystemExit('Isolated DSH failed; inspect private acceptance log.')
 time.sleep(.5)
if not endpoint:raise SystemExit('Isolated DSH did not report readiness.')
base=endpoint.split('/?')[0]
jar=http.cookiejar.CookieJar();op=urllib.request.build_opener(urllib.request.ProxyHandler({}),urllib.request.HTTPCookieProcessor(jar))
op.open(endpoint,timeout=20).read()
def rpc(name,body):
 request=urllib.request.Request(base+'/api/'+name,data=json.dumps({'type':'client-request','rpcId':str(uuid.uuid4()),
   'method':name,'payload':{'args':body}}).encode(),headers={'Content-Type':'application/json'})
 value=json.loads(op.open(request,timeout=30).read())['result']
 if not value['ok']:raise RuntimeError('Isolated DSH rejected '+name)
 return value['value']
view=rpc('settings/describe',{})
namespace=next(n for n in view['namespaces'] if n['ns']=='llm-pi-ai')
profile={'api':'openai-completions','baseURL':model['baseURL'],'apiKeyEnv':'OPENMUSE_RESOURCE_QA_API_KEY',
 'headers':{'x-opencode-session':'openmuse-resource-ui-acceptance','User-Agent':'OpenMuse-Resource-Acceptance/1.0'},
 'models':[{'id':model['model'],'name':model['model'],'input':['text'],'contextWindow':131072,'maxTokens':8192}]}
rpc('settings/mutate',{'ns':'llm-pi-ai','ops':[{'op':'set','path':['providers','resource-qa'],'value':profile}],
 'expectedRevision':namespace['revision']})
workspace=args.state/'workspace';workspace.mkdir(exist_ok=True)
created=rpc('workspace/create',{'request':{'path':str(workspace)}})
def native(path,body):
 request=urllib.request.Request(base+'/openmuse-native/v1/'+path,data=json.dumps(body).encode(),
   headers={'Content-Type':'application/json','x-openmuse-bridge-token':bridge})
 return json.loads(op.open(request,timeout=30).read())
session=native('session/create',{'workspaceId':created['workspace']['workspaceId'],'agentPreset':'standard'})
native('session/model',{'sessionId':session['sessionId'],'provider':'resource-qa','model':model['model']})
runtime={'dshEndpoint':endpoint,'bridgeToken':bridge,'account':'resource-ui-acceptance',
 'accessToken':secrets.token_hex(32),'pairingCode':'684291','gatewayPort':13188,'controlPort':13189,
 'provider':'resource-qa','model':model['model'],'workspace':str(workspace)}
private=args.state/'runtime.json';private.write_text(json.dumps(runtime));os.chmod(private,0o600)
print('Isolated real DSH ready; private runtime input saved. Model provider configured.')
