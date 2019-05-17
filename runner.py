#!/usr/bin/env python3
import os
import sys
import tempfile
from pathlib import Path
from subprocess import run


def kill_open_ocd():
    r = run(["killall", "openocd"])


def delete_flash():
     result = run(["nrfjprog",  "--eraseall", "-f", "nrf52"])
     if result.returncode != 0:
        raise Exception("Could not delete flash!")


def convert_elf_to_hex(elf, destination):
    result = run(["arm-none-eabi-objcopy", "-O", "ihex", f"{elf}", f"{destination}"])
    if result.returncode != 0:
        raise Exception("Could not convert elf to ihex!")


def flash_hex_file(hexfile):
    result = run(["nrfjprog", "--program", f"{hexfile}"])
    if result.returncode != 0:
        raise Exception("Could not flash hex file!")

def start_debugger(elf_file):
    shell = os.environ['SHELL']
    result = run(f'openocd -f openocd.cfg --log_output openocd.log & arm-none-eabi-gdb -q -x openocd.gdb {elf_file}', shell=True)
    if result.returncode != 0:
        raise Exception("Could start openocd and/or gdb!")

if __name__ == '__main__':
    elf_file = Path(sys.argv[1]).resolve()
    with tempfile.TemporaryDirectory() as tmp_dir:
        kill_open_ocd()
        delete_flash()
        hexfile = Path(tmp_dir, 'image.hex') 
        convert_elf_to_hex(elf_file, hexfile)
        flash_hex_file(hexfile)
        start_debugger(elf_file)
