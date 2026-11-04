fields_complex = [
    ('animate', 'Animate', 1, 'Animate::new_from_bin(bin.as_ref(), self.zone)'),
    ('animateColorBehavior', 'AnimateColorBehavior', 2, 'AnimateColorBehavior::new_from_bin(bin.as_ref(), self.zone)'),
    ('animateEffect', 'AnimateEffect', 3, 'AnimateEffect::new_from_bin(bin.as_ref(), self.zone)'),
    ('animateMotion', 'AnimateMotion', 4, 'AnimateMotion::new_from_bin(bin.as_ref(), self.zone)'),
    ('animateRotation', 'AnimateRotation', 5, 'AnimateRotation::new_from_bin(bin.as_ref(), self.zone)'),
    ('animateScale', 'AnimateScale', 6, 'AnimateScale::new_from_bin(bin.as_ref(), self.zone)'),
    ('audio', 'Audio', 7, 'Audio::new_from_bin(bin.as_ref(), self.zone)'),
    ('command', 'Command', 8, 'Command::new_from_bin(bin.as_ref(), self.zone)'),
    ('exclusive', 'CommonTimeNode', 9, 'CommonTimeNode::new_from_bin(bin.as_ref(), self.zone)'),
    ('parallelTimeNode', 'CommonTimeNode', 10, 'CommonTimeNode::new_from_bin(bin.as_ref(), self.zone)'),
    ('sequenceTimeNode', 'SequenceTimeNode', 11, 'SequenceTimeNode::new_from_bin(bin.as_ref(), self.zone)'),
    ('setTimeNodeBehavior', 'SetTimeNodeBehavior', 12, 'SetTimeNodeBehavior::new_from_bin(bin.as_ref(), self.zone)'),
    ('video', 'Video', 13, 'Video::new_from_bin(bin.as_ref(), self.zone)'),
]

def cap(s):
    return s[0].upper() + s[1:]

all_field_names = [f[0] for f in fields_complex]
all_none_init = ', '.join([f'{n}: None' for n in all_field_names])

# --- HEADER ---
header = f'''#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.kso.kproto.KoAr
use crate::kso::kproto::KoAr::KoAr;
// Java: import cn.wps.kso.kproto.KoBin
use crate::kso::kproto::KoBin::{{self, KoBin as KoBinTrait, Decode, Encode, Factory, Null}};
// Java: import cn.wps.kso.kproto.KoClosable
use crate::kso::kproto::KoClosable::KoClosable;
// Java: import cn.wps.kso.kproto.KoZone
use crate::kso::kproto::KoZone::KoZone;
// Java: import cn.wps.kso.kproto.internal.KpBinBox
use crate::kso::kproto::internal::KpBinBox::KpBinBox;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::kso::kproto::internal::KpMap::KpMap;
use super::Animate::Animate;
use super::AnimateColorBehavior::AnimateColorBehavior;
use super::AnimateEffect::AnimateEffect;
use super::AnimateMotion::AnimateMotion;
use super::AnimateRotation::AnimateRotation;
use super::AnimateScale::AnimateScale;
use super::Audio::Audio;
use super::Command::Command;
use super::CommonTimeNode::CommonTimeNode;
use super::SequenceTimeNode::SequenceTimeNode;
use super::SetTimeNodeBehavior::SetTimeNodeBehavior;
use super::Video::Video;

// Java: public class TimeNode implements KoAr, KoClosable
pub struct TimeNode {{
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
'''

for name, tname, key, _ in fields_complex:
    header += f'    // Java field: {tname} {name}\n'
    header += f'    pub(crate) {name}: Option<Box<{tname}>>,\n'

header += '''    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TimeNode parent
    pub(crate) parent: Option<Box<TimeNode>>,
}

// Java: public class TimeNode implements KoAr, KoClosable
impl TimeNode {
'''

# --- INITIALIZE, CREATE, OPEN ---
header += '''    // Java: static protected static <Value1 extends TimeNode> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = TimeNode::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TimeNode create()
    pub fn create() -> Self {
      // Java: return initialize(TimeNode.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }

    // Java: static TimeNode open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TimeNode.class, bin);
      Self::initialize(bin)
    }

'''

# --- NEW ---
header += f'''    // Java: TimeNode()
    pub fn new(zone: &'static KoZone) -> Self {{
      Self {{
        zone,
        {all_none_init},
        // Java: this.box = new KpBinBox();
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        map: KpMap::new(),
        parent: None,
      }}
    }}

    // Java: TimeNode(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {{
      Self {{
        zone,
        {all_none_init},
        // Java: this.box = new KpBinBox(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        map: KpMap::KpMap_from(bin),
        parent: None,
      }}
    }}
    pub fn TimeNode_from(bin: &dyn KoBinTrait) -> Self {{ Self::new_from_bin(bin, KoZone::defaultZone()) }}

'''

