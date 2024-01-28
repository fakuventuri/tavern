const fs = require("node:fs");

const content = "Some content!";

fs.writeFile("./testJS.txt", content, (err) => {
  if (err) {
    console.error(err);
  } else {
    // file written successfully
  }
});
