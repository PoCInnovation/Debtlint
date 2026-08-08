import { z } from "zod";
import { Diagnostic, diagnosticSchema } from '@/typings/diagnostic';

import { getOctokit, context } from '@actions/github';

import { execCommand } from "@/utils/executor"
import { parsePullRequestContext } from "@/utils/convert"
import { createPullRequestComments } from "@/api/routes";

async function run() {
    //const GITHUB_TOKEN: string = process.env.GITHUB_TOKEN!;
    //const octokit = getOctokit(GITHUB_TOKEN);
    //const eventContext = parsePullRequestContext(context)

    const rawOutput = await execCommand('cargo', ['run', 'main.rs']);
    const diagnostic: Diagnostic[] = z.array(diagnosticSchema).parse(JSON.parse(rawOutput));

    console.log(diagnostic)
    //await createPullRequestComments(octokit, diagnostic, eventContext)
}

run()