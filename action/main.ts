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

    const args: string[] = [];
    addOption(args, "--load-vocab", getInput("config-path"));

    const rawOutput = await execCommand('cargo', ['run', 'main.rs'].concat(args));
    const diagnostics: Diagnostic[] = z.array(diagnosticSchema).parse(JSON.parse(rawOutput));
    await createPullRequestComments(octokit, diagnostics, eventContext)
}

run()