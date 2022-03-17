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
              :title="normalizePath(block.workingNode.path)"
              class="py-0.5 px-2 flex items-center truncate bg-blue-400 text-gray-900 space-x-2 rounded-l"
            >
              <i-mdi-folder-open class="shrink-0" />

              <span class="truncate">{{ normalizePath(block.workingNode.path) }}</span>
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
      <div>|--</div>

      <div v-if="!block.isCommand" class="py-1 px-2">{{ block.command }}</div>
      <!-- eslint-disable-next-line vue/no-mutating-props -->
      <input
        v-else
        class="py-1 px-2 flex-1 min-w-0"
        :value="block.command"
        autocomplete="off"
        @keydown.enter="event => emit('enter', (event.target as HTMLInputElement).value)"
      />
    </div>
    <!-- END "Line 2" -->
    <!-- END "Header" -->

    <!-- "Body" -->
    <template v-if="!block.error && block.parsedArgv && block.data">
      <!-- "--help || --version" -->
      <div v-if="block.parsedArgv.help || block.parsedArgv.version" class="whitespace-pre-wrap">
        {{ block.data }}
      </div>
      <!-- END "--help || --version" -->

      <!-- "cd FOLDER PATH" -->
      <!-- NO BODY -->
      <!-- END "cd FOLDER PATH" -->

      <!-- "cr [-p] PATH [DATA]" -->
      <div v-else-if="block.parsedArgv._[0] === YargsCommand.CR">
        <span class="text-gray-400">{{ format(block.data.createdAt, 'PP - kk:mm:ss') }}</span>
      </div>
      <!-- END "cr [-p] PATH [DATA]" -->

      <!-- "cat FILE_PATH" -->
      <div v-else-if="block.parsedArgv._[0] === YargsCommand.CAT">
        {{ block.data }}
      </div>
      <!-- END "cat FILE_PATH" -->

      <!-- "ls [FOLDER_PATH]" -->
      <div v-else-if="block.parsedArgv._[0] === YargsCommand.LS">
        <table class="w-full">
          <colgroup>
            <col width="0%" />
            <col width="0%" />
            <col width="100%" />
          </colgroup>
          <tbody>
            <tr v-for="node in block.data" :key="node.id">
              <td class="text-gray-400 whitespace-nowrap">{{ format(node.createdAt, DateFormat.FULL) }}</td>
              <td class="text-amber-600 text-right pl-4 whitespace-nowrap">{{ node.size }}</td>
              <td
                class="truncate pl-4 max-w-[1px]"
                :class="{
                  'text-cyan-500 font-bold': node.isFolder,
                }"
              >
                {{ node.path[node.path.length - 1] }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <!-- END "ls [FOLDER_PATH]" -->

      <!-- "find NAME [FOLDER_PATH]" -->
      <div v-else-if="block.parsedArgv._[0] === YargsCommand.FIND">
        <table class="w-full">
          <colgroup>
            <col width="0%" />
            <col width="100%" />
          </colgroup>
          <tbody>
            <tr v-for="node in block.data" :key="node.id">
              <td class="text-gray-400 whitespace-nowrap">{{ format(node.createdAt, DateFormat.FULL) }}</td>
              <td
                class="pl-4 truncate max-w-[1px]"
                :class="{
                  'text-cyan-500 font-bold': node.isFolder,
                }"
              >
                {{ normalizePath(node.path) }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <!-- END "find NAME [FOLDER_PATH]" -->

      <!-- "up PATH NAME [DATA]" -->
      <div v-else-if="block.parsedArgv._[0] === YargsCommand.UP">
        {{ normalizePath(block.data.path) }}
      </div>
      <!-- END "up PATH NAME [DATA]" -->

      <!-- "mv PATH FOLDER_PATH" -->
      <div v-else-if="block.parsedArgv._[0] === YargsCommand.MV">
        {{ normalizePath(block.data.path) }}
      </div>
      <!-- END "mv PATH FOLDER_PATH" -->

      <!-- "rm PATH [PATH2 PATH3...]" -->
      <div v-else-if="block.parsedArgv._[0] === YargsCommand.RM">
        <p v-for="(path, index) in block.data.nonExistedPaths" :key="index" class="truncate">
          <span class="text-rose-500">File/folder does not exist:</span>&nbsp;<span
            :class="{
              'text-cyan-500 font-bold': path.isFolder,
            }"
            >{{ normalizePath(path) }}</span
          >
        </p>
      </div>
      <!-- END "rm PATH [PATH2 PATH3...]" -->
    </template>

    <!-- "Error" -->
    <p v-else-if="block.error" class="text-rose-500">{{ block.error.message }}</p>
    <!-- END "Error" -->
    <!-- END "Body" -->
  </div>
</template>

<script setup lang="ts">
  import { format } from 'date-fns';
  import { YargsCommand, normalizePath } from '~/composables';
  import { DateFormat } from '~/constants';

  defineProps<{
    block: Block;
  }>();

  const emit = defineEmits<{
    (e: 'enter', value: string): void;
  }>();
</script>
