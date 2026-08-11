import { readFile } from 'fs';

import { Diagnostic, Fragment } from '@/typings/diagnostic';
import { PullRequestContext, Octokit } from '@/typings/github';

async function getSuggestion(diagnostic: Diagnostic): Promise<string> {
    const workspace: string = process.env.GITHUB_WORKSPACE!;
    let content: string = "<!-- debtlint -->\n## Debtlint suggestion\n"

    console.log("Workspace: ", workspace)
    for (const range of diagnostic.ranges) {
        readFile(range.source, 'utf-8', (err: NodeJS.ErrnoException | null, data: string) => {
            if (err) console.error(err);
            let fragment: string = "```" + range.source
            const lines: string[] = data.split("/\r?\n/").slice(range.start.line, range.end.line)
            for (const line of lines) {
                fragment += line
            }
            content += (fragment + "```")
        });
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

//<!-- cpp linter action -->
//### clang-format suggestion
//
//```suggestion
//        raylib::Vector3 alignToWorldPlane(raylib::Vector3 vector) const;
//```
//### clang-tidy diagnostics
//- function 'alignToWorldPlane' should be marked [[nodiscard]] [[modernize-use-nodiscard](https://clang.llvm.org/extra/clang-tidy/checks/modernize/use-nodiscard.html)]
//
//```suggestion
//        [[nodiscard]] raylib::Vector3 alignToWorldPlane(
//            raylib::Vector3 vector) const;
//```

