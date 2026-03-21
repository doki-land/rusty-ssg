
import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const srcDir = path.join(__dirname, 'src');
const toolsDir = path.join(srcDir, 'tools');
const disabledToolsDir = path.join(srcDir, 'tools.disabled');

async function main() {
    try {
        // Check if tools directory exists
        await fs.access(toolsDir);
        console.log('Renaming tools directory to tools.disabled...');
        await fs.rename(toolsDir, disabledToolsDir);
        console.log('Successfully renamed.');
    } catch (err) {
        if (err.code === 'ENOENT') {
            console.log('tools directory does not exist, checking for tools.disabled...');
            try {
                await fs.access(disabledToolsDir);
                console.log('tools.disabled exists, no action needed.');
            } catch (err2) {
                console.error('Neither tools nor tools.disabled exist!');
            }
        } else {
            console.error('Error:', err);
        }
    }
}

main();
