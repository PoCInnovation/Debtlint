import { z } from "zod";
import { context, getOctokit } from "@actions/github";

export type Octokit = ReturnType<typeof getOctokit>;
export type GithubContext = typeof context;

export const PullRequestSchema = z.object({
    owner: z.string(),
    repo: z.string(),
    issueNumber: z.number()
});

export type PullRequestContext = z.infer<typeof PullRequestSchema>
