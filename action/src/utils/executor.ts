import { spawn } from 'child_process';

export const execCommand = (cmd: string, args: string[] = []): Promise<string> => {
    return new Promise((resolve, reject) => {
        const process = spawn(cmd, args);
        let output: string = ""
        let errorOutput: string = ""
        
        process.stdout.on('data', (data: Buffer) => {
            output += data.toString();
        });

        process.stderr.on('data', (data: Buffer) => {
            errorOutput += data.toString();
        });
        
        process.on('close', (code: number) => {
            if (code !== 0) {
                return reject(new Error(`Process exited with code ${code}: ${errorOutput}`));
            }
            resolve(output);
        });
        process.on('error', reject);
    });
};
