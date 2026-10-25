import { readFile } from "fs/promises";

import { Diagnostic, Fragment } from '@/typings/diagnostic';
import { PullRequestContext, Octokit } from '@/typings/github';
import { buildBlobUrl, resolveRepoRelativePath } from '@/utils/links';

function fragmentHeading(context: PullRequestContext, range: Fragment): string {
    const path = resolveRepoRelativePath(range.source);
    const link = `[\`${path}\`](${buildBlobUrl(context, range)})`;

    if (range.lines === null) {
        return link;
    }
    if (range.lines.start === range.lines.end) {
        return `${link} · line ${range.lines.start}`;
    }
    return `${link} · lines ${range.lines.start}–${range.lines.end}`;
}

async function getSuggestion(diagnostic: Diagnostic, context: PullRequestContext): Promise<string> {
    let content = "<!-- debtlint -->\n## Debtlint suggestion\n";
    content += `### ${diagnostic.ranges.length} duplicated copies\n`

    for (const range of diagnostic.ranges) {
        try {
            content += fragmentHeading(context, range);

            const data = await readFile(range.source, "utf-8");
            let fragment = "```" + range.source.split(".").pop() + "\n";
            if (range.lines !== null) {
                const lines = data.split(/\r?\n/).slice(range.lines.start - 1, range.lines.end);
                for (const line of lines) {
                    fragment += line + "\n";
                }
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
            body: await getSuggestion(diagnostic, context),
            headers: { 'X-GitHub-Api-Version': '2026-03-10'}
        });
    }
}

async function createThreadComments() {}
