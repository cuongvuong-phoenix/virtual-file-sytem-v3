<template>
  <div class="overflow-hidden text-gray-300 bg-gray-800 rounded-lg">
    <!-- "Header" -->
    <div class="flex items-center justify-between p-2 space-x-16 bg-gray-900">
      <!-- "Left" -->
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 rounded-full bg-cyan-400"></div>
      </div>
      <!-- END "Left" -->

      <!-- "Right" -->
      <div class="flex items-center justify-end space-x-2">
        <div class="w-4 h-4 rounded-full bg-amber-500"></div>
        <div class="w-4 h-4 bg-green-600 rounded-full"></div>
        <div class="w-4 h-4 rounded-full bg-rose-600"></div>
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
      <div v-for="block in blocks" :key="block.id">
        <VWindowBlockHeader :block="block" />
        <VWindowBlockBody :block="block" />
      </div>

      <div v-if="!commandBlock.loading">
        <VWindowBlockHeader ref="commandBlockRef" :block="commandBlock" @enter="(value) => onEnter(value)" />
      </div>
    </div>
    <!-- END "Body" -->
  </div>
</template>

<script setup lang="ts">
  import { type Ref, nextTick, onMounted, ref } from 'vue';
  import { parseISO } from 'date-fns';
  import { type AxiosError } from 'axios';
  import { YargsCommand, yargs } from '~/composables';
  import { axios, encodePath } from '~/helpers';
  import { blocks as mockBlocks } from '~/mocks';
  import type VWindowBlockHeader from '~/components/VWindowBlockHeader.vue';

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

  blocks.value.push(...mockBlocks);

  /* ----------------------------------------------------------------
  Command Block
  ---------------------------------------------------------------- */
  const commandBlockRef = ref<InstanceType<typeof VWindowBlockHeader>>();

  const commandBlock = ref({
    id: -1,
    workingNode: {
      id: 3,
      path: [] as string[],
    },
    isCommand: true,
    createdAt: new Date(),
  }) as Ref<Block>;

  /* ----------------------------------------------------------------
  Handler
  ---------------------------------------------------------------- */
  function onEnter(value: string) {
    yargs.parse(value, async (err: any, argv: any, output: string) => {
      commandBlock.value.loading = true;

      const block: Block = {
        id: blockCount.value++,
        workingNode: { ...commandBlock.value.workingNode, path: [...commandBlock.value.workingNode.path] },
        command: value,
        parsedArgv: argv,
        data: output.length > 0 ? output : undefined,
        createdAt: new Date(),
      };

      const blockIndex = blocks.value.push(block) - 1;

      if (err) {
        block.error = {
          code: '400',
          message: err.message,
        };
      }
      // API call.
      else if (output.length === 0) {
        await command(blocks.value[blockIndex]);
      }

      // Reset and scroll.
      commandBlock.value.createdAt = new Date();
      commandBlock.value.command = undefined;
      commandBlock.value.loading = false;

      await nextTick();

      if (windowBodyRef.value) {
        windowBodyRef.value.scrollTop = windowBodyRef.value.scrollHeight;
      }

      if (commandBlockRef.value?.inputRef) {
        commandBlockRef.value.inputRef.focus();
      }
    });
  }

  async function command(block: Block) {
    try {
      block.loading = true;

      const argv = block.parsedArgv!;

      switch (argv._[0] as YargsCommand) {
        /* ----------------------------------------------------------------
        Apis
        ---------------------------------------------------------------- */
        case YargsCommand.CD: {
          const res = (
            await axios.post('/api/cd', {
              path: encodePath(block.workingNode.path, argv.FOLDER_PATH),
            })
          ).data;

          block.data = { ...res.data, createdAt: parseISO(res.data.createdAt) };
          commandBlock.value.workingNode = res.data;

          break;
        }
        case YargsCommand.CR: {
          const res = (
            await axios.post('/api/cr', {
              path: encodePath(block.workingNode.path, argv.PATH),
              data: argv.DATA,
              parents_opt: argv.p,
            })
          ).data;

          block.data = { ...res.data, createdAt: parseISO(res.data.createdAt) };

          break;
        }
        case YargsCommand.CAT: {
          const res = (
            await axios.post('/api/cat', {
              path: encodePath(block.workingNode.path, argv.FILE_PATH),
            })
          ).data;

          block.data = res.data;

          break;
        }
        case YargsCommand.LS: {
          const res = (
            await axios.post('/api/ls', {
              path: encodePath(block.workingNode.path, argv.FOLDER_PATH),
            })
          ).data;

          block.data = res.data.map((node: any) => ({ ...node, createdAt: parseISO(node.createdAt) }));

          break;
        }
        case YargsCommand.FIND: {
          const res = (
            await axios.post('/api/find', {
              path: encodePath(block.workingNode.path, argv.FOLDER_PATH),
              name: argv.NAME,
            })
          ).data;

          block.data = res.data.map((node: any) => ({ ...node, createdAt: parseISO(node.createdAt) }));

          break;
        }
        case YargsCommand.UP: {
          const res = (
            await axios.post('/api/up', {
              path: encodePath(block.workingNode.path, argv.PATH),
              name: argv.NAME,
              data: argv.DATA,
            })
          ).data;

          block.data = { ...res.data, createdAt: parseISO(res.data.createdAt) };

          break;
        }
        case YargsCommand.MV: {
          const path = encodePath(block.workingNode.path, argv.PATH);
          const folder_path = encodePath(block.workingNode.path, argv.FOLDER_PATH);

          if (folder_path.join('/').includes(path.join('/'))) {
            throw new Error('cannot move PATH to a subdirectory of itself');
          }

          const res = (
            await axios.post('/api/mv', {
              path: encodePath(block.workingNode.path, argv.PATH),
              folder_path: encodePath(block.workingNode.path, argv.FOLDER_PATH),
            })
          ).data;

          block.data = { ...res.data, createdAt: parseISO(res.data.createdAt) };

          break;
        }
        case YargsCommand.RM: {
          const res = (
            await axios.post('/api/rm', {
              paths: argv.PATHS.map((path: string) => encodePath(block.workingNode.path, path)),
            })
          ).data;

          block.data = res.data;

          break;
        }
        /* ----------------------------------------------------------------
        Extras
        ---------------------------------------------------------------- */
        case YargsCommand.CLEAR: {
          blocks.value = [];

          break;
        }
      }
    } catch (err: any) {
      // AxiosError.
      if (err.response) {
        block.error = err.response.data.error;
      }
      // Error.
      else if (err.message) {
        block.error = {
          code: '400',
          message: err.message,
        };
      }
    } finally {
      block.loading = false;
    }
  }
</script>
