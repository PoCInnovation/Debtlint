import { spawn } from 'child_process';
import { Octokit } from '@octokit/action';

import { diagnosticSchema } from './schemas.js';



/**
 * Create a comment on a pull request
 * @param { Octokit } octokit
 * @param { Diagnostic } diagnostic
 * @param { Object } context
 */
export async function createComment(octokit, diagnostic, context)
{
    for (const range of diagnostic.ranges) {
        await octokit.request('POST /repos/{owner}/{repo}/issues/{issue_number}/comments', {
            owner: context.repo.owner,
            repo: context.repo.repo,
            issue_number: context.payload.pull_request.number,
            body: diagnostic.code_description,
            headers: { 'X-GitHub-Api-Version': '2026-03-10'}
        })
    }
}