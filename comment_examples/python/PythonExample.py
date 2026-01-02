# This is a Python single line comment
def hello_world():
    """This is a Python docstring comment"""
    print("Hello, World!")  # Another comment
    
# Multiple line comment
# spanning several lines
x = 5

def calculate_sum(a, b):
    # Function to add two numbers
    return a + b  # Return the sum

# TODO: Add error handling
# FIXME: This needs optimization
class Calculator:
    """A simple calculator class"""
    
    def __init__(self):
        # Initialize the calculator
        self.result = 0
        
    def add(self, value):
        """Add a value to the result"""
        self.result += value  # Update result
        return self.result