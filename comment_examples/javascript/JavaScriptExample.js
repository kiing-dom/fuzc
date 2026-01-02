// This is a JavaScript single line comment
function helloWorld() {
    /* This is a multi-line comment
       in JavaScript */
    console.log("Hello, World!"); // Another comment
}

// Another single line comment
const numbers = [1, 2, 3, 4, 5];

/* 
 * Multi-line comment with asterisks
 * for better formatting
 */
function calculateSum(arr) {
    // Use reduce to sum all numbers
    return arr.reduce((sum, num) => {
        return sum + num; // Add each number to the sum
    }, 0);
}

// TODO: Add input validation
// FIXME: Handle edge cases
const result = calculateSum(numbers); // Calculate the sum