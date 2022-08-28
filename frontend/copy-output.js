const fs = require("fs/promises");
const fsSync = require("fs")
const path = require("path");

copyOutput().then()
    .catch(console.error)

async function copyOutput() {
    const inputPath = path.join(".", "build");

    if (!fsSync.existsSync(inputPath)) {
        console.error("Missing input files cannot copy");
        return
    }

    const outputPath = path.join("..", "backend", "public");

    if (fsSync.existsSync(outputPath)) {
        console.log("Removing output directory")
        await fs.rm(outputPath, {recursive: true});
    }

    console.log("Copying Build Files")
    await copyFolder(inputPath, outputPath)
}

async function copyFolder(source, target) {
    if (!fsSync.existsSync(target)) {
        await fs.mkdir(target);
    }
    const files = await fs.readdir(source)
    const promises = files.map(async (file) => {
        const inPath = path.join(source, file)
        const outPath = path.join(target, file)
        const isDirectory = (await fs.lstat(inPath)).isDirectory()
        if (isDirectory) {
            await copyFolder(inPath, outPath)
        } else {
            await fs.copyFile(inPath, outPath)
        }
    });
    await Promise.all(promises);
}
