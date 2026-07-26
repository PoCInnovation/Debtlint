import * as core from '@actions/core';
import * as github from '@actions/github';
import { Octokit } from '@octokit/action';

import { execCommand } from "./executor.js"
import { createComment } from "./api.js";


async function run() {
    const GITHUB_TOKEN = core.getInput('GITHUB_TOKEN');
    const octokit = github.getOctokit(GITHUB_TOKEN);
    const { context = {} } = github;
    const { pull_request } = context.payload;

    const rawOutput = await execCommand('cargo', ['run', 'main.rs']);
    console.log(rawOutput)
    const diagnostic = diagnosticSchema.parse(JSON.parse(rawOutput));
    await createComment(octokit, diagnostic, context)
}
