import { PullRequestSchema, type PullRequestContext } from '@/typings/github'
import type { GithubContext } from "@/typings/github";

export function parsePullRequestContext(githubContext: GithubContext): PullRequestContext
{
    const pullRequest = githubContext.payload.pull_request;
    if (pullRequest === undefined) {
        throw new Error("Pull request number is not defined")
    }
    const sha = pullRequest.head?.sha;
    if (typeof sha !== "string" || sha.length === 0) {
        throw new Error("Pull request head SHA is not defined")
    }
    return PullRequestSchema.parse({
        owner: githubContext.repo.owner,
        repo: githubContext.repo.repo,
        issueNumber: pullRequest.number,
        sha,
    });
}