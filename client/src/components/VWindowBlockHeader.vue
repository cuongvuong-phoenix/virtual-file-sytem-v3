<template>
  <!-- "Line 1" -->
  <div class="flex justify-between space-x-8">
    <!-- "Left - Working Directory" -->
    <div class="flex items-center flex-1 min-w-0 space-x-2">
      <div class="flex truncate">
        <div
          :title="decodePath(block.workingNode.path)"
          class="py-0.5 px-2 flex items-center truncate bg-blue-400 text-gray-900 space-x-2 rounded-l"
        >
          <i-mdi-folder-open class="shrink-0" />

          <span class="truncate">{{ decodePath(block.workingNode.path) }}</span>
        </div>

        <VTriangle :height="28" :width="12" color="#60a5fa" />
      </div>

      <template v-if="!block.isCommand">
        <template v-if="!block.loading">
          <i-mdi-check v-if="!block.error && block.data" class="text-teal-500 shrink-0" />
          <i-mdi-close v-else-if="block.error" class="text-rose-500 shrink-0" />
        </template>

        <i-mdi-loading v-else class="text-gray-500 animate-spin shrink-0" />
      </template>
    </div>
    <!-- END "Left - Working Directory" -->

    <!-- "Right - Created Time" -->
    <div v-if="!block.isCommand" class="flex">
      <VTriangle :height="28" :width="12" direction="left" color="#fde68a" />

      <div
        :title="format(block.createdAt, DateFormat.FULL)"
        class="py-0.5 px-2 bg-amber-200 text-gray-900 flex items-center space-x-2 rounded-r"
      >
        <span>
          {{ format(block.createdAt, DateFormat.TIME) }}
        </span>
        <i-mdi-clock-time-eight-outline />
      </div>
    </div>
    <!-- END "Right - Created Time" -->
  </div>
  <!-- END "Line 1" -->

  <!-- "Line 2" -->
  <div class="flex items-center">
    <i-mdi-chevron-right class="text-fuchsia-400 shrink-0" />

    <div v-if="!block.isCommand" class="px-2 py-1">{{ block.command }}</div>
    <input
      v-else
      ref="inputRef"
      v-model="block.command"
      type="text"
      class="flex-1 min-w-0 px-2 py-1"
      autocomplete="off"
      spellcheck="false"
      @keydown.enter="event => emit('enter', (event.target as HTMLInputElement).value)"
    />
  </div>
  <!-- END "Line 2" -->
</template>

<script setup lang="ts">
  import { format } from 'date-fns';
  import { ref } from 'vue';
  import { DateFormat } from '~/constants';
  import { decodePath } from '~/helpers';

  defineProps<{
    block: Block;
  }>();

  const emit = defineEmits<{
    (e: 'enter', value: string): void;
  }>();

  const inputRef = ref<HTMLInputElement | null>(null);

  defineExpose({
    inputRef,
  });
</script>
