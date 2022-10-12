T = int(input())
for _ in range(T):
    user_input = input()
    a = tuple(int(x) for x in user_input.split(' '))
    (x1, y1) = a
    real_cost = 0
    for x in range(x1, y1+1):
        dummy_cost = x**2
        real_cost += dummy_cost

    print(real_cost % (10**9 + 7))
