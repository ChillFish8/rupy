import dis


class Foo:
    def __init__(self):
        self.x = 1

    def __add__(self, other):
        return self.x + other

    def hello(self) -> str:
        return f"aggg{self.x}"


def test():
    x = Foo()
    x.a = 123

    x.hello()

dis.dis(test)