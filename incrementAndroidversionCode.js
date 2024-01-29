const fs = require("node:fs");

const file_path = "./mobile/manifest.yaml";

let content = fs.readFileSync(file_path, "utf-8");

let versionCode = Number(content.match("version_code: ([0-9]+) ")[1]);

console.log(`Actual version_code: ${versionCode}`);

let newContent = content.replace(
  `version_code: ${versionCode} `,
  `version_code: ${versionCode + 1} `
);

fs.writeFileSync(file_path, newContent);

console.log(`New version_code: ${versionCode + 1}`);
