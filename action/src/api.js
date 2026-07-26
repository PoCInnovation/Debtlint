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
        await octokit.request('POST /repos/{owner}/{repo}/pulls/{pull_number}/comments', {
            owner: context.repo.owner,
            repo: context.repo.repo,
            pull_number: context.payload.pull_request.number,
            body: diagnostic.code_description,
            commit_id: context.payload.pull_request.head.sha,
            path: diagnostic.source,
            start_line: range.start.line,
            start_side: 'RIGHT',
            line: range.end.line,
            side: 'RIGHT',
            headers: { 'X-GitHub-Api-Version': '2026-03-10'}
        })
    }
}