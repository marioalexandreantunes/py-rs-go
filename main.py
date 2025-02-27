import time

def complex_calculation(n):
    total = 0
    for i in range(n):
        total += i * i
        if i % 2 == 0:
            total += i
        else:
            total -= i
    return total

def main():
    n = 100_000_000
    
    start_time = time.time()
    result = complex_calculation(n)
    end_time = time.time()
    
    execution_time = end_time - start_time
    print(f"Python Results:")
    print(f"Result: {result}")
    print(f"Time taken: {execution_time:.4f} seconds")

if __name__ == "__main__":
    main()