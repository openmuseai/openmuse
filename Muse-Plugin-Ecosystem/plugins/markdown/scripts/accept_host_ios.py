"""Compatibility CLI for the current resource-centric Host UI acceptance."""
from pathlib import Path
import runpy

runpy.run_path(str(Path(__file__).with_name('accept_resource_ios.py')), run_name='__main__')
