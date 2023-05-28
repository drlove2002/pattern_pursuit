const { readdirSync } = require("fs");
const { exec } = require("child_process");
readdirSync("./src/app/js").forEach((file) => {
  if (file.endsWith(".js")) {
    exec(
      `terser ./src/app/js/${file} --compress --mangle -o ./docs/js/${file}`,
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
