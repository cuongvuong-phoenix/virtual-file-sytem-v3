<template>
  <div>
    <!-- "Header" -->
    <!-- "Line 1" -->
    <div class="flex justify-between space-x-8">
      <!-- "Left - Working Directory" -->
      <div class="flex items-center flex-1 min-w-0">
        <div>|--</div>

        <div class="flex items-center flex-1 min-w-0 space-x-2">
          <div class="flex truncate">
            <div
              :title="block.workingNode.path"
              class="py-0.5 px-2 flex items-center truncate bg-blue-400 text-gray-900 space-x-2 rounded-l"
            >
              <i-mdi-folder-open class="shrink-0" />

              <span class="truncate">{{ block.workingNode.path }}</span>
            </div>

            <VTriangle :height="28" :width="12" color="#60a5fa" />
          </div>

          <template v-if="!block.isCommand">
            <template v-if="!block.loading">
              <i-mdi-check v-if="!block.error && block.ready" class="text-teal-500 shrink-0" />
              <i-mdi-close v-else-if="block.error && block.ready" class="text-rose-500 shrink-0" />
            </template>

            <i-mdi-loading v-else class="animate-spin text-gray-500 shrink-0" />
          </template>

          <i-mdi-chevron-right v-else class="text-fuchsia-400 shrink-0" />
        </div>
      </div>
      <!-- END "Left - Working Directory" -->

      <!-- "Right - Created Time" -->
      <div class="flex">
        <VTriangle :height="28" :width="12" direction="left" color="#fde68a" />

        <div
          :title="format(block.createdAt, 'PP - kk:mm:ss')"
          class="py-0.5 px-2 bg-amber-200 text-gray-900 flex items-center space-x-2 rounded-r"
        >
          <span>
            {{ format(block.createdAt, 'kk:mm:ss') }}
          </span>
          <i-mdi-clock-time-eight-outline />
        </div>
      </div>
      <!-- END "Right - Created Time" -->
    </div>
    <!-- END "Line 1" -->

    <!-- "Line 2" -->
    <div class="flex items-center">
      <div>|--</div>

      <div v-if="!block.isCommand" class="py-1 px-2">{{ block.command }}</div>
      <!-- eslint-disable-next-line vue/no-mutating-props -->
      <input
        v-else
        class="py-1 px-2 flex-1 min-w-0"
        autocomplete="off"
        @keydown.enter="event => emit('enter', (event.target as HTMLInputElement).value)"
      />
    </div>
    <!-- END "Line 2" -->
    <!-- END "Header" -->
  </div>
</template>

<script setup lang="ts">
  import { format } from 'date-fns';

  defineProps<{
    block: Block;
  }>();

  const emit = defineEmits<{
    (e: 'enter', value: string): void;
  }>();
</script>
