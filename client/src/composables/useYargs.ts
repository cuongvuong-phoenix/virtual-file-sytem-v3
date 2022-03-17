import { type Argv } from 'yargs';
// @ts-expect-error external library.
import Yargs from 'https://unpkg.com/yargs@16.0.0-beta.1/browser.mjs';

export enum YargsCommand {
  CD = 'cd',
  CR = 'cr',
  CAT = 'cat',
  LS = 'ls',
  FIND = 'find',
  UP = 'up',
  MV = 'mv',
  RM = 'rm',
}

export const yargs = (Yargs() as Argv)
  .command(`${YargsCommand.CD} <FOLDER_PATH>`, 'Change current working directory to specified FOLDER_PATH')
  .command(
    `${YargsCommand.CR} <PATH> [DATA]`,
    'Create a new file if DATA is specified, otherwise create a new folder at the specified PATH',
    {
      p: {
        alias: 'parents',
        type: 'boolean',
        describe: 'Create missing parent folders (if any)',
        default: false,
      },
    }
  )
  .command(`${YargsCommand.CAT} <FILE_PATH>`, 'Show the content of a file at FILE_PATH')
  .command(`${YargsCommand.LS} [FOLDER_PATH]`, 'List out all items directly under a folder')
  .command(
    `${YargsCommand.FIND} <NAME> [FOLDER_PATH]`,
    'Search all files/folders whose name contains the substring NAME (also find in subfolder)'
  )
  .command(
    `${YargsCommand.UP} <PATH> <NAME> [DATA]`,
    'Update the file/folder at PATH to have new NAME and, optionally, new DATA'
  )
  .command(`${YargsCommand.MV} <PATH> <FOLDER_PATH>`, 'Move a file/folder at PATH into the destination FOLDER_PATH')
  .command(`${YargsCommand.RM} <PATHS..>`, 'Remove files/folders at the specified PATHs')
  .wrap(null)
  .version('3.0.0');
