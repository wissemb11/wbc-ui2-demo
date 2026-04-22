// =============================================================================
// samples.js — Multiple JavaScript output examples for testing & demos
// Copy this entire file and run it with Node.js or in the browser console
// =============================================================================
alert('ddddddddddddddd')
console.log("=== 1. Basic Outputs ===");
console.log("Hello, world!");
console.log(42);
console.log(true);
console.log(null);
console.log(undefined);

console.log("\n=== 2. Strings & Template Literals ===");
const name = "Ada Lovelace";
const year = 1843;
console.log(`Did you know? ${name} wrote the first algorithm in ${year}.`);
console.log("Escaped characters: \tTab \\ Backslash \"Quotes\"");

console.log("\n=== 3. Numbers & Math ===");
console.log("Math.PI =", Math.PI);
console.log("42 in hex:", 0x2A);
console.log("BigInt example:", 12345678901234567890n);
console.log("Random number:", Math.random());
console.log("Rounded:", Math.round(3.14159 * 100) / 100);

console.log("\n=== 4. Arrays & Objects ===");
const fruits = ["apple", "banana", "cherry"];
console.log("Fruits:", fruits);
console.log("Second fruit:", fruits[1]);

const person = {
  name: "Marie Curie",
  born: 1867,
  awards: ["Nobel Physics 1903", "Nobel Chemistry 1911"],
  alive: false
};
console.log("Person object:", person);
console.log("Awards count:", person.awards.length);

console.log("\n=== 5. JSON Pretty Print ===");
console.log(JSON.stringify(person, null, 2));

console.log("\n=== 6. Table Output (Node.js / modern browsers) ===");
console.table(fruits);
console.table([person, { name: "Albert Einstein", born: 1879, alive: false }]);

console.log("\n=== 7. Colored Output (Node.js with chalk-like style) ===");
// In browser you can use %c for styling
console.log("%cSuccess!%c This is green", "color: green; font-weight: bold", "");
console.log("%cWarn!%c Something might be wrong", "color: orange; font-size: 14px", "");
console.log("%cErr!%c Something went wrong", "color: red; background: #fee", "");

console.log("\n=== 8. Timed Operations ===");
console.time("Loop timer");
let sum = 0;
for (let i = 1; i <= 1_000_000; i++) {
  sum += i;
}
console.timeEnd("Loop timer");
console.log("Sum of first 1M numbers =", sum.toLocaleString());

console.log("\n=== 9. Assertions (great for demos) ===");
console.assert(1 + 1 === 2, "Math still works");
console.assert(1 + 1 === 0, "This assertion will fail → you’ll see the message");

console.log("\n=== 10. Grouped Output ===");
console.group("User Login Process");
console.log("Validating credentials...");
console.log("Credentials OK");
console.group("Loading profile");
console.log("Profile picture loaded");
console.log("Preferences loaded");
console.groupEnd();
console.log("Redirecting to dashboard...");
console.groupEnd();

console.log("\n=== ALL DONE ===");
console.log("This file demonstrates 10 different console output styles.");
console.log("Perfect for testing syntax highlighters, logs, or documentation generators.");