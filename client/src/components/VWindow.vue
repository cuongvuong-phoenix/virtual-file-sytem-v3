<template>
  <div class="rounded-lg bg-gray-800 text-gray-300 overflow-hidden">
    <!-- "Header" -->
    <div class="flex items-center bg-gray-900 justify-between space-x-16 p-2">
      <!-- "Left" -->
      <div class="flex items-center space-x-2">
        <div class="rounded-full w-4 h-4 bg-cyan-400"></div>
      </div>
      <!-- END "Left" -->

      <!-- "Right" -->
      <div class="flex items-center justify-end space-x-2">
        <div class="rounded-full w-4 h-4 bg-amber-500"></div>
        <div class="rounded-full w-4 h-4 bg-green-600"></div>
        <div class="rounded-full w-4 h-4 bg-rose-600"></div>
      </div>
      <!-- END "Right" -->
    </div>
    <!-- END "Header" -->

    <!-- "Body" -->
    <div
      ref="windowBodyRef"
      class="px-2 py-4 space-y-4 min-h-[16rem] h-[calc(100vh-23rem)] overflow-auto"
      style="--scrollbar--thumb: #6b7280; --scrollbar--thumb-hover: #7a808d"
    >
      <VWindowBlock v-for="block in blocks" :key="block.id" :block="block" />

      <VWindowBlock :block="commandBlock" @enter="(value) => onEnter(value)" />
    </div>
    <!-- END "Body" -->
  </div>
</template>

<script setup lang="ts">
  import { type Ref, nextTick, onMounted, ref, watch } from 'vue';
  import { parseISO } from 'date-fns';
  import { yargs } from '~/composables';

  const windowBodyRef = ref<HTMLDivElement | null>(null);

  onMounted(() => {
    if (windowBodyRef.value) {
      windowBodyRef.value.scrollTop = windowBodyRef.value.scrollHeight;
    }
  });

  /* ----------------------------------------------------------------
  Blocks
  ---------------------------------------------------------------- */
  const blockCount = ref(0);

  const blocks = ref([]) as Ref<Block[]>;

  blocks.value.push(
    // `cd /usr`
    {
      id: blockCount.value++,
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
      id: blockCount.value++,
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
      id: blockCount.value++,
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
      id: blockCount.value++,
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
      id: blockCount.value++,
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
      id: blockCount.value++,
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
      id: blockCount.value++,
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
      id: blockCount.value++,
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
      id: blockCount.value++,
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
    }
  );

  /* ----------------------------------------------------------------
  Command Block
  ---------------------------------------------------------------- */
  const commandBlock = ref({
    id: -1,
    workingNode: {
      id: 3,
      path: ['usr', 'holistic'],
    },
    isCommand: true,
    createdAt: new Date(),
  }) as Ref<Block>;

  /* ----------------------------------------------------------------
  Handler
  ---------------------------------------------------------------- */
  function onEnter(value: string) {
    yargs.parse(value, async (err: any, argv: any, output: string) => {
      const block: Block = {
        id: blockCount.value++,
        workingNode: commandBlock.value.workingNode,
        command: value,
        parsedArgv: argv,
        data: output.length > 0 ? output : undefined,
        error: err,
        createdAt: new Date(),
      };

      blocks.value.push(block);

      await nextTick();

      commandBlock.value = {
        ...commandBlock.value,
        createdAt: new Date(),
      };

      if (windowBodyRef.value) {
        windowBodyRef.value.scrollTop = windowBodyRef.value.scrollHeight;
      }
    });
  }
</script>
