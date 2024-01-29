from generatorworld import exports

import random

class Generator(exports.Generator):
    def generate(self) -> int:
        return random.randint(50, 100)

    def generate_many(self) -> list[int]:
        return [random.randint(10, 20)]