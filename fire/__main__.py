import os
import compiler
import argparse
import tempfile
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
''' + compiler.check_and_compile(os.path.dirname(args.file), code, main=True)

    tmpname = tempfile.mktemp(suffix='.py')
    tmp = open(tmpname, 'w')
    tmp.write(python)
    tmp.close()

    if not compiler.errors:
        pt = subprocess.run(["nuitka", tmpname,
            "--remove-output", "--python-version=3.6"],
            stdout=subprocess.PIPE)
        os.rename(os.path.splitext
            (os.path.basename(tmpname))[0] + '.exe',
            os.path.splitext(os.path.basename(args.file))[0])
    else:
        pass

    os.unlink(tmpname)

if __name__ == '__main__':
    main()
