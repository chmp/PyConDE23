import json
import time

from pathlib import Path

from IPython import get_ipython

self_path = Path(__file__).parent.resolve()


def setup():
    shell = get_ipython()
    shell.register_magic_function(record_time_magic, "cell", "record-time")


def record_time_magic(line, cell, module=None):
    name = line
    start = time.time()

    try:
        get_ipython().run_cell(cell)

    finally:
        end = time.time()
        took = end - start
        print(f"[{name}] took {took}s")

    with open(self_path / "data" / f"timing.{name}.json", "wt") as fobj:
        json.dump({name: took}, fobj)


def load_timings():
    res = {}

    for p in self_path.joinpath("data").glob("timing.*.json"):
        with open(p, encoding="utf-8") as fobj:
            res.update(json.load(fobj))

    return res
