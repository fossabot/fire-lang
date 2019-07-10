import os
import lexer
import pygen
import tempfile
import type_check
import subprocess

errors = False

def check_and_compile(path, code, main=False, module=None, mall=False):
    global errors
    NUMBER = r'((0x[0-9A-F]+)|([0-9]+))'

    rules = {
        'IMPORT': r'use( )*([a-zA-Z_][a-zA-Z_0-9]*(::|))*(\*|)',
        'EXPR-FOR': ' for ',
        'DICT': r'dict( )*\({|}\)',
        'STR': r'"(\\\"|\\\\|[^"\n])*?"i?',
        'ID': r'[a-zA-Z_][a-zA-Z_0-9]*'
    }

    rules['RANGE'] = NUMBER + r'( )*\.\.( )*' + NUMBER
    rules['NUMBER'] = NUMBER

    for i in '{<([])>}+-*/%;:.,=!':
        rules[i] = '\\' + i

    rules['newline'] = r'\n'
    rules['whitespace'] = r' |\t'

    lex = lexer.Lexer(rules, skip_whitespace=False)
    lex.input(code)

    mindent = 0
    if module != None and not mall:
        mindent = module.count('::') + 1

    tokens = list(lex.tokens())
    python = ''
    for i in range(mindent):
        python += ' ' * i + f'class {module.split("::")[i]}:\n'
    python += pygen.pygen(path + '/', code, tokens, main, mindent)

    tmpname = tempfile.mktemp(suffix='.py')
    tmp = open(tmpname, 'w')
    tmp.write(python)
    tmp.close()
    
    pt = type_check.check_pytype_output(subprocess.run(["pytype",
        tmpname, "-n"], stdout=subprocess.PIPE).stdout.decode())
    
    mp = type_check.check_mypy_output(python,
        subprocess.run(["mypy", "--cache-dir=/dev/null", tmpname],
        stdout=subprocess.PIPE).stdout.decode())

    if not (pt and mp):
        errors = True

    return python
