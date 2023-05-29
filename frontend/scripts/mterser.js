const { readdirSync, mkdirSync } = require("fs");
const { exec } = require("child_process");
const targetFolder = "./static/js";

// Check if the target folder exists, create it if not
if (!readdirSync("./static").includes("js")) {
  mkdirSync(targetFolder);
}

readdirSync("./src/app/js").forEach((file) => {
  if (file.endsWith(".js")) {
    exec(
      `terser ./src/app/js/${file} --compress --mangle -o ${targetFolder}/${file}`,
      (e) => {
        if (e !== null) {
          console.log(`exec error: ${e}`);
        } else {
          console.log(`${file} minified`);
        }
      }
    );
  }
});