# --- LOADBIN, LOAD_BIN, LOAD_VALUE ---
header += '''    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.map.load(this.box.get());
      let bin_ref = self.bin_box.get(); self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }

    // Java: void load(TimeNode value)
    pub fn load_value(&mut self, value: Option<&TimeNode>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }

'''

# --- SAVEBIN ---
header += '''    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
'''
for name, tname, key, _ in fields_complex:
    header += f'''      // Java: if (this.{name} != null && this.{name}.dirty())
      if self.{name}.as_ref().map_or(false, |v| v.dirty())
      {{
         // Java: this.map.put({key}, this.{name}.saveBin(zone));
         let bin = self.{name}.as_mut().unwrap().saveBin(zone);
         self.map.put({key}, bin);
      }}
'''

header += '''      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }

'''

# --- EQUALSLAX, EQUALS, EQUALS_INNER ---
header += '''    // Java: boolean equalsLax(TimeNode o)
    pub fn equalsLax(&mut self, o: &mut TimeNode) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut TimeNode) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&mut self, o: &mut TimeNode, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TimeNode)
      {
         // Java: TimeNode dst = (TimeNode)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _)
         {
            // Java: return true;
            return true;
         }
'''

# Generate the deeply nested equals_inner body
def gen_equals_field(name, tname, key, indent, is_last=False):
    spc = ' ' * indent
    has_name = f'has{cap(name)}'
    get_name = f'get_{name}_mut'
    lines = []
    lines.append(f'{spc}// Java: else if ((this.{has_name}() || !dst.{has_name}()) && (!this.{has_name}() || dst.{has_name}()))')
    lines.append(f'{spc}else if (self.{has_name}() || !dst.{has_name}()) && (!self.{has_name}() || dst.{has_name}())')
    lines.append(f'{spc}{{')
    lines.append(f'{spc}   // Java: if (this.{has_name}() && dst.{has_name}())')
    lines.append(f'{spc}   if self.{has_name}() && dst.{has_name}()')
    lines.append(f'{spc}   {{')
    lines.append(f'{spc}      // Java: if (isStrict && !this.{name}().equals(dst.{name}()))')
    lines.append(f'{spc}      if isStrict && !self.{get_name}().equals(dst.{get_name}())')
    lines.append(f'{spc}      {{')
    lines.append(f'{spc}         // Java: return false;')
    lines.append(f'{spc}         return false;')
    lines.append(f'{spc}      }}')
    lines.append(f'{spc}      // Java: if (!isStrict && !this.{name}().equalsLax(dst.{name}()))')
    lines.append(f'{spc}      if !isStrict && !self.{get_name}().equalsLax(dst.{get_name}())')
    lines.append(f'{spc}      {{')
    lines.append(f'{spc}         // Java: return false;')
    lines.append(f'{spc}         return false;')
    lines.append(f'{spc}      }}')
    lines.append(f'{spc}   }}')
    return lines

# Build the nested if-else chain for equals_inner
# The structure: first field uses "else if", then each subsequent field is nested inside
indent = 9
all_complex = fields_complex[:]

# First field (animate) starts the chain
name, tname, key, _ = all_complex[0]
has0 = f'has{cap(name)}'
get0 = f'get_{name}_mut'
spc = ' ' * indent
header += f'''{spc}// Java: else if ((this.{has0}() || !dst.{has0}()) && (!this.{has0}() || dst.{has0}()))
{spc}else if (self.{has0}() || !dst.{has0}()) && (!self.{has0}() || dst.{has0}())
{spc}{{
{spc}   // Java: if (this.{has0}() && dst.{has0}())
{spc}   if self.{has0}() && dst.{has0}()
{spc}   {{
{spc}      // Java: if (isStrict && !this.{name}().equals(dst.{name}()))
{spc}      if isStrict && !self.{get0}().equals(dst.{get0}())
{spc}      {{
{spc}         // Java: return false;
{spc}         return false;
{spc}      }}
{spc}      // Java: if (!isStrict && !this.{name}().equalsLax(dst.{name}()))
{spc}      if !isStrict && !self.{get0}().equalsLax(dst.{get0}())
{spc}      {{
{spc}         // Java: return false;
{spc}         return false;
{spc}      }}
{spc}   }}
'''

