<template>
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
              {{ decodePath(node.path) }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <!-- END "find NAME [FOLDER_PATH]" -->

    <!-- "up PATH NAME [DATA]" -->
    <div v-else-if="block.parsedArgv._[0] === YargsCommand.UP">
      {{ decodePath(block.data.path) }}
    </div>
    <!-- END "up PATH NAME [DATA]" -->

    <!-- "mv PATH FOLDER_PATH" -->
    <div v-else-if="block.parsedArgv._[0] === YargsCommand.MV">
      {{ decodePath(block.data.path) }}
    </div>
    <!-- END "mv PATH FOLDER_PATH" -->

    <!-- "rm PATH [PATH2 PATH3...]" -->
    <div v-else-if="block.parsedArgv._[0] === YargsCommand.RM">
      <p v-for="(path, index) in block.data.nonExistedPaths" :key="index" class="truncate">
        <span class="text-rose-500">File/folder does not exist:</span>&nbsp;<span
          :class="{
            'text-cyan-500 font-bold': path.isFolder,
          }"
          >{{ decodePath(path) }}</span
        >
      </p>
    </div>
    <!-- END "rm PATH [PATH2 PATH3...]" -->
  </template>

  <!-- "Error" -->
  <p v-else-if="block.error" class="text-rose-700">{{ block.error.message }}</p>
  <!-- END "Error" -->
</template>

<script setup lang="ts">
  import { format } from 'date-fns';
  import { YargsCommand } from '~/composables';
  import { decodePath } from '~/helpers';
  import { DateFormat } from '~/constants';

  defineProps<{
    block: Block;
  }>();
</script>
