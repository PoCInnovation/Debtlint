import { z } from "zod";
import { Diagnostic, diagnosticSchema } from '@/typings/diagnostic';

import { getOctokit, context } from '@actions/github';
import { getInput } from "@actions/core";

import { addOption } from "@/utils/flags";
import { execCommand } from "@/utils/executor"
import { parsePullRequestContext } from "@/utils/convert"
import { createPullRequestComments } from "@/api/routes";

async function run() {
    const GITHUB_TOKEN: string = process.env.GITHUB_TOKEN!;
    const octokit = getOctokit(GITHUB_TOKEN);
    const eventContext = parsePullRequestContext(context)

    const workspace = process.env.GITHUB_WORKSPACE;
    if (!workspace) {
        throw new Error("GITHUB_WORKSPACE is not set");
    }

    const args: string[] = [];
    addOption(args, "--report-format", "json");
    addOption(args, "--load-vocab", getInput("vocab-path"));

    const rawOutput = await execCommand("debtlint", args, workspace);
    const diagnostics: Diagnostic[] = z.array(diagnosticSchema).parse(JSON.parse(rawOutput));
    await createPullRequestComments(octokit, diagnostics, eventContext)
}

run()