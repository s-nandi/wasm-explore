from generatorworld import exports
import generator
import random

class Generator(exports.Generator):
    def generate(self) -> int:
        return random.randint(50, 100)

    def generate_many(self) -> list[int]:
        return [random.randint(10, 20) for _ in range(4)]

    def generate_paths(self) -> list[generator.Path]:
        paths = [__file__]
        for _ in range(5):
            tag = "".join(random.choices("abcdef", k=3))
            paths.append(f"generating from library2/main.py random_{tag}")
        return [generator.Path(p) for p in paths]
