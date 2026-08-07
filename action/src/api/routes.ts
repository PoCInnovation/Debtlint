import { spawn } from 'child_process';
import { getOctokit, context } from '@actions/github';

import { Diagnostic } from '../typings/diagnostic';
import { PullRequestContext, Octokit } from '../typings/github';


export async function createPullRequestComments(octokit: Octokit, diagnostic: Diagnostic[], context: PullRequestContext)
{
    for (const range of diagnostic) {
        await octokit.request('POST /repos/{owner}/{repo}/issues/{issue_number}/comments', {
            owner: context.owner,
            repo: context.repo,
            issue_number: context.issueNumber,
            body: "**Debtlint suggestion**",
            headers: { 'X-GitHub-Api-Version': '2026-03-10'}
        });
    };
}

async function createThreadComments() {}

