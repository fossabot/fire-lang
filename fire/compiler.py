import lexer
import pygen

def compile(path, filename, code, main=False):
    NUMBER = r'((0x[0-9A-F]+)|([0-9]+))'

    rules = {
        'ELSE-IF': r'else( )+if',
        'INCLUDE': r'#include\([a-zA-Z_][a-zA-Z_0-9\.]*:[a-zA-Z_][a-zA-Z_0-9]*\)',
        'IMPORT': r'use( )*([a-zA-Z_][a-zA-Z_0-9]*(::|))*;',
        'DICT': r'dict( )*\({|}\)',
        'STR': r'"(\\\"|\\\\|[^"\n])*?"i?',
        'ID': r'[a-zA-Z_][a-zA-Z_0-9]*'
    }
    
    rules['::'] = r'::'
    rules['->'] = r'->'
    rules['=='] = r'=='

    rules['RANGE'] = NUMBER + r'( )*\.\.( )*' + NUMBER
    rules['NUMBER'] = NUMBER

    for i in '@{<([])>}+-*/%;:.,=!':
        rules[i] = '\\' + i

    rules['newline'] = r'\n'
    rules['whitespace'] = r' |\t'

    lex = lexer.Lexer(rules, skip_whitespace=False)
    lex.input(code)
    tokens = list(lex.tokens())

    return pygen.pygen(path + '/', filename, code, tokens, main)