# Remaining fields nested inside each other
for i, (name, tname, key, _) in enumerate(all_complex[1:], 1):
    ind = indent + 3 * i
    spc = ' ' * ind
    has_n = f'has{cap(name)}'
    get_n = f'get_{name}_mut'
    header += f'''{spc}// Java: if ((this.{has_n}() || !dst.{has_n}()) && (!this.{has_n}() || dst.{has_n}()))
{spc}if (self.{has_n}() || !dst.{has_n}()) && (!self.{has_n}() || dst.{has_n}())
{spc}{{
{spc}   // Java: if (this.{has_n}() && dst.{has_n}())
{spc}   if self.{has_n}() && dst.{has_n}()
{spc}   {{
{spc}      // Java: if (isStrict && !this.{name}().equals(dst.{name}()))
{spc}      if isStrict && !self.{get_n}().equals(dst.{get_n}())
{spc}      {{
{spc}         // Java: return false;
{spc}         return false;
{spc}      }}
{spc}      // Java: if (!isStrict && !this.{name}().equalsLax(dst.{name}()))
{spc}      if !isStrict && !self.{get_n}().equalsLax(dst.{get_n}())
{spc}      {{
{spc}         // Java: return false;
{spc}         return false;
{spc}      }}
{spc}   }}
'''

# The innermost level: type field (key=14, simple int)
max_ind = indent + 3 * len(all_complex)  # after all complex fields
spc = ' ' * max_ind
header += f'''{spc}// Java: if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
{spc}if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
{spc}{{
{spc}   // Java: return !this.hasType() || !dst.hasType() || this.type() == dst.type();
{spc}   return !self.hasType() || !dst.hasType() || self.r#type() == dst.r#type();
{spc}}}
{spc}// Java: else
{spc}else
{spc}{{
{spc}   // Java: return false;
{spc}   return false;
{spc}}}
'''

# Close all the nested blocks (from innermost to outermost)
for i in range(len(all_complex) - 1, 0, -1):
    ind = indent + 3 * i
    spc = ' ' * ind
    header += f'''{spc}}}
{spc}// Java: else
{spc}else
{spc}{{
{spc}   // Java: return false;
{spc}   return false;
{spc}}}
'''

# Close the first field's block
spc = ' ' * indent
header += f'''{spc}}}
{spc}// Java: else
{spc}else
{spc}{{
{spc}   // Java: return false;
{spc}   return false;
{spc}}}
'''

# Close the outer else block and the function
header += '''      }
      // Java: else
      // Java: return false;
      return false;
    }

'''

# --- HASHCODE ---
header += '''    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
'''
for name, tname, key, _ in fields_complex:
    has_n = f'has{cap(name)}'
    get_n = f'get_{name}_mut'
    header += f'''      // Java: if (this.{has_n}())
      if self.{has_n}()
      {{
         // Java: ret += this.{name}().hashCode();
         ret = ret.wrapping_add(self.{get_n}().hashCode());
      }}
'''

header += '''      // Java: if (this.hasType())
      if self.hasType()
      {
         // Java: ret += Integer.valueOf(this.type()).hashCode();
         ret = ret.wrapping_add(self.r#type());
      }
      // Java: return ret;
      ret
    }

'''

# --- DIRTY ---
header += '''    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
'''
for i, (name, tname, key, _) in enumerate(fields_complex):
    prefix = 'if' if i == 0 else 'else if'
    header += f'''      // Java: {"if" if i == 0 else "else if"} (this.{name} != null && this.{name}.dirty())
      {prefix} self.{name}.as_ref().map_or(false, |v| v.dirty())
      {{
         // Java: return true;
         return true;
      }}
'''

header += '''      // Java: else
      else
      {
         // Java: return this.video != null && this.video.dirty() ? true : this.map.dirty();
         self.map.dirty()
      }
    }

'''

# --- SAVE, LASTBIN, DISCARD ---
header += '''    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanParent();
      self.cleanParent();
      // Java: this.map.load(KoBin.Null.value1());
      let null_bin = Null::value1(); self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.clear();
      // Java: this.zone.leave();
      self.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
'''
for name, tname, key, _ in fields_complex:
    header += f'''      // Java: this.{name} = null;
      self.{name} = None;
'''

header += '''    }

    // Java: void follow(TimeNode parent)
    pub fn follow(&mut self, parent: Option<Box<TimeNode>>) {
      // Java: if (parent != null)
      if let Some(mut parent) = parent
      {
         // Java: parent.discard();
         parent.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(parent);
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some()
      {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: TimeNode parent()
    pub fn parent(&self) -> Option<&TimeNode> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone; self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

'''

