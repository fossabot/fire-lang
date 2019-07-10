import re

class Token(object):
    def __init__(self, type, val):
        self.type = type
        self.val = val

    def __str__(self):
        return '%s' % (self.val)

class Lexer(object):
    def __init__(self, rules, skip_whitespace=True):
        idx = 1
        regex_parts = []
        self.group_type = {}

        for type in rules:
            regex = rules[type]
            groupname = 'GROUP%s' % idx
            regex_parts.append('(?P<%s>%s)' % (groupname, regex))
            self.group_type[groupname] = type
            idx += 1

        self.regex = re.compile('|'.join(regex_parts))
        self.skip_whitespace = skip_whitespace
        self.re_ws_skip = re.compile('\S')

    def input(self, buf):
        self.buf = buf
        self.pos = 0

    def token(self):
        if self.pos >= len(self.buf):
            return None
        else:
            if self.skip_whitespace:
                m = self.re_ws_skip.search(self.buf, self.pos)

                if m:
                    self.pos = m.start()
                else:
                    return None

            m = self.regex.match(self.buf, self.pos)
            if m:
                groupname = m.lastgroup
                tok_type = self.group_type[groupname]
                tok = Token(tok_type, m.group(groupname))
                self.pos = m.end()
                return tok

            raise Exception(f"invalid syntax at {self.pos}")

    def tokens(self):
        while 1:
            tok = self.token()
            if tok is None: break
            yield tok
