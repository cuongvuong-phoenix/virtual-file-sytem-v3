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
  import { type Ref, onMounted, ref } from 'vue';

  const windowBodyRef = ref<HTMLDivElement | null>(null);

  onMounted(() => {
    if (windowBodyRef.value) {
      windowBodyRef.value.scrollTop = windowBodyRef.value.scrollHeight;
    }
  });

  /* ----------------------------------------------------------------
  Blocks
  ---------------------------------------------------------------- */
  const blocks = ref([]) as Ref<Block[]>;

  blocks.value.push(
    {
      id: 1,
      workingNode: {
        id: 1,
        path: '/',
      },
      command: 'cd /usr',
      ready: true,
      createdAt: new Date(),
    },
    {
      id: 2,
      workingNode: {
        id: 2,
        path: '/usr',
      },
      command: 'ls',
      ready: true,
      createdAt: new Date(),
    },
    {
      id: 3,
      workingNode: {
        id: 2,
        path: '/usr',
      },
      command: 'cd holistic',
      ready: true,
      createdAt: new Date(),
    },
    {
      id: 4,
      workingNode: {
        id: 3,
        path: '/usr/holistic',
      },
      command: 'find o /zzz',
      error: 'Path does not exists',
      ready: true,
      createdAt: new Date(),
    },
    {
      id: 5,
      workingNode: {
        id: 3,
        path: '/usr/holistic',
      },
      command: 'find o',
      loading: true,
      createdAt: new Date(),
    }
  );

  /* ----------------------------------------------------------------
  Command Block
  ---------------------------------------------------------------- */
  const commandBlock = ref({
    id: 5,
    workingNode: {
      id: 3,
      path: '/usr/holistic',
    },
    isCommand: true,
    createdAt: new Date(),
  }) as Ref<Block>;

  function onEnter(value: string) {
    commandBlock.value.command = value;

    if (windowBodyRef.value) {
      windowBodyRef.value.scrollTop = windowBodyRef.value.scrollHeight;
    }
  }
</script>
