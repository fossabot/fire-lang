import subprocess

pps = []

def check_pytype_output(python: str, out: str) -> bool:
    global pps

    if out.endswith('Success: no errors found\n'):
        return True
    
    else:
        err: bool = False
        note: bool = True
        
        for line in out.splitlines():

            if line.strip() == '':
                err = False
            
            if line.startswith('File'):
                err = True

                lns = line[line.index('line') + 5:].strip()
                try:
                    ln_no = int(lns[:lns.index(',')])
                except:
                    ln_no = int(lns[:lns.index(':')])
                
                if ', in' in line:
                    e = line[line.index(', in')+4:].strip()
                else:
                    e = line
                
                e = e[e.index(':')+1:].strip()
                if '[' in e:
                    p1 = e[e.rindex('[')+1:-1].strip().replace('-', ' ')
                    p2 = e[:e.rindex('[')].strip()
                
                p1 = p1.replace('python', '').strip()

                pps += [p1, p2]

                note = True
                lerr = False

                for i, ln in enumerate(python.splitlines()):
                    if i == ln_no - 1:
                        lerr = True

                    if lerr and ln.startswith('##'):
                        fl, ln = ln[2:].split('~')
                        print(f'\n\033[31;1;1merror\033[0m: {p1}')
                        print(f' \033[34;1;1m--> \033[37;4m{fl}\033[0m')
                        print(f'\033[34;1;1m  |\033[0m')
                        print(f'\033[34;1;1m  |\033[0m    {ln}')
                        print(f'\033[34;1;1m  |\033[0m     \033[31;1;1m^^^ {p2}\033[0m')
                        print(end='\033[34;1;1m  |\033[0m\n')
                        break

            elif err:
                if note:
                    note = False
                    print('  \033[34;1;1m= \033[1;93mnote:\033[35;1;1m ' + line.strip() + '')
                else:
                    print('  \033[35;1;1m        ' + line.strip())
        
        return False

def check_mypy_output(python: str, out: str) -> bool:
    
    if out == '':
        return True
    
    else:
        
        for line in out.splitlines():

            lns = line[line.index('.py:') + 4:].strip()
            ln_no = int(lns[:lns.index(':')])
            err = False

            try:
                line = line[line.index(': error:') + 8:].strip()
            except:
                pass

            for i, ln in enumerate(python.splitlines()):
                if i == ln_no - 1:
                    err = True

                if err and ln.startswith('##'):
                    if '(' in line:
                        p1 = line[:line.index('(')].strip().lower()
                        p2 = line[line.index('(')+1:-1].strip().lower()
                    else:
                        p1 = line.lower()
                        p2 = ''

                    br = False
                    for e in pps:
                        if e in p1 or e[-1] in p1:
                            br = True
                    if br: break

                    fl, ln = ln[2:].split('~')
                    print(f'\n\033[31;1;1merror\033[0m: {p1}')
                    print(f' \033[34;1;1m--> \033[37;4m{fl}\033[0m')
                    print(f'\033[34;1;1m  |\033[0m')
                    print(f'\033[34;1;1m  |\033[0m    {ln}')
                    print(f'\033[34;1;1m  |\033[0m     \033[31;1;1m^^^ {p2}\033[0m')
                    print(end='\n')
                    break

        return False

def check(filepath: str, python: str) -> bool:

    pt = check_pytype_output(python, subprocess.run(["pytype",
        filepath, "-n"], stdout=subprocess.PIPE).stdout.decode())
    
    mp = check_mypy_output(python, subprocess.run
        (["mypy", "--cache-dir=/dev/null", filepath],
        stdout=subprocess.PIPE).stdout.decode())

    if not (pt and mp):
        print(end='\n')
        return False
    else:
        return True
