import raug


def repl(runtime: raug.Runtime):
    print('"q" to quit')
    print('"l" to list params')
    while True:
        inp = input("> ").strip()
        if inp == "q":
            break
        if inp == "l":
            for name in runtime.param_names():
                val = runtime.param_named(name).get()
                print(f"{name}: {val}")
            continue
        try:
            p, v = inp.split()
        except:
            print("invalid input")
            continue
        p = p.strip()
        v = v.strip()
        if v.startswith("b"):
            v = raug.Bang()
        else:
            v = float(v)

        try:
            param = runtime.param_named(p)
        except:
            print("param not found")

        try:
            param.set(v)
        except:
            print("invalid value")
