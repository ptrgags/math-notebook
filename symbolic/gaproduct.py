import kingdon
import argparse


def display_componentwise(multivec, basis):
    '''
    for index, value in zip(multivec.keys(), multivec.values()):
        component = value * basis[index]
        print(component)
    '''
    for x in multivec.grades:
        grade_str = str(multivec.grade(x))
        print(grade_str.replace("+ (", "\n("))


BINARY_PRODUCT_FUNCS = {
    'gp': {
        'label': "Geometric Product",
        'func': lambda a, b: a * b
    },
    'dot': {
        'label': "Inner Product",
        'func': lambda a, b: a | b
    },
    'wedge': {
        'label': 'Outer Product',
        'func': lambda a, b: a ^ b
    },
    'vee': {
        'label': 'Regressive Product',
        'func': lambda a, b: a & b,
    },
    'sandwich': {
        'label': "Sandwich Product (A * B * A^{-1})",
        'func': lambda a, b: (a * b * a.inv()),
    },
    'unit_sandwich': {
        'label': "Sandwich product (A * B * A.rev()))",
        "func": lambda a, b: a >> b
    },
    'left_contraction': {
        'label': "Left contraction",
        "func": lambda a, b: a.lc(b)
    },
    'right_contraction': {
        'label': "Right contraction",
        "func": lambda a, b: a.rc(b)
    },
    'commutator': {
        'label': "Commutator product",
        "func": lambda a, b: a.cp(b)
    },
    'anticommutator': {
        'label': "Anticommutator product",
        "func": lambda a, b: a.acp(b)
    }
}


def display_binary_product(algebra, product, input_a, input_b):
    basis = algebra.basis

    func = product['func']
    print(product['label'], "========================")
    print("A:", input_a)
    print("B:", input_b)
    display_componentwise(func(input_a, input_b), basis)


PRODUCT_CHOICES = BINARY_PRODUCT_FUNCS.keys()


class PGA1D:
    def __init__(self):
        self.alg = kingdon.Algebra(1, 0, 1)
        self.basis = list(self.alg.blades.values())

    def make_even(self, label):
        return self.alg.multivector(
            e=f'{label}s',
            e01=f'{label}ox',
        )

    def make_odd(self, label):
        return self.alg.multivector(
            e0=f'{label}o',
            e1=f'{label}x',
        )


class PGA2D:
    def __init__(self):
        self.alg = kingdon.Algebra(2, 0, 1)
        self.basis = list(self.alg.blades.values())

    def make_even(self, label):
        return self.alg.multivector(
            e=f'{label}s',
            e01=f'{label}ox',
            e02=f'{label}oy',
            e12=f"{label}xy"
        )

    def make_odd(self, label):
        return self.alg.multivector(
            e0=f'{label}o',
            e1=f'{label}x',
            e2=f'{label}y',
            e012=f"{label}oxy"
        )


class PGA3D:
    def __init__(self):
        self.alg = kingdon.Algebra(3, 0, 1)
        self.basis = list(self.alg.blades.values())

    def make_even(self, label):
        return self.alg.multivector(
            e=f'{label}s',
            e01=f'{label}ox',
            e02=f'{label}oy',
            e03=f'{label}oz',
            e12=f"{label}xy",
            e13=f"{label}xz",
            e23=f"{label}yz",
            e0123=f"{label}oxyz"
        )

    def make_odd(self, label):
        return self.alg.multivector(
            e0=f'{label}o',
            e1=f'{label}x',
            e2=f'{label}y',
            e3=f'{label}z',
            e012=f"{label}oxy",
            e013=f"{label}oxz",
            e023=f"{label}oyz",
            e123=f"{label}xyz",
        )


class CGA1D:
    def __init__(self):
        self.alg = kingdon.Algebra(2, 1)
        self.basis = list(self.alg.blades.values())

    def make_even(self, label):
        return self.alg.multivector(
            e=f'{label}s',
            e12=f'{label}xp',
            e13=f'{label}xn',
            e23=f"{label}pn",
        )

    def make_odd(self, label):
        return self.alg.multivector(
            e1=f'{label}x',
            e2=f'{label}p',
            e3=f'{label}n',
            e123=f"{label}xpn",
        )


