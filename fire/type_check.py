def check_pytype_output(out: str) -> bool:
    
    if out.endswith('Success: no errors found\n'):
        return True
    
    else:
        err: bool = False
        
        for line in out.splitlines():
            
            if line.strip() == '':
                err = False
            
            if line.startswith('File'):
                err = True
                if ', in' in line:
                    print(line[line.index(', in') + 4:].strip())
                else:
                    print(line[line.index(':') + 1:].strip())
            
            elif err:
                print(line)
        
        return False

def check_mypy_output(python: str, out: str) -> bool:
    
    if out == '':
        return True
    
    else:
        
        for line in out.splitlines():

            lns = line[line.index('.py:') + 4:].strip()
            ln_no = int(lns[:lns.index(':')])
            err = False

            for i, ln in enumerate(python.splitlines()):
                if i == ln_no - 1:
                    err = True

                if err and ln.startswith('##'):
                    print(f'Error in line {ln[2:]}')
                    break

            print(line[line.index(': error:') + 8:].strip())

        return False
