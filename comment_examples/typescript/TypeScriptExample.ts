// TypeScript single line comment
interface User {
    name: string; // Name field comment
    age: number;  /* Age field comment */
}

/* Multi-line comment
   in TypeScript */
const user: User = {
    name: "John",
    age: 30
};

// Generic type comment
type ApiResponse<T> = {
    data: T;     // The response data
    status: number; /* HTTP status code */
};

/**
 * JSDoc style comment in TypeScript
 * @param users Array of user objects
 * @returns Formatted user names
 */
function formatUserNames(users: User[]): string[] {
    // Map over users to extract names
    return users.map(user => {
        return user.name.toUpperCase(); // Convert to uppercase
    });
}

// TODO: Add error handling for null users
// FIXME: Optimize for large arrays