class CGA2D:
    def __init__(self):
        self.alg = kingdon.Algebra(3, 1)
        self.basis = list(self.alg.blades.values())

    def make_even(self, label):
        return self.alg.multivector(
            e=f'{label}s',
            e12=f'{label}xy',
            e13=f'{label}xp',
            e14=f'{label}xn',
            e23=f"{label}yp",
            e24=f"{label}yn",
            e34=f"{label}pn",
            e1234=f"{label}xypn"
        )

    def make_odd(self, label):
        return self.alg.multivector(
            e1=f'{label}x',
            e2=f'{label}y',
            e3=f'{label}p',
            e4=f'{label}n',
            e123=f"{label}xyp",
            e124=f'{label}xyn',
            e134=f'{label}xpn',
            e234=f'{label}ypn'
        )


class CGA3D:
    def __init__(self):
        self.alg = kingdon.Algebra(4, 1)
        self.basis = list(self.alg.blades.values())

    def make_even(self, label):
        return self.alg.multivector(
            e=f'{label}s',
            e12=f'{label}xy',
            e13=f'{label}xz',
            e14=f'{label}xp',
            e15=f'{label}xn',
            e23=f"{label}yz",
            e24=f"{label}yp",
            e25=f"{label}yn",
            e34=f"{label}zp",
            e35=f"{label}zn",
            e45=f"{label}pn",
            e1234=f"{label}xyzn",
            e1235=f"{label}xyzn",
            e1245=f"{label}xypn",
            e1345=f"{label}xzpn",
            e2345=f"{label}yzpn",
        )

    def make_odd(self, label):
        return self.alg.multivector(
            e1=f'{label}x',
            e2=f'{label}y',
            e3=f'{label}z',
            e4=f'{label}p',
            e5=f'{label}n',
            e123=f'{label}xyz',
            e124=f'{label}xyp',
            e125=f'{label}xyn',
            e134=f'{label}xzp',
            e135=f'{label}xzn',
            e145=f'{label}xpn',
            e234=f'{label}yzp',
            e235=f'{label}yzn',
            e245=f'{label}ypn',
            e345=f'{label}zpn',
            e12345=f'{label}xyzpn'
        )


ALGEBRAS = {
    'pga1': PGA1D(),
    'pga2': PGA2D(),
    'pga3': PGA3D(),
    'cga1': CGA1D(),
    'cga2': CGA2D(),
    'cga3': CGA3D(),
}

ALGEBRA_CHOICES = ALGEBRAS.keys()

INPUT_CHOICES = [
    "even", "odd", "scalar", "vec", "bivec", "trivec", "quadvec", "pentavec"
]


def get_input(label, algebra, input_type):
    even = algebra.make_even(label)
    odd = algebra.make_odd(label)
    match input_type:
        case "even":
            return even
        case "odd":
            return odd
        case "scalar":
            return even.grade(0)
        case "vec":
            return odd.grade(1)
        case "bivec":
            return even.grade(2)
        case "trivec":
            return odd.grade(3)
        case "quadvec":
            return even.grade(4)
        case "pentavec":
            return odd.grade(5)
        case _:
            raise Exception("invalid input type")


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("algebra", choices=ALGEBRA_CHOICES)
    parser.add_argument("product", choices=PRODUCT_CHOICES)
    parser.add_argument("input_a", choices=INPUT_CHOICES)
    parser.add_argument("input_b", choices=INPUT_CHOICES)
    args = parser.parse_args()

    algebra = ALGEBRAS[args.algebra]
    product = BINARY_PRODUCT_FUNCS[args.product]
    input_a = get_input("A", algebra, args.input_a)
    input_b = get_input("B", algebra, args.input_b)
    display_binary_product(algebra, product, input_a, input_b)