# --- ACCESSOR METHODS (getter, get_mut, has, delete, write, forceWrite) ---
for name, tname, key, ctor in fields_complex:
    has_n = f'has{cap(name)}'
    get_n = f'get_{name}'
    header += f'''    // Java: {tname} {name}()
    pub fn {get_n}(&mut self) -> &{tname} {{
      // Java: if (this.{name} == null)
      if self.{name}.is_none()
      {{
         // Java: if (this.map.has({key}))
         if self.map.has({key})
         {{
            // Java: KoBin bin = this.map.byKey({key});
            let bin = self.map.byKey({key});
            // Java: this.{name} = ({tname})KoBin.Decode.toAny({tname}.class, bin, this.zone);
            self.{name} = Some(Box::new({ctor}));
         }}
         // Java: else if (this.parent != null && this.parent.{has_n}())
         else if self.parent.as_ref().map_or(false, |p| p.{has_n}())
         {{
            // Java: this.{name} = this.parent.{name}();
            let bin = self.parent.as_mut().unwrap().{get_n}().lastBin(self.zone);
            self.{name} = Some(Box::new({ctor.replace('bin.as_ref()', 'bin.as_ref()')}));
         }}
         // Java: else
         else
         {{
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.{name} = ({tname})KoBin.Decode.toAny({tname}.class, bin, this.zone);
            self.{name} = Some(Box::new({ctor}));
         }}
      }}
      // Java: return this.{name};
      self.{name}.as_deref().unwrap()
    }}

    pub fn get_{name}_mut(&mut self) -> &mut {tname} {{
      if self.{name}.is_none() {{ self.{get_n}(); }}
      self.{name}.as_deref_mut().unwrap()
    }}

    // Java: boolean {has_n}()
    pub fn {has_n}(&self) -> bool {{
      // Java: return this.map.has({key}) || this.{name} != null && this.{name}.dirty() || this.parent != null && this.parent.{has_n}();
      self.map.has({key}) || self.{name}.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.{has_n}())
    }}

    // Java: void delete{cap(name)}()
    pub fn delete{cap(name)}(&mut self) {{
      // Java: this.map.put({key}, (KoBin)null);
      self.map.clear_key({key});
      // Java: if (this.{name} != null)
      if self.{name}.is_some()
      {{
         // Java: this.{name} = null;
         self.{name} = None;
      }}
    }}

    // Java: void write{cap(name)}({tname} value)
    pub fn write{cap(name)}(&mut self, value: Option<&{tname}>) {{
      // Java: if (value != null)
      if let Some(value) = value
      {{
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {{
            // Java: this.map.put({key}, bin);
            self.map.put({key}, bin);
            // Java: if (this.{name} != null)
            if self.{name}.is_some()
            {{
               // Java: this.{name} = null;
               self.{name} = None;
            }}
         }}
      }}
    }}

    // Java: void forceWrite{cap(name)}({tname} value)
    pub fn forceWrite{cap(name)}(&mut self, value: Option<&{tname}>) {{
      // Java: if (value != null)
      if let Some(value) = value
      {{
         // Java: this.map.put({key}, value.lastBin(this.zone));
         self.map.put({key}, value.lastBin(self.zone));
         // Java: if (this.{name} != null)
         if self.{name}.is_some()
         {{
            // Java: this.{name} = null;
            self.{name} = None;
         }}
      }}
    }}

'''

# --- TYPE field ---
header += '''    // Java: int type()
    pub fn r#type(&self) -> i32 {
      // Java: if (!this.map.has(14))
      if !self.map.has(14)
      {
         // Java: return this.parent == null ? 9 : this.parent.type();
         return if self.parent.is_none() { 9 } else { self.parent.as_ref().unwrap().r#type() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(14);
         let bin = self.map.byKey(14);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(14) || this.parent != null && this.parent.hasType();
      self.map.has(14) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.clear_key(14);
      self.map.clear_key(14);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(14, KoBin.Encode.from(value, this.zone));
      self.map.put(14, Encode::from_int(value, self.zone));
    }

}

impl KoAr for TimeNode {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) {
        TimeNode::loadBin(self, var1, var2);
    }
    fn saveBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> {
        let this = unsafe { &mut *(self as *const Self as *mut Self) };
        this.saveBin(var1)
    }
    fn dirty(&self) -> bool { TimeNode::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { TimeNode::lastBin(self, var1) }
}

impl KoClosable for TimeNode {
    fn close(&mut self) { TimeNode::close(self); }
}
'''

with open(r'd:\agentic\src\wordOffice\libs\ppt-core\src\show\data\timing\timenode\TimeNode.rs', 'w', encoding='utf-8') as f:
    f.write(header)

print(f'Written {len(header)} chars')
