import { context } from "@actions/github";
import { PullRequestSchema, type PullRequestContext } from '@/typings/github'
import type { GithubContext } from "@/typings/github";

export function parsePullRequestContext(githubContext: GithubContext): PullRequestContext
{
    if (githubContext.payload.pull_request === undefined) {
        throw new Error("Pull request number is not defined")
    }
    return PullRequestSchema.parse({
        owner: githubContext.repo.owner,
        repo: githubContext.repo.repo,
        issueNumber: githubContext.payload.pull_request.number
    });
}