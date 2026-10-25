export function addOption(args: string[], flag: string, value: string): void
{
    if (value) {
        args.push(flag, value);
    }
}
