#!/usr/bin/env sage

from sage.all import *

q = 57896044618658097711785492504343953926225696987256860989792804023844074237167

a4  = 31257679751294767726203335542381905569456340380844946531343019374821596229325
a6  = 40361771284572583964927969700930680951746224013842818163834336483522497112790

F = GF(q)
E = EllipticCurve(F, [0, 0, 0, a4, a6])

num_points = E.cardinality()
assert num_points == 2^255 - 19

def find_basept():
    for u_int in range(1, 1000):
        u = F(u_int)
        v2 = u^3 + 31257679751294767726203335542381905569456340380844946531343019374821596229325*u + 40361771284572583964927969700930680951746224013842818163834336483522497112790
        
        if not v2.is_square():
            continue
        v = v2.sqrt()
        pt = E(u, v)
        order = pt.order()
        if order > 2 and order.is_prime():
            return (order, pt)
        
(order, pt) = find_basept()
print(order, pt)
assert order == 2^255 - 19