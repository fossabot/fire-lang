import os
import shutil
import compiler
import argparse
import tempfile
import type_check
import subprocess

def main():
    parser = argparse.ArgumentParser(description='fire-lang compiler.')
    parser.add_argument('file', metavar='file.fi', type=str, help='source file')
    args = parser.parse_args()

    file = open(args.file, 'r')
    code = file.read()
    file.close()
    
    python = '''
# pytype: disable=not-supported-yet
from typing import *
# pytype: enable=not-supported-yet
__print__: Callable = print
def print(fmt, *args):
 return __print__(fmt.format(*args), end="")
def println(fmt, *args):
 return __print__(fmt.format(*args), end="\\n")
''' + compiler.compile(os.path.dirname(args.file), args.file, code, main=True)

    tmpname = tempfile.mktemp(suffix='.py')
    tmp = open(tmpname, 'w')
    tmp.write(python)
    tmp.close()

    if type_check.check(tmpname, python):
        try:
            shutil.rmtree('build')
        except:
            pass
        output = os.path.splitext(os.path.basename(args.file))[0]
        pyio = os.path.splitext(os.path.basename(tmpname))[0]
        subprocess.run(["pyinstaller", "-F", "--specpath=build/specfile", "--distpath=build/bin/", tmpname],
            stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        os.rename(f'build/{pyio}', f'build/{output}')
        os.rename(f'build/bin/{pyio}', f'build/bin/{output}')
        shutil.rmtree('build/specfile')
    else:
        pass

    os.unlink(tmpname)

if __name__ == '__main__':
    main()
