import { spawn } from 'child_process';

import { diagnosticSchema } from './schemas.js';

/**
 * Execute a command in a child process
 * @param {string} cmd
 * @param {string[]} args
 * @returns {Promise<string>}
 */
const execCommand = (cmd, args = []) => {
    return new Promise((resolve, reject) => {
        const process = spawn(cmd, args);
        let output = ""
        let errorOutput = ""
        
        process.stdout.on('data', (data) => {
            output += data.toString();
        });

        process.stderr.on('data', (data) => {
            errorOutput += data.toString();
        });
        
        process.on('close', (code) => {
            if (code !== 0) {
                return reject(new Error(`Process exited with code ${code}: ${errorOutput}`));
            }
            resolve(output);
        });
        process.on('error', reject);
    });
};
