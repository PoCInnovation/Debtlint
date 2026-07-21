import * as core from '@actions/core';
import * as github from '@actions/github';
import { Octokit } from '@octokit/action';
import { spawn } from 'child_process';

import { diagnosticSchema } from './schemas.js';

const octokit = new Octokit();

/**
 * Execute a command in a child process
 * @param {string} cmd
 * @param {string[]} args
 * @returns {Promise<string>}
 */
const execCommand = (cmd, args = []) => {
  return new Promise((resolve, reject) => {
    const process = spawn(cmd, args);
    let output;
    let errorOutput;

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

const rawOutput = await execCommand('cargo', ['run', 'main.rs']);
const diagnostic = diagnosticSchema.parse(JSON.parse(rawOutput));

const range = diagnostic.ranges[0];

await octokit.request('POST /repos/{owner}/{repo}/pulls/{pull_number}/comments', {
  owner: github.context.repo.owner,
  repo: github.context.repo.repo,
  pull_number: github.context.payload.pull_request.number,
  body: diagnostic.code_description,
  commit_id: github.context.payload.pull_request.head.sha,
  path: diagnostic.source,
  start_line: range.start.line,
  start_side: 'RIGHT',
  line: range.end.line,
  side: 'RIGHT',
  headers: {
    'X-GitHub-Api-Version': '2026-03-10',
  },
});