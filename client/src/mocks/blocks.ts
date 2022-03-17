import { parseISO } from 'date-fns';

let count = 0;

export const blocks: Block[] = [
  // `cd /usr`
  {
    id: count++,
    workingNode: {
      id: 1,
      path: [] as string[],
      isFolder: true,
    },
    command: 'cd /usr',
    ready: true,
    createdAt: new Date(),
  },
  // `cr /usr/bin/new-file`
  {
    id: count++,
    workingNode: {
      id: 2,
      path: ['usr'],
      isFolder: true,
    },
    command: 'cr /usr/bin/new-file',
    parsedArgv: {
      _: ['cr', '/usr/bin/new-file'],
    },
    ready: true,
    data: {
      createdAt: parseISO('2022-03-17T09:38:59.838400Z'),
      id: 49,
      isFolder: false,
      path: ['usr', 'bin', 'new-file'],
    },
    createdAt: new Date(),
  },
  // `cr /usr/non-bin/non-exist-parent/new-file`
  {
    id: count++,
    workingNode: {
      id: 2,
      path: ['usr'],
      isFolder: true,
    },
    command: 'cr /usr/non-bin/non-exist-parent/new-file',
    parsedArgv: {
      _: ['cr', '/usr/non-bin/non-exist-parent/new-file'],
    },
    ready: true,
    data: {
      createdAt: parseISO('2022-03-17T09:39:58.886355Z'),
      id: 51,
      isFolder: false,
      path: ['usr', 'non-bin', 'non-exist-parent', 'new-file'],
    },
    createdAt: new Date(),
  },
  // `cat /usr/bin/sh`
  {
    id: count++,
    workingNode: {
      id: 2,
      path: ['usr'],
      isFolder: true,
    },
    command: 'cat /usr/bin/sh',
    parsedArgv: {
      _: ['cat', '/usr/bin/sh'],
    },
    ready: true,
    data: 'Data for `sh` file',
    createdAt: new Date(),
  },
  // `ls /share`
  {
    id: count++,
    workingNode: {
      id: 2,
      path: ['usr'],
      isFolder: true,
    },
    command: 'ls /share',
    parsedArgv: {
      _: ['ls', '/share'],
    },
    ready: true,
    data: [
      {
        createdAt: parseISO('2022-03-17T09:36:16.329440Z'),
        id: 45,
        isFolder: true,
        path: ['share', 'local'],
        size: 34,
      },
      {
        createdAt: parseISO('2022-03-17T09:36:16.329440Z'),
        id: 48,
        isFolder: true,
        path: ['share', 'lib'],
        size: 0,
      },
    ],
    createdAt: new Date(),
  },
  // `find /usr/holistic`
  {
    id: count++,
    workingNode: {
      id: 2,
      path: ['usr'],
      isFolder: true,
    },
    command: 'find o /usr/holistic',
    parsedArgv: {
      _: ['find', 'o', '/usr/holistic'],
    },
    ready: true,
    data: [
      {
        createdAt: parseISO('2022-03-17T09:36:16.329440Z'),
        id: 39,
        isFolder: true,
        path: ['usr', 'holistic', 'cuong'],
      },
      {
        createdAt: parseISO('2022-03-17T09:36:16.329440Z'),
        id: 42,
        isFolder: false,
        path: ['usr', 'holistic', 'cuong', 'b', 'hello'],
      },
      {
        createdAt: parseISO('2022-03-17T09:36:16.329440Z'),
        id: 43,
        isFolder: false,
        path: ['usr', 'holistic', 'nothing'],
      },
    ],
    createdAt: new Date(),
  },
  // `up /usr/holistic holistic-2`
  {
    id: count++,
    workingNode: {
      id: 2,
      path: ['usr'],
      isFolder: true,
    },
    command: 'up /usr/holistic holistic-2',
    parsedArgv: {
      _: ['up', '/usr/holistic holistic-2'],
    },
    ready: true,
    data: {
      createdAt: parseISO('2022-03-17T09:49:21.525986Z'),
      id: 60,
      isFolder: true,
      path: ['usr', 'holistic-2'],
    },
    createdAt: new Date(),
  },
  // `mv /usr/holistic /share/local`
  {
    id: count++,
    workingNode: {
      id: 2,
      path: ['usr'],
      isFolder: true,
    },
    command: 'mv /usr/holistic /share/local',
    parsedArgv: {
      _: ['up', '/usr/holistic /share/local'],
    },
    ready: true,
    data: {
      createdAt: parseISO('2022-03-17T09:49:21.525986Z'),
      id: 60,
      isFolder: true,
      path: ['share', 'local', 'holistic'],
    },
    createdAt: new Date(),
  },
  // `rm /share/lib/holistic /share/lib/holistic/cuong /zzzz`
  {
    id: count++,
    workingNode: {
      id: 2,
      path: ['usr'],
      isFolder: true,
    },
    command: 'rm /usr/holistic /share/local',
    parsedArgv: {
      _: ['rm', '/usr/holistic /share/local'],
    },
    ready: true,
    data: {
      nonExistedPaths: [['share', 'lib', 'holistic', 'cuong'], ['fkajsdfkjaskdfj']],
      removedPaths: [['share', 'lib', 'holistic']],
    },
    createdAt: new Date(),
  },
];
