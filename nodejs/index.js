// The JavaScript Data() object has alwasy been confusing to me
// I used this YouTube video https://www.youtube.com/watch?v=GmwNPEZict4

const dateOptions = {
  timeZone: "America/New_York",
  year: "numeric",
  month: "long",
  day: "numeric",
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
};

const date = new Date().toLocaleString("en-US", dateOptions);

console.log("Hello ASL!"); // required text
console.log(`Process completed on: ${date} EST`); // Date in string template literal
