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
}

def pygen(fpath, code, toks, main, mindent) -> str:
    global errors
    indent: int = mindent
    out: str = '\nfrom sys import argv\n' if main else ' ' * indent
    ln_no: int = 0

    for tok in toks:
        if tok.type == 'whitespace':
            continue
        
        if tok.type == 'newline':
            out += f'\n##{ln_no + 1}: ' + code.splitlines()[ln_no].strip()
            out += '\n' + ' ' * indent
            ln_no += 1
            continue
        
        # print(tok, tok.type)

        if tok.type == 'ID':
            if tok.val in keywords:
                out += keywords[tok.val] + ' '
            elif tok.val == 'let':
                pass
            elif tok.val in pykeywords:
                out += '_' + tok.val
            else:
                out += tok.val
        elif tok.type == 'IMPORT':
            mall = False
            if tok.val[-3:] == '::*':
                mall = True
                tok.val = tok.val[:-3]
            mod = fpath + tok.val[3:].strip().replace('::', '/') + '.fi'
            f = open(mod, 'r')
            mcode = f.read()
            f.close()
            out += '\n' + compiler.check_and_compile(fpath, mcode, module=tok.val[4:], mall=True) + '\n'
        elif tok.type == 'RANGE':
            a, b = tok.val.split('..')
            out += f'list(range({a}, {b}))'
        elif tok.type == 'DICT':
            if tok.val.endswith('({'):
                out += '{'
            elif tok.val == '})':
                out += '}'
        elif tok.val == ';':
            out += ';pass\n' + ' ' * indent
        elif tok.val == '{':
            indent += 1
            out += ':\n' + ' ' * indent
            out += '\n' + ' ' * indent
        elif tok.val == '}':
            indent -= 1
            out += 'pass\n' + ' ' * indent
        else:
            out += tok.val

    if main:
        out += '\nif __name__=="__main__":'
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
