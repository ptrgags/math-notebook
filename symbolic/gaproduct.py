import kingdon
import argparse


def display_componentwise(multivec, basis):
    for index, value in zip(multivec.keys(), multivec.values()):
        component = value * basis[index]
        print(component)


BINARY_PRODUCT_FUNCS = {
    'gp': {
        'label': "Geometric Product",
        'func': lambda a, b: a * b
    },
    'inner': {
        'label': "Inner Product",
        'func': lambda a, b: a | b
    },
    'outer': {
        'label': 'Outer Product',
        'func': lambda a, b: a ^ b
    },
    'regressive': {
        'label': 'Regressive Product',
        'func': lambda a, b: a & b,
    },
    'sandwich': {
        'label': "Sandwich Product (ABA^{-1})",
        'func': lambda a, b: (a * b * a.inv()),
    },
    'unit_sandwich': {
        'label': "Sandwich product (ABA.rev()))",
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


def display_binary_product(algebra, product):
    basis = algebra.basis
    even_a = algebra.make_even('A')
    even_b = algebra.make_even('B')
    odd_a = algebra.make_odd('A')
    odd_b = algebra.make_odd('B')

    func = product['func']
    print(product['label'], "========================")
    print("Even, Even")
    display_componentwise(func(even_a, even_b), basis)
    print("Even, Odd")
    display_componentwise(func(even_a, odd_b), basis)
    print("Odd, Even")
    display_componentwise(func(odd_a, even_b), basis)
    print("Odd, Odd")
    display_componentwise(func(odd_a, odd_b), basis)


PRODUCT_CHOICES = BINARY_PRODUCT_FUNCS.keys()


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


ALGEBRAS = {
    'pga2': PGA2D(),
    'pga3': PGA3D(),
}

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("algebra", choices=['pga2', 'pga3'])
    parser.add_argument("product", choices=PRODUCT_CHOICES)
    args = parser.parse_args()

    algebra = ALGEBRAS[args.algebra]
    product = BINARY_PRODUCT_FUNCS[args.product]
    display_binary_product(algebra, product)
