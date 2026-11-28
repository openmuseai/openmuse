"""Destructive QA ONLY: reset the explicitly owned disposable test emulator.
Refuses physical devices and any other AVD name. Requires adb and UIAutomator2.
Network counters are bounded evidence, not a complete privacy audit.
"""
from pathlib import Path
import json,re,time,subprocess,xml.etree.ElementTree as ET
import uiautomator2 as u2
import argparse
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--serial',required=True)
parser.add_argument('--output',type=Path,required=True)
args=parser.parse_args()
SERIAL=args.serial;PACKAGE='com.openmuseai.mobile';OUT=args.output;OUT.mkdir(mode=0o700,parents=True,exist_ok=False)
def adb(*args):return subprocess.check_output(['adb','-s',SERIAL,*args],text=True).strip()
assert adb('shell','getprop','ro.boot.qemu.avd_name')=='Muse103Consent112504','Refuse to reset any other environment'
assert SERIAL.startswith('emulator-')
d=u2.connect(SERIAL)
installed=adb('shell','pm','path',PACKAGE).splitlines()
assert len(installed)==1 and installed[0].startswith('package:')
assert adb('shell','sha256sum',installed[0][8:]).split()[0]=='d815b488061f6f064c1a38681a0f1722dae6cd5a6adc86ae565df87749583201', 'Wrong candidate APK'
d.app_stop(PACKAGE)
# This is the disposable test AVD created for this case, never the user's phone.
assert d.shell(['pm','clear',PACKAGE]).exit_code==0
uid=int(re.search(r'appId=(\d+)',adb('shell','dumpsys','package',PACKAGE)).group(1));time.sleep(1)
def traffic(name):
 adb('shell','dumpsys','netstats','--poll')
 raw=adb('shell','dumpsys','netstats','--full','--uid');(OUT/(name+'-private.txt')).write_text(raw)
 totals={'rxBytes':0,'txBytes':0,'rxPackets':0,'txPackets':0};active=False
 for line in raw.splitlines():
  if 'ident=' in line:active=bool(re.search(r'\buid='+str(uid)+r'\b',line))
  if active:
   m=re.search(r'\brb=(\d+) rp=(\d+) tb=(\d+) tp=(\d+)',line)
   if m:
    for key,value in zip(['rxBytes','rxPackets','txBytes','txPackets'],m.groups()):totals[key]+=int(value)
 return totals
def label(s):return d(text=s) if d(text=s).exists else d(description=s)
def cap(name):
 (OUT/(name+'.xml')).write_text(d.dump_hierarchy());d.screenshot(str(OUT/(name+'.png')),display_id=0)
baseline=traffic('baseline')
d.shell(['am','start','-W','-n',PACKAGE+'/com.openmuseai.mobile.MainActivity'])
checkbox=d(className='android.widget.CheckBox');assert checkbox.wait(timeout=20)
assert not checkbox.info['checked'] and not label('同意并继续').info['enabled'];cap('01-before-consent')
time.sleep(10);gated=traffic('gated')
assert gated==baseline,('Traffic changed behind consent gate',baseline,gated)
checkbox.click();label('暂不同意').click();assert not checkbox.info['checked'];cap('02-declined')
time.sleep(3);refused=traffic('refused');assert refused==baseline,('Traffic changed after refusal',baseline,refused)
checkbox.click();label('同意并继续').click()
deadline=time.monotonic()+120
while time.monotonic()<deadline:
 if label('协作工作区').exists:break
 time.sleep(1)
assert label('协作工作区').exists,'Business home did not start after explicit consent'
cap('03-home-after-consent');accepted=traffic('accepted')
assert accepted['rxBytes']+accepted['txBytes']>baseline['rxBytes']+baseline['txBytes'],'No positive network observation after consent; cannot use a disconnected environment as proof'
result={'schema':'openmuse.android-consent-network-observation/1','checkedOn':'2026-11-28','version':'1.0.3','build':112504,'environment':'Fresh disposable AOSP34arm64; exact signed productionAPK','packageId':PACKAGE,'uid':uid,'baseline':baseline,'behindGateAfter10Seconds':gated,'afterDecline':refused,'afterAcceptAndHome':accepted,'noObservedAppUidTrafficBeforeConsent':True,'positiveAppUidTrafficAfterConsent':True,'passed':True,'scope':'Android UID network accounting in bounded intervals plus actual legal/home UI; not a complete packet capture or all-SDK/privacy certification. Only disposable emulator data reset; physical phone data preserved.'}
(OUT/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
