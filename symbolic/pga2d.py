import kingdon
import argparse

pga2d = kingdon.Algebra(2, 0, 1)
BASIS = list(pga2d.blades.values())


def even(label):
    return pga2d.multivector(e=f'{label}s', e01=f'{label}ox', e02=f'{label}oy', e12=f"{label}xy")


def odd(label):
    return pga2d.multivector(e0=f'{label}o', e1=f'{label}x', e2=f'{label}y', e012=f"{label}oxy")


def display_componentwise(multivec):
    for index, value in zip(multivec.keys(), multivec.values()):
        component = value * BASIS[index]
        print(component)


even_a = even('A')
even_b = even('B')
odd_a = odd('A')
odd_b = odd('B')

'''
print("Geometric Product ======================")
print("Even, Even = Even")
display_componentwise(even_a * even_b)
print("Even, Odd = Odd")
display_componentwise(even_a * odd_b)
print("opposite order:")
display_componentwise(odd_b * even_a)
print("Odd, Odd = Even")
display_componentwise(odd_a * odd_b)


def sandwich(a, b):
    return a * b * a.inv()


print("Sandwich product ======================")
print("Even, Even")
display_componentwise(sandwich(even_a, even_b))
print("Even, Odd")
display_componentwise(sandwich(even_a, odd_b))
print("opposite order:")
display_componentwise(sandwich(odd_b, even_a))
print("Odd, Odd = Even")
display_componentwise(sandwich(odd_a, odd_b))
'''
