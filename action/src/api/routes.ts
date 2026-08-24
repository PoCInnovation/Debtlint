import { readFile } from "fs/promises";

import { Diagnostic, Fragment } from '@/typings/diagnostic';
import { PullRequestContext, Octokit } from '@/typings/github';


async function getSuggestion(diagnostic: Diagnostic): Promise<string> {
    let content = "<!-- debtlint -->\n## Debtlint suggestion\n";
    content += `### ${diagnostic.description}\n`

    for (const range of diagnostic.ranges) {
        try {
            const data = await readFile(range.source, "utf-8");
            let fragment = "```" + range.source.split(".").pop() + "\n";
            const lines = data.split(/\r?\n/).slice(range.start.line, range.end.line);
            for (const line of lines) {
                fragment += line + "\n";
            }
            content += fragment + "```\n";
        } catch (err) {
            console.error(`Error reading ${range.source}:`, err);
        }
    }
    return content;
}

export async function createPullRequestComments(octokit: Octokit, diagnostics: Diagnostic[], context: PullRequestContext)
{
    for (const diagnostic of diagnostics) {
        await octokit.request('POST /repos/{owner}/{repo}/issues/{issue_number}/comments', {
            owner: context.owner,
            repo: context.repo,
            issue_number: context.issueNumber,
            body: await getSuggestion(diagnostic),
            headers: { 'X-GitHub-Api-Version': '2026-03-10'}
        });
    }
}

async function createThreadComments() {}

