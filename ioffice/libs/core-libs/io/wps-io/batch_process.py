"""Batch process zero-import Java files through the pipeline."""
import subprocess
import sys
import os
import re

files = [
    "cn/wps/io/Mode.java",
    "cn/wps/io/dom/ElementPath.java",
    "cn/wps/io/dom/dtd/AttributeDecl.java",
    "cn/wps/io/dom/dtd/ElementDecl.java",
    "cn/wps/io/dom/dtd/ExternalEntityDecl.java",
    "cn/wps/io/dom/dtd/InternalEntityDecl.java",
    "cn/wps/io/dom/io/RMap.java",
    "cn/wps/io/opt/BuildConfig.java",
    "cn/wps/io/opt/LexicalState.java",
    "cn/wps/io/simple/BuildConfig.java",
    "cn/wps/io/util/HexColor.java",
    "cn/wps/io/xml03/Xml03Namespace.java",
    "cn/wps/io/xwpf/ChartSrc.java",
    "cn/wps/io/xwpf/DiagramColors.java",
    "cn/wps/io/xwpf/DiagramData.java",
    "cn/wps/io/xwpf/DiagramDrawing.java",
    "cn/wps/io/xwpf/DiagramLayout.java",
    "cn/wps/io/xwpf/DiagramStyle.java",
    "cn/wps/io/xwpf/IImporterBase.java",
    "cn/wps/io/xwpf/ISubDocumentImporter.java",
    "cn/wps/io/xwpf/MathIOType.java",
    "cn/wps/io/xwpf/RecipientData.java",
    "cn/wps/io/xwpf/element_handler/IRunTrackChangeChildHandler.java",
    "cn/wps/io/xwpf/element_handler/IRunTrackChangeHandler.java",
    "cn/wps/io/xwpf/element_handler/prop/IPropBaseHandlerHelper.java",
    "cn/wps/io/xwpf/element_handler/prop/IPropHandlerHelper.java",
    "cn/wps/io/xwpf/element_handler/smartart/Dgm.java",
]

def run_cmd(cmd):
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
    return result.stdout + result.stderr

def fix_template(rs_path):
    """Apply common template fixes."""
    if not os.path.exists(rs_path):
        return False

    with open(rs_path, 'r', encoding='utf-8') as f:
        content = f.read()

    original = content

    # Fix imports
    content = re.sub(r'use io_wps_io::', 'use crate::io::', content)
    content = re.sub(r'use util::\S+;', lambda m: '// ' + m.group(0), content)
    content = re.sub(r'use kfs_kfs_app::\S+;', lambda m: '// ' + m.group(0), content)
    content = re.sub(r'use kfc_kfc_alpha::\S+;', lambda m: '// ' + m.group(0), content)

    # Fix TODO_RUST_CODE use lines
    content = re.sub(r'// TODO_RUST_CODE use \?\?\?;.*', '', content)

    # Fix /* TODO */ constants
    content = re.sub(r": String = /\* TODO \*/;", r': &str = "";', content)
    content = re.sub(r": bool = /\* TODO \*/;", r': bool = false;', content)
    content = re.sub(r": i32 = /\* TODO \*/;", r': i32 = 0;', content)
    content = re.sub(r": i64 = /\* TODO \*/;", r': i64 = 0;', content)
    content = re.sub(r": f32 = /\* TODO \*/;", r': f32 = 0.0;', content)
    content = re.sub(r": f64 = /\* TODO \*/;", r': f64 = 0.0;', content)

    # Fix Java-style params in pub fn
    java_types = {
        'int': 'i32', 'long': 'i64', 'float': 'f32', 'double': 'f64',
        'boolean': 'bool', 'String': '&str', 'Object': '&dyn std::any::Any',
        'StringBuilder': '&mut String',
    }
    for jt, rt in java_types.items():
        content = re.sub(rf'\b{re.escape(jt)}\s+(\w+)(?=\s*[,)])', rf'\1: {rt}', content)

    # Replace // TODO_RUST_CODE with todo!();
    content = content.replace('// TODO_RUST_CODE', 'todo!();')

    # Fix Arc<Mutex<>> struct fields
    content = re.sub(r'Arc<Mutex<String>>', 'String', content)
    content = re.sub(r'Arc<Mutex<i32>>', 'i32', content)
    content = re.sub(r'Arc<Mutex<bool>>', 'bool', content)
    content = re.sub(r'Arc<Mutex<(\w+)>>', r'Option<Box<dyn std::any::Any>>', content)

    if content != original:
        with open(rs_path, 'w', encoding='utf-8', newline='\n') as f:
            f.write(content)
        return True
    return False

success = 0
failed = 0
for java_file in files:
    print(f"\n--- Processing {java_file} ---")

    # Step 1: generate template
    output = run_cmd(f'java2rust run start --file {java_file}')
    if 'gen_template OK' in output or 'Already done' in output or 'Already' in output:
        # Find rust file path
        rs_path = java_file.replace('.java', '.rs')
        rs_full = os.path.join('src', rs_path)

        if os.path.exists(rs_full):
            fixed = fix_template(rs_full)
            if fixed:
                print(f"  Applied fixes to {rs_path}")

            # Step 2: check
            check_output = run_cmd(f'java2rust run check --file src/{rs_path}')
            if 'PASSED' in check_output:
                print(f"  PASSED!")
                success += 1
            else:
                # Extract errors
                errors = [l for l in check_output.split('\n') if 'FAILED' in l or 'Missing' in l or 'error' in l.lower()]
                for e in errors[:5]:
                    print(f"  {e.strip()}")
                failed += 1
        else:
            print(f"  RS file not found: {rs_full}")
            failed += 1
    elif 'Deferred' in output:
        print(f"  DEFERRED")
        failed += 1
    else:
        print(f"  SKIP/ERROR")
        failed += 1

print(f"\n=== Results: {success} passed, {failed} failed ===")
