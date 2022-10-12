T = int(input())
    
for _ in range(T):
    x = int(input())
    while x != 1:
      print(x, end=' ')
      if x%2 == 0 and  x  != 1:
         y = x//2         
         x = y
      elif x%2 == 1 and x  != 1:
          y = (3*x) +1          
          x = y
    print(1)
