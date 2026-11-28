"""Production PPTX UI test: use only an authorized QA phone and bundled QA demo.
Requires uiautomator2==3.7.1. Does not clear App data; adds the chosen QA marker.
Run: python pptx_roundtrip.py --serial SERIAL --output OUT --marker QA-UNIQUE
Device/app must already be on Home Recent or inside its bundled introduction PPTX.
"""
from pathlib import Path
import json,time,hashlib,re,xml.etree.ElementTree as ET
import uiautomator2 as u2

import argparse
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--serial',required=True)
parser.add_argument('--output',type=Path,required=True)
parser.add_argument('--marker',required=True)
args=parser.parse_args()
assert args.marker.startswith('QA') and len(args.marker)<80
OUT=args.output;OUT.mkdir(mode=0o700,parents=True,exist_ok=False)
d=u2.connect(args.serial);MARKER=args.marker;events=[]
installed=d.shell(['pm','path','com.openmuseai.mobile']).output.strip().splitlines()
assert len(installed)==1 and installed[0].startswith('package:'), 'Unexpected package layout'
actual=d.shell(['sha256sum',installed[0][8:]]).output.split()[0]
assert actual=='d815b488061f6f064c1a38681a0f1722dae6cd5a6adc86ae565df87749583201', 'Wrong candidate APK; refuse document mutation'

def label(text):
 a=d(text=text)
 return a if a.exists else d(description=text)
def wait_label(text,timeout=15):
 deadline=time.monotonic()+timeout
 while time.monotonic()<deadline:
  candidate=label(text)
  if candidate.exists:return candidate
  time.sleep(.25)
 raise AssertionError('UI label not available: '+text)
def capture(name):
 x=d.dump_hierarchy();(OUT/(name+'.xml')).write_text(x);d.screenshot(str(OUT/(name+'.png')),display_id=0)
 events.append({'stage':name,'elapsed':round(time.monotonic()-started,2)})
 (OUT/'events.json').write_text(json.dumps(events,indent=2))
def blur():
 notes=d(className='android.widget.EditText',text='');assert notes.exists;notes.click();time.sleep(.8)
 label('开始').click();time.sleep(.8)
def stage_bounds():
 image=d.screenshot(display_id=0).convert('RGB');w,h=image.size;px=image.load();ranges=[];start=None
 for y in range(280,h-140):
  dark=sum(1 for x in range(20,w-20,3) if max(px[x,y])<80)
  yes=dark>80
  if yes and start is None:start=y
  if not yes and start is not None:ranges.append((start,y));start=None
 if start is not None:ranges.append((start,h-140))
 candidates=[]
 for top,bottom in ranges:
  if bottom-top<100:continue
  middle=(top+bottom)//2;xs=[x for x in range(w) if max(px[x,middle])<80]
  if not xs:continue
  left,right=min(xs),max(xs)+1;ratio=(right-left)/(bottom-top)
  if 1.65<ratio<1.9:candidates.append((left,top,right,bottom))
 if not candidates:raise AssertionError('Cannot identify the actual16:9slide surface')
 return max(candidates,key=lambda b:(b[2]-b[0])*(b[3]-b[1]))
class SubtitleEditor:
 def get_text(self):
  nodes=[n for n in ET.fromstring(d.dump_hierarchy()).iter('node') if n.attrib.get('class')=='android.widget.EditText' and n.attrib.get('text','').startswith('Documents')]
  assert len(nodes)==1,'Actual subtitle field is not uniquely visible'
  return nodes[0].attrib['text']
 def set_text(self,value):
  d(className='android.widget.EditText',textStartsWith='Documents').set_text(value)
def edit_subtitle():
 nodes=[n for n in ET.fromstring(d.dump_hierarchy()).iter('node') if n.attrib.get('class')=='android.widget.EditText' and n.attrib.get('text','').startswith('Documents') and n.attrib.get('focused')=='true']
 if nodes:return SubtitleEditor()
 left,top,right,bottom=stage_bounds();x=int(left+200*(right-left)/1280);y=int(top+290*(right-left)/1280)
 d.click(x,y);time.sleep(.7);d.click(x,y)
 deadline=time.monotonic()+15
 while time.monotonic()<deadline:
  nodes=[n for n in ET.fromstring(d.dump_hierarchy()).iter('node') if n.attrib.get('class')=='android.widget.EditText' and n.attrib.get('text','').startswith('Documents') and n.attrib.get('focused')=='true']
  if len(nodes)==1:return SubtitleEditor()
  time.sleep(.25)
 raise AssertionError('Actual subtitle editor did not focus')

started=time.monotonic()
try:
 if not label('文件').exists:
  file=d(description='OpenMuse Introduction.pptx\nDocuments · Documents');assert file.exists;file.click()
  wait_label('文件',90)
 time.sleep(1);capture('00-loaded')
 editor=edit_subtitle();before=editor.get_text();assert MARKER not in before
 editor.set_text(before+' '+MARKER);time.sleep(.8)
 assert MARKER in SubtitleEditor().get_text()
 capture('01-typed');blur();capture('02-committed')
 label('撤销').click();time.sleep(.7)
 editor=edit_subtitle();undone=editor.get_text();assert MARKER not in undone,'Undo did not remove the QA marker'
 capture('03-undone');blur()
 label('恢复').click();time.sleep(.7)
 editor=edit_subtitle();redone=editor.get_text();assert MARKER in redone,'Redo did not restore the QA marker'
 capture('04-redone');blur()
 save=label('保存 (Ctrl+S)');assert save.info['enabled'],'Save is disabled after the changed text was committed'
 save.click();time.sleep(2)
 assert not label('保存 (Ctrl+S)').info['enabled'],'Save did not reach the saved state'
 capture('05-saved')
 label('返回').click();time.sleep(1)
 file=d(description='OpenMuse Introduction.pptx\nDocuments · Documents');assert file.wait(timeout=10)
 capture('06-closed');file.click();wait_label('文件',90);time.sleep(1)
 editor=edit_subtitle();reopened=editor.get_text();assert MARKER in reopened,'Reopened actual resource does not contain the saved QA marker'
 capture('07-reopened')
 result={'schema':'openmuse.android-production-pptx-acceptance/1','version':'1.0.3','build':112504,'packageId':'com.openmuseai.mobile','device':'OPPO PKM110 / Android16','technology':'UIAutomator2 3.7.1, explicitly authorized by user','marker':MARKER,'originalSubtitle':before,'undoSubtitle':undone,'redoSubtitle':redone,'reopenedSubtitle':reopened,'actualEditText':True,'touchUndoRedo':True,'actualResourceSaveExitReopen':True,'passed':True,'elapsedSeconds':round(time.monotonic()-started,2),'scope':'Real signed production App and its Recent resource, no app-data clearing or hidden JavaScript model updates'}
 (OUT/'result.json').write_text(json.dumps(result,indent=2,ensure_ascii=False)+'\n');print(json.dumps(result,ensure_ascii=False))
except Exception as error:
 capture('failure')
 (OUT/'failure.json').write_text(json.dumps({'passed':False,'stage':events[-2]['stage'] if len(events)>1 else 'setup','error':str(error),'elapsedSeconds':round(time.monotonic()-started,2)},ensure_ascii=False,indent=2))
 raise
