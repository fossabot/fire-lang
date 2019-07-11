import os.path
import compiler

pykeywords = (
    'False','class','finally','is',
    'lambda','try','True','def','from',
    'nonlocal','and','del','global',
    'not','with','as','or','yield',
    'assert','else','import','pass',
    'except','in','raise'
)

keywords = {
    'if': 'if',
    'fn': 'def',
    'return': 'return',
    'while': 'while',
    'else': 'else',
    'continue': 'continue',
    'break': 'break',
    'in': ' in',
    'for': 'for',
    'struct': 'class',
    'i64': 'int',
}

def pygen(fpath, filename, code, toks, main) -> str:
    global errors
    indent: int = 0
    out: str = '' if main else ' ' * indent
    ln_no: int = 0
    struct: bool = False

    for tok in toks:
        if tok.type == 'whitespace':
            continue
        
        if tok.type == 'newline':
            out += f'\n##{filename}:{ln_no + 1}~ ' + code.splitlines()[ln_no].strip()
            out += '\n' + ' ' * indent
            ln_no += 1
            continue
        
        # print(tok, tok.type)

        if tok.type == 'ID':
            if tok.val in keywords:
                if tok.val == 'struct':
                    struct = True
                out += keywords[tok.val] + ' '
            elif tok.val == 'let':
                pass
            elif tok.val in pykeywords:
                out += '_' + tok.val
            else:
                out += tok.val
        elif tok.type == 'INCLUDE':
            mnam = tok.val[8:].strip()[1:-1]
            if mnam == 're:find':
                out += '\n' + ' ' * indent
                out += f'from re import match as find'
            elif mnam == 'subprocess:run':
                out += '\n' + ' ' * indent
                out += f'from subprocess import run as __run__'
            else:
                mnam, e = mnam.split(':')
                out += '\n' + ' ' * indent
                out += f'from {mnam} import {e}'
        elif tok.type == 'IMPORT':
            tok.val = tok.val[:-1]
            mod = tok.val[3:].strip().replace('::', '/') + '.fi'
            try:
                f = open(fpath + mod, 'r')
                mcode = f.read()
                f.close()
                out += '\n' + compiler.compile(fpath, fpath + mod, mcode) + '\n'
            except:
                ipath = os.path.dirname(__file__) + '/../include/'
                f = open(ipath + mod, 'r')
                mcode = f.read()
                f.close()
                out += '\n' + compiler.compile(ipath, ipath + mod, mcode) + '\n'
        elif tok.type == 'RANGE':
            a, b = tok.val.split('..')
            out += f'list(range({a}, {b}))'
        elif tok.type == 'DICT':
            if tok.val.endswith('({'):
                out += '{'
            elif tok.val == '})':
                out += '}'
        elif tok.val == ',':
            if struct:
                out += '\n' + ' ' * indent
            else:
                out += ','
        elif tok.val == ';':
            if struct:
                raise Exception("invalid syntax")
            else:
                out += ';pass\n' + ' ' * indent
        elif tok.val == '{':
            indent += 1
            out += ':\n' + ' ' * indent
            out += '\n' + ' ' * indent
            if struct:
                out += 'def __str__(self):'
                out += '\n' + ' ' * (indent + 1)
                out += 'ret: str = self.__class__.__name__ + " { "'
                out += '\n' + ' ' * (indent + 1)
                out += 'for e in dir(self):'
                out += '\n' + ' ' * (indent + 2)
                out += 'if not e.startswith("_"):'
                out += '\n' + ' ' * (indent + 3)
                out += 'if type(eval(\'self.\'+e)) == str:'
                out += '\n' + ' ' * (indent + 4)
                out += 'ret += f"{e} = {str(eval(\'self.\'+e).encode())[1:]}, "'
                out += '\n' + ' ' * (indent + 3)
                out += 'else: ret += f"{e} = {str(eval(\'self.\'+e))}, "'
                out += '\n' + ' ' * (indent + 1)
                out += 'return ret[:-2] + " }"'
                out += '\n' + ' ' * indent
        elif tok.val == '}':
            if struct:
                struct = False
            indent -= 1
            out += 'pass\n' + ' ' * indent
        elif tok.val == '::':
            out += '.'
        else:
            out += tok.val

    if main:
        out += '\nif __name__=="__main__":'
        out += '\n from sys import argv'
        out += '\n main(len(argv),argv)'
        out += '\n del argv'

    x = out.splitlines()
    y = ''
    
    for l in x:
        if l.strip() != '':
            y += l + '\n'

    out = y

    while '\n\n' in out:
        out = out.replace('\n\n', '\n')

    return out
