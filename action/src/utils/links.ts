import { isAbsolute, join } from "path";

import { Fragment } from '@/typings/diagnostic';
import { PullRequestContext } from '@/typings/github';

export function resolveRepoRelativePath(source: string): string {
    let path = source.replace(/\\/g, "/");

    const workspace = process.env.GITHUB_WORKSPACE?.replace(/\\/g, "/").replace(/\/$/, "");
    if (workspace && path.startsWith(`${workspace}/`)) {
        path = path.slice(workspace.length + 1);
    }
    return path.replace(/^\.\//, "");
}

export function resolveWorkspaceFilePath(source: string): string {
    if (isAbsolute(source)) {
        return source;
    }
    const workspace = process.env.GITHUB_WORKSPACE;
    if (!workspace) {
        throw new Error("GITHUB_WORKSPACE is not set");
    }
    return join(workspace, resolveRepoRelativePath(source));
}

export function buildBlobUrl(context: PullRequestContext, range: Fragment): string {
    const path = resolveRepoRelativePath(range.source);
    const base = `https://github.com/${context.owner}/${context.repo}/blob/${context.sha}/${path}`;

    if (range.lines === null) {
        return base;
    }
    if (range.lines.start === range.lines.end) {
        return `${base}#L${range.lines.start}`;
    }
    return `${base}#L${range.lines.start}-L${range.lines.end}`;
}
