"""Test the fixed Store composition against hostile user overrides, without a model request."""
import json, os, re, selectors, subprocess, tempfile, time
from pathlib import Path
root=Path(__file__).resolve().parents[2]
node=root/'target/node-v22.19.0-universal/node'
runner=root/'target/dsh-closure/node_modules/openmuse-store-runner.mjs'
with tempfile.TemporaryDirectory(prefix='openmuse-store-dsh-test-') as home:
    home=Path(home)
    # A malformed override would fail the ordinary CLI before boot. Store must
    # ignore it altogether, including a custom web profile manifest.
    (home/'cordis.patch.yml').write_text('malicious: [invalid YAML')
    profile=home/'profiles/web'; profile.mkdir(parents=True)
    (profile/'package.json').write_text('{invalid user manifest')
    env={**os.environ,'DSH_HOME':str(home),'DSH_TELEMETRY_DISABLED':'1'}
    dumped=subprocess.check_output([node,runner],env={**env,'OPENMUSE_STORE_DUMP':'1'},text=True)
    rows=json.loads(dumped)
    names=[]
    def walk(v):
        if isinstance(v,list):
            for i in v:walk(i)
        elif isinstance(v,dict):
            name=v.get('name','')
            if isinstance(name,str) and '@deepseek-ai/dsh-' in name:
                if any(x in name for x in ['dsh-otel','dsh-session-telemetry-otel','dsh-host-product-telemetry-otel','dsh-client-product-analytics','dsh-plugin-manager','dsh-tool-bash','dsh-tool-pwsh','dsh-terminal','dsh-api-terminal','dsh-ptc-','dsh-workflow-ptc','dsh-tool-workflow','dsh-tool-cordis','dsh-cordis-host-runner','dsh-config-editor','dsh-hmr']):
                    assert v.get('disabled') is True, name
                    names.append(name)
            for child in v.values():walk(child)
    walk(rows)
    assert len(names)>20
    p=subprocess.Popen([node,runner],env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
    sel=selectors.DefaultSelector();sel.register(p.stdout,selectors.EVENT_READ)
    ready=False;logs=[];deadline=time.monotonic()+30
    try:
        while time.monotonic()<deadline and p.poll() is None:
            if sel.select(1):
                line=p.stdout.readline();logs.append(line)
                if re.search(r'dsh web: http://127\.0\.0\.1:\d+',line):ready=True;break
        assert ready, ''.join(logs)
    finally:
        p.terminate()
        try:p.wait(timeout=8)
        except subprocess.TimeoutExpired:p.kill();p.wait();raise
    assert (home/'cordis.patch.yml').read_text()=='malicious: [invalid YAML'
    print(json.dumps({'status':'passed','fixedCompositionIgnoresUserOverrides':True,
        'disabledRuntimeEntries':len(names),'loopbackWebReady':ready,
        'shutdownExitCode':p.returncode,'modelRequests':0}))
