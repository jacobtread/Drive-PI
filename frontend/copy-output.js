const fs = require("fs/promises");
const fsSync = require("fs")
const path = require("path");

// The input directory path (./build)
const inputPath = path.join(".", "build");
// The output directory path (../backend/public)
const outputPath = path.join("..", "backend", "public");

// Check the build files actually exist
if (fsSync.existsSync(inputPath)) {
    console.log("Copying Build Files")
    // Copy the build files
    copyFolder(inputPath, outputPath)
        .then(() => console.log("Successfully copied output"))
        .catch(e => console.error("Failed to copy output", e))
} else {
    console.error("Missing input files cannot copy");
}


/**
 * Recursive copy folder function. Recursively copies all files
 * and folders from the source path to the target path.
 *
 * Will remove the target directory if it exists.
 *
 * @param source The source path
 * @param target The target path
 * @returns {Promise<void>} Promise indicating the completion of the copy
 */
async function copyFolder(source, target) {
    if (fsSync.existsSync(target)) {
        await fs.rm(target, {recursive: true});
    }
    await fs.mkdir(target);
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
