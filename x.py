import argparse
import pathlib
import subprocess
import sys

self_path = pathlib.Path(__file__).parent.resolve()

_md = lambda effect: lambda f: [f, effect(f)][0]
_ps = lambda o: vars(o).setdefault("__chmp__", {})
_as = lambda o: _ps(o).setdefault("__args__", [])
cmd = lambda **kw: _md(lambda f: _ps(f).update(kw))
arg = lambda *a, **k: _md(lambda f: _as(f).insert(0, (a, k)))


@cmd()
def precommit():
    cargo("fmt")
    python("-m", "black", *self_path.glob("*.ipynb"), *self_path.glob("*.py"))

    cargo("check")


def cargo(*args, **kwargs):
    return run("cargo", *args, **kwargs)


def python(*args, **kwargs):
    return run(sys.executable, *args, **kwargs)


def run(*args, **kwargs):
    kwargs.setdefault("check", True)
    kwargs.setdefault("cwd", self_path)

    args = [str(arg) for arg in args]
    print("::", " ".join(args))
    return subprocess.run(args, **kwargs)


def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers()

    for func in globals().values():
        if not hasattr(func, "__chmp__"):
            continue

        desc = dict(func.__chmp__)
        name = desc.pop("name", func.__name__.replace("_", "-"))
        args = desc.pop("__args__", [])

        subparser = subparsers.add_parser(name, **desc)
        subparser.set_defaults(__main__=func)

        for arg_args, arg_kwargs in args:
            subparser.add_argument(*arg_args, **arg_kwargs)

    args = vars(parser.parse_args())
    return args.pop("__main__")(**args) if "__main__" in args else parser.print_help()


if __name__ == "__main__":
    main()
