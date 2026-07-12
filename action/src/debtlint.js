import { ChildProcess, spawn } from 'child_process';


/**
 * Execute a command asynchronously.
 * @param {string} cmd
 * @param {array} arguments
 */
export const execCommand = (cmd, arg = []) => {
    return spawn(cmd, arg)
};

// convert object to json
// parse list of duplicate element
// send element with the api github