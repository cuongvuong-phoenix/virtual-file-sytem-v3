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
      <div v-for="block in blocks" :key="block.id">
        <VWindowBlockHeader :block="block" />
        <VWindowBlockBody :block="block" />
      </div>

      <div>
        <VWindowBlockHeader :block="commandBlock" @enter="(value) => onEnter(value)" />
      </div>
    </div>
    <!-- END "Body" -->
  </div>
</template>

<script setup lang="ts">
  import { type Ref, nextTick, onMounted, ref, watch } from 'vue';
  import { yargs } from '~/composables';
  import { blocks as mockBlocks } from '~/mocks';

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
      if (err) {
        return;
      }

      const block: Block = {
        id: blockCount.value++,
        workingNode: commandBlock.value.workingNode,
        command: value,
        parsedArgv: argv,
        data: output.length > 0 ? output : undefined,
        createdAt: new Date(),
      };

      blocks.value.push(block);

      // Reset and scroll.
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